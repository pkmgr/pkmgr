use anyhow::Result;
use std::env;
use crate::core::config::Config;
use crate::ui::output::Output;

pub mod resolver;
pub mod installer;
mod executor;

use executor::LanguageExecutor;

pub async fn handle_language_command(language: &str, config: &Config, output: &Output) -> Result<()> {
    // Get the original command name from argv[0]
    let program_name = env::args()
        .next()
        .map(|arg0| {
            std::path::Path::new(&arg0)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("pkmgr")
                .to_string()
        })
        .unwrap_or_else(|| "pkmgr".to_string());

    // Get all command line arguments
    let args: Vec<String> = env::args().collect();

    output.debug(&format!("🔍 Language command detected: {} (language: {})", program_name, language));
    output.debug(&format!("📝 Arguments: {:?}", args));

    // Create executor and run the command
    let executor = LanguageExecutor::new(
        language.to_string(),
        program_name,
        output.clone(),
    );

    executor.execute(args).await?;

    Ok(())
}