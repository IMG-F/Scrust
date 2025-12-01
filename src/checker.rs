use crate::ast::{Item, Program};
use anyhow::Result;
use colored::*;

pub fn check_program(program: &Program) -> Result<()> {
    for item in &program.items {
        if let Item::Stmt(_) = item {
            println!(
                "{}",
                "Warning: Top-level statement found. It will be compiled as a hat-less block."
                    .yellow()
            );
        }
    }
    Ok(())
}
