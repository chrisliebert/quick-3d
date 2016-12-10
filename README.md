Copyright(C) 2016 Chris Liebert

[![Build Status](https://travis-ci.org/chrisliebert/quick-3d.svg?branch=master)](https://travis-ci.org/chrisliebert/quick-3d) 
[![wercker status](https://app.wercker.com/status/57dce6bad65185424701112d1613acb3/m/master "wercker status")](https://app.wercker.com/project/byKey/57dce6bad65185424701112d1613acb3)
===================
The goal of Quick3D is to replace the need for C++ in hardware-accelerated 3D graphics-based applications by using the Rust programming language. The main motivation for Rust is it's extensive type checking and ownership/borrowing system for managing memory. By using SWIG, the Simplified Wrapper Interface Generator, a large portion of the Quick3D programming interface is accessible from other programming languages such as C, LUA, Java, Python, JavaScript and Perl. The Glium API is a safe interface for OpenGL, the Open Graphics Library which is the open source library responsible for low-level communication with the GPU. By using Glium instead of OpenGL directly, Quick3D is able to take advantage of some of the built-in error-checking functionality  of Glium.

| Feature     | Status | Description   |
| :------- | :----: | :---- |
| Load 3D objects from binary files. | SQLite databases are supported as feature (disabled by default) and used as an intermediate representation | Graphics data is loaded from processed sources (obj2db is a tool that can be used to generate databases from wavefront .obj files. Once a scene is loaded it can be serialized to a binary file with optional compression.) |
| Update and Render Geometry | Supported |  *Currently the ability to update OpenGL uniforms is only available in Rust |
| Diffuse Texture Maps | Supported | Diffuse texture maps are loaded from image blobs stored in SQLite or serialized binaries |
| Multiple Hardware Profiles | Supported | Multiple GLSL hardware profiles to support different shader versions on multiple platforms. Different versions of the shader programs are stored in the SQLite database.|
| LUA Scripting | Supported | Scripting integration, an API is exposed to C and SWIG. There is an LUA Example that demonstrates a console with dynamic instrumentation to enable rapid prototyping in addition to examples for C, Java and Python. |
| Tests | *In-Progress | Unit test, benchmark and integration tests |
| Optimizations | Planned | Utilize uniform buffer objects on systems that support them and switch to a single vertex buffer object of possible.|
| Example usage | Included | A basic example of how to use quick-3d in Rust, LUA and C|
| Example for Android | *Nice to have | A basic example for android |


Often applications will need to access large amounts of information that is subject to change. This issue is often addressed by leveraging existing database systems that have been optimized and tested extensively. SQLite is a lightwieght database system that will store arbirary amounts of data in a single file for convenience. Rusqlite is a Rust API that is used to read SQLite databases that can be produced from Wavefront .obj files using a C++ tool called obj2db which is included and built automatically as of v0.1.4.

With Quick3D, it is possible to leverage GPU technology in a way that is likely to run on a wide range of devices while maintaining code readability. There is considerable room for optimization in the Quick3D library and future versions have the potential to improve performance of applications written using version 0.1.

Quick3D includes an example LUA script that demonstrates dynamic instrumentation by providing a console which allows the programmer to write scripts and issue commands while the application is running. The camera can also be rotated in the direction that the mouse is dragged. The camera can be moved forward, backward, left and right by using the arrow keys or W/A/S/D on the keyboard. In addition to the LUA example, there is a more secure example written in Rust that is similar but lacking the ability to execute commands.


Make sure the following dependencies are installed, most Linux distributions already include these libraries with a package manager.


| Dependency | Website |
|:-----------|:--------|
| A C compiler such as GCC or Clang and the `make` tool | https://gcc.gnu.org/ |
| The Rust compiler (1.8 or greater) | https://www.rust-lang.org |
| LUA version 5.1 or greater with the developer libraries (LuaJIT 2.0 or greater is also supported) | https://www.lua.org/ or http://luajit.org/ |
| SWIG the Simplified Wrapper Interface Generator | http://www.swig.org/ |
| CMake 3.0 or greater | https://cmake.org/download/ |
| SQLite Browser (Optional, this is a useful tool for reading and writing SQLite databases) | http://sqlitebrowser.org/ |

**Note for Windows Users**

Quick3D can be built for Windows using the MSVC ABI.

**Building the Rust Library**

`cargo build`

**Linking with SQLite**
`cargo build --features sqlite`

When the sqlite feature is enabled, some additional functionality is included which is required to import data from .obj files.
Note: To build the sqlite features with the MSVC ABI, it is important to download the SQLite source from https://sqlite.org/ and create a blank C++ empty project in MSVC, add sqlite3.c and sqlite3.h, rename the project 'sqlite3', open properties and change Configuration Type from Application (.exe) to Static library (.lib) for all configurations.
Make sure pkg-config.exe is in PATH (it can be download from https://sourceforge.net/projects/pkgconfiglite/), and the PKG_CONFIG_PATH is set to a directory containing the following file, sqlite3.pc:
`
Name: sqlite3
Description: Portable database
Version: 3
Libs: -LC:/dev/sqlite-amalgamation-3150200 -lsqlite3
Cflags: -IC:/dev/sqlite-amalgamation-3150200
`
In this example, "C:\dev\sqlite-amalgamation-3150200" points to the path of the extracted archive containing the SQLite3 source code. 
It may also be nessisary to place copy of sqlite3.lib in the dependencies folder and set SQLITE3_LIB_DIR prior to building with cargo.

**Running the Rust Example**

`cargo run`

**Building the LUA Library**

The LUA wrapper for Quick3D will try to build the debug shared library automatically if no library is present, making the following step optional:
`cargo build` from the `ffi/Lua` directory.
This should produce quick3dwrapper.so on a Unix system and quick3dwrapper.dll on Windows.


**Running the LUA Example**
`cd ffi/Lua`

`lua example.lua` or `luajit example.lua`

Once example.lua is running, code can be entered directly into the console including functions, statements and expressions. For example you can type `f = function() print("Hello World") end` and now f will be available in the console as `f()`. If you enter `5+5`, the result will be evaluated and printed. It is also possible to create new variables or access global variables, for example `x = screen_width / screen_height` would store the result of the quotient of global script variables screen_width and screen_height in a new variable x.

After the Quick3D Rust source is updated, the Lua example can be run with the `clean` argument which will rebuild the Rust library without rebuilding the dependencies:
`lua example.lua clean`

As of Lua example v0.1.3, to build the optimized release version of the shared libraries, run with:
`lua example.lua clean release`


**A Note about Shared Libraries**

If you are using Uinux, it is likely that your operating system does not know where to find the shared libraries.
This can be resolved by updating the LD_LIBRARY_PATH environment variable: `export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:.` The operating system will now search the current directory when attempting to load shared libraries. After building the debug library with `cargo build` it is recommended that you create a symlink to target/debug/libquick3d.so, i.e `ln -s target/debug/libquick3d.so .` The Quick3D Lua example will automatically attempt to relaunch with the updated LD_LIBRARY_PATH as of v0.1.2.
On Windows, quick3d.dll is copied to the current directory if it is not found when running Quick3D from LUA which is already configured to be in the search path for shared libraries.

  **License:**
  
  This program and it's source are available under the "MIT License" please see LICENSE
