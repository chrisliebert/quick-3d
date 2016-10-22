// Copyright (C) 2016 Chris Liebert

#include <assert.h>
#include <malloc.h>
#include <stdlib.h>
#include <stdbool.h>
#include <stdint.h>
#include "quick3d.h"

Display* new_display() {
	return create_display(800, 600, "My C Window");
}

int main(int argc, char** argv) {
	Display* display = new_display();
	bool running = true;
	int ESCAPE = 1;
	while(running) {
		EventBuffer events = get_events(display);
		running = !display_closed(events) && !key_pressed(events, ESCAPE);
		free_events(events);
	}
	free_display(display);
	return 0;
}
