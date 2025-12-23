use console::{style, Color, Term};
use std::io::{self, Write};

#[derive(Clone)]
pub struct Output {
    pub term: Term,
    pub color_enabled: bool,
    pub emoji_enabled: bool,
    pub verbose: bool,
}

impl Output {
    pub fn new(color_output: String, emoji_enabled: bool) -> Self {
        let term = Term::stdout();
        let color_enabled = match color_output.as_str() {
            "always" => true,
            "never" => false,
            "auto" | _ => term.features().colors_supported(),
        };

        Self {
            term,
            color_enabled,
            emoji_enabled,
            verbose: false,
        }
    }

    pub fn with_verbose(color_output: String, emoji_enabled: bool, verbose: bool) -> Self {
        let term = Term::stdout();
        let color_enabled = match color_output.as_str() {
            "always" => true,
            "never" => false,
            "auto" | _ => term.features().colors_supported(),
        };

        Self {
            term,
            color_enabled,
            emoji_enabled,
            verbose,
        }
    }

    pub fn success(&self, message: &str) {
        let prefix = if self.emoji_enabled { "✅" } else { "[OK]" };
        if self.color_enabled {
            println!("{} {}", prefix, style(message).green());
        } else {
            println!("{} {}", prefix, message);
        }
    }

    pub fn error(&self, message: &str) {
        let prefix = if self.emoji_enabled { "❌" } else { "[ERROR]" };
        if self.color_enabled {
            eprintln!("{} {}", prefix, style(message).red());
        } else {
            eprintln!("{} {}", prefix, message);
        }
    }

    pub fn warn(&self, message: &str) {
        let prefix = if self.emoji_enabled { "⚠️" } else { "[WARN]" };
        if self.color_enabled {
            println!("{} {}", prefix, style(message).yellow());
        } else {
            println!("{} {}", prefix, message);
        }
    }

    pub fn info(&self, message: &str) {
        let prefix = if self.emoji_enabled { "ℹ️" } else { "[INFO]" };
        if self.color_enabled {
            println!("{} {}", prefix, style(message).blue());
        } else {
            println!("{} {}", prefix, message);
        }
    }

    pub fn progress(&self, message: &str) {
        let prefix = if self.emoji_enabled { "⏳" } else { "[-]" };
        if self.color_enabled {
            println!("{} {}", prefix, style(message).cyan());
        } else {
            println!("{} {}", prefix, message);
        }
    }

    pub fn step(&self, message: &str) {
        let prefix = if self.emoji_enabled { "🔄" } else { "[>]" };
        if self.color_enabled {
            println!("{} {}", prefix, style(message).magenta());
        } else {
            println!("{} {}", prefix, message);
        }
    }

    pub fn print(&self, message: &str) {
        println!("{}", message);
    }

    pub fn print_header(&self, title: &str) {
        if self.color_enabled {
            println!("\n{}", style(title).bold().underlined());
        } else {
            println!("\n{}", title);
            println!("{}", "=".repeat(title.len()));
        }
    }

    pub fn print_section(&self, title: &str) {
        if self.color_enabled {
            println!("\n{}", style(title).bold());
        } else {
            println!("\n{}", title);
            println!("{}", "-".repeat(title.len()));
        }
    }

    pub fn print_table(&self, headers: &[&str], rows: &[Vec<String>]) {
        if rows.is_empty() {
            return;
        }

        // Calculate column widths
        let mut widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();
        for row in rows {
            for (i, cell) in row.iter().enumerate() {
                if i < widths.len() {
                    widths[i] = widths[i].max(cell.len());
                }
            }
        }

        // Print header
        print!("┏");
        for (i, width) in widths.iter().enumerate() {
            print!("{}", "━".repeat(width + 2));
            if i < widths.len() - 1 {
                print!("┳");
            }
        }
        println!("┓");

        print!("┃");
        for (i, (header, width)) in headers.iter().zip(widths.iter()).enumerate() {
            if self.color_enabled {
                print!(" {} ", style(format!("{:width$}", header, width = width)).bold());
            } else {
                print!(" {:width$} ", header, width = width);
            }
            if i < headers.len() - 1 {
                print!("┃");
            }
        }
        println!("┃");

        // Print separator
        print!("┣");
        for (i, width) in widths.iter().enumerate() {
            print!("{}", "━".repeat(width + 2));
            if i < widths.len() - 1 {
                print!("╋");
            }
        }
        println!("┫");

        // Print rows
        for row in rows {
            print!("┃");
            for (i, (cell, width)) in row.iter().zip(widths.iter()).enumerate() {
                print!(" {:width$} ", cell, width = width);
                if i < row.len() - 1 {
                    print!("┃");
                }
            }
            println!("┃");
        }

        // Print bottom border
        print!("┗");
        for (i, width) in widths.iter().enumerate() {
            print!("{}", "━".repeat(width + 2));
            if i < widths.len() - 1 {
                print!("┻");
            }
        }
        println!("┛");
    }

    pub fn print_list(&self, items: &[String]) {
        for item in items {
            let bullet = if self.emoji_enabled { "📦" } else { "•" };
            println!("  {} {}", bullet, item);
        }
    }

    pub fn clear_line(&self) {
        print!("\r\x1B[2K");
        io::stdout().flush().unwrap();
    }

    pub fn is_tty(&self) -> bool {
        self.term.is_term()
    }

    pub fn width(&self) -> u16 {
        self.term.size().1
    }

    pub fn height(&self) -> u16 {
        self.term.size().0
    }

    // Progress indicators for different operations
    pub fn download_start(&self, name: &str, size: Option<u64>) {
        let icon = if self.emoji_enabled { "📥" } else { "[DL]" };
        if let Some(size) = size {
            self.progress(&format!("Downloading {}: {} bytes", name, size));
        } else {
            self.progress(&format!("Downloading {}", name));
        }
    }

    pub fn install_start(&self, package: &str) {
        let icon = if self.emoji_enabled { "📦" } else { "[INSTALL]" };
        self.progress(&format!("Installing {}", package));
    }

    pub fn remove_start(&self, package: &str) {
        let icon = if self.emoji_enabled { "🗑️" } else { "[REMOVE]" };
        self.progress(&format!("Removing {}", package));
    }

    pub fn update_start(&self, package: &str) {
        let icon = if self.emoji_enabled { "🔄" } else { "[UPDATE]" };
        self.progress(&format!("Updating {}", package));
    }

    pub fn build_start(&self, package: &str) {
        let icon = if self.emoji_enabled { "🏗️" } else { "[BUILD]" };
        self.progress(&format!("Building {}", package));
    }

    pub fn verify_start(&self, item: &str) {
        let icon = if self.emoji_enabled { "🔍" } else { "[VERIFY]" };
        self.progress(&format!("Verifying {}", item));
    }

    pub fn cleanup_start(&self) {
        let icon = if self.emoji_enabled { "🧹" } else { "[CLEANUP]" };
        self.progress("Cleaning up");
    }

    // Alias for print_section to match usage in code
    pub fn section(&self, title: &str) {
        self.print_section(title);
    }

    pub fn debug(&self, message: &str) {
        if self.verbose {
            let prefix = if self.emoji_enabled { "🐛" } else { "[DEBUG]" };
            if self.color_enabled {
                println!("{} {}", prefix, style(message).dim());
            } else {
                println!("{} {}", prefix, message);
            }
        }
    }
}