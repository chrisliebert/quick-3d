/* Copyright (C) 2016 Chris Liebert */

typedef void* Camera;
typedef void* DBLoader;
typedef void* Renderer;
typedef void* Shader;
typedef void* Display;
typedef void* ConsoleInput;

typedef struct Input {
	bool key_1, key_2, key_3, key_4, key_5, key_6, key_7, key_8, key_9, key_0;
	bool a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z;
	bool up, left, right, down;
	bool space;
	bool escape;
	bool closed;
	int mouse_dx, mouse_dy;
	int mouse_x, mouse_y;
	bool mouse_left, mouse_right;
} Input;

extern Camera camera_aim(Camera camera, double x, double y);
extern Camera camera_move_forward(Camera camera, float amount);
extern Camera camera_move_backward(Camera camera, float amount);
extern Camera camera_move_left(Camera camera, float amount);
extern Camera camera_move_right(Camera camera, float amount);
extern Camera create_camera(float screen_width, float screen_height);
extern ConsoleInput create_console_reader();
extern DBLoader create_db_loader(const char* filename);
extern Display create_display(int screen_width, int screen_height, const char* title);
extern Renderer create_renderer_from_db_loader(DBLoader loader, Display display);
extern bool console_is_closed(ConsoleInput console);

extern void free_camera(Camera camera);
extern void free_db_loader(DBLoader dbloader);
extern void free_display(Display memory);
extern void free_event(Input* memory);
extern void free_renderer(Renderer renderer);
extern void free_shader(Shader shader);

extern Shader get_shader_from_db_loader(const char* name, DBLoader dbloader, Renderer renderer, Display display);
extern Input* poll_event(Display display);
extern char* read_console_buffer(ConsoleInput console);
extern void render(Renderer renderer, Shader shader, Camera camera, Display display);
extern void wait_console_quit(ConsoleInput console);
extern void window_hide(Display display);
extern void window_show(Display display);
extern void thread_sleep(int ms);
extern void thread_yield();

/* C/C++ Methods */
extern void obj2sqlite(const char* wavefront, const char* database);

/* libgamepad_ Wrapper Methods */
extern void gamepad_init();
extern void gamepad_shutdown();
extern void gamepad_update();
extern bool gamepad_is_connected(int device);
extern bool gamepad_button_down(int device, int button);
extern bool gamepad_button_triggered(int device, int button);
extern bool gamepad_button_released(int device, int button);
extern int gamepad_trigger_value(int device, int trigger);
extern float gamepad_trigger_length(int device, int trigger);
extern bool gamepad_trigger_down(int device, int trigger);
extern bool gamepad_trigger_triggered(int device, int trigger);
extern bool gamepad_trigger_released(int device, int trigger);
extern void gamepad_set_rumble(int device, float left, float right);
extern void gamepad_stick_xy(int device, int stick, int* out_x, int* out_y);
extern void gamepad_stick_norm_xy(int device, int stick, float* out_x, float* out_y);
extern float gamepad_stick_length(int device, int stick);
extern float gamepad_stick_angle(int device, int stick);
extern int gamepad_stick_dir(int device, int stick);
extern bool gamepad_stick_dir_triggered(int device, int stick, int dir);