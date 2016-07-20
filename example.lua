-- Copyright (C) 2016 Chris Liebert

-- Look for lua modules in the current directory
package.path = package.path .. ";./?.lua"

-- Include and initialise the Quick3D LUA API
require "quick3d"
local quick3d = quick3d_init()

local window = quick3d.create_window(640, 480, "My Lua Window")
local loader = quick3d.create_db_loader("test.db")
local renderer = quick3d.create_renderer_from_db_loader(loader, window)
local shader = quick3d.get_shader_from_db_loader("default", loader, renderer, window)

while quick3d.poll_quit_event(window) == 0 do
    quick3d.render(renderer, shader, window)
end

shader = quick3d.take_memory(shader)
renderer = quick3d.take_memory(renderer)
loader = quick3d.take_memory(loader)
window = quick3d.take_memory(window)

print "Quit"

os.exit() -- Removing this call will cause Lua to crash on exit.
