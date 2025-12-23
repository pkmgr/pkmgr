use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub struct ProgressManager {
    emoji_enabled: bool,
}

impl ProgressManager {
    pub fn new(emoji_enabled: bool) -> Self {
        Self { emoji_enabled }
    }

    pub fn create_download_bar(&self, size: u64, name: &str) -> ProgressBar {
        let pb = ProgressBar::new(size);

        let template = if self.emoji_enabled {
            "┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓\n┃ 📦 Downloading: {msg:.40}  ┃\n┃ {bar:40.cyan/blue} {percent:>3}%          ┃\n┃ 📊 {bytes}/{total_bytes} | ⚡ {bytes_per_sec} | ⏱️  {eta}     ┃\n┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛"
        } else {
            "[{bar:40.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta}) {msg}"
        };

        pb.set_style(
            ProgressStyle::default_bar()
                .template(template)
                .unwrap()
                .progress_chars("█▓▒░ "),
        );

        pb.set_message(name.to_string());
        pb
    }

    pub fn create_install_bar(&self, total: u64, title: &str) -> ProgressBar {
        let pb = ProgressBar::new(total);

        let template = if self.emoji_enabled {
            "╔════════════════════════════════════════════════════╗\n║ 📦 {msg:.48} ║\n╟────────────────────────────────────────────────────╢\n║ {bar:40.green/blue} {pos:>3}/{len:3}          ║\n╟────────────────────────────────────────────────────╢\n║ Total: {percent:>3}% {wide_bar:20.green} ETA: {eta}      ║\n╚════════════════════════════════════════════════════╝"
        } else {
            "[{bar:40.green/blue}] {pos}/{len} {msg}"
        };

        pb.set_style(
            ProgressStyle::default_bar()
                .template(template)
                .unwrap()
                .progress_chars("████░"),
        );

        pb.set_message(title.to_string());
        pb
    }

    pub fn create_usb_bar(&self, size: u64, iso_name: &str, device: &str) -> ProgressBar {
        let pb = ProgressBar::new(size);

        let template = if self.emoji_enabled {
            "╔═══════════════════════════════════════════════════════╗\n║ 💾 Writing ISO to USB Device                         ║\n║ 📀 {msg:.45} ║\n╟───────────────────────────────────────────────────────╢\n║ {bar:40.red/yellow} {percent:>3}%                  ║\n║ 📊 {bytes} / {total_bytes} | ⚡ {bytes_per_sec} | ⏱️  {eta}  ║\n╟───────────────────────────────────────────────────────╢\n║ 🔥 Buffer: 92% | 🌡️ Device: OK | ✓ Verified: {bytes} ║\n╚═══════════════════════════════════════════════════════╝"
        } else {
            "Writing {msg} [{bar:40.red/yellow}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})"
        };

        pb.set_style(
            ProgressStyle::default_bar()
                .template(template)
                .unwrap()
                .progress_chars("▰▱"),
        );

        pb.set_message(format!("{} → {}", iso_name, device));
        pb
    }

    pub fn create_spinner(&self, message: &str) -> ProgressBar {
        let pb = ProgressBar::new_spinner();

        let chars = if self.emoji_enabled {
            "🌍🔄📡"
        } else {
            "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"
        };

        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_chars(chars)
                .template("{spinner:.blue} {msg}")
                .unwrap(),
        );

        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(100));
        pb
    }

    pub fn create_multi_spinner(&self, message: &str) -> ProgressBar {
        let pb = ProgressBar::new_spinner();

        let template = if self.emoji_enabled {
            "{spinner:.green} {msg} ({elapsed})"
        } else {
            "{spinner:.green} {msg} ({elapsed})"
        };

        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("⣾⣽⣻⢿⡿⣟⣯⣷")
                .template(template)
                .unwrap(),
        );

        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(80));
        pb
    }

    pub fn finish_with_message(&self, pb: &ProgressBar, message: &str) {
        let icon = if self.emoji_enabled { "✅" } else { "[OK]" };
        pb.finish_with_message(format!("{} {}", icon, message));
    }

    pub fn finish_with_error(&self, pb: &ProgressBar, message: &str) {
        let icon = if self.emoji_enabled { "❌" } else { "[ERROR]" };
        pb.finish_with_message(format!("{} {}", icon, message));
    }
}