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
  local make_program = "make"
  if isWindows() then
	make_program = "mingw32-make.exe"
  end
  local make_cmd = make_program.." lualib"
  local make_result = os.execute(make_cmd)
  if not make_result == 0 then
    os.exit(2)
  end
  if isWindows() then
	os.execute("copy target\\debug\\quick3d.dll .")
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

-- Load LUA code from a string
function load_string(command)
  -- Load code in a way that supports multiple versions of LUA
  -- See http://stackoverflow.com/questions/9268954/lua-pass-context-into-loadstring
  
  if setenv and loadstring then
    -- Lua 5.1/5.2
    local context = {}
    setmetatable(context, { __index = _G })
    context.string = string
    context.table = table
    local f = loadstring(command)
    if f == nil then return nil end
    -- The enviroment must be set in order to access the scripts global variables
    setenv(f, context)
    return f
  else
    -- Lua >= 5.3
    local f = load(command, "function() " ..command .. " end", "t", _ENV)
    return f
  end
end

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
  wrapper.camera_aim(self.struct, x, y)
  self:update()
end

function Camera.create(self, screen_width, screen_height)
  local camera = {}
  setmetatable(camera, Camera)
  self.struct = wrapper.create_camera(screen_width, screen_height)
  return camera
end

function Camera.move_forward(self, amount)
  wrapper.camera_move_forward(self.struct, amount)
  self:update()
end

function Camera.move_backward(self, amount)
  wrapper.camera_move_backward(self.struct, amount)
  self:update()
end

function Camera.move_left(self, amount)
  wrapper.camera_move_left(self.struct, amount)
  self:update()
end

function Camera.move_right(self, amount)
  wrapper.camera_move_right(self.struct, amount)
  self:update()
end

function Camera.update(self)
  wrapper.camera_update(self.struct)
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

