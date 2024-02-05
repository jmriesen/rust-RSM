#![feature(let_chains)]
use crate::var_u::AlphaVAR_U;
#[allow(unused)]
use clap::{Parser, Subcommand};
use std::ffi::CString;
use units::DatabaseSize;

#[allow(clippy::all,unused)]
mod bindings;
mod create;

#[allow(unused)]
mod run;
mod start;
mod units;
mod util;
mod var_u;
use bindings::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,
    /// Number of times to greet
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Info,
    Kill,
    Start {
        #[arg(short, long, default_value_t = 1)]
        jobs: u32,
        #[arg(short, long, default_value_t = 0)]
        global_buffer: u32,
        #[arg(short, long, default_value_t = 0)]
        routine_buffer: u32,
        #[arg(short, long, default_value_t = 0)]
        additional_buffer: u32,
    },
    Create {
        #[arg(short = 's', long, value_parser=DatabaseSize::parse)]
        block_num: DatabaseSize,
        #[arg(short, long)]
        block_size: u32,
        #[arg(short, long, default_value_t = 0)]
        map: u32, //TODO I don't know what this is
        #[arg(short, long, value_parser=AlphaVAR_U::parse)]
        volnam: AlphaVAR_U,
        #[arg(short, long, value_parser=AlphaVAR_U::parse)]
        env: Option<AlphaVAR_U>,
    },
    Run {
        #[arg(short, long)]
        command: String,
        #[arg(short, long, default_value_t = String::new())]
        env: String,
    },
}

fn main() {
    use Commands::*;
    let args = Args::parse();
    let name = CString::new(args.name.clone()).unwrap();
    match args.command {
        Info => {
            util::info(args.name.clone());
            println!("-----------------------------");
            //TODO create some A/B tests
            /*
            unsafe {
                bindings::info(name.as_ptr());
            };
            */
        }
        Kill => {
            /*
            unsafe {
                bindings::shutdown(name.as_ptr());
            };
            */
        }
        Start {
            jobs,
            global_buffer,
            routine_buffer,
            additional_buffer,
        } => {
            let _ = start::start(
                args.name,
                jobs,
                global_buffer,
                routine_buffer,
                additional_buffer,
            );
        }
        Create {
            block_num,
            block_size,
            map,
            volnam,
            env,
        } => {
            create::create_file(
                block_num,
                block_size * 1024,
                map * 1024,
                volnam,
                env,
                args.name.clone(),
            )
            .unwrap();
        }
        Run { env, command } => {
            let env = CString::new(env).unwrap();
            let command = CString::new(command).unwrap();

            //NOTE for right now I am just going to leak the strings passed to C.
            unsafe {
                INIT_Run(name.into_raw(), env.into_raw(), command.into_raw());
            };
        }
    }
}
