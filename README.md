Copyright(C) 2016 Chris Liebert

quick-3d  [![Build Status](https://travis-ci.org/chrisliebert/quick-3d.svg?branch=master)](https://travis-ci.org/chrisliebert/quick-3d)
===================
The goal of quick-3d is to replace the need for C++ in hardware-accelerated 3D graphics-based applications by using the Rust language. The main motivation for rust is it's ownership/borrowing system of managing memory.

| Feature     | Status | Description   |
| :------- | :----: | :---- |
| Load Data from SQLite | Supported | Graphics data is loaded from an SQLite database. (obj2sqlite is a tool that can be used to generate databases from wavefront .obj files) |
| Update and Render Geometry | *Supported |  *Currently the ability to update OpenGL uniforms is only availible in Rust |
| Diffuse Texture Maps | Supported | Diffuse textures map are loaded from image blobs stored in SQLite |
| Multiple Hardware Profiles | Supported | Multiple GLSL hardware profiles to support different shader versions on multiple platforms. Different versions of the shader programs are stored in the SQLite database.|
| LUA Scripting | Supported | Scripting integration, an API is exposed to C and SWIG. There is an LUA Example that demonstrates a console with dynamic instrumentation to enable rapid prototyping|
| Tests | *In-Progress | Unit test, benchmark and integration tests |
| Optimizations | Planned | Hide geometry that is outside the view frustum using linear algebra (1), and utilize uniform buffer objects on systems that support them and switch to a single vertex buffer object of possible (2).|
| Example usage | Included | A basic example of how to use quick-3d in Rust, LUA and C|
| Example for Android | *Nice to have | A basic example for android |


  **License:**
  This program and it's source are available under the "MIT License" please see LICENSE
