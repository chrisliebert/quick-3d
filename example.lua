-- Copyright (C) 2016 Chris Liebert

-- Look for lua modules in the current directory
package.path = package.path .. ";./?.lua"

-- Ensure stdout is captured in all threads
-- This improves support on consoles such as msys
io.stdout:setvbuf 'no'

-- Include and initialise the Quick3D LUA API
require "quick3d"

-- Initialize Quick3D
quick3d = quick3d_init()

screen_width = 800
screen_height = 600
display = Display:create(screen_width, screen_height, "My Lua Window")
camera = Camera:create(screen_width, screen_height)
shader_loader = quick3d.create_db_loader("shaders.db")
renderer = Renderer:create("test.db", display)
shader = quick3d.get_shader_from_db_loader("default", shader_loader, renderer.struct, display.struct)
console = quick3d.create_console_reader()

camera_speed = 0.01
mouse_factor = 0.01

-- Put all the global variables in the command context
local function get_context()
  local context = {}
  setmetatable(context, { __index = _G })
  context.string = string
  context.table = table
  return context
end

local function execute_command(command, environment)
  -- TODO: only use the depricated setenv()/loadstring() if load() is unavailible
  -- See http://stackoverflow.com/questions/9268954/lua-pass-context-into-loadstring

  local f = loadstring(command)
  if f == nil then
    -- If f is nill, it could still be an expression in which case we return it and print the value 
    f = loadstring("return " .. command)
    assert(f)
    -- The enviroment must be set in order to access the scripts global variables
    setfenv(f, environment)
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

function demo()
  
  for i=1000,10000 do 
    if i < 1040 then camera:move_right(0.001) end
    camera:move_forward(i * 0.0001) camera:aim(i * 0.001, 0)
    renderer:render(shader, camera, display)
    quick3d.thread_sleep(10)
    local input = quick3d.poll_event(display.struct)    
    if input.space then
      return
    end
   end
end

function quit()
  quick3d.free_camera(camera.struct)
  quick3d.free_shader(shader)
  quick3d.free_renderer(renderer.struct)
  quick3d.free_db_loader(shader_loader)
  quick3d.free_display(display.struct)
  collectgarbage()
  os.exit() -- Removing this call will cause Lua to crash on exit.
end

local console_command = ""
while not quick3d.console_is_closed(console) do
  renderer:render(shader, camera, display)
  local input = quick3d.poll_event(display.struct)
  if input.mouse_left and not (input.mouse_dx == 0 and input.mouse_dy == 0) then
    camera:aim(input.mouse_dx * mouse_factor, input.mouse_dy * mouse_factor)
  end
  -- Read from console buffer
  console_command = quick3d.read_console_buffer(console)
  if string.len(console_command) > 0 then	
    local context = get_context()
    local success, errormsg = pcall(execute_command, console_command, context)
    if not success then
      print ("Failed to execute command: " .. console_command)
      print("Error: " .. errormsg)
    end
  end

  if input.w or input.up then
    camera:move_forward(camera_speed)
  end

  if input.a or input.left then
    camera:move_left(camera_speed)
  end

  if input.d or input.right then
    camera:move_right(camera_speed)
  end
  
  if input.s or input.down then
    camera:move_backward(camera_speed)
  end

  if input.escape then
    display:hide()
  end

  if input.closed then
    quit()
  end
end

quick3d.wait_console_quit(console)
quit()
