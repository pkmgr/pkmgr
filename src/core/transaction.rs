use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub operation: String,
    pub status: TransactionStatus,
    pub packages: TransactionPackages,
    pub files: TransactionFiles,
    pub repositories: TransactionRepositories,
    pub config_backup: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionStatus {
    InProgress,
    Completed,
    Failed,
    RollingBack,
    RolledBack,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionPackages {
    pub installed: Vec<String>,
    pub removed: Vec<String>,
    pub upgraded: Vec<(String, String)>, // (package, old_version -> new_version)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionFiles {
    pub created: Vec<String>,
    pub modified: Vec<String>,
    pub removed: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionRepositories {
    pub added: Vec<String>,
    pub removed: Vec<String>,
}

impl Transaction {
    pub fn new(operation: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            operation,
            status: TransactionStatus::InProgress,
            packages: TransactionPackages {
                installed: Vec::new(),
                removed: Vec::new(),
                upgraded: Vec::new(),
            },
            files: TransactionFiles {
                created: Vec::new(),
                modified: Vec::new(),
                removed: Vec::new(),
            },
            repositories: TransactionRepositories {
                added: Vec::new(),
                removed: Vec::new(),
            },
            config_backup: HashMap::new(),
        }
    }

    pub async fn save(&self, data_dir: &PathBuf) -> Result<()> {
        let transactions_dir = data_dir.join("transactions");
        tokio::fs::create_dir_all(&transactions_dir).await?;

        let transaction_file = transactions_dir.join(format!("{}.toml", self.id));
        let content = toml::to_string_pretty(self)?;
        tokio::fs::write(transaction_file, content).await?;

        // Also save as current transaction if in progress
        if matches!(self.status, TransactionStatus::InProgress) {
            let current_file = transactions_dir.join("current.toml");
            let content = toml::to_string_pretty(self)?;
            tokio::fs::write(current_file, content).await?;
        }

        Ok(())
    }

    pub async fn load(data_dir: &PathBuf, id: &str) -> Result<Option<Self>> {
        let transaction_file = data_dir.join("transactions").join(format!("{}.toml", id));

        if !transaction_file.exists() {
            return Ok(None);
        }

        let content = tokio::fs::read_to_string(transaction_file).await?;
        let transaction: Transaction = toml::from_str(&content)?;
        Ok(Some(transaction))
    }

    pub async fn load_current(data_dir: &PathBuf) -> Result<Option<Self>> {
        let current_file = data_dir.join("transactions").join("current.toml");

        if !current_file.exists() {
            return Ok(None);
        }

        let content = tokio::fs::read_to_string(current_file).await?;
        let transaction: Transaction = toml::from_str(&content)?;
        Ok(Some(transaction))
    }

    pub fn complete(&mut self) {
        self.status = TransactionStatus::Completed;
    }

    pub fn fail(&mut self) {
        self.status = TransactionStatus::Failed;
    }

    pub fn start_rollback(&mut self) {
        self.status = TransactionStatus::RollingBack;
    }

    pub fn complete_rollback(&mut self) {
        self.status = TransactionStatus::RolledBack;
    }

    pub fn add_installed_package(&mut self, package: String) {
        self.packages.installed.push(package);
    }

    pub fn add_removed_package(&mut self, package: String) {
        self.packages.removed.push(package);
    }

    pub fn add_upgraded_package(&mut self, package: String, old_version: String, new_version: String) {
        self.packages.upgraded.push((package, format!("{} -> {}", old_version, new_version)));
    }

    pub fn add_created_file(&mut self, file: String) {
        self.files.created.push(file);
    }

    pub fn add_modified_file(&mut self, file: String) {
        self.files.modified.push(file);
    }

    pub fn add_removed_file(&mut self, file: String) {
        self.files.removed.push(file);
    }

    pub fn add_repository(&mut self, repo: String) {
        self.repositories.added.push(repo);
    }

    pub fn remove_repository(&mut self, repo: String) {
        self.repositories.removed.push(repo);
    }

    pub fn backup_config(&mut self, original_path: String, backup_path: String) {
        self.config_backup.insert(original_path, backup_path);
    }
}

pub struct TransactionManager {
    data_dir: PathBuf,
    current_transaction: Option<Transaction>,
}

impl TransactionManager {
    pub fn new(data_dir: PathBuf) -> Self {
        Self {
            data_dir,
            current_transaction: None,
        }
    }

    pub async fn start_transaction(&mut self, operation: String) -> Result<()> {
        // Complete any existing transaction first
        if let Some(mut current) = self.current_transaction.take() {
            current.fail();
            current.save(&self.data_dir).await?;
        }

        // Start new transaction
        let transaction = Transaction::new(operation);
        transaction.save(&self.data_dir).await?;
        self.current_transaction = Some(transaction);

        Ok(())
    }

    pub async fn complete_transaction(&mut self) -> Result<()> {
        if let Some(mut transaction) = self.current_transaction.take() {
            transaction.complete();
            transaction.save(&self.data_dir).await?;

            // Remove current transaction file
            let current_file = self.data_dir.join("transactions").join("current.toml");
            if current_file.exists() {
                tokio::fs::remove_file(current_file).await?;
            }
        }

        Ok(())
    }

    pub async fn fail_transaction(&mut self) -> Result<()> {
        if let Some(mut transaction) = self.current_transaction.take() {
            transaction.fail();
            transaction.save(&self.data_dir).await?;

            // Keep current transaction file for potential rollback
        }

        Ok(())
    }

    pub async fn rollback_transaction(&mut self, transaction_id: &str) -> Result<bool> {
        let transaction = Transaction::load(&self.data_dir, transaction_id).await?;

        if let Some(mut transaction) = transaction {
            transaction.start_rollback();
            transaction.save(&self.data_dir).await?;

            // TODO: Implement actual rollback logic
            // This would involve:
            // 1. Restoring configuration files from backup
            // 2. Removing newly installed packages
            // 3. Reinstalling removed packages
            // 4. Removing added repositories
            // 5. Cleaning temporary files

            transaction.complete_rollback();
            transaction.save(&self.data_dir).await?;

            return Ok(true);
        }

        Ok(false)
    }

    pub fn current_transaction(&self) -> Option<&Transaction> {
        self.current_transaction.as_ref()
    }

    pub fn current_transaction_mut(&mut self) -> Option<&mut Transaction> {
        self.current_transaction.as_mut()
    }
}