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
	try!(fs::copy(Path::new(&(build_dir.clone() + "/quick3d.dll")), Path::new("quick3d.dll")));
	try!(fs::copy(Path::new(&(build_dir.clone() + "/quick3d.dll")), Path::new("wrapper/quick3d.dll")));
	try!(fs::copy(Path::new(&(build_dir.clone() + "/quick3d.dll.exp")), Path::new("wrapper/quick3d.dll.exp")));
	try!(fs::copy(Path::new(&(build_dir.clone() + "/quick3d.dll.lib")), Path::new("wrapper/quick3d.dll.lib")));
	try!(fs::copy(Path::new(&(build_dir.clone() + "/quick3d.pdb")), Path::new("wrapper/quick3d.pdb")));
	Ok(())
}

#[cfg(not(target_os = "windows"))]
fn copy_quick3d_shared_library_files(debug: bool) -> std::io::Result<()> {
	let build_type: String = match debug {
	    true => String::from("debug"),
		false => String::from("release"),
	};
	let build_dir: String = String::from("../../target/") + &build_type;
	try!(fs::copy(Path::new(&(build_dir.clone() + "/libquick3d.so")), Path::new("libquick3d.so")));
	try!(fs::copy(Path::new(&(build_dir.clone() + "/libquick3d.so")), Path::new("wrapper/libquick3d.so")));
	Ok(())
}

fn copy_swig_interface_file() -> std::io::Result<()> {
	let quick3d_i_src = Path::new("../../quick3d.i");
	let quick3d_i_dst = Path::new("wrapper/quick3d.i");
	try!(fs::copy(quick3d_i_src, quick3d_i_dst));
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
}
