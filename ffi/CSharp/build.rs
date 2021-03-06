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
	try!(fs::copy(Path::new(&(build_dir.clone() + "/quick3d.lib")), Path::new("wrapper/quick3d.lib")));
	let dependencies_dir: String = String::from("../../dependencies/");
	let _ = fs::copy(Path::new(&(dependencies_dir.clone() + "/sqlite3.lib")), Path::new("wrapper/sqlite3.lib"));
	Ok(())
}

#[cfg(not(target_os = "windows"))]
fn copy_quick3d_shared_library_files(debug: bool) -> std::io::Result<()> {
	let build_type: String = match debug {
	    true => String::from("debug"),
		false => String::from("release"),
	};
	let build_dir: String = String::from("../../target/") + &build_type;
	try!(fs::copy(Path::new(&(build_dir.clone() + "/libquick3d.a")), Path::new("wrapper/libquick3d.a")));
	let dependencies_dir: String = String::from("../../dependencies/");
	let _ = fs::copy(Path::new(&(dependencies_dir.clone() + "/libsqlite3.a")), Path::new("wrapper/libsqlite3.a"));
	Ok(())
}

fn copy_swig_interface_file() -> std::io::Result<()> {
	try!(fs::copy(Path::new("../../quick3d.i"), Path::new("wrapper/quick3d.i")));
	try!(fs::copy(Path::new("../../quick3d.h"), Path::new("wrapper/quick3d.h")));
	Ok(())
}

fn copy_runtime_files() -> std::io::Result<()> {
	for entry in try!(fs::read_dir("build")) {
		let path = entry.unwrap().path();
		if !path.is_dir() {
			let filename = path.file_name().unwrap().to_str().unwrap();
			try!(fs::copy(path.clone(), Path::new(filename)));
		}
	}
	Ok(())
}

fn main() {
	let debug = bool::from_str(&env::var("DEBUG").expect("Unable to get DEBUG env")).expect("Unable to parse DEBUG env");
	
	// Clean quick3d to ensure copy commands don't fail
	let clean_cmd_status = match debug {
		true => Command::new("cargo")
			.arg("clean")
			.arg("-p")
			.arg("quick3d")
			.current_dir("../../")
			.output()
			.expect("Unable to build quick3d static library"),
		false => Command::new("cargo")
			.arg("clean")
			.arg("-p")
			.arg("quick3d")
			.arg("--release")
			.current_dir("../../")
			.output()
			.expect("Unable to build quick3d static library"),
	}.status;
	assert!(clean_cmd_status.success());
	
	// build Quick3D
	let build_cmd_output = match debug {
		true => Command::new("cargo")
				.arg("build")
				.current_dir("../../")
				.output()
				.expect("Unable to build debug quick3d static library"),
		false => Command::new("cargo")
				.arg("build")
				.arg("--release")
				.current_dir("../../")
				.output()
				.expect("Unable to build quick3d static library"),
	};
		
	assert!(build_cmd_output.status.success());
	
	copy_quick3d_shared_library_files(debug).expect("Unable to copy shared libraries");
	copy_swig_interface_file().expect("Unable to copy swig interface file");

	// cmake build for language wrappers
	let dst = cmake::Config::new("wrapper").build();
	println!("cargo:rustc-link-search=wrapper");
	println!("cargo:rustc-link-search=native={}", dst.display());
	
	copy_runtime_files().expect("Unable to copy runtime files");
}
