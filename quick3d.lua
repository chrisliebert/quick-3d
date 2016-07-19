-- Copyright (C) 2016 Chris Liebert
local wrapper = nil

function require_shared_library()
  wrapper = require "quick3dwrapper"
end

-- Determine whether platform is Windows
function isWindows()
  if package.config:sub(1,1) == "\\" then return true end
end

-- Generate the wrapper source and compile the shared libarary
function build_wrapper()
  -- Generate quick3d_wrapper.c
  local swig_cmd = "swig -lua quick3d.h"
  local swig_result = os.execute("swig -lua quick3d.h")
  if not swig_result == 0 then
    os.exit(1)
  end
  
  local quick3d_dylib = "libquick3d"
  local wrapper_dylib = "quick3dwrapper"
  local dylib_ext
  local lua_lib
  
  if isWindows() then
    quick3d_dylib = "quick3d"
    dylib_ext = ".dll"
    lua_lib = "lua51.dll"
  else
    dylib_ext = ".so"
    lua_lib = "-lluajit-5.1"
  end
  
  quick3d_dylib = quick3d_dylib .. dylib_ext
  wrapper_dylib = wrapper_dylib .. dylib_ext
  
  cargo_cmd = os.execute("cargo build")
  if not cargo_cmd then
    print "Unable to build rust library"
    os.exit(1)
  end

  local lua_include = "/usr/include/lua5.1"  

  if isWindows() then
    lua_include = "/usr/include"
    os.execute("copy target\\debug\\" ..quick3d_dylib.." .")
  else
    os.execute("cp target/debug/"..quick3d_dylib.." .")
  end
  
  local gcc_cmd = "gcc quick3d_wrap.c -fpic -shared -I"..lua_include.." "..quick3d_dylib.." "..lua_lib.." -o "..wrapper_dylib
  local gcc_result = os.execute(gcc_cmd)
  if not gcc_result == 0 then
    os.exit(2)
  end

end

-- Load the shared library
function quick3d_init()
  if pcall(require_shared_library) then
    print "Loaded shared library"
  else
    print "Building shared library"
    build_wrapper()
    -- try to load the shared library again
    if not pcall(require_shared_library) then
      print "Unable to load quick3dwrapper shared library"
      os.exit(2)
    end
  end
  return wrapper
end

