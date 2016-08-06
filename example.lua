-- Copyright (C) 2016 Chris Liebert

-- Look for lua modules in the current directory
package.path = package.path .. ";./?.lua"

-- Include and initialise the Quick3D LUA API
require "quick3d"
local quick3d = quick3d_init()

local screen_width = 640
local screen_height = 480
local display = quick3d.create_display(screen_width, screen_height, "My Lua Window")
local camera = quick3d.create_camera(screen_width, screen_height)
local scene_loader = quick3d.create_db_loader("test.db")
local shader_loader = quick3d.create_db_loader("shaders.db")
local renderer = quick3d.create_renderer_from_db_loader(scene_loader, display)
local shader = quick3d.get_shader_from_db_loader("default", shader_loader, renderer, display)
local console = quick3d.create_console_reader()

local mouse_factor = 0.01

function execute_command(command)
    local f = loadstring(command)
    
    if f == nil then
      -- If f is nill, it could still be an expression in which case we return it and print the value
      f = assert(loadstring("return " .. command))
      result = f()
      print("Expression returned "..f())
    else
      -- Otherwise we just call the function
      local result = f()
      if not result == nil then
         -- If the function returns a value, print it
         print("Function returned ".. result)
      end
    end
end

local console_command = ""
while not quick3d.console_is_closed(console) do
    quick3d.render(renderer, shader, camera, display)
    local input = quick3d.poll_event(display)
    if input.mouse.left_button_pressed and not (input.mouse.dx == 0 and input.mouse.dy == 0) then
        quick3d.camera_aim(camera, input.mouse.dx * mouse_factor, input.mouse.dy * mouse_factor)
        quick3d.camera_update(camera)
    end
    -- Read from console buffer
    console_command = quick3d.read_console_buffer(console)
    if string.len(console_command) > 2 then
        if not pcall(execute_command, console_command) then
            print ("Failed to execute command: " .. console_command)
        end
    end
   
    if input.closed then
        quick3d.hide_window(display)
    end
end

quick3d.wait_console_quit(console)

quick3d.free_camera(camera)
quick3d.free_shader(shader)
quick3d.free_renderer(renderer)
quick3d.free_db_loader(shader_loader)
quick3d.free_db_loader(scene_loader)
quick3d.free_display(display)

os.exit() -- Removing this call will cause Lua to crash on exit.
