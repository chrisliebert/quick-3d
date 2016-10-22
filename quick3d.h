/* Copyright (C) 2016 Chris Liebert */

typedef void* Camera;
typedef void* DBLoader;
typedef void* Renderer;
typedef void* Shader;
typedef void* Display;
typedef void* ConsoleInput;
typedef void* EventBuffer;

typedef enum KeyCode {
    KEY1, KEY2, KEY3, KEY4, KEY5, KEY6, KEY7, KEY8, KEY9, KEY0,
    A, B, C, D, E, F, G, H,I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    ESCAPE,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11,F12, F13, F14, F15,
    SNAPSHOT, SCROLL, PAUSE, INSERT, HOME, DELETE, END, PAGEDOWN, PAGEUP,
	LEFT, UP, RIGHT, DOWN, BACK, RETURN, SPACE, NUMLOCK,
    NUMPAD0, NUMPAD1, NUMPAD2, NUMPAD3, NUMPAD4, NUMPAD5, NUMPAD6, NUMPAD7, NUMPAD8, NUMPAD9,
    ABNTC1, ABNTC2, ADD, APOSTROPHE, APPS, AT, AX, BACKSLASH, CALCULATOR, CAPITAL, COLON, COMMA,
    CONVERT, DECIMAL, DIVIDE, EQUALS, GRAVE, KANA, KANJI, LALT, LBRACKET, LCONTROL, LMENU, LSHIFT, LWIN,
    MAIL, MEDIASELECT, MEDIASTOP, MINUS, MULTIPLY, MUTE, MYCOMPUTER, NAVIGATEFORWARD, NAVIGATEBACKWARD, NEXTTRACK,
	NOCONVERT, NUMPADCOMMA, NUMPADENTER, NUMPADEQUALS, OEM102, PERIOD, PLAYPAUSE, POWER, PREVTRACK, RALT, RBRACKET,
	RCONTROL, RMENU, RSHIFT, RWIN, SEMICOLON, SLASH, SLEEP, STOP, SUBTRACT, SYSRQ, TAB, UNDERLINE, UNLABELED,
	VOLUMEDOWN, VOLUMEUP, WAKE, WEBBACK, WEBFAVORITES, WEBFORWARD, WEBHOME, WEBREFRESH, WEBSEARCH, WEBSTOP, YEN,
} KeyCode;

typedef struct Mouse {
	int x, y;
} Mouse;

extern Camera camera_aim(Camera camera, double x, double y);
extern Camera camera_move_forward(Camera camera, float amount);
extern Camera camera_move_backward(Camera camera, float amount);
extern Camera camera_move_left(Camera camera, float amount);
extern Camera camera_move_right(Camera camera, float amount);
extern Camera create_camera(float screen_width, float screen_height);
extern ConsoleInput create_console_reader();
extern DBLoader create_db_loader(const char* filename);
extern Display create_display(int screen_width, int screen_height, const char* title);
extern Display create_hidden_display(int screen_width, int screen_height, const char* title);
extern Renderer create_renderer_from_binary(const char* filename, Display display);
extern Renderer create_renderer_from_compressed_binary(const char* filename, Display display);
extern Renderer create_renderer_from_db_loader(DBLoader loader, Display display);
extern bool console_is_closed(ConsoleInput console);

extern bool display_closed(EventBuffer event);
extern EventBuffer get_events(Display display);
extern bool events_empty(EventBuffer events);

extern void print_events(EventBuffer events);
extern bool key_pressed(EventBuffer event, KeyCode keycode);
extern bool key_released(EventBuffer event, KeyCode keycode);
extern bool mouse_pressed_left(EventBuffer event);
extern bool mouse_pressed_right(EventBuffer event);
extern bool mouse_released_left(EventBuffer event);
extern bool mouse_released_right(EventBuffer event);
extern Mouse* mouse_moved(EventBuffer event);

extern void free_camera(Camera camera);
extern void free_db_loader(DBLoader dbloader);
extern void free_display(Display memory);
extern void free_events(EventBuffer events);
extern void free_mouse(Mouse* mouse);
extern void free_renderer(Renderer renderer);
extern void free_shader(Shader shader);

extern Shader get_shader_from_dbloader(const char* name, DBLoader dbloader, Display display);
extern Shader get_shader_from_source(const char* vertex, const char* fragment, Display display);
extern Shader shader_default(Display display);
extern bool shader_source_is_valid(const char* vertex, const char* fragment, Display display);
extern char* read_console_buffer(ConsoleInput console);
extern void render(Renderer renderer, Shader shader, Camera camera, Display display);
extern void wait_console_quit(ConsoleInput console);
extern void window_hide(Display display);
extern void window_show(Display display);
extern void thread_sleep(int ms);
extern void thread_yield();

/* C/C++ Methods */
extern void obj2sqlite(const char* wavefront, const char* database);
extern void obj2bin(const char* wavefront, const char* filename);
extern void obj2compressed(const char* wavefront, const char* filename);
