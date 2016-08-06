/* Copyright (C) 2016 Chris Liebert */
#ifndef _QUICK3D_H_
#define _QUICK3D_H_

#ifdef SWIG
 %module quick3dwrapper

 %{
  #include "quick3d.h"
 %}

#endif

#include <stdio.h>
#include <stdbool.h>
#include <stdint.h>

typedef void* Camera;
typedef void* DBLoader;
typedef void* Renderer;
typedef void* Shader;
typedef void* Display;
typedef void* ConsoleInput;

typedef struct Mouse {
	int dx, dy, last_x, last_y;
	bool left_button_pressed, right_button_pressed;
} Mouse;

typedef struct Input {
	Mouse mouse;
	bool closed;
} Input;

extern void camera_aim(Camera camera, double x, double y);
extern void camera_move_forward(Camera camera, float amount);
extern void camera_move_backward(Camera camera, float amount);
extern void camera_move_left(Camera camera, float amount);
extern void camera_move_right(Camera camera, float amount);
extern void camera_update(Camera camera);

extern Camera create_camera(float screen_width, float screen_height);
extern DBLoader create_db_loader(const char* filename);
extern Display create_display(int screen_width, int screen_height, const char* title);
extern Renderer create_renderer_from_db_loader(DBLoader loader, Display display);

extern void free_camera(Camera camera);
extern void free_db_loader(DBLoader dbloader);
extern void free_display(Display memory);
extern void free_renderer(Renderer renderer);
extern void free_shader(Shader shader);

extern void thread_sleep(int ms);
extern void thread_yield();
extern ConsoleInput create_console_reader();
extern bool console_is_closed(ConsoleInput console);
extern char* read_console_buffer(ConsoleInput console);
extern void wait_console_quit(ConsoleInput console);

extern Shader get_shader_from_db_loader(const char* name, DBLoader dbloader, Renderer renderer, Display display);
extern Input poll_event(Display display);
extern void window_hide(Display display);
extern void window_show(Display display);
extern void render(Renderer renderer, Shader shader, Camera camera, Display display);

#endif
