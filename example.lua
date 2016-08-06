-- Copyright (C) 2016 Chris Liebert

-- Look for lua modules in the current directory
package.path = package.path .. ";./?.lua"

-- Ensure stdout is captured in all threads
-- This improves support on consoles such as msys
io.stdout:setvbuf 'no'

-- Include and initialise the Quick3D LUA API
require "quick3d"
quick3d = quick3d_init()

screen_width = 640
screen_height = 480
display = quick3d.create_display(screen_width, screen_height, "My Lua Window")
camera = quick3d.create_camera(screen_width, screen_height)
scene_loader = quick3d.create_db_loader("test.db")
shader_loader = quick3d.create_db_loader("shaders.db")
renderer = quick3d.create_renderer_from_db_loader(scene_loader, display)
shader = quick3d.get_shader_from_db_loader("default", shader_loader, renderer, display)
console = quick3d.create_console_reader()

mouse_factor = 0.01

local function get_context()
	local context = {}

	-- Put all the global variables in the command context
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
    if string.len(console_command) > 0 then	
		local context = get_context()
        local success, errormsg = pcall(execute_command, console_command, context)
		if not success then
            print ("Failed to execute command: " .. console_command)
			print("Error: " .. errormsg)
        end
    end
   
    if input.closed then
        quick3d.window_hide(display)
    end
end

quick3d.wait_console_quit(console)

quick3d.free_camera(camera)
quick3d.free_shader(shader)
quick3d.free_renderer(renderer)
quick3d.free_db_loader(shader_loader)
quick3d.free_db_loader(scene_loader)
quick3d.free_display(display)

collectgarbage()

os.exit() -- Removing this call will cause Lua to crash on exit.
