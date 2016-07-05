Copyright(C) 2016 Chris Liebert

quick-3d
===================
The goal of quick-3d is to replace the need for C++ in hardware-accelerated 3D graphics-based applications by using the Rust language. The main motivation for rust is it's ownership/borrowing system of managing memory.

| Feature     | Status | Description   |
| :------- | :----: | :---- |
| Load Data from SQLite | Supported | Graphics data is loaded from an SQLite database. (obj2sqlite is a tool that can be used to generate databases from wavefront .obj files) |
| Update and Render Geometry | Render only |  *Currently the update feature is missing which would allow access to uniforms |
| Diffuse Texture Maps | Supported | Diffuse textures map are loaded from image blobs stored SQLite |
| Multiple Hardware Profiles | *In-Progress | Multiple GLSL hardware profiles to support different shader versions on multiple platforms. Different versions of the shader programs are stored in the SQLite database.|
| LUA Scripting | Planned | Lua scripting integration, preferrably the entire API will be exposed to LUA and LuaJIT or SWIG.|
| Tests | *In-Progress | Unit test, benchmark and integration tests |
| Optimizations | Planned | Hide geometry that is outside the view frustum using linear algebra (1), and utilize uniform buffer objects on systems that support them and switch to a single vertex buffer object of possible (2).|
| Example usage | Included as main | A basic example of how to use quick-3d |
| Example for Android | Nice to have | A basic example for android |


  **License:**
  This program and it's source are availible under the "MIT License" please see LICENSE