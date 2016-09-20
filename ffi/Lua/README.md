Copyright(C) 2016 Chris Liebert

quick-3d  [![Build Status](https://travis-ci.org/chrisliebert/quick-3d.svg?branch=master)](https://travis-ci.org/chrisliebert/quick-3d)

**Quick3D Lua Example**

If you are using LuaJIT, Quick3D will use the LuaJIT FFI interface. Otherwise, an additional wrapper library will be built using CMake and SWIG.

**Building the LUA Wrapper Library**

The LUA wrapper for Quick3D will try to build the debug shared library automatically if no library is present, making the following step optional:
`cargo build` from the `ffi/Lua` directory.
This should produce quick3dwrapper.so on a Unix system and quick3dwrapper.dll on Windows.

**Running the LUA Example**

`lua example.lua` or `luajit example.lua`

Once example.lua is running, code can be entered directly into the console including functions, statements and expressions. For example you can type `f = function() print("Hello World") end` and now f will be available in the console as `f()`. If you enter `5+5`, the result will be evaluated and printed. It is also possible to create new variables or access global variables, for example `x = screen_width / screen_height` would store the result of the quotient of global script variables screen_width and screen_height in a new variable x.

After the Quick3D Rust source is updated, the Lua example can be run with the `clean` argument which will rebuild the Rust library without rebuilding the dependencies:
`lua example.lua clean`

Release mode can be enabled with `clean release` or `release` when using the LuaJIT FFI

LuaJIT FFI can be disabled if you still want to use LuaJIT with the SWIG wrapper:
`luajit example.lua --noluajitffi`

Build the optimized release version of the shared libraries:
`lua example.lua clean release`

  **License:**
  
  This program and it's source are available under the "MIT License" please see ../../LICENSE
