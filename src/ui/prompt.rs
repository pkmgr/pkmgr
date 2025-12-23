use anyhow::Result;
use dialoguer::{Confirm, Input, MultiSelect, Select};

pub struct Prompt {
    emoji_enabled: bool,
}

impl Prompt {
    pub fn new(emoji_enabled: bool) -> Self {
        Self { emoji_enabled }
    }

    pub fn confirm(&self, message: &str) -> Result<bool> {
        let prompt = if self.emoji_enabled {
            format!("❓ {}", message)
        } else {
            format!("[?] {}", message)
        };

        Ok(Confirm::new()
            .with_prompt(prompt)
            .default(false)
            .interact()?)
    }

    pub fn confirm_default_yes(&self, message: &str) -> Result<bool> {
        let prompt = if self.emoji_enabled {
            format!("❓ {}", message)
        } else {
            format!("[?] {}", message)
        };

        Ok(Confirm::new()
            .with_prompt(prompt)
            .default(true)
            .interact()?)
    }

    pub fn input(&self, message: &str) -> Result<String> {
        let prompt = if self.emoji_enabled {
            format!("📝 {}", message)
        } else {
            format!("[INPUT] {}", message)
        };

        Ok(Input::new()
            .with_prompt(prompt)
            .interact()?)
    }

    pub fn input_with_default(&self, message: &str, default: &str) -> Result<String> {
        let prompt = if self.emoji_enabled {
            format!("📝 {}", message)
        } else {
            format!("[INPUT] {}", message)
        };

        Ok(Input::new()
            .with_prompt(prompt)
            .default(default.to_string())
            .interact()?)
    }

    pub fn select<T>(&self, message: &str, items: &[T]) -> Result<usize>
    where
        T: ToString,
    {
        let prompt = if self.emoji_enabled {
            format!("🎯 {}", message)
        } else {
            format!("[SELECT] {}", message)
        };

        Ok(Select::new()
            .with_prompt(prompt)
            .items(items)
            .default(0)
            .interact()?)
    }

    pub fn multiselect<T>(&self, message: &str, items: &[T]) -> Result<Vec<usize>>
    where
        T: ToString,
    {
        let prompt = if self.emoji_enabled {
            format!("☑️ {}", message)
        } else {
            format!("[MULTISELECT] {}", message)
        };

        Ok(MultiSelect::new()
            .with_prompt(prompt)
            .items(items)
            .interact()?)
    }

    pub fn destructive_confirm(&self, message: &str, confirmation_text: &str) -> Result<bool> {
        let warning = if self.emoji_enabled {
            format!("⚠️ DESTRUCTIVE OPERATION")
        } else {
            format!("[WARNING] DESTRUCTIVE OPERATION")
        };

        println!("{}", warning);
        println!("{}", message);
        println!();

        let input: String = Input::new()
            .with_prompt(format!("Type '{}' to confirm", confirmation_text))
            .interact()?;

        Ok(input == confirmation_text)
    }

    pub fn version_select(&self, versions: &[String], current: Option<&str>) -> Result<usize> {
        let title = if self.emoji_enabled {
            "🐍 Select Version"
        } else {
            "Select Version"
        };

        println!("╔════════════════════════════════════════════╗");
        println!("║ {} {:28} ║", title, "");
        println!("╟────────────────────────────────────────────╢");

        for (i, version) in versions.iter().enumerate() {
            let marker = if Some(version.as_str()) == current {
                if self.emoji_enabled { "✅" } else { "*" }
            } else {
                " "
            };

            let status = if i == 0 {
                if self.emoji_enabled { "✨ NEW" } else { "(latest)" }
            } else if version.contains("lts") || version.contains("LTS") {
                if self.emoji_enabled { "🏷️ STABLE" } else { "(stable)" }
            } else {
                ""
            };

            println!("║ {} {} {:<20} {:<10} ║",
                if i == 0 { "→" } else { " " },
                marker,
                version,
                status
            );
        }

        println!("╟────────────────────────────────────────────╢");
        if let Some(current) = current {
            println!("║ 📝 Current: {} | 🎯 Project: auto   ║", current);
        }
        println!("║ Use ↑↓ to navigate, Enter to select       ║");
        println!("╚════════════════════════════════════════════╝");

        Ok(Select::new()
            .items(versions)
            .default(0)
            .interact()?)
    }

    pub fn usb_device_select(&self, devices: &[(String, String, u64)]) -> Result<usize> {
        let title = if self.emoji_enabled {
            "💾 USB Device Setup Wizard"
        } else {
            "USB Device Setup Wizard"
        };

        println!("╔════════════════════════════════════════════════════════╗");
        println!("║ {} {:35} ║", title, "");
        println!("╟────────────────────────────────────────────────────────╢");
        println!("║ 🔍 Detected USB devices: {:27} ║", "");
        println!("║ {:54} ║", "");

        for (i, (device, name, size)) in devices.iter().enumerate() {
            let size_gb = *size as f64 / 1_000_000_000.0;
            println!("║ {}. {} - {} {:10} ║",
                i + 1,
                device,
                name,
                format!("({:.1} GB)", size_gb)
            );
            println!("║    └─ {:.1} GB available, currently formatted ║", size_gb);
        }

        println!("║ {:54} ║", "");
        println!("║ R. 🔄 Refresh device list {:20} ║", "");
        println!("║ Q. ❌ Quit wizard {:28} ║", "");
        println!("╟────────────────────────────────────────────────────────╢");
        println!("║ Select device [1-{}]: {:26} ║", devices.len(), "");
        println!("╚════════════════════════════════════════════════════════╝");

        let items: Vec<String> = devices.iter()
            .map(|(dev, name, size)| format!("{} - {} ({:.1} GB)", dev, name, *size as f64 / 1_000_000_000.0))
            .collect();

        Ok(Select::new()
            .items(&items)
            .default(0)
            .interact()?)
    }
}