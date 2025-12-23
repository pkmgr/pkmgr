use anyhow::Result;
use crate::shell::ShellType;
use crate::ui::output::Output;
use std::path::PathBuf;

pub struct ShellIntegration {
    shell: ShellType,
    output: Output,
}

impl ShellIntegration {
    pub fn new(shell: ShellType, output: Output) -> Self {
        Self { shell, output }
    }

    /// Generate shell integration script
    pub fn generate_script(&self) -> String {
        match self.shell {
            ShellType::Bash => self.bash_script(),
            ShellType::Zsh => self.zsh_script(),
            ShellType::Fish => self.fish_script(),
            ShellType::PowerShell => self.powershell_script(),
            ShellType::Nushell => self.nushell_script(),
            ShellType::Unknown => "# Shell type could not be detected\n".to_string(),
        }
    }

    /// Generate PATH update script
    pub fn generate_path_script(&self, add: bool) -> String {
        let local_bin = dirs::home_dir()
            .map(|h| h.join(".local").join("bin"))
            .unwrap_or_else(|| PathBuf::from("~/.local/bin"));

        let path_str = local_bin.to_string_lossy();

        match self.shell {
            ShellType::Bash | ShellType::Zsh => {
                if add {
                    format!(
                        r#"
# Add ~/.local/bin to PATH if not already present
if [[ ":$PATH:" != *":{}:"* ]]; then
    export PATH="{}:$PATH"
fi
"#,
                        path_str, path_str
                    )
                } else {
                    format!(
                        r#"
# Remove ~/.local/bin from PATH
export PATH=$(echo "$PATH" | sed 's|{}:||g' | sed 's|:{}||g')
"#,
                        path_str, path_str
                    )
                }
            }
            ShellType::Fish => {
                if add {
                    format!(
                        r#"
# Add ~/.local/bin to PATH if not already present
if not contains {} $PATH
    set -gx PATH {} $PATH
end
"#,
                        path_str, path_str
                    )
                } else {
                    format!(
                        r#"
# Remove ~/.local/bin from PATH
set -e PATH[contains -i {} $PATH]
"#,
                        path_str
                    )
                }
            }
            ShellType::PowerShell => {
                if add {
                    format!(
                        r#"
# Add ~/.local/bin to PATH if not already present
if ($env:PATH -notlike "*{}*") {{
    $env:PATH = "{};$env:PATH"
}}
"#,
                        path_str, path_str
                    )
                } else {
                    format!(
                        r#"
# Remove ~/.local/bin from PATH
$env:PATH = $env:PATH -replace '{}[;:]?', ''
"#,
                        path_str
                    )
                }
            }
            ShellType::Nushell => {
                if add {
                    format!(
                        r#"
# Add ~/.local/bin to PATH
let-env PATH = ($env.PATH | prepend {})
"#,
                        path_str
                    )
                } else {
                    format!(
                        r#"
# Remove ~/.local/bin from PATH
let-env PATH = ($env.PATH | where $it != {})
"#,
                        path_str
                    )
                }
            }
            ShellType::Unknown => "# Unknown shell\n".to_string(),
        }
    }

    /// Bash integration script
    fn bash_script(&self) -> String {
        r#"
# pkmgr Bash Integration
# Add this to your ~/.bashrc

# Add ~/.local/bin to PATH if not already present
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
    export PATH="$HOME/.local/bin:$PATH"
fi

# pkmgr environment variables
export PKMGR_SHELL="bash"

# Language version detection
_pkmgr_detect_version() {
    local lang="$1"
    if [ -f ".${lang}-version" ]; then
        cat ".${lang}-version"
    elif [ -f ".tool-versions" ]; then
        grep "^$lang " .tool-versions | awk '{print $2}'
    fi
}

# Python wrapper
python() {
    local version=$(_pkmgr_detect_version "python")
    if [ -n "$version" ]; then
        PKMGR_PYTHON_VERSION="$version" command pkmgr python "$@"
    else
        command pkmgr python "$@"
    fi
}

python3() { python "$@"; }
pip() { command pkmgr python -m pip "$@"; }
pip3() { pip "$@"; }

# Node.js wrapper
node() {
    local version=$(_pkmgr_detect_version "node")
    if [ -n "$version" ]; then
        PKMGR_NODE_VERSION="$version" command pkmgr node "$@"
    else
        command pkmgr node "$@"
    fi
}

npm() { command pkmgr node npm "$@"; }
yarn() { command pkmgr node yarn "$@"; }
pnpm() { command pkmgr node pnpm "$@"; }

# Ruby wrapper
ruby() {
    local version=$(_pkmgr_detect_version "ruby")
    if [ -n "$version" ]; then
        PKMGR_RUBY_VERSION="$version" command pkmgr ruby "$@"
    else
        command pkmgr ruby "$@"
    fi
}

gem() { command pkmgr ruby gem "$@"; }
bundle() { command pkmgr ruby bundle "$@"; }

# Go wrapper
go() {
    local version=$(_pkmgr_detect_version "go")
    if [ -n "$version" ]; then
        PKMGR_GO_VERSION="$version" command pkmgr go "$@"
    else
        command pkmgr go "$@"
    fi
}

# Rust wrapper
rustc() { command pkmgr rust rustc "$@"; }
cargo() { command pkmgr rust cargo "$@"; }
rustup() { command pkmgr rust rustup "$@"; }

# Java wrapper
java() {
    local version=$(_pkmgr_detect_version "java")
    if [ -n "$version" ]; then
        PKMGR_JAVA_VERSION="$version" command pkmgr java "$@"
    else
        command pkmgr java "$@"
    fi
}

javac() { command pkmgr java javac "$@"; }
mvn() { command pkmgr java mvn "$@"; }
gradle() { command pkmgr java gradle "$@"; }

# .NET wrapper
dotnet() { command pkmgr dotnet "$@"; }

# PHP wrapper
php() {
    local version=$(_pkmgr_detect_version "php")
    if [ -n "$version" ]; then
        PKMGR_PHP_VERSION="$version" command pkmgr php "$@"
    else
        command pkmgr php "$@"
    fi
}

composer() { command pkmgr php composer "$@"; }

# Helpful aliases
alias pki='pkmgr install'
alias pkr='pkmgr remove'
alias pku='pkmgr update'
alias pks='pkmgr search'
alias pkl='pkmgr list'

# Load completions if available
if [ -f "$HOME/.local/share/bash-completion/completions/pkmgr" ]; then
    source "$HOME/.local/share/bash-completion/completions/pkmgr"
elif [ -f "/usr/share/bash-completion/completions/pkmgr" ]; then
    source "/usr/share/bash-completion/completions/pkmgr"
fi

echo "✅ pkmgr shell integration loaded for Bash"
"#
        .to_string()
    }

    /// Zsh integration script
    fn zsh_script(&self) -> String {
        r#"
# pkmgr Zsh Integration
# Add this to your ~/.zshrc

# Add ~/.local/bin to PATH if not already present
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
    export PATH="$HOME/.local/bin:$PATH"
fi

# pkmgr environment variables
export PKMGR_SHELL="zsh"

# Language version detection
_pkmgr_detect_version() {
    local lang="$1"
    if [ -f ".${lang}-version" ]; then
        cat ".${lang}-version"
    elif [ -f ".tool-versions" ]; then
        grep "^$lang " .tool-versions | awk '{print $2}'
    fi
}

# Python wrapper
python() {
    local version=$(_pkmgr_detect_version "python")
    if [ -n "$version" ]; then
        PKMGR_PYTHON_VERSION="$version" command pkmgr python "$@"
    else
        command pkmgr python "$@"
    fi
}

python3() { python "$@"; }
pip() { command pkmgr python -m pip "$@"; }
pip3() { pip "$@"; }

# Node.js wrapper
node() {
    local version=$(_pkmgr_detect_version "node")
    if [ -n "$version" ]; then
        PKMGR_NODE_VERSION="$version" command pkmgr node "$@"
    else
        command pkmgr node "$@"
    fi
}

npm() { command pkmgr node npm "$@"; }
yarn() { command pkmgr node yarn "$@"; }
pnpm() { command pkmgr node pnpm "$@"; }

# Ruby wrapper
ruby() {
    local version=$(_pkmgr_detect_version "ruby")
    if [ -n "$version" ]; then
        PKMGR_RUBY_VERSION="$version" command pkmgr ruby "$@"
    else
        command pkmgr ruby "$@"
    fi
}

gem() { command pkmgr ruby gem "$@"; }
bundle() { command pkmgr ruby bundle "$@"; }

# Go wrapper
go() {
    local version=$(_pkmgr_detect_version "go")
    if [ -n "$version" ]; then
        PKMGR_GO_VERSION="$version" command pkmgr go "$@"
    else
        command pkmgr go "$@"
    fi
}

# Rust wrapper
rustc() { command pkmgr rust rustc "$@"; }
cargo() { command pkmgr rust cargo "$@"; }
rustup() { command pkmgr rust rustup "$@"; }

# Java wrapper
java() {
    local version=$(_pkmgr_detect_version "java")
    if [ -n "$version" ]; then
        PKMGR_JAVA_VERSION="$version" command pkmgr java "$@"
    else
        command pkmgr java "$@"
    fi
}

javac() { command pkmgr java javac "$@"; }
mvn() { command pkmgr java mvn "$@"; }
gradle() { command pkmgr java gradle "$@"; }

# .NET wrapper
dotnet() { command pkmgr dotnet "$@"; }

# PHP wrapper
php() {
    local version=$(_pkmgr_detect_version "php")
    if [ -n "$version" ]; then
        PKMGR_PHP_VERSION="$version" command pkmgr php "$@"
    else
        command pkmgr php "$@"
    fi
}

composer() { command pkmgr php composer "$@"; }

# Helpful aliases
alias pki='pkmgr install'
alias pkr='pkmgr remove'
alias pku='pkmgr update'
alias pks='pkmgr search'
alias pkl='pkmgr list'

# Add completions to fpath
if [[ -d "$HOME/.zsh/completions" ]]; then
    fpath=($HOME/.zsh/completions $fpath)
fi

# Load completions
autoload -Uz compinit && compinit

echo "✅ pkmgr shell integration loaded for Zsh"
"#
        .to_string()
    }

    /// Fish integration script
    fn fish_script(&self) -> String {
        r#"
# pkmgr Fish Integration
# Add this to your ~/.config/fish/config.fish

# Add ~/.local/bin to PATH if not already present
if not contains $HOME/.local/bin $PATH
    set -gx PATH $HOME/.local/bin $PATH
end

# pkmgr environment variables
set -gx PKMGR_SHELL "fish"

# Language version detection
function _pkmgr_detect_version
    set lang $argv[1]
    if test -f ".{$lang}-version"
        cat ".{$lang}-version"
    else if test -f ".tool-versions"
        grep "^$lang " .tool-versions | awk '{print $2}'
    end
end

# Python wrapper
function python
    set version (_pkmgr_detect_version "python")
    if test -n "$version"
        env PKMGR_PYTHON_VERSION="$version" command pkmgr python $argv
    else
        command pkmgr python $argv
    end
end

function python3; python $argv; end
function pip; command pkmgr python -m pip $argv; end
function pip3; pip $argv; end

# Node.js wrapper
function node
    set version (_pkmgr_detect_version "node")
    if test -n "$version"
        env PKMGR_NODE_VERSION="$version" command pkmgr node $argv
    else
        command pkmgr node $argv
    end
end

function npm; command pkmgr node npm $argv; end
function yarn; command pkmgr node yarn $argv; end
function pnpm; command pkmgr node pnpm $argv; end

# Ruby wrapper
function ruby
    set version (_pkmgr_detect_version "ruby")
    if test -n "$version"
        env PKMGR_RUBY_VERSION="$version" command pkmgr ruby $argv
    else
        command pkmgr ruby $argv
    end
end

function gem; command pkmgr ruby gem $argv; end
function bundle; command pkmgr ruby bundle $argv; end

# Go wrapper
function go
    set version (_pkmgr_detect_version "go")
    if test -n "$version"
        env PKMGR_GO_VERSION="$version" command pkmgr go $argv
    else
        command pkmgr go $argv
    end
end

# Rust wrapper
function rustc; command pkmgr rust rustc $argv; end
function cargo; command pkmgr rust cargo $argv; end
function rustup; command pkmgr rust rustup $argv; end

# Java wrapper
function java
    set version (_pkmgr_detect_version "java")
    if test -n "$version"
        env PKMGR_JAVA_VERSION="$version" command pkmgr java $argv
    else
        command pkmgr java $argv
    end
end

function javac; command pkmgr java javac $argv; end
function mvn; command pkmgr java mvn $argv; end
function gradle; command pkmgr java gradle $argv; end

# .NET wrapper
function dotnet; command pkmgr dotnet $argv; end

# PHP wrapper
function php
    set version (_pkmgr_detect_version "php")
    if test -n "$version"
        env PKMGR_PHP_VERSION="$version" command pkmgr php $argv
    else
        command pkmgr php $argv
    end
end

function composer; command pkmgr php composer $argv; end

# Helpful abbreviations
abbr -a pki 'pkmgr install'
abbr -a pkr 'pkmgr remove'
abbr -a pku 'pkmgr update'
abbr -a pks 'pkmgr search'
abbr -a pkl 'pkmgr list'

echo "✅ pkmgr shell integration loaded for Fish"
"#
        .to_string()
    }

    /// PowerShell integration script
    fn powershell_script(&self) -> String {
        r#"
# pkmgr PowerShell Integration
# Add this to your $PROFILE

# Add ~/.local/bin to PATH if not already present
$localBin = "$env:USERPROFILE\.local\bin"
if ($env:PATH -notlike "*$localBin*") {
    $env:PATH = "$localBin;$env:PATH"
}

# pkmgr environment variables
$env:PKMGR_SHELL = "powershell"

# Language version detection
function Get-PkmgrVersion {
    param([string]$Lang)

    if (Test-Path ".${Lang}-version") {
        Get-Content ".${Lang}-version"
    } elseif (Test-Path ".tool-versions") {
        Select-String "^$Lang " .tool-versions | ForEach-Object { $_.Line.Split()[1] }
    }
}

# Python wrapper
function python {
    $version = Get-PkmgrVersion "python"
    if ($version) {
        $env:PKMGR_PYTHON_VERSION = $version
        & pkmgr python @args
    } else {
        & pkmgr python @args
    }
}

function python3 { python @args }
function pip { & pkmgr python -m pip @args }
function pip3 { pip @args }

# Node.js wrapper
function node {
    $version = Get-PkmgrVersion "node"
    if ($version) {
        $env:PKMGR_NODE_VERSION = $version
        & pkmgr node @args
    } else {
        & pkmgr node @args
    }
}

function npm { & pkmgr node npm @args }
function yarn { & pkmgr node yarn @args }
function pnpm { & pkmgr node pnpm @args }

# Ruby wrapper
function ruby {
    $version = Get-PkmgrVersion "ruby"
    if ($version) {
        $env:PKMGR_RUBY_VERSION = $version
        & pkmgr ruby @args
    } else {
        & pkmgr ruby @args
    }
}

function gem { & pkmgr ruby gem @args }
function bundle { & pkmgr ruby bundle @args }

# Go wrapper
function go {
    $version = Get-PkmgrVersion "go"
    if ($version) {
        $env:PKMGR_GO_VERSION = $version
        & pkmgr go @args
    } else {
        & pkmgr go @args
    }
}

# Rust wrapper
function rustc { & pkmgr rust rustc @args }
function cargo { & pkmgr rust cargo @args }
function rustup { & pkmgr rust rustup @args }

# Java wrapper
function java {
    $version = Get-PkmgrVersion "java"
    if ($version) {
        $env:PKMGR_JAVA_VERSION = $version
        & pkmgr java @args
    } else {
        & pkmgr java @args
    }
}

function javac { & pkmgr java javac @args }
function mvn { & pkmgr java mvn @args }
function gradle { & pkmgr java gradle @args }

# .NET wrapper
function dotnet { & pkmgr dotnet @args }

# PHP wrapper
function php {
    $version = Get-PkmgrVersion "php"
    if ($version) {
        $env:PKMGR_PHP_VERSION = $version
        & pkmgr php @args
    } else {
        & pkmgr php @args
    }
}

function composer { & pkmgr php composer @args }

# Helpful aliases
Set-Alias pki 'pkmgr install'
Set-Alias pkr 'pkmgr remove'
Set-Alias pku 'pkmgr update'
Set-Alias pks 'pkmgr search'
Set-Alias pkl 'pkmgr list'

Write-Host "✅ pkmgr shell integration loaded for PowerShell" -ForegroundColor Green
"#
        .to_string()
    }

    /// Nushell integration script
    fn nushell_script(&self) -> String {
        r#"
# pkmgr Nushell Integration
# Add this to your ~/.config/nushell/config.nu

# Add ~/.local/bin to PATH
let-env PATH = ($env.PATH | prepend $"($env.HOME)/.local/bin")

# pkmgr environment variables
let-env PKMGR_SHELL = "nushell"

# Language version detection
def pkmgr-detect-version [lang: string] {
    let version_file = $".($lang)-version"
    if ($version_file | path exists) {
        open $version_file | str trim
    } else if (".tool-versions" | path exists) {
        open .tool-versions | lines | where $it =~ $"^($lang) " | first | split column " " | get column2
    } else {
        ""
    }
}

# Python wrapper
def python [...args] {
    let version = (pkmgr-detect-version "python")
    if ($version | is-empty) {
        ^pkmgr python ...$args
    } else {
        with-env [PKMGR_PYTHON_VERSION $version] { ^pkmgr python ...$args }
    }
}

def python3 [...args] { python ...$args }
def pip [...args] { ^pkmgr python -m pip ...$args }
def pip3 [...args] { pip ...$args }

# Node.js wrapper
def node [...args] {
    let version = (pkmgr-detect-version "node")
    if ($version | is-empty) {
        ^pkmgr node ...$args
    } else {
        with-env [PKMGR_NODE_VERSION $version] { ^pkmgr node ...$args }
    }
}

def npm [...args] { ^pkmgr node npm ...$args }
def yarn [...args] { ^pkmgr node yarn ...$args }
def pnpm [...args] { ^pkmgr node pnpm ...$args }

# Ruby wrapper
def ruby [...args] {
    let version = (pkmgr-detect-version "ruby")
    if ($version | is-empty) {
        ^pkmgr ruby ...$args
    } else {
        with-env [PKMGR_RUBY_VERSION $version] { ^pkmgr ruby ...$args }
    }
}

def gem [...args] { ^pkmgr ruby gem ...$args }
def bundle [...args] { ^pkmgr ruby bundle ...$args }

# Go wrapper
def go [...args] {
    let version = (pkmgr-detect-version "go")
    if ($version | is-empty) {
        ^pkmgr go ...$args
    } else {
        with-env [PKMGR_GO_VERSION $version] { ^pkmgr go ...$args }
    }
}

# Rust wrapper
def rustc [...args] { ^pkmgr rust rustc ...$args }
def cargo [...args] { ^pkmgr rust cargo ...$args }
def rustup [...args] { ^pkmgr rust rustup ...$args }

# Java wrapper
def java [...args] {
    let version = (pkmgr-detect-version "java")
    if ($version | is-empty) {
        ^pkmgr java ...$args
    } else {
        with-env [PKMGR_JAVA_VERSION $version] { ^pkmgr java ...$args }
    }
}

def javac [...args] { ^pkmgr java javac ...$args }
def mvn [...args] { ^pkmgr java mvn ...$args }
def gradle [...args] { ^pkmgr java gradle ...$args }

# .NET wrapper
def dotnet [...args] { ^pkmgr dotnet ...$args }

# PHP wrapper
def php [...args] {
    let version = (pkmgr-detect-version "php")
    if ($version | is-empty) {
        ^pkmgr php ...$args
    } else {
        with-env [PKMGR_PHP_VERSION $version] { ^pkmgr php ...$args }
    }
}

def composer [...args] { ^pkmgr php composer ...$args }

# Helpful aliases
alias pki = pkmgr install
alias pkr = pkmgr remove
alias pku = pkmgr update
alias pks = pkmgr search
alias pkl = pkmgr list

print "✅ pkmgr shell integration loaded for Nushell"
"#
        .to_string()
    }

    /// Display current shell environment
    pub fn display_env(&self) {
        self.output.section("Shell Environment");

        self.output.info(&format!("🐚 Current shell: {}", self.shell.display_name()));

        let local_bin = dirs::home_dir()
            .map(|h| h.join(".local").join("bin"))
            .unwrap_or_else(|| PathBuf::from("~/.local/bin"));

        // Check if local bin is in PATH
        if let Ok(path) = std::env::var("PATH") {
            if path.contains(local_bin.to_str().unwrap_or("")) {
                self.output.success("✅ ~/.local/bin is in PATH");
            } else {
                self.output.warn("⚠️  ~/.local/bin is not in PATH");
                self.output.info("   Run: eval $(pkmgr shell add)");
            }
        }

        // Check for language wrappers
        self.output.section("Language Wrappers");
        let wrappers = vec![
            ("Python", vec!["python", "python3", "pip"]),
            ("Node.js", vec!["node", "npm", "yarn"]),
            ("Ruby", vec!["ruby", "gem", "bundle"]),
            ("Go", vec!["go"]),
            ("Rust", vec!["rustc", "cargo"]),
            ("Java", vec!["java", "javac"]),
            (".NET", vec!["dotnet"]),
            ("PHP", vec!["php", "composer"]),
        ];

        for (lang, commands) in wrappers {
            let available: Vec<_> = commands
                .iter()
                .filter(|cmd| which::which(cmd).is_ok())
                .collect();

            if !available.is_empty() {
                self.output.info(&format!("   {} {}: {}",
                    if available.len() == commands.len() { "✅" } else { "⚠️" },
                    lang,
                    available.iter().map(|s| **s).collect::<Vec<_>>().join(", ")
                ));
            } else {
                self.output.info(&format!("   ❌ {}: Not configured", lang));
            }
        }

        // Show configuration files
        self.output.section("Configuration Files");
        for config_file in self.shell.config_files() {
            if std::path::Path::new(&config_file).exists() {
                self.output.info(&format!("   ✅ {}", config_file));
            }
        }

        // Show completion status
        if let Some(comp_dir) = self.shell.completion_dir() {
            self.output.section("Shell Completions");
            let comp_file = PathBuf::from(&comp_dir).join("pkmgr");
            if comp_file.exists() {
                self.output.success(&format!("✅ Completions installed: {}", comp_file.display()));
            } else {
                self.output.info(&format!("❌ No completions found in: {}", comp_dir));
                self.output.info("   Run: pkmgr shell completions <shell>");
            }
        }
    }
}