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
    shader = q3d.shader_default(display)
    console_reader = q3d.create_console_reader()
    return display, camera, renderer, shader, console_reader

def destroy(display, camera, renderer, shader):
    q3d.free_display(display)
    q3d.free_camera(camera)
    q3d.free_renderer(renderer)
    q3d.free_shader(shader)
    
def main():
    move_speed = 0.08
    rotate_speed = 0.3
    
    display, camera, renderer, shader, console_reader = init()
    running = True
    while not q3d.console_is_closed(console_reader):
        q3d.render(renderer, shader, camera, display)
        events = q3d.get_events(display)
        if q3d.display_closed(events):
            break
        if q3d.key_pressed(events, q3d.ESCAPE):
            q3d.window_hide(display)
        if q3d.key_pressed(events, q3d.W):
            camera = q3d.camera_move_forward(camera, move_speed)
        if q3d.key_pressed(events, q3d.S):
            camera = q3d.camera_move_backward(camera, move_speed)
        if q3d.key_pressed(events, q3d.A):
            camera = q3d.camera_move_left(camera, move_speed)
        if q3d.key_pressed(events, q3d.D):
            camera = q3d.camera_move_right(camera, move_speed)
        if q3d.key_pressed(events, q3d.LEFT):
            camera = q3d.camera_aim(camera, rotate_speed, 0)
        if q3d.key_pressed(events, q3d.RIGHT):
            camera = q3d.camera_aim(camera, -rotate_speed, 0)		
        q3d.free_events(events)
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
    #clean up
    destroy(display, camera, renderer, shader)

if __name__=='__main__':
    main()
