#![feature(let_chains)]
#![feature(iter_intersperse)]
#[allow(unused)]
use clap::{Parser, Subcommand};
use interpreter::{units::Kibibytes, var_u::AlphaVAR_U};
use std::ffi::CString;
use std::num::NonZeroU32;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Info,
    Kill,
    Start {
        #[arg(short, long)]
        jobs: NonZeroU32,
        #[arg(short, long)]
        global_buffer: Option<NonZeroU32>,
        #[arg(short, long)]
        routine_buffer: Option<NonZeroU32>,
    },
    ///Create a new database file
    Create {
        ///The number of database blocks.
        #[arg(short = 's', long)]
        block_num: u32,
        ///The size of each database block.
        #[arg(short, long)]
        block_size: u32,
        ///The minimum size of the header block.
        #[arg(short, long)]
        map: Option<u32>, //TODO I don't know what this is
        #[arg(short, long, value_parser=AlphaVAR_U::parse)]
        ///Volume name
        volnam: AlphaVAR_U,
        ///Environment name
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

fn main() -> Result<(), String> {
    use Commands::*;
    let args = Args::parse();
    let name = CString::new(args.name.clone()).unwrap();
    match args.command {
        Info => {
            interpreter::util::info(args.name.clone());
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
        } => {
            let _ = interpreter::start::start(args.name, jobs, global_buffer, routine_buffer);
        }
        Create {
            block_num,
            block_size,
            map,
            volnam,
            env,
        } => {
            interpreter::create::FileConfig::new(
                args.name.try_into().unwrap(),
                volnam,
                env,
                block_num,
                Kibibytes(block_size as usize),
                map.map(|x| Kibibytes(x as usize)),
            )
            .map_err(|errors| {
                errors
                    .iter()
                    .map(|e| e.to_string())
                    .intersperse("\n".to_string())
                    .collect::<String>()
            })?
            .create()
            .map_err(|_| "error writing file")?;
        }
        Run { env, command } => {
            let env = CString::new(env).unwrap();
            let command = CString::new(command).unwrap();

            //NOTE for right now I am just going to leak the strings passed to C.
            unsafe {
                rsm::bindings::INIT_Run(name.into_raw(), env.into_raw(), command.into_raw());
            };
        }
    }
    Ok(())
}
