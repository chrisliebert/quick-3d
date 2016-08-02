-- Copyright (C) 2016 Chris Liebert

-- Look for lua modules in the current directory
package.path = package.path .. ";./?.lua"

-- Include and initialise the Quick3D LUA API
require "quick3d"
local quick3d = quick3d_init()

local screen_with = 640
local screen_height = 480
local display = quick3d.create_display(screen_with, screen_height, "My Lua Window")
local camera = quick3d.create_camera(screen_with, screen_height)
local scene_loader = quick3d.create_db_loader("test.db")
local shader_loader = quick3d.create_db_loader("shaders.db")
local renderer = quick3d.create_renderer_from_db_loader(scene_loader, display)
local shader = quick3d.get_shader_from_db_loader("default", shader_loader, renderer, display)

local running = true

while running do
    quick3d.render(renderer, shader, camera, display)
    
    local event = quick3d.poll_event(display)
    print ("mouse: ".. mouse_event.dx .. ", " .. mouse_event.dy)
    if event.quit then running = false end
end

quick3d.free_camera(camera)
quick3d.free_shader(shader)
quick3d.free_renderer(renderer)
quick3d.free_db_loader(shader_loader)
quick3d.free_db_loader(scene_loader)
quick3d.free_display(display)

print "Quit"

os.exit() -- Removing this call will cause Lua to crash on exit.
