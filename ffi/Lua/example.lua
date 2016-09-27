-- Copyright (C) 2016 Chris Liebert

-- Look for lua modules in the current directory
package.path = package.path .. ";./?.lua"

-- Include and initialise the Quick3D LUA API
require "quick3d"

local target_build = "debug"
local use_luajitffi = (type(jit) == 'table')

local quit_after_start = false

-- Parse program arguments
local i = 1
while arg[i] do
  if arg[i] == "--noluajitffi" then
    use_luajitffi = false
  -- Clean shared libraries if "clean" is the first argument
  elseif arg[i] == "clean" then
    print("Cleaning shared libraries")
    if arg[i] == "release" then
      target_build = "release"
    end
    -- Initialize Quick3D
    quick3d_clean(target_build)
  elseif arg[i] == "release" then
    if use_luajitffi then
      target_build = "release"
    else
      print("Did you mean `clean release`? `release` is only valid when using the LuaJIT FFI")
    end
  elseif arg[i] == "--quit" then
    quit_after_start = true
  end

  i = i + 1
end

local quick3d = nil

if use_luajitffi then
  quick3d = quick3d_init_luajitffi(target_build)
else
-- Initialize Quick3D
  quick3d = quick3d_init(target_build)
end

screen_width = 800
screen_height = 600
display = Display:create(screen_width, screen_height, "My Lua Window")
display:hide()
camera = Camera:create(screen_width, screen_height)
camera:move_backward(6)

-- Load ../../test.db unless a file ending in .obj or .db is specified as an argument

database_file = "../../test.db"

function create_renderer()
  local i = 1
  -- Parse program arguments ending in ".db" or ".obj"
  while arg[i] do
    if string.sub(arg[i], -3) == ".db" then
      database_file = arg[i]
      return Renderer:create_from_database(database_file, display)
    elseif string.sub(arg[i], -4) == ".obj" then
      local wavefront_file = arg[i]
      -- If the argument specified after the .obj file is '--compressed', use compression
      if arg[i + 1] == "--compressed" then
        local bin_file = wavefront_file .. ".bin.gz"
        quick3d.obj2compressed(arg[i], bin_file)
        return Renderer:create_from_compressed_binary(bin_file, display)
      else
        local bin_file = wavefront_file .. ".bin"
        quick3d.obj2bin(arg[i], bin_file)
        return Renderer:create_from_binary(bin_file, display)
      end
    elseif string.sub(arg[i], -4) == ".bin" then
      return Renderer:create_from_binary(arg[i], display)
    elseif string.sub(arg[i], -3) == ".gz" then
      return Renderer:create_from_compressed_binary(arg[i], display)
    end
    i = i + 1
  end
  return Renderer:create_from_database(database_file, display)
end

renderer = create_renderer()

shader = Shader:create("default", "../../shaders.db", renderer, display)

function quit()
  quick3d.free_shader(shader.struct)
  quick3d.free_renderer(renderer.struct)
  quick3d.free_display(display.struct)
  quick3d.free_camera(camera.struct)
  collectgarbage()
  os.exit() -- Removing this call will cause Lua to crash on exit.
end

if quit_after_start then quit() end

console = quick3d.create_console_reader()

camera_speed = 0.01
mouse_factor = 0.01

function demo()
  -- Make the camera move in a circle, user can abort by pressing space
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

display:show()
local console_command = ""
while not quick3d.console_is_closed(console) do
  renderer:render(shader, camera, display)
  local input = quick3d.poll_event(display.struct)
  if input.mouse_left and not (input.mouse_dx == 0 and input.mouse_dy == 0) then
    camera:aim(input.mouse_dx * mouse_factor, input.mouse_dy * mouse_factor)
  end
  -- Read from console buffer
  console_command = quick3d_read_console_buffer(console)
  if string.len(console_command) > 0 then	
    local success, errormsg = pcall(eval, console_command)
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
    quick3d.free_event(input)
    quit()
    break
  end
  
  quick3d.free_event(input)
end

quick3d.wait_console_quit(console)
quit()
