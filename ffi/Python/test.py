#!/usr/bin/env python
# Copyright (C) 2016 Chris Liebert

try:
    import quick3dwrapper as q3d
except:
    import os
    os.system("cargo build")
    import quick3dwrapper as q3d

def init():
    screen_width, screen_height = 640, 480
    display = q3d.create_display(screen_width, screen_height, "PyQuick3DTest")
    camera = q3d.create_camera(screen_width, screen_height)
    camera = q3d.camera_move_backward(camera, 6)
    renderer = q3d.create_renderer_from_compressed_binary("../../test.bin.gz", display)
    shader = q3d.shader_default(display)
    return display, camera, renderer, shader

def destroy(display, camera, renderer, shader):
    q3d.free_display(display)
    q3d.free_camera(camera)
    q3d.free_renderer(renderer)
    q3d.free_shader(shader)
    
def main():
    display, camera, renderer, shader = init()
    q3d.render(renderer, shader, camera, display)
    # Sleep 100 ms
    q3d.thread_sleep(100)
    # Clean up
    destroy(display, camera, renderer, shader)

if __name__=='__main__':
    main()
