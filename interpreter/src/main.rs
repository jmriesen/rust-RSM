/*
 * Package: Rust Reference Standard M
 *
 * Jacob Riesen <jacobriesen@gmail.com>
 * https://github.com/jmriesen/rust-RSM
 *
 * Based on Reference Standard M by David Wicksell
 * Copyright © 2020-2024 Fourth Watch Software LC
 * https://gitlab.com/Reference-Standard-M/rsm
 *
 * Which was based on MUMPS V1 by Raymond Douglas Newman
 * Copyright © 1999-2018
 * https://gitlab.com/Reference-Standard-M/mumpsv1
 *
 * This program is free software: you can redistribute it and/or modify it
 * under the terms of the GNU Affero General Public License (AGPL) as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero
 * General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see https://www.gnu.org/licenses/.
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */
#![feature(iter_intersperse)]
#[allow(unused)]
use clap::{Parser, Subcommand};
use interpreter::{units::Kibibytes, var_u::AlphaVAR_U};
use std::{ffi::CString, num::NonZeroU32};

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
        map: Option<u32>,
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
            interpreter::util::info(&args.name);
            println!("-----------------------------");
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
                args.name,
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
                ffi::INIT_Run(name.into_raw(), env.into_raw(), command.into_raw());
            };
        }
    }
    Ok(())
}
