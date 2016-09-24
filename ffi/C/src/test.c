// Copyright (C) 2016 Chris Liebert

#include <assert.h>
#include <malloc.h>
#include <stdlib.h>
#include <stdbool.h>
#include <stdint.h>
#include "quick3d.h"

int main(int argc, char** argv) {
	Display* display = create_display(800, 600, "My C Test Window");
	free_display(display);
	return 0;
}
