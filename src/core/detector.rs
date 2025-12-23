use std::collections::HashMap;

#[derive(Debug)]
pub struct SymlinkDetector {
    language_map: HashMap<&'static str, &'static str>,
}

impl SymlinkDetector {
    pub fn new() -> Self {
        let mut language_map = HashMap::new();

        // Python
        language_map.insert("python", "python");
        language_map.insert("python3", "python");
        language_map.insert("pip", "python");
        language_map.insert("pip3", "python");

        // Node.js
        language_map.insert("node", "node");
        language_map.insert("npm", "node");
        language_map.insert("npx", "node");
        language_map.insert("yarn", "node");

        // Ruby
        language_map.insert("ruby", "ruby");
        language_map.insert("gem", "ruby");
        language_map.insert("bundle", "ruby");
        language_map.insert("irb", "ruby");

        // Rust
        language_map.insert("cargo", "rust");
        language_map.insert("rustc", "rust");
        language_map.insert("rustup", "rust");

        // Go
        language_map.insert("go", "go");
        language_map.insert("gofmt", "go");

        // Java
        language_map.insert("java", "java");
        language_map.insert("javac", "java");
        language_map.insert("jar", "java");

        // .NET
        language_map.insert("dotnet", "dotnet");

        // PHP
        language_map.insert("php", "php");
        language_map.insert("composer", "php");

        Self { language_map }
    }

    pub fn detect_language(&self, program_name: &str) -> Option<&'static str> {
        // Skip if this is the actual pkmgr binary
        if program_name == "pkmgr" {
            return None;
        }

        self.language_map.get(program_name).copied()
    }

    pub fn is_language_command(&self, program_name: &str) -> bool {
        self.detect_language(program_name).is_some()
    }
}

impl Default for SymlinkDetector {
    fn default() -> Self {
        Self::new()
    }
}