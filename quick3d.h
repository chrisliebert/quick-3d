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

typedef struct Scene {
	void* materials;
	void* meshes;
	void* images;
} Scene;

typedef struct Window {
	void* context;
	void* backend;
} Window;

extern Window create_window(int screen_width, int screen_height, const char* title);
extern DBLoader* create_db_loader(const char* filename);
extern int poll_quit_event(Window* window);

extern void hello();


#endif
