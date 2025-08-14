use crate::cli::GenerateCommands;
use anyhow::{Context, Result};
use heck::{ToPascalCase, ToSnakeCase};
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn handle_init(name: &str) -> Result<()> {
    if !is_pnpm_installed()? {
        anyhow::bail!("pnpm is not installed. Please install it before running this command.");
    }

    println!("Initializing a new Tauri app named: {}", name);

    let status = Command::new("pnpm")
        .arg("create")
        .arg("tauri-app")
        .arg(name)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("Failed to execute pnpm command")?;

    if !status.success() {
        anyhow::bail!("Failed to initialize Tauri app");
    } else {
        let project_dir = Path::new(name);
        std::env::set_current_dir(&project_dir)
            .with_context(|| format!("Failed to change into project directory: {}", name))?;
    }

    Ok(())
}

pub fn handle_generate(command: &GenerateCommands) -> Result<()> {
    match command {
        GenerateCommands::Service { name } => handle_generate_service(name),
        GenerateCommands::Provider { name } => handle_generate_provider(name),
        GenerateCommands::Schema { name } => handle_generate_schema(name),
    }
}

fn handle_generate_service(name: &str) -> Result<()> {
    println!("Generating a new service named: {}", name);
    let name_snake = name.to_snake_case();
    let name_pascal = name.to_pascal_case();

    let service_dir = Path::new("src-tauri/src/services");
    fs::create_dir_all(service_dir).context("Failed to create services directory")?;

    let file_path = service_dir.join(format!("{}.rs", name_snake));
    let content = format!(
        "// Service: {}\n\npub struct {}Service;\n\nimpl {}Service {{\n    pub fn new() -> Self {{\n        Self {{}}\n    }}\n}}\n",
        name, name_pascal, name_pascal
    );

    fs::write(&file_path, content).context("Failed to write service file")?;
    println!("Created service file: {:?}", file_path);
    Ok(())
}

fn handle_generate_provider(name: &str) -> Result<()> {
    println!("Generating a new provider named: {}", name);
    let name_snake = name.to_snake_case();
    let name_pascal = name.to_pascal_case();

    let provider_dir = Path::new("src-tauri/src/providers");
    fs::create_dir_all(provider_dir).context("Failed to create providers directory")?;

    let file_path = provider_dir.join(format!("{}.rs", name_snake));
    let content = format!(
        "// Provider: {}\n\npub struct {}Provider;\n\nimpl {}Provider {{\n    pub fn new() -> Self {{\n        Self {{}}\n    }}\n}}\n",
        name, name_pascal, name_pascal
    );

    fs::write(&file_path, content).context("Failed to write provider file")?;
    println!("Created provider file: {:?}", file_path);
    Ok(())
}

fn handle_generate_schema(name: &str) -> Result<()> {
    println!("Generating a new schema named: {}", name);
    let name_snake = name.to_snake_case();
    let name_pascal = name.to_pascal_case();

    let schema_dir = Path::new("src-tauri/src/schemas");
    fs::create_dir_all(schema_dir).context("Failed to create schemas directory")?;

    let file_path = schema_dir.join(format!("{}.rs", name_snake));
    let content = format!(
        "// Schema: {}\n\n#[derive(serde::Serialize, serde::Deserialize)]\npub struct {} {{\n    pub id: i32,\n}}\n",
        name, name_pascal
    );

    fs::write(&file_path, content).context("Failed to write schema file")?;
    println!("Created schema file: {:?}", file_path);

    // Create a migration file
    let migration_dir = Path::new("migrations");
    fs::create_dir_all(migration_dir).context("Failed to create migrations directory")?;
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let migration_file_path = migration_dir.join(format!("{}_{}.sql", timestamp, name_snake));
    let migration_content = format!("-- Migration for {}\n\nCREATE TABLE {} (\n    id SERIAL PRIMARY KEY\n);\n", name, name_snake);
    fs::write(&migration_file_path, migration_content).context("Failed to write migration file")?;
    println!("Created migration file: {:?}", migration_file_path);

    Ok(())
}

fn is_pnpm_installed() -> Result<bool> {
    Ok(Command::new("pnpm")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_or(false, |s| s.success()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_handle_generate_service() {
        let dir = tempdir().unwrap();
        std::env::set_current_dir(&dir).unwrap();
        let result = handle_generate_service("test-service");
        assert!(result.is_ok());
        let expected_path = Path::new("src-tauri/src/services/test_service.rs");
        assert!(expected_path.exists());
        let content = fs::read_to_string(expected_path).unwrap();
        assert!(content.contains("pub struct TestServiceService;"));
    }

    #[test]
    fn test_handle_generate_provider() {
        let dir = tempdir().unwrap();
        std::env::set_current_dir(&dir).unwrap();
        let result = handle_generate_provider("test-provider");
        assert!(result.is_ok());
        let expected_path = Path::new("src-tauri/src/providers/test_provider.rs");
        assert!(expected_path.exists());
        let content = fs::read_to_string(expected_path).unwrap();
        assert!(content.contains("pub struct TestProviderProvider;"));
    }

    #[test]
    fn test_handle_generate_schema() {
        let dir = tempdir().unwrap();
        std::env::set_current_dir(&dir).unwrap();
        let result = handle_generate_schema("test-schema");
        assert!(result.is_ok());
        let expected_path = Path::new("src-tauri/src/schemas/test_schema.rs");
        assert!(expected_path.exists());
        let content = fs::read_to_string(expected_path).unwrap();
        assert!(content.contains("pub struct TestSchema {"));
        let migrations_dir = Path::new("migrations");
        assert!(migrations_dir.exists());
        // Check for a file starting with a timestamp and ending with _test_schema.sql
        let migration_file = fs::read_dir(migrations_dir)
            .unwrap()
            .find(|entry| {
                let entry = entry.as_ref().unwrap();
                let file_name = entry.file_name().into_string().unwrap();
                file_name.ends_with("_test_schema.sql")
            });
        assert!(migration_file.is_some());
    }
}
