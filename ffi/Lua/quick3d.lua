-- Copyright (C) 2016 Chris Liebert
local wrapper = nil
local ffi = nil

function require_shared_library()
  wrapper = require "quick3dwrapper"
end

-- Determine whether platform is Windows
function isWindows()
  if package.config:sub(1,1) == "\\" then return true end
end

-- Generate the wrapper source and compile the shared libarary
function build_wrapper(target_build)
  -- Generate quick3d_wrapper.c -> quick3dwrapper
  local make_cmd = "cargo build"
  if target_build == "release" then make_cmd = make_cmd .. " --release" end
  local make_result = os.execute(make_cmd)
  if not make_result == 0 then
    os.exit(2)
  end
end

function quick3d_relaunch_with_exported_unix_lib_path()
  -- Attempt to export the current directory to LD_LIBRARY_PATH on Unix systems
  local ld_lib_path_exported = os.getenv("QUICK3D_SHARED_LIBRARY_PATH_EXPORTED")
  if not (ld_lib_path_exported == nil) then
    if ld_lib_path_exported == "TRUE" then ld_lib_path_exported = true
    else ld_lib_path_exported = false end
  else
    ld_lib_path_exported = false
  end

  -- Generate command used to launch this file
  -- See https://www.lua.org/pil/1.4.html
  local i = 0
  local rerun_command = ""
  while not (arg[i] == nil) do
    rerun_command = arg[i] .. " " .. rerun_command
    i = i - 1
  end

  i=1
  while not (arg[i] == nil) do
    rerun_command = rerun_command .. arg[i] .. " "
    i = i + 1
  end

  -- Update LD_LIBRARY_PATH and QUICK3D_SHARED_LIBRARY_PATH_EXPORTED
  rerun_command = "export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:`pwd` && " .. rerun_command
  rerun_command = "export QUICK3D_SHARED_LIBRARY_PATH_EXPORTED=TRUE && " .. rerun_command
  
  if not ld_lib_path_exported then
    print(rerun_command)
    if not os.execute(rerun_command) == 0 then
      print("Failed to set current directory in LD_LIBRARY_PATH")
    else
      os.exit(0)
    end
  end
end

-- Load the shared library
function quick3d_init(target_build)
  if pcall(require_shared_library) then
    print "Loaded shared library"
  else
    print "Unable to load shared libraries"
    -- On Unix systems, attempt to set LD_LIBRARY_PATH in order to find .so files
    if not isWindows() then
      print("Attempting to export LD_LIBRARY_PATH")
      quick3d_relaunch_with_exported_unix_lib_path()
      if not pcall(require_shared_library) then
        print "Unable to find shared libraries"
      else
        return wrapper
      end
    end

    -- Unable to load shared libraries, try to build the wrapper
    print ("Building " .. target_build .. " shared libraries")
    build_wrapper(target_build)
    -- try to load the shared library again
    if not pcall(require_shared_library) then
      print "Unable to load quick3dwrapper shared libraries"
      os.exit(2)
    end
  end
  return wrapper
end

function get_quick3d_target_path(target)
  if isWindows() then
    return "../../target/" .. target .. "/quick3d.dll"
  else
    return "../../target/" .. target .. "/libquick3d.so"
  end
end

function quick3d_init_luajitffi(target_build)
  if ffi == nil then ffi = require("ffi") end
  print("Using " .. target_build .. " " .. jit.version .. " FFI module")

  local quick3d_interface_file = io.open("../../quick3d.h", "r")
  if not quick3d_interface_file then
    error("Unable to load ../../quick3d.h")
  end
  local quick3d_interface_file_contents = quick3d_interface_file:read("*a")
  quick3d_interface_file:close()
  ffi.cdef(quick3d_interface_file_contents)
  -- LuaJIT doesn't require an additional wrapper library to access the quick3d shared library
  -- however the LuaJIT FFI library needs to load the native declarations in ../../quick3d.i
  -- this file is also used to build the wrapper library (not used here)
  local quick3d_shared_lib_path = get_quick3d_target_path(target_build)
  local success
  success, wrapper = pcall(ffi.load, quick3d_shared_lib_path)
  if not success then
    local cargo_cmd = "cargo build"
    if target_build == "release" then
  	  cargo_cmd = cargo_cmd .. " --release"
    end
    local make_cmd = "cd ../.. && cargo clean -p quick3d && " .. cargo_cmd .. " && cd ffi/Lua"
    print(make_cmd)
    if not os.execute(make_cmd) == 0 then
      error("Unable to execute " .. make_cmd)
    end
    
	wrapper = ffi.load(quick3d_shared_lib_path)
    success, wrapper = pcall(ffi.load, quick3d_shared_lib_path)
	if not success then
	  error("Unable to load LuaJIT FFI module")
	end
  end
  print("Loaded " .. quick3d_shared_lib_path)
  return wrapper
end

function append_shared_lib_ext(filename)
  if isWindows() then
    return filename .. ".dll"
  else
    return filename .. ".so"
  end
end

function prepend_shared_lib_prefix(filename)
  if isWindows() then
    return filename
  else
    return "lib" .. filename
  end
end

-- Clean the shared library residual files
function quick3d_clean(target_build)  
  local quick3d_filename = append_shared_lib_ext("quick3d")
  -- filenames contains a list of files that will attempt to be deleted
  local filenames = {
  	quick3d_filename,
  	append_shared_lib_ext("quick3dwrapper"),
  	append_shared_lib_ext("wrapper/quick3d"),
  	"wrapper/quick3d.i",
	"wrapper/quick3d.h"
  }
  
  if isWindows() then
    table.insert(filenames, "wrapper/quick3d.dll.exp")
    table.insert(filenames, "wrapper/quick3d.dll.lib")
    table.insert(filenames, "wrapper/quick3d.pdb")
    table.insert(filenames, "../../target/" .. target_build .. "/quick3d.dll.exp")
    table.insert(filenames, "../../target/" .. target_build .. "/quick3d.dll.lib")
    table.insert(filenames, "../../target/" .. target_build .. "/quick3d.pdb")
  else
    quick3d_filename = prepend_shared_lib_prefix(quick3d_filename)
    table.insert(filenames, quick3d_filename)
    table.insert(filenames, "wrapper/" .. quick3d_filename)
  end

  if not os.execute("cargo clean") == 0 then
    print("Unable to run cargo clean")
  end  

  local quick3d_clean_cmd = "cargo clean -p quick3d"
  if target_build == "release" then quick3d_clean_cmd = quick3d_clean_cmd .. " --release" end
  local make_result = os.execute("cd ../.. && " .. quick3d_clean_cmd .. " && cd ffi/Lua")
  if not make_result == 0 then
    print("Unable to clean quick3d build target")
    os.exit(3)
  else
    print("Executed " .. quick3d_clean_cmd .. " in ../..")
  end
  
  for i, filename in ipairs(filenames) do
    if os.remove(filename) then
      print("Removed "..filename)
    end
  end
end

function quick3d_read_console_buffer(console)
  local use_luajitffi = not(ffi == nil)
  local console_command = wrapper.read_console_buffer(console)
  -- The LuaJIT FFI api must wrap functions returning char* in ffi.string
  -- The Lua SWIG wrapper version does not require this
  if use_luajitffi then
    console_command = ffi.string(console_command) end
  return console_command
end

-- Load LUA code from a string
function load_string(command)
  -- Load code in a way that supports multiple versions of LUA
  -- See http://stackoverflow.com/questions/9268954/lua-pass-context-into-loadstring
  
  if (setenv or setfenv) and loadstring then
    -- Lua 5.1/5.2
    local context = {}
    setmetatable(context, { __index = _G })
    context.string = string
    context.table = table
    local f = loadstring(command)
    if f == nil then return nil end
    -- The enviroment must be set in order to access the scripts global variables
    if setenv then
      setenv(f, context)
    elseif setfenv then
      setfenv(1, context)
    end
    return f
  else
    -- Lua >= 5.3
    local f = load(command, "function() " ..command .. " end", "t", _ENV)
    return f
  end
end

-- Ensure stdout is captured in all threads
-- This improves support on consoles such as msys
io.stdout:setvbuf 'no'

-- Evaluate an expression or function
function eval(command)
  local f = load_string(command)
  if f == nil then
    -- If f is nill, it could still be an expression in which case we return it and print the value 
    f = load_string("return " .. command)
    assert(f)
    result = f()
    print("Expression returned "..result)
  else
    -- Otherwise we just call the function
    local result = f()
    if not result == nil then
      -- If the function returns a value, print it
      print("Function returned ".. result)
    end
  end
end

-- Camera object wrapper
Camera = {}
Camera.__index = Camera

function Camera.aim(self, x, y)
  self.struct = wrapper.camera_aim(self.struct, x, y)
end

function Camera.create(self, screen_width, screen_height)
  local camera = {}
  setmetatable(camera, Camera)
  self.struct = wrapper.create_camera(screen_width, screen_height)
  return camera
end

function Camera.move_forward(self, amount)
  self.struct = wrapper.camera_move_forward(self.struct, amount)
end

function Camera.move_backward(self, amount)
  self.struct = wrapper.camera_move_backward(self.struct, amount)
end

function Camera.move_left(self, amount)
  self.struct = wrapper.camera_move_left(self.struct, amount)
end

function Camera.move_right(self, amount)
  self.struct = wrapper.camera_move_right(self.struct, amount)
end


-- Display object wrapper
Display = {}
Display.__index = Display

function Display.create(self, screen_width, screen_height, window_title)
  local display = {}
  setmetatable(display, Display)
  self.struct = wrapper.create_display(screen_width, screen_height, window_title)
  return display
end

function Display.hide(self)
  wrapper.window_hide(self.struct)
end

function Display.show(self)
  wrapper.window_show(self.struct)
end


-- Renderer object wrapper
Renderer = {}
Renderer.__index = Renderer

function Renderer.create(self, db_filename, display)
  local renderer = {}
  setmetatable(renderer, Renderer)
  local dbloader = wrapper.create_db_loader(db_filename)
  self.struct = wrapper.create_renderer_from_db_loader(dbloader, display.struct)
  wrapper.free_db_loader(dbloader)
  return renderer
end

function Renderer.render(self, shader, camera, display)
  wrapper.render(self.struct, shader.struct, camera.struct, display.struct)
end


-- Shader object wrapper
Shader = {}
Shader.__index = Shader

function Shader.create(self, name, db_filename, renderer, display)
  local shader = {}
  setmetatable(shader, Shader)
  local dbloader = wrapper.create_db_loader(db_filename)
  self.struct = wrapper.get_shader_from_db_loader(name, dbloader, renderer.struct, display.struct)
  wrapper.free_db_loader(dbloader)
  return shader
end

