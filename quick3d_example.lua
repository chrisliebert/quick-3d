-- Copyright (C) 2016 Chris Liebert

require "quick3d"
local quick3d = quick3d_init()

local window = quick3d.create_window(640, 480, "My Lua Window")
--local loader = quick3d.create_db_loader("test.db")
--local shader = loader.load_shader("default")
while quick3d.poll_quit_event(window) == 0 do
	--renderer.render(window, shader)
end

print "Quit"

os.exit() -- Removing this call will cause Lua to crash on exit.