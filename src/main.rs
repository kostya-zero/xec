use std::process::exit;

use clap::{Command, Arg};
use crate::{config::ConfigModel, shell::Shell};

mod config;
mod shell;
mod term;

fn app() -> Command {
    Command::new("xec")
        .about("Minimalist command executor for your workspace.")
        .version("0.1.0")
        .author("Konstanin Zhigaylo")
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommands([
            Command::new("run")
            .short_flag('r')
            .long_flag("run")
            .about("Run specific scriptlet.")
            .arg(Arg::new("scriptlet")
                 .num_args(1)
                 .required(true)
                 .help("Name of scriptlet.")
                 .value_parser(clap::value_parser!(String))),

            Command::new("list")
                .short_flag('l')
                .long_flag("list")
                .about("List available scriptlets.")
        ])
}

fn main() {
    let args = app().get_matches();
    match args.subcommand() {
        Some(("run", submatches)) => {
            let scriptlet_name: String = submatches.get_one::<String>("scriptlet").unwrap().to_string();

            if !config::ConfigManager::check() {
                term::Term::fatal("Configuration file not found! Make xec.conf file and follow documentation.".to_string());
                exit(1);
            }            

            let cfg: ConfigModel = config::ConfigManager::get_config();
            if !cfg.scriptlets.contains_key(&scriptlet_name) {
                term::Term::fatal(format!("Scriptlet \"{}\" are not specified.", &scriptlet_name).to_string());
                exit(1);
            }

            if cfg.scriptlets[&scriptlet_name].clone().into_iter().len() == 0 {
                term::Term::warn("Scriptlet have no command to execute!".to_string());
                exit(0);
            }

            term::Term::stage("Starting...".to_string());
            
            for i in cfg.scriptlets[&scriptlet_name].clone().into_iter() {
                term::Term::info(format!("Running command: {}", i).to_string());
                Shell::run(i);
            }

            term::Term::stage("All commands have been executed.".to_string());
        },
        Some(("list", _submatches)) => {
            if !config::ConfigManager::check() {
                term::Term::fatal("Configuration file not found!".to_string());
                exit(1);
            } 
            
            let cfg: ConfigModel = config::ConfigManager::get_config();

            if cfg.scriptlets.len() == 0 {
                println!("No scriptlets specified in xec.conf!");
                exit(0);
            }

            for (k, _v) in cfg.scriptlets.into_iter() {
                println!("{}", k);
            }
        }
        _ => term::Term::fatal("Unknown command! Use \"-h\" argument to get help.".to_string())
    }
}
