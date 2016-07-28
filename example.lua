-- Copyright (C) 2016 Chris Liebert

-- Look for lua modules in the current directory
package.path = package.path .. ";./?.lua"

-- Include and initialise the Quick3D LUA API
require "quick3d"
local quick3d = quick3d_init()

local window = quick3d.create_window(640, 480, "My Lua Window")
local scene_loader = quick3d.create_db_loader("test.db")
local shader_loader = quick3d.create_db_loader("shaders.db")
local renderer = quick3d.create_renderer_from_db_loader(scene_loader, window)
local shader = quick3d.get_shader_from_db_loader("default", shader_loader, renderer, window)

while quick3d.poll_quit_event(window) == 0 do
    quick3d.render(renderer, shader, window)
end

quick3d.free_shader(shader)
quick3d.free_renderer(renderer)
quick3d.free_db_loader(shader_loader)
quick3d.free_db_loader(scene_loader)
quick3d.free_window(window)

print "Quit"

os.exit() -- Removing this call will cause Lua to crash on exit.
