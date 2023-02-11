use clap::{Parser, Subcommand};
mod config;
use directories::BaseDirs;
use std::env::current_exe;
use std::path::Path;

/// Simple program to greet a person
#[derive(Parser)]
#[command(author = "croatia. <wuli.croatia@foxmail.com>")]
#[command(version, about = "Does awesome things", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Add {
        // the alias of the path
        alias: String,

        // the path to be added
        path: Option<String>,
    },

    Rm {
        alias: String,
    },

    Ls,

    To {
        alias: String,
    },

    Clear,
}

fn main() {
    let cli = Args::parse();
    let base_dir = BaseDirs::new().unwrap();
    let home_dir = base_dir.home_dir();
    let exec_dir = current_exe().unwrap();

    println!("exec_dir: {:?}", exec_dir);

    let config_path = Path::new(home_dir).join(".dgo.toml");

    let mut dgo = config::Dgo::new(config_path);

    match &cli.command {
        Some(Commands::Add { alias, path }) => {
            let path_value = if let Some(temp_path) = path {
                temp_path
            } else {
                "./"
            };
            let absolut_path = if path_value.starts_with("./") {
                println!("path_value: {}", path_value);
                exec_dir.join(path_value[2..].to_string())
            } else {
                Path::new(path_value).to_path_buf()
            };
            dgo.add_alias(alias, absolut_path);
        }
        Some(Commands::Rm { alias }) => {
            dgo.rm_alias(alias);
        }
        Some(Commands::Ls) => {
            dgo.alias_list();
        }
        Some(Commands::Clear) => {
            dgo.clear_alias();
        }
        Some(Commands::To { alias }) => {}
        _ => {}
    }

    // Continued program logic goes here...
}
