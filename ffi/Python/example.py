#!/usr/bin/env python
# Copyright (C) 2016 Chris Liebert

try:
    import quick3dwrapper as q3d
except:
    import os
    os.system("cargo build")
    import quick3dwrapper as q3d

screen_width, screen_height = 640, 480

def init():
    display = q3d.create_display(screen_width, screen_height, "PyQuick3D")
    camera = q3d.create_camera(screen_width, screen_height)
    camera = q3d.camera_move_backward(camera, 6)
    renderer = q3d.create_renderer_from_compressed_binary("../../test.bin.gz", display)
    shaderloader = q3d.create_db_loader("../../shaders.db")
    shader = q3d.get_shader_from_dbloader("default", shaderloader, display)
    q3d.free_db_loader(shaderloader)
    console_reader = q3d.create_console_reader()
    return display, camera, renderer, shader, console_reader

def destroy(display, camera, renderer, shader):
    q3d.free_display(display)
    q3d.free_camera(camera)
    q3d.free_renderer(renderer)
    q3d.free_shader(shader)
    
def main():
    move_speed = 0.01
    rotate_speed = 0.1
    
    display, camera, renderer, shader, console_reader = init()
    running = True
    while not q3d.console_is_closed(console_reader):
        q3d.render(renderer, shader, camera, display)
        event = q3d.poll_event(display)
        if event.closed:
            break
        if event.escape:
            q3d.window_hide(display)
        if event.w:
            camera = q3d.camera_move_forward(camera, move_speed)
        if event.s:
            camera = q3d.camera_move_backward(camera, move_speed)
        if event.a:
            camera = q3d.camera_move_left(camera, move_speed)
        if event.d:
            camera = q3d.camera_move_right(camera, move_speed)
        if event.left:
            camera = q3d.camera_aim(camera, rotate_speed, 0)
        if event.right:
            camera = q3d.camera_aim(camera, -rotate_speed, 0)
        
        console_command = q3d.read_console_buffer(console_reader)
        if len(console_command) > 0:
            try:
                result = eval(console_command)
                print("Expression returned " + str(result))
            except:
	        try:
	            #Expression failed to evaluate
	            #try to execute the command as a statement instead
	            exec(console_command)
	        except:
	            print("Invalid command: " + console_command)
        q3d.free_event(event)
    #clean up
    destroy(display, camera, renderer, shader)

if __name__=='__main__':
    main()
