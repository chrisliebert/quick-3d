Copyright(C) 2016 Chris Liebert

The goal of quick-3d is to replace the need for C++ in hardware-accelerated 3D graphics-based applications by using the Rust language. The main motivation for rust is it's ownership/borrowing system of managing memory.

Features:
  -Load data from sqlite. (obj2sqlite is a tool that can be used to generate databases from wavefront .obj files)
  -Update uniforms and render geometry.
  -Diffuse texture maps
  -Multiple GLSL hardware profiles to support different shader versions on multiple platforms

Nice To Have:
  -Lua scripting integration, preferrably the entire API will be exposed to LUA and LuaJIT.
  -Unit tests and benchmark tests
  -Frustum culling optimisation: hide geometry that is outside the view frustum using linear algebra.
  -Normal and specular maps, transparency, lighting effects
  -Configurable shadows
  -Example usage of the API
  -Example for Android JNI
