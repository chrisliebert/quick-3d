local wrapper = nil

function require_shared_library()
  wrapper = require "quick3dwrapper"
end

-- Determine whether platform is Windows
function isWindows()
  if package.config:sub(1,1) == "\\" then return true end
end

-- Determine whether platform is Unix
function isUnix()
  if package.config:sub(1,1) == "/" then return false end
end

-- Generate the wrapper source and compile the shared libarary
function build_wrapper()
  -- Generate quick3d_wrapper.c
  local swig_cmd = "swig -lua quick3d.h"
  local swig_result = os.execute("swig -lua quick3d.h")
  if not swig_result == 0 then
    os.exit(1)
  end
  
  local quick3d_dylib = "quick3d"
  local wrapper_dylib = "quick3dwrapper"
  local dylib_ext
  local lua_lib
  
  if isWindows() then
    dylib_ext = ".dll"
  lua_lib = "lua51.dll"
  elseif isUnix() then
    dylib_ext = ".so"
  lua_lib = "-llua"
  end
  
  quick3d_dylib = quick3d_dylib .. dylib_ext
  wrapper_dylib = wrapper_dylib .. dylib_ext
  
  cargo_cmd = os.execute("cargo build")
  if not cargo_cmd then
    print "Unable to build rust library"
    os.exit(1)
  end
  
  if isUnix() then
    os.execute("cp target/debug/"..quick3d_dylib.." .")
  elseif isWindows() then
    os.execute("copy target\\debug\\" ..quick3d_dylib.." .")
  end
  
  local gcc_cmd = "gcc quick3d_wrap.c -fpic -shared "..quick3d_dylib.." "..lua_lib.." -o "..wrapper_dylib
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

