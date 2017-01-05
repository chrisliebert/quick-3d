// Copyright (C) 2016 Chris Liebert

extern crate cmake;

use std::fs;
use std::path::Path;
use std::process::Command;
use std::env;
use std::str::FromStr;

#[cfg(target_os = "windows")]
fn copy_quick3d_shared_library_files(debug: bool) -> std::io::Result<()> {
	let build_type: String = match debug {
	    true => String::from("debug"),
		false => String::from("release"),
	};
	let build_dir: String = String::from("../../target/") + &build_type;
	try!(fs::copy(Path::new(&(build_dir.clone() + "/quick3d.lib")), Path::new("src/quick3d.lib")));
	Ok(())
}

#[cfg(not(target_os = "windows"))]
fn copy_quick3d_shared_library_files(debug: bool) -> std::io::Result<()> {
	let build_type: String = match debug {
	    true => String::from("debug"),
		false => String::from("release"),
	};
	let build_dir: String = String::from("../../target/") + &build_type;
	try!(fs::copy(Path::new(&(build_dir.clone() + "/libquick3d.a")), Path::new("src/libquick3d.a")));
	Ok(())
}

fn copy_swig_interface_file() -> std::io::Result<()> {
	try!(fs::copy(Path::new("../../quick3d.h"), Path::new("src/quick3d.h")));
	Ok(())
}

fn main() {
	let debug = bool::from_str(&env::var("DEBUG").expect("Unable to get DEBUG env")).expect("Unable to parse DEBUG env");
	// build Quick3D
	let build_cmd_output = match debug {
		true => Command::new("cargo")
				.arg("build")
				.arg("--verbose")
				.current_dir("../../")
				.output()
				.expect("Unable to build debug quick3d static library"),
		false => Command::new("cargo")
				.arg("build")
				.arg("--verbose")
				.arg("--release")
				.current_dir("../../")
				.output()
				.expect("Unable to build quick3d static library"),
	};
	// Ensure cargo succeeded
	assert!(build_cmd_output.status.success());
	
	copy_quick3d_shared_library_files(debug).expect("Unable to copy shared libraries");
	copy_swig_interface_file().expect("Unable to copy swig interface file");

	// cmake build
	let dst = cmake::Config::new("src").build();
	println!("cargo:rustc-link-search=src");
	println!("cargo:rustc-link-search=native={}", dst.display());
}
