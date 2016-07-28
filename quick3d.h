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

typedef void* DBLoader;
typedef void* Camera;
typedef void* Renderer;
typedef void* Shader;
typedef void* Window;

extern DBLoader create_db_loader(const char* filename);
extern Camera create_camera(float screen_width, float screen_height);
extern Renderer create_renderer_from_db_loader(DBLoader loader, Window window);
extern Window create_window(int screen_width, int screen_height, const char* title);

extern void free_db_loader(DBLoader dbloader);
extern void free_camera(Camera camera);
extern void free_renderer(Renderer renderer);
extern void free_shader(Shader shader);
extern void free_window(Window memory);

extern Shader get_shader_from_db_loader(const char* name, DBLoader dbloader, Renderer renderer, Window display);
extern int poll_quit_event(Window window);
extern void render(Renderer renderer, Shader shader, Camera camera, Window window);

#endif
