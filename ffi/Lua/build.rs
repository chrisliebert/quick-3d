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
	try!(fs::copy(Path::new(&(build_dir.clone() + "/libquick3d.so")), Path::new("wrapper/libquick3d.so")));
	let dependencies_dir: String = String::from("../../dependencies/");
	let _ = fs::copy(Path::new(&(dependencies_dir.clone() + "/libsqlite3.a")), Path::new("wrapper/libsqlite3.a"));
	Ok(())
}

fn copy_swig_interface_file() -> std::io::Result<()> {
	try!(fs::copy(Path::new("../../quick3d.i"), Path::new("wrapper/quick3d.i")));
	try!(fs::copy(Path::new("../../quick3d.h"), Path::new("wrapper/quick3d.h")));
	Ok(())
}

#[cfg(target_os = "windows")]
fn post_build() -> std::io::Result<()> {
	Ok(())
}

#[cfg(not(target_os = "windows"))]
fn post_build() -> std::io::Result<()> {
	// Move shared library out of the wrapper directory once the wrapper is built
	try!(fs::rename(Path::new("wrapper/libquick3d.so"), Path::new("libquick3d.so")));
	Ok(())
}

fn main() {
	let debug = bool::from_str(&env::var("DEBUG").unwrap()).unwrap();
	
	// Clean quick3d to ensure copy commands don't fail
	let clean_cmd_status = match debug {
		true => Command::new("cargo")
			.arg("clean")
			.arg("-p")
			.arg("quick3d")
			.current_dir("../../")
			.output()
			.unwrap(),
		false => Command::new("cargo")
			.arg("clean")
			.arg("-p")
			.arg("quick3d")
			.arg("--release")
			.current_dir("../../")
			.output()
			.unwrap(),
	}.status;
	assert!(clean_cmd_status.success());
	
	// build Quick3D
	let build_cmd_output = match debug {
		true => Command::new("cargo")
				.arg("build")
				.current_dir("../../")
				.output()
				.unwrap(),
		false => Command::new("cargo")
				.arg("build")
				.arg("--release")
				.current_dir("../../")
				.output()
				.unwrap(),
	};
		
	assert!(build_cmd_output.status.success());
	
	copy_quick3d_shared_library_files(debug).unwrap();
	copy_swig_interface_file().unwrap();

	// cmake build for language wrappers
	let dst = cmake::Config::new("wrapper").build();
	println!("cargo:rustc-link-search=wrapper");
	println!("cargo:rustc-link-search=native={}", dst.display());

	// Post-CMake build
	post_build().unwrap();
}
