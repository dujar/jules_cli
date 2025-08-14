use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new Tauri v2 application
    Init {
        /// The name of the application
        name: String,
    },
    /// Generate a new resource
    Generate(Generate),
}

#[derive(Parser)]
#[command(name = "generate")]
pub struct Generate {
    #[command(subcommand)]
    pub command: GenerateCommands,
}

#[derive(Subcommand)]
pub enum GenerateCommands {
    /// Generate a new service
    Service {
        /// The name of the service
        name: String,
    },
    /// Generate a new provider
    Provider {
        /// The name of the provider
        name: String,
    },
    /// Generate a new schema
    Schema {
        /// The name of the schema
        name: String,
    },
}
