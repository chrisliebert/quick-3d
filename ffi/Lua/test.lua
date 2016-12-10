-- Copyright (C) 2016 Chris Liebert

-- Look for lua modules in the current directory
package.path = package.path .. ";./?.lua"

-- Include and initialise the Quick3D LUA API
require "quick3d"

local target_build = "debug"
local use_luajitffi = (type(jit) == 'table')

-- Parse program arguments
local i = 1
local sqlite = false
while arg[i] do
  if arg[i] == "--noluajitffi" then
    use_luajitffi = false
  -- Clean shared libraries if "clean" is the first argument
  elseif arg[i] == "release" then
    target_build = "release"
  elseif arg[i] == "sqlite" then
    sqlite = true;
  end
  i = i + 1
end

local quick3d = nil

if use_luajitffi then
  quick3d = quick3d_init_luajitffi(target_build, sqlite)
else
-- Initialize Quick3D
  quick3d = quick3d_init(target_build, sqlite)
end

screen_width = 800
screen_height = 600
display = Display:create(screen_width, screen_height, "My Lua Window")
camera = Camera:create(screen_width, screen_height)
camera:move_backward(6)

scene_file = "../../test.bin.gz"
renderer = Renderer:create_from_compressed_binary(scene_file, display)
shader = Shader:default(display)

renderer:render(shader, camera, display)
quick3d.thread_sleep(100)

quick3d.free_shader(shader.struct)
quick3d.free_renderer(renderer.struct)
quick3d.free_display(display.struct)
quick3d.free_camera(camera.struct)
collectgarbage()
os.exit() -- Removing this call will cause Lua to crash on exit.
