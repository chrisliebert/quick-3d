// Copyright (C) 2016 Chris Liebert

#include <malloc.h>
#include <stdio.h>
#include <stdlib.h>
#include "quick3d.h"

int main(int argc, char** argv) {
	Window window = create_window(800, 600, "My C Window");

	while(poll_quit_event(&window) == 0) {
		//render
	}
}
