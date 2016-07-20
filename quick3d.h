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

typedef struct DBLoader {
	char* filename;
} DBLoader;

typedef struct Program {
	void* loc;///
} Program;

typedef struct Renderer {
	//
	void* loc;
} Renderer;

typedef struct Scene {
	void* materials;
	void* meshes;
	void* images;
} Scene;

typedef struct Window {
	void* context;
	void* backend;
} Window;

extern DBLoader* create_db_loader(const char* filename);
extern Renderer* create_renderer_from_db_loader(DBLoader* loader, Window* window);
extern Window* create_window(int screen_width, int screen_height, const char* title);
extern int poll_quit_event(Window* window);
extern Program* get_shader_from_db_loader(const char* name, DBLoader* dbloader, Renderer* renderer, Window* display);
extern void render(Renderer* renderer, Program* program, Window* window);
extern void* take_memory(void* memory);

#endif
