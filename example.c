// Copyright (C) 2016 Chris Liebert

#include <assert.h>
#include <malloc.h>
#include <stdio.h>
#include <stdlib.h>
#include "quick3d.h"

Window* new_window() {
	return create_window(800, 600, "My C Window");
}

int main(int argc, char** argv) {
	Window* window = new_window();
	while(poll_quit_event(window) == 0) {}
	free(take_memory(window));
}
