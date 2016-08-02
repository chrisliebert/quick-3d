// Copyright (C) 2016 Chris Liebert

#include <assert.h>
#include <malloc.h>
#include <stdio.h>
#include <stdlib.h>
#include "quick3d.h"

Display* new_display() {
	return create_display(800, 600, "My C Window");
}

int main(int argc, char** argv) {
	Display* display = new_display();
	while(poll_quit_event(display) == 0) {}
	free_display(display);
	return 0;
}
