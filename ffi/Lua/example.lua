-- Copyright (C) 2016 Chris Liebert

-- Look for Lua modules in the current directory
package.path = package.path .. ";./?.lua"

-- Include and initialize the Quick3D LUA API
require "quick3d"

local target_build = "debug"
local use_luajitffi = (type(jit) == 'table')

local quit_after_start = false
local sqlite3 = false
local sqlite_file = ""
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
  elseif arg[i] == "sqlite" then
    print("Enabling SQLite")
    sqlite3 = true
	sqlite_file = arg[i + 1]
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
  quick3d = quick3d_init_luajitffi(target_build, sqlite3)
else
-- Initialize Quick3D
  quick3d = quick3d_init(target_build, sqlite3)
end

screen_width = 800
screen_height = 600
display = Display:create(screen_width, screen_height, "My Lua Window")
camera = Camera:create(screen_width, screen_height)
camera:move_backward(6)

-- Load ../../test.db unless a file ending in .obj or .db is specified as an argument

scene_file = "../../test.bin.gz"

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
  return Renderer:create_from_compressed_binary(scene_file, display)
end

renderer = create_renderer()

shader = Shader:default(display)

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
mouse_factor = 0.1

display:show()
local console_command = ""
local mouse_x, mouse_y = 0, 0
local mouse_last_x, mouse_last_y = 0, 0
local mouse_dx, mouse_dy = 0, 0
local mouse_left_pressed = false
while not quick3d.console_is_closed(console) do
  renderer:render(shader, camera, display)
  local events = EventBuffer:get(display)
  
  -- Debug event queue
  -- if not events:empty() then events:print() end
  
  if events:display_closed() then
    events:free()
    quit()
    break
  end


  if events:key_pressed(quick3d.ESCAPE) then display:hide() break end
  if events:key_pressed(quick3d.W) then camera:move_forward(camera_speed) end
  if events:key_pressed(quick3d.S) then camera:move_backward(camera_speed) end
  if events:key_pressed(quick3d.A) then camera:move_left(camera_speed) end
  if events:key_pressed(quick3d.D) then camera:move_right(camera_speed) end
  mouse_x, mouse_y = events:mouse_moved()
  if mouse_x == 0 then
    mouse_dx = 0
  else
    mouse_dx = mouse_x - mouse_last_x
    mouse_last_x = mouse_x
  end
  if mouse_y == 0 then
    mouse_dy = 0  
  else
    mouse_dy = mouse_y - mouse_last_y
    mouse_last_y = mouse_y
  end
     
  if events:mouse_pressed_left() then
    mouse_left_pressed = true
  end
  
  if events:mouse_released_left() then
    mouse_left_pressed = false
  end
  
  if mouse_left_pressed and not (mouse_dx == 0 or mouse_dy == 0) then
    camera:aim(mouse_dx * mouse_factor, mouse_dy * mouse_factor)
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
  
  events:free()
end

quick3d.wait_console_quit(console)
quit()
