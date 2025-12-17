use std::io::Result;
use std::process::Command;

pub fn execute_command(command_str: &str) -> Result<()> {
    println!("🦀 Executing: {}", command_str);

    Command::new("sh").arg("-c").arg(command_str).status()?;

    Ok(())
}
