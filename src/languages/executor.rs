use anyhow::{Context, Result, bail};
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::os::unix::process::CommandExt;
use crate::languages::resolver::{ResolvedVersion, VersionResolver};
use crate::ui::output::Output;

/// Language command executor
pub struct LanguageExecutor {
    language: String,
    command_name: String,
    output: Output,
}

impl LanguageExecutor {
    pub fn new(language: String, command_name: String, output: Output) -> Self {
        Self {
            language,
            command_name,
            output,
        }
    }

    /// Execute the language command with version resolution
    pub async fn execute(&self, args: Vec<String>) -> Result<()> {
        // Check if this is a version override request
        let override_version = self.extract_version_override(&args);

        // Resolve version
        let resolver = VersionResolver::new(self.language.clone(), self.output.clone());
        let resolved = resolver.resolve_version(override_version).await?;

        self.output.debug(&format!(
            "🎯 Resolved {} version: {} ({})",
            self.language, resolved.version, resolved.description
        ));

        // Set up environment variables
        let env_vars = self.setup_environment(&resolved)?;

        // Get the actual executable path
        let executable_path = self.get_executable_path(&resolved)?;

        // Filter out pkmgr-specific arguments
        let filtered_args = self.filter_arguments(args);

        self.output.debug(&format!(
            "🚀 Executing: {} with args: {:?}",
            executable_path.display(),
            filtered_args
        ));

        // Execute with execve() (replaces current process)
        let mut cmd = Command::new(&executable_path);
        cmd.args(&filtered_args);

        // Set environment variables
        for (key, value) in env_vars {
            cmd.env(key, value);
        }

        // On Unix, use exec to replace the current process
        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            let error = cmd.exec();
            // If we reach here, exec failed
            bail!("Failed to execute {}: {}", executable_path.display(), error);
        }

        // On non-Unix platforms, run as child process
        #[cfg(not(unix))]
        {
            let status = cmd.status().context("Failed to execute command")?;
            std::process::exit(status.code().unwrap_or(1));
        }
    }

    /// Extract version override from arguments (--version flag)
    fn extract_version_override(&self, args: &[String]) -> Option<String> {
        for (i, arg) in args.iter().enumerate() {
            if arg == "--version" && i + 1 < args.len() {
                return Some(args[i + 1].clone());
            }
        }
        None
    }

    /// Set up environment variables for the language
    fn setup_environment(&self, resolved: &ResolvedVersion) -> Result<HashMap<String, String>> {
        let mut env_vars = HashMap::new();

        match self.language.as_str() {
            "python" => self.setup_python_env(&mut env_vars, resolved)?,
            "node" => self.setup_node_env(&mut env_vars, resolved)?,
            "ruby" => self.setup_ruby_env(&mut env_vars, resolved)?,
            "rust" => self.setup_rust_env(&mut env_vars, resolved)?,
            "go" => self.setup_go_env(&mut env_vars, resolved)?,
            "php" => self.setup_php_env(&mut env_vars, resolved)?,
            "java" => self.setup_java_env(&mut env_vars, resolved)?,
            "dotnet" => self.setup_dotnet_env(&mut env_vars, resolved)?,
            _ => {},
        }

        Ok(env_vars)
    }

    /// Set up Python environment variables
    fn setup_python_env(&self, env_vars: &mut HashMap<String, String>, resolved: &ResolvedVersion) -> Result<()> {
        if resolved.version != "system" {
            let base_path = &resolved.path;

            // Extract major.minor from version for site-packages path
            let version_parts: Vec<&str> = resolved.version.split('.').collect();
            let major_minor = if version_parts.len() >= 2 {
                format!("{}.{}", version_parts[0], version_parts[1])
            } else {
                resolved.version.clone()
            };

            env_vars.insert("PYTHONPATH".to_string(),
                format!("{}/lib/python{}/site-packages", base_path.display(), major_minor));
            env_vars.insert("PYTHONUSERBASE".to_string(), base_path.display().to_string());
            env_vars.insert("PYTHONNOUSERSITE".to_string(), "1".to_string());
        }
        Ok(())
    }

    /// Set up Node.js environment variables
    fn setup_node_env(&self, env_vars: &mut HashMap<String, String>, resolved: &ResolvedVersion) -> Result<()> {
        if resolved.version != "system" {
            let base_path = &resolved.path;
            env_vars.insert("NODE_PATH".to_string(),
                format!("{}/lib/node_modules", base_path.display()));
            env_vars.insert("NPM_CONFIG_PREFIX".to_string(), base_path.display().to_string());
            env_vars.insert("NPM_CONFIG_USERCONFIG".to_string(),
                format!("{}/.npmrc", base_path.display()));
        }
        Ok(())
    }

    /// Set up Ruby environment variables
    fn setup_ruby_env(&self, env_vars: &mut HashMap<String, String>, resolved: &ResolvedVersion) -> Result<()> {
        if resolved.version != "system" {
            let base_path = &resolved.path;
            env_vars.insert("GEM_HOME".to_string(),
                format!("{}/lib/ruby/gems/{}", base_path.display(), resolved.version));
            env_vars.insert("GEM_PATH".to_string(),
                format!("{}/lib/ruby/gems/{}", base_path.display(), resolved.version));
            env_vars.insert("RUBYLIB".to_string(),
                format!("{}/lib/ruby/{}", base_path.display(), resolved.version));
        }
        Ok(())
    }

    /// Set up Rust environment variables
    fn setup_rust_env(&self, env_vars: &mut HashMap<String, String>, resolved: &ResolvedVersion) -> Result<()> {
        if resolved.version != "system" {
            let base_path = &resolved.path;
            env_vars.insert("RUSTUP_HOME".to_string(), base_path.display().to_string());
            env_vars.insert("CARGO_HOME".to_string(), base_path.display().to_string());
            env_vars.insert("RUSTC".to_string(),
                format!("{}/bin/rustc", base_path.display()));
        }
        Ok(())
    }

    /// Set up Go environment variables
    fn setup_go_env(&self, env_vars: &mut HashMap<String, String>, resolved: &ResolvedVersion) -> Result<()> {
        if resolved.version != "system" {
            let base_path = &resolved.path;
            env_vars.insert("GOROOT".to_string(), base_path.display().to_string());

            // Set GOPATH to user's go directory
            if let Some(home) = dirs::home_dir() {
                env_vars.insert("GOPATH".to_string(),
                    home.join("go").display().to_string());
            }

            env_vars.insert("GOBIN".to_string(),
                format!("{}/bin", base_path.display()));
            env_vars.insert("GO111MODULE".to_string(), "on".to_string());
        }
        Ok(())
    }

    /// Set up PHP environment variables
    fn setup_php_env(&self, env_vars: &mut HashMap<String, String>, resolved: &ResolvedVersion) -> Result<()> {
        if resolved.version != "system" {
            let base_path = &resolved.path;
            env_vars.insert("PHP_INI_DIR".to_string(),
                format!("{}/etc", base_path.display()));
            env_vars.insert("COMPOSER_HOME".to_string(),
                format!("{}/.composer", base_path.display()));
        }
        Ok(())
    }

    /// Set up Java environment variables
    fn setup_java_env(&self, env_vars: &mut HashMap<String, String>, resolved: &ResolvedVersion) -> Result<()> {
        if resolved.version != "system" {
            let base_path = &resolved.path;
            env_vars.insert("JAVA_HOME".to_string(), base_path.display().to_string());
            env_vars.insert("JRE_HOME".to_string(),
                format!("{}/jre", base_path.display()));
            env_vars.insert("CLASSPATH".to_string(),
                format!("{}/lib", base_path.display()));
        }
        Ok(())
    }

    /// Set up .NET environment variables
    fn setup_dotnet_env(&self, env_vars: &mut HashMap<String, String>, resolved: &ResolvedVersion) -> Result<()> {
        if resolved.version != "system" {
            let base_path = &resolved.path;
            env_vars.insert("DOTNET_ROOT".to_string(), base_path.display().to_string());
            env_vars.insert("DOTNET_CLI_HOME".to_string(), base_path.display().to_string());
            env_vars.insert("DOTNET_TOOLS_PATH".to_string(),
                format!("{}/tools", base_path.display()));
        }
        Ok(())
    }

    /// Get the actual executable path for the command
    fn get_executable_path(&self, resolved: &ResolvedVersion) -> Result<PathBuf> {
        if resolved.version == "system" {
            // For system version, use the resolved path directly
            return Ok(resolved.path.clone());
        }

        // For pkmgr-managed versions, construct the binary path
        let binary_name = self.map_command_to_binary();
        let executable_path = resolved.path.join("bin").join(binary_name);

        if !executable_path.exists() {
            bail!("Executable not found: {}", executable_path.display());
        }

        Ok(executable_path)
    }

    /// Map the called command name to the actual binary name
    fn map_command_to_binary(&self) -> &str {
        match self.command_name.as_str() {
            // Python commands
            "python" | "python3" => "python3",
            "pip" | "pip3" => "pip3",

            // Node.js commands
            "node" => "node",
            "npm" => "npm",
            "npx" => "npx",
            "yarn" => "yarn",

            // Ruby commands
            "ruby" => "ruby",
            "gem" => "gem",
            "bundle" => "bundle",
            "irb" => "irb",

            // Rust commands
            "cargo" => "cargo",
            "rustc" => "rustc",
            "rustup" => "rustup",

            // Go commands
            "go" => "go",
            "gofmt" => "gofmt",

            // Java commands
            "java" => "java",
            "javac" => "javac",
            "jar" => "jar",

            // .NET commands
            "dotnet" => "dotnet",

            // PHP commands
            "php" => "php",
            "composer" => "composer",

            // Default to the command name itself
            _ => &self.command_name,
        }
    }

    /// Filter out pkmgr-specific arguments
    fn filter_arguments(&self, args: Vec<String>) -> Vec<String> {
        let mut filtered = Vec::new();
        let mut skip_next = false;

        for arg in args.into_iter().skip(1) { // Skip argv[0] (program name)
            if skip_next {
                skip_next = false;
                continue;
            }

            // Skip pkmgr-specific arguments
            match arg.as_str() {
                "--version" => {
                    skip_next = true; // Skip the version value too
                    continue;
                }
                _ => filtered.push(arg),
            }
        }

        filtered
    }
}