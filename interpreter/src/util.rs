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
use ffi::{
    systab, UTIL_Share, COMP_VER, DB_VER, VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH,
    VERSION_TEST, VOL_DEF,
};
use std::{ffi::CString, fs::OpenOptions};

use crate::shared_seg::sys_tab::SystemTab;

//TODO this is currently only been manually tested.
pub fn info(file: &str) {
    println!("{}", rsm_version());
    println!("Database Version: {DB_VER}\tCompiler Version: {COMP_VER}");
    println!();
    println!("Jacob Riesen");
    println!("Database and Environment Configuration Information:");
    println!();
    match systab_info(file) {
        Ok(sucsses) => println!("{sucsses}"),
        Err(error) => println!("{error}"),
    }
}
//TODO for test-ability it would probably be best to take in a reference.
//but currently there is no synchronization.
fn systab_info(file: &str) -> Result<String, String> {
    let cfile = CString::new(file.to_string()).unwrap();
    OpenOptions::new()
        .read(true)
        .open(file)
        .map_err(|_| format!("Cannot open database file {file}"))?;
    (unsafe { UTIL_Share(cfile.into_raw()) } == 0)
        .then_some(0)
        .ok_or("Cannot connect to environment.".to_string())?;

    //TODO I am not sure if this is safe.
    //In the C code we use the full path every time.
    let vol = unsafe { (*systab).vol[0] };

    (vol != std::ptr::null::<VOL_DEF>().cast_mut())
        .then_some(0)
        .ok_or("Cannot connect to environment.".to_string())?;

    let sys_tab = unsafe { systab.cast::<SystemTab>().as_ref() }.unwrap();
    let temp = if sys_tab.vols().next().unwrap().is_none() {
        Err("Cannot connect to environment.".to_string())
    } else {
        Ok(format!("{sys_tab}"))
    };
    //unmounting shared memory segment.
    unsafe { libc::shmdt(systab as *const libc::c_void) };
    temp
}

fn rsm_version() -> String {
    let mut output =
        format!("Reference Standard M V {VERSION_MAJOR}.{VERSION_MINOR}.{VERSION_PATCH} ");
    if VERSION_TEST != 0 {
        output.push_str(&format!("T{VERSION_TEST} "));
    }
    let uname = uname::uname().unwrap();
    output.push_str(&format!("for {} {}", uname.sysname, uname.machine));
    output.push_str(&format!("Built {} at {}", "---", "----"));
    output
}
