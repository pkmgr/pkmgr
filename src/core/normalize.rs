use anyhow::Result;
use std::collections::HashMap;
use once_cell::sync::Lazy;

use crate::core::platform::Platform;
use crate::core::platform::PackageManager;

/// Package name normalizer for cross-platform compatibility
pub struct PackageNormalizer {
    platform: Platform,
    package_manager: String,
}

impl PackageNormalizer {
    pub fn new(platform: Platform, package_manager: impl Into<String>) -> Self {
        Self {
            platform,
            package_manager: package_manager.into(),
        }
    }

    /// Normalize a package name for the current platform
    pub fn normalize(&self, name: &str) -> Vec<String> {
        let normalized_name = name.to_lowercase();

        // First check exact mappings
        if let Some(mapping) = PACKAGE_MAPPINGS.get(normalized_name.as_str()) {
            if let Some(platform_name) = mapping.get_platform_name(&self.package_manager) {
                return vec![platform_name.to_string()];
            }
        }

        // Try common variations
        let variations = self.get_variations(&normalized_name);

        // If no mapping found, return original and variations
        if variations.is_empty() {
            vec![name.to_string()]
        } else {
            variations
        }
    }

    /// Get common variations of a package name
    fn get_variations(&self, name: &str) -> Vec<String> {
        let mut variations = Vec::new();

        // Original name
        variations.push(name.to_string());

        // Python variations
        if name.starts_with("python") {
            if !name.contains("python3") {
                variations.push(name.replace("python", "python3"));
            }
            if self.package_manager == "apt" || self.package_manager == "dnf" {
                // Add versioned variants
                variations.push(format!("{}-3", name));
                variations.push(format!("python3-{}", name.strip_prefix("python-").unwrap_or(name)));
            }
        }

        // Development package variations
        if !name.ends_with("-dev") && !name.ends_with("-devel") {
            match self.package_manager.as_str() {
                "apt" => variations.push(format!("{}-dev", name)),
                "dnf" | "yum" => variations.push(format!("{}-devel", name)),
                _ => {}
            }
        }

        // Library variations
        if name.starts_with("lib") {
            let without_lib = name.strip_prefix("lib").unwrap_or(name);
            variations.push(without_lib.to_string());
        } else if !name.starts_with("lib") {
            variations.push(format!("lib{}", name));
        }

        // Remove duplicates
        variations.sort();
        variations.dedup();
        variations
    }

    /// Check if a package requires a specific repository
    pub fn requires_repository(&self, name: &str) -> Option<RepositoryRequirement> {
        let normalized = name.to_lowercase();

        // Check repository requirements
        REPOSITORY_REQUIREMENTS.get(normalized.as_str()).cloned()
    }

    /// Get installation notes for a package
    pub fn get_install_notes(&self, name: &str) -> Option<String> {
        let normalized = name.to_lowercase();

        INSTALL_NOTES.get(normalized.as_str()).map(|s| s.to_string())
    }
}

/// Package mapping structure
#[derive(Debug, Clone)]
pub struct PackageMapping {
    pub universal_name: &'static str,
    pub apt: Option<&'static str>,
    pub dnf: Option<&'static str>,
    pub pacman: Option<&'static str>,
    pub brew: Option<&'static str>,
    pub winget: Option<&'static str>,
    pub choco: Option<&'static str>,
    pub apk: Option<&'static str>,
    pub zypper: Option<&'static str>,
}

impl PackageMapping {
    /// Get platform-specific package name
    pub fn get_platform_name(&self, package_manager: &str) -> Option<&'static str> {
        match package_manager {
            "apt" | "apt-get" => self.apt,
            "dnf" | "yum" => self.dnf,
            "pacman" => self.pacman,
            "brew" | "homebrew" => self.brew,
            "winget" => self.winget,
            "choco" | "chocolatey" => self.choco,
            "apk" => self.apk,
            "zypper" => self.zypper,
            _ => None,
        }
    }
}

/// Repository requirement for a package
#[derive(Debug, Clone)]
pub struct RepositoryRequirement {
    pub repository_name: String,
    pub repository_url: Option<String>,
    pub setup_command: Option<String>,
}

// Static package mappings database
static PACKAGE_MAPPINGS: Lazy<HashMap<&'static str, PackageMapping>> = Lazy::new(|| {
    let mut mappings = HashMap::new();

    // Python and related
    mappings.insert("python", PackageMapping {
        universal_name: "python",
        apt: Some("python3"),
        dnf: Some("python3"),
        pacman: Some("python"),
        brew: Some("python@3"),
        winget: Some("Python.Python.3"),
        choco: Some("python"),
        apk: Some("python3"),
        zypper: Some("python3"),
    });

    mappings.insert("pip", PackageMapping {
        universal_name: "pip",
        apt: Some("python3-pip"),
        dnf: Some("python3-pip"),
        pacman: Some("python-pip"),
        brew: Some("python@3"),  // pip comes with python
        winget: Some("Python.Python.3"),
        choco: Some("python"),
        apk: Some("py3-pip"),
        zypper: Some("python3-pip"),
    });

    // Node.js and related
    mappings.insert("node", PackageMapping {
        universal_name: "node",
        apt: Some("nodejs"),
        dnf: Some("nodejs"),
        pacman: Some("nodejs"),
        brew: Some("node"),
        winget: Some("OpenJS.NodeJS"),
        choco: Some("nodejs"),
        apk: Some("nodejs"),
        zypper: Some("nodejs"),
    });

    mappings.insert("nodejs", PackageMapping {
        universal_name: "nodejs",
        apt: Some("nodejs"),
        dnf: Some("nodejs"),
        pacman: Some("nodejs"),
        brew: Some("node"),
        winget: Some("OpenJS.NodeJS"),
        choco: Some("nodejs"),
        apk: Some("nodejs"),
        zypper: Some("nodejs"),
    });

    mappings.insert("npm", PackageMapping {
        universal_name: "npm",
        apt: Some("npm"),
        dnf: Some("npm"),
        pacman: Some("npm"),
        brew: Some("node"),  // npm comes with node
        winget: Some("OpenJS.NodeJS"),
        choco: Some("nodejs"),
        apk: Some("npm"),
        zypper: Some("npm"),
    });

    // Docker
    mappings.insert("docker", PackageMapping {
        universal_name: "docker",
        apt: Some("docker-ce"),
        dnf: Some("docker-ce"),
        pacman: Some("docker"),
        brew: Some("docker"),
        winget: Some("Docker.DockerDesktop"),
        choco: Some("docker-desktop"),
        apk: Some("docker"),
        zypper: Some("docker"),
    });

    // Git
    mappings.insert("git", PackageMapping {
        universal_name: "git",
        apt: Some("git"),
        dnf: Some("git"),
        pacman: Some("git"),
        brew: Some("git"),
        winget: Some("Git.Git"),
        choco: Some("git"),
        apk: Some("git"),
        zypper: Some("git"),
    });

    // VS Code
    mappings.insert("vscode", PackageMapping {
        universal_name: "vscode",
        apt: Some("code"),
        dnf: Some("code"),
        pacman: Some("visual-studio-code-bin"),
        brew: Some("visual-studio-code"),
        winget: Some("Microsoft.VisualStudioCode"),
        choco: Some("vscode"),
        apk: None,
        zypper: Some("code"),
    });

    mappings.insert("code", PackageMapping {
        universal_name: "code",
        apt: Some("code"),
        dnf: Some("code"),
        pacman: Some("visual-studio-code-bin"),
        brew: Some("visual-studio-code"),
        winget: Some("Microsoft.VisualStudioCode"),
        choco: Some("vscode"),
        apk: None,
        zypper: Some("code"),
    });

    // Chrome
    mappings.insert("chrome", PackageMapping {
        universal_name: "chrome",
        apt: Some("google-chrome-stable"),
        dnf: Some("google-chrome-stable"),
        pacman: Some("google-chrome"),
        brew: Some("google-chrome"),
        winget: Some("Google.Chrome"),
        choco: Some("googlechrome"),
        apk: None,
        zypper: Some("google-chrome-stable"),
    });

    // Build tools
    mappings.insert("gcc", PackageMapping {
        universal_name: "gcc",
        apt: Some("build-essential"),
        dnf: Some("gcc"),
        pacman: Some("gcc"),
        brew: Some("gcc"),
        winget: None,
        choco: Some("mingw"),
        apk: Some("gcc"),
        zypper: Some("gcc"),
    });

    mappings.insert("make", PackageMapping {
        universal_name: "make",
        apt: Some("build-essential"),
        dnf: Some("make"),
        pacman: Some("make"),
        brew: Some("make"),
        winget: None,
        choco: Some("make"),
        apk: Some("make"),
        zypper: Some("make"),
    });

    mappings.insert("build-essential", PackageMapping {
        universal_name: "build-essential",
        apt: Some("build-essential"),
        dnf: Some("@development-tools"),
        pacman: Some("base-devel"),
        brew: None,  // Xcode Command Line Tools
        winget: None,
        choco: Some("mingw"),
        apk: Some("build-base"),
        zypper: Some("devel_basis"),
    });

    // Databases
    mappings.insert("mysql", PackageMapping {
        universal_name: "mysql",
        apt: Some("mysql-server"),
        dnf: Some("mysql-server"),
        pacman: Some("mariadb"),
        brew: Some("mysql"),
        winget: Some("Oracle.MySQL"),
        choco: Some("mysql"),
        apk: Some("mysql"),
        zypper: Some("mysql-community-server"),
    });

    mappings.insert("postgresql", PackageMapping {
        universal_name: "postgresql",
        apt: Some("postgresql"),
        dnf: Some("postgresql-server"),
        pacman: Some("postgresql"),
        brew: Some("postgresql@16"),
        winget: Some("PostgreSQL.PostgreSQL"),
        choco: Some("postgresql"),
        apk: Some("postgresql"),
        zypper: Some("postgresql-server"),
    });

    mappings.insert("postgres", PackageMapping {
        universal_name: "postgres",
        apt: Some("postgresql"),
        dnf: Some("postgresql-server"),
        pacman: Some("postgresql"),
        brew: Some("postgresql@16"),
        winget: Some("PostgreSQL.PostgreSQL"),
        choco: Some("postgresql"),
        apk: Some("postgresql"),
        zypper: Some("postgresql-server"),
    });

    mappings.insert("redis", PackageMapping {
        universal_name: "redis",
        apt: Some("redis-server"),
        dnf: Some("redis"),
        pacman: Some("redis"),
        brew: Some("redis"),
        winget: Some("Redis.Redis"),
        choco: Some("redis-64"),
        apk: Some("redis"),
        zypper: Some("redis"),
    });

    // Web servers
    mappings.insert("nginx", PackageMapping {
        universal_name: "nginx",
        apt: Some("nginx"),
        dnf: Some("nginx"),
        pacman: Some("nginx"),
        brew: Some("nginx"),
        winget: None,
        choco: Some("nginx"),
        apk: Some("nginx"),
        zypper: Some("nginx"),
    });

    mappings.insert("apache", PackageMapping {
        universal_name: "apache",
        apt: Some("apache2"),
        dnf: Some("httpd"),
        pacman: Some("apache"),
        brew: Some("httpd"),
        winget: Some("ApacheFriends.Xampp"),
        choco: Some("apache-httpd"),
        apk: Some("apache2"),
        zypper: Some("apache2"),
    });

    mappings.insert("httpd", PackageMapping {
        universal_name: "httpd",
        apt: Some("apache2"),
        dnf: Some("httpd"),
        pacman: Some("apache"),
        brew: Some("httpd"),
        winget: Some("ApacheFriends.Xampp"),
        choco: Some("apache-httpd"),
        apk: Some("apache2"),
        zypper: Some("apache2"),
    });

    // Programming languages
    mappings.insert("rust", PackageMapping {
        universal_name: "rust",
        apt: Some("rustc"),
        dnf: Some("rust"),
        pacman: Some("rust"),
        brew: Some("rust"),
        winget: Some("Rustlang.Rust.MSVC"),
        choco: Some("rust"),
        apk: Some("rust"),
        zypper: Some("rust"),
    });

    mappings.insert("go", PackageMapping {
        universal_name: "go",
        apt: Some("golang"),
        dnf: Some("golang"),
        pacman: Some("go"),
        brew: Some("go"),
        winget: Some("GoLang.Go"),
        choco: Some("golang"),
        apk: Some("go"),
        zypper: Some("go"),
    });

    mappings.insert("golang", PackageMapping {
        universal_name: "golang",
        apt: Some("golang"),
        dnf: Some("golang"),
        pacman: Some("go"),
        brew: Some("go"),
        winget: Some("GoLang.Go"),
        choco: Some("golang"),
        apk: Some("go"),
        zypper: Some("go"),
    });

    mappings.insert("ruby", PackageMapping {
        universal_name: "ruby",
        apt: Some("ruby-full"),
        dnf: Some("ruby"),
        pacman: Some("ruby"),
        brew: Some("ruby"),
        winget: Some("RubyInstallerTeam.Ruby"),
        choco: Some("ruby"),
        apk: Some("ruby"),
        zypper: Some("ruby"),
    });

    mappings.insert("java", PackageMapping {
        universal_name: "java",
        apt: Some("default-jdk"),
        dnf: Some("java-latest-openjdk"),
        pacman: Some("jdk-openjdk"),
        brew: Some("openjdk"),
        winget: Some("Oracle.JDK.21"),
        choco: Some("openjdk"),
        apk: Some("openjdk17"),
        zypper: Some("java-17-openjdk"),
    });

    mappings.insert("php", PackageMapping {
        universal_name: "php",
        apt: Some("php"),
        dnf: Some("php"),
        pacman: Some("php"),
        brew: Some("php"),
        winget: None,
        choco: Some("php"),
        apk: Some("php"),
        zypper: Some("php"),
    });

    // Utilities
    mappings.insert("curl", PackageMapping {
        universal_name: "curl",
        apt: Some("curl"),
        dnf: Some("curl"),
        pacman: Some("curl"),
        brew: Some("curl"),
        winget: Some("cURL.cURL"),
        choco: Some("curl"),
        apk: Some("curl"),
        zypper: Some("curl"),
    });

    mappings.insert("wget", PackageMapping {
        universal_name: "wget",
        apt: Some("wget"),
        dnf: Some("wget"),
        pacman: Some("wget"),
        brew: Some("wget"),
        winget: Some("GNU.Wget"),
        choco: Some("wget"),
        apk: Some("wget"),
        zypper: Some("wget"),
    });

    mappings.insert("vim", PackageMapping {
        universal_name: "vim",
        apt: Some("vim"),
        dnf: Some("vim"),
        pacman: Some("vim"),
        brew: Some("vim"),
        winget: Some("vim.vim"),
        choco: Some("vim"),
        apk: Some("vim"),
        zypper: Some("vim"),
    });

    mappings.insert("emacs", PackageMapping {
        universal_name: "emacs",
        apt: Some("emacs"),
        dnf: Some("emacs"),
        pacman: Some("emacs"),
        brew: Some("emacs"),
        winget: Some("GNU.Emacs"),
        choco: Some("emacs"),
        apk: Some("emacs"),
        zypper: Some("emacs"),
    });

    mappings.insert("htop", PackageMapping {
        universal_name: "htop",
        apt: Some("htop"),
        dnf: Some("htop"),
        pacman: Some("htop"),
        brew: Some("htop"),
        winget: None,
        choco: Some("htop"),
        apk: Some("htop"),
        zypper: Some("htop"),
    });

    mappings.insert("tmux", PackageMapping {
        universal_name: "tmux",
        apt: Some("tmux"),
        dnf: Some("tmux"),
        pacman: Some("tmux"),
        brew: Some("tmux"),
        winget: None,
        choco: Some("tmux"),
        apk: Some("tmux"),
        zypper: Some("tmux"),
    });

    mappings.insert("zsh", PackageMapping {
        universal_name: "zsh",
        apt: Some("zsh"),
        dnf: Some("zsh"),
        pacman: Some("zsh"),
        brew: Some("zsh"),
        winget: None,
        choco: Some("zsh"),
        apk: Some("zsh"),
        zypper: Some("zsh"),
    });

    mappings.insert("fish", PackageMapping {
        universal_name: "fish",
        apt: Some("fish"),
        dnf: Some("fish"),
        pacman: Some("fish"),
        brew: Some("fish"),
        winget: None,
        choco: Some("fish"),
        apk: Some("fish"),
        zypper: Some("fish"),
    });

    mappings
});

// Repository requirements for packages
static REPOSITORY_REQUIREMENTS: Lazy<HashMap<&'static str, RepositoryRequirement>> = Lazy::new(|| {
    let mut requirements = HashMap::new();

    requirements.insert("docker-ce", RepositoryRequirement {
        repository_name: "Docker".to_string(),
        repository_url: Some("https://download.docker.com/linux/".to_string()),
        setup_command: Some("pkmgr repos add docker".to_string()),
    });

    requirements.insert("code", RepositoryRequirement {
        repository_name: "Microsoft VSCode".to_string(),
        repository_url: Some("https://packages.microsoft.com/repos/code".to_string()),
        setup_command: Some("pkmgr repos add vscode".to_string()),
    });

    requirements.insert("google-chrome-stable", RepositoryRequirement {
        repository_name: "Google Chrome".to_string(),
        repository_url: Some("https://dl.google.com/linux/chrome/deb/".to_string()),
        setup_command: Some("pkmgr repos add chrome".to_string()),
    });

    requirements.insert("postgresql-16", RepositoryRequirement {
        repository_name: "PostgreSQL PGDG".to_string(),
        repository_url: Some("https://apt.postgresql.org/pub/repos/apt".to_string()),
        setup_command: Some("pkmgr repos add postgresql".to_string()),
    });

    requirements.insert("mongodb-org", RepositoryRequirement {
        repository_name: "MongoDB".to_string(),
        repository_url: Some("https://repo.mongodb.org/".to_string()),
        setup_command: Some("pkmgr repos add mongodb".to_string()),
    });

    requirements
});

// Special installation notes
static INSTALL_NOTES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut notes = HashMap::new();

    notes.insert("docker", "After installation, add your user to the docker group: sudo usermod -aG docker $USER");
    notes.insert("mysql", "MySQL has been replaced with MariaDB on many distributions. Consider using 'mariadb' instead.");
    notes.insert("python", "Python 2 is deprecated. This will install Python 3.");
    notes.insert("node", "Consider using pkmgr's built-in version management: pkmgr node install <version>");
    notes.insert("java", "Multiple JDK versions available. This installs the default/latest version.");
    notes.insert("apache", "Apache is called 'httpd' on RedHat-based systems and 'apache2' on Debian-based systems.");
    notes.insert("postgresql", "PostgreSQL may require initialization: sudo postgresql-setup --initdb");

    notes
});