#!/usr/bin/env python
# Copyright (C) 2016 Chris Liebert
import quick3dwrapper as q3d

screen_width, screen_height = 640, 480

def init():
    display = q3d.create_display(screen_width, screen_height, "PyQuick3D")
    camera = q3d.create_camera(screen_width, screen_height)
    console_reader = q3d.create_console_reader()
    return display, camera, console_reader

def destroy(display, camera):
    q3d.free_display(display)

def main():
    display, camera, console_reader = init()
    running = True
    while not q3d.console_is_closed(console_reader):
        event = q3d.poll_event(display)
        if event.closed:
            break
        if event.escape:
            q3d.window_hide(display)
        
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
    destroy(display)

if __name__=='__main__':
    main()
