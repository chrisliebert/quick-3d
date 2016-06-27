#[macro_use]
extern crate glium;
extern crate glium_sdl2;
extern crate nalgebra;
extern crate sdl2;
//extern crate sdl2_image;

//use glium::index::PrimitiveType;
//use sdl2_image::{LoadTexture, INIT_PNG, INIT_JPG};

use glium::uniforms::UniformBuffer;
use glium::buffer::BufferMode;
use glium::buffer::BufferType;
use loader::Scene;
use loader::Vertex8f32;
use nalgebra::{Matrix4, PerspectiveMatrix3};

mod loader;

fn main() {
    use glium_sdl2::DisplayBuild;
    use glium::Surface;
    use sdl2::keyboard::Keycode;
    
    let screen_width = 800;
    let screen_height = 600;

	let db_file: &str = "test.db";

	let scene: Scene = loader::load_db(db_file);
    println!("Loaded {} vertices", scene.vertices.len());

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let display = video_subsystem.window(db_file, screen_width, screen_height)
        //.resizable()
        .build_glium()
        .unwrap();
        
   /*
       let _image_context = sdl2_image::init(INIT_PNG | INIT_JPG).unwrap();
    let hidden_window = video_subsystem.window("", 0, 0).hidden().build().unwrap();
    // software() <- accelerated()
	let renderer = hidden_window.renderer().software().build().unwrap();
	let texture = renderer.load_texture(Path::new("test.png")).unwrap();
*/
    
    implement_vertex!(Vertex8f32, position, normal, texcoord);
    let vertex_buffer = glium::vertex::VertexBuffer::new(&display, &scene.vertices).unwrap();
    let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

	//TODO: create version 330 shader that uses uniform block
    let vertex_shader_src = r#"
 #version 140
 //buffer layout(std140);
 //layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
 
 // Input data
 in vec3 position;
 in vec3 normal;
 in vec2 texcoord;

// Uniforms
//buffer ViewMatrixBlock {
	 uniform mat4 projection;
	 uniform mat4 modelview;
//};

uniform vec3 light_position_worldspace;

// Output data
out vec3 out_position;
out vec3 out_normal;
out vec2 out_texcoord;
out vec3 camera_direction;
out vec3 light_direction;
out vec3 light_position;

 void main() {
	mat4 mvp = projection * modelview;
	out_position = (mvp * vec4(position, 1.0)).xyz;
	camera_direction = normalize(vec3(0, 0, 0) - out_position);
	light_direction = vec3(0,-1, 0);
	light_position = light_position_worldspace;
	gl_Position = mvp * vec4(position, 1.0);
	out_normal = ( mvp * vec4(normal, 0)).xyz;
	out_texcoord = texcoord;
 }
 "#;

    let fragment_shader_src = r#"
#version 140
precision mediump float;

// Interpolated values from the vertex shaders
in vec3 out_position;
in vec3 out_normal;
in vec2 out_texcoord;
in vec3 camera_direction;
in vec3 light_direction;

// Ouput data
out vec3 color;

// Values that stay constant for the whole mesh.
uniform sampler2D diffuse_texture;
//uniform sampler2D shadow;

uniform vec3 ambient;
uniform vec3 diffuse;
uniform vec3 specular;
uniform vec3 light_position;



void main(){
	vec3 light_color = vec3(1,1,1);
	float light_power = 3.8f;

	// Material properties
	vec3 diffuseColor = diffuse; //vec3(0.3, 0.3, 0.3); * ( diffuse + texture(diffuse_texture, out_texcoord).rgb );

	//diffuseColor = diffuseColor + texture(shadow, out_texcoord).rgb;
	
	vec3 ambientColor = ambient + vec3(0.3, 0.3, 0.3) * diffuseColor;
	vec3 specularColor = specular;

	// Distance to the light
	float distance = length(light_position - out_position);
	// Normal of the computed fragment, in camera space
	vec3 n = normalize(out_normal);
	// Direction of the light (from the fragment to the light)
	vec3 l = normalize(light_direction);
	float cosTheta = clamp(dot(n, l), 0.0, 1.0);
	vec3 E = normalize(camera_direction);
	vec3 R = reflect(-l, n);
	float cosAlpha = clamp(dot(E, R), 0.0, 1.0);

	color = 
		// Ambient : simulates indirect lighting
		ambientColor +
		// Diffuse : "color" of the object
		diffuseColor * light_color * light_power * cosTheta / (distance * distance) +
		// Specular : reflective highlight, like a mirror
		specularColor * light_color * light_power * pow(cosAlpha, 5.0) / (distance * distance);
}

 "#;
	let program = program!(&display,
		140 => {
			vertex: vertex_shader_src,
			fragment: fragment_shader_src,
		}
	).unwrap();

	// Set up camera
	let perspective = PerspectiveMatrix3::new(screen_width as f32 / screen_height as f32, 45.0, 0.1, 1000.0);
	let mut perspective_matrix: Matrix4<f32> = perspective.to_matrix();
	
	
	let modelview_matrix_array = 
	[
        [1.0f32, -0.0f32, -0.0f32, 0.0f32],
        [ 0.0f32, 1.0f32, -0.0f32, 0.0f32],
        [ 0.0f32, 0.0f32, 1.0f32, 0.0f32],
        [ -5.0f32, -3.0f32, -12.0f32, 1.0f32]
    ];
	/* camera modelview matrix used for testing
	
    
	let projection_matrix_array = [
		[1.19506f32, 0.0f32, 0.0f32, 0.0f32],
        [0.0f32, 1.79259, 0.0f32, 0.0f32],
        [ 0.0f32, 0.0f32,-1.0f32,-1.0f32],
        [0.0f32, 0.0f32, -0.2f32, 0.0f32] 
   ];
    */

/*
	// Set uniforms, TODO: load from sqlite
	let uniforms = uniform! {
        projection: *perspective_matrix.as_ref(),
        modelview: modelview_matrix_array,
        light_position_worldspace: [2.0, 10.0, 1.0f32],
        diffuse: [1.0f32, 0.0, 0.0]
    };
    
    
	*/
	

	
	
	/*
	//#[derive(Copy, Clone)]
    struct ViewMatrixBlockData {
        projection: [[f32; 4]; 4],
        modelview: [[f32; 4]; 4],
    }
    
    //implement_buffer_content!(ViewMatrixBlockData);
   //implement_uniform_block!(ViewMatrixBlockData, projection, modelview);
	
	let mut view_matrix_block_buffer = glium::uniforms::UniformBuffer::<ViewMatrixBlockData>::dynamic(&display,
	//let mut view_matrix_block_buffer = glium::uniforms::UniformBuffer::<ViewMatrixBlockData>::empty_unsized(&display, 	 
	//	32       
		ViewMatrixBlockData {
			projection: projection_matrix_array,
			modelview: modelview_matrix_array,
		}
	).unwrap();
	*/
	/* 
	// How to map the buffer to local memory
	let mut mapping = view_matrix_block_buffer.map();
	for val in mapping.projection.iter_mut() {
		*val = 
			[1.19506f32, 0.00f32, 0.0f32, 0.0f32];
	      
	}
	
	//#[derive(Debug)]
	#[derive(Copy, Clone)]
	struct ViewMatrixBlockData {
	    projection: [[f32; 4]; 4],
        modelview: [[f32; 4]; 4],
    } */
	
	//implement_buffer_content!(ViewMatrixBlockData);    // without this, you can't put `Data` in a glium buffer
	//implement_uniform_block!(ViewMatrixBlockData, projection, modelview);
	
	/*let view_matrix_block_data: ViewMatrixBlockData = ViewMatrixBlockData {
		projection: projection_matrix_array,
		modelview: modelview_matrix_array,
	};*/
	// creates a buffer of 64 bytes, which thus holds 8 f32s
	//let mut buffer = glium::buffer::Buffer::<ViewMatrixBlockData>::empty_unsized(&display, BufferType::UniformBuffer,
	//                                                              4 * 32, BufferMode::Dynamic).unwrap();
	//let buffer = UniformBuffer::new(&display, view_matrix_block_data).unwrap();
	/*
	let mut buffer: glium::uniforms::UniformBuffer<ViewMatrixBlockData> =
              glium::uniforms::UniformBuffer::empty_unsized(&display, 32).unwrap();
              */
	// you can then write to it like you normally would
	//buffer.map().projection[0] = projection_matrix_array[0];
	//let mut buffer = glium::uniforms::UniformBuffer::new(&display, 4 * 32).unwrap();
	
	/*
	let mut buffer: glium::uniforms::UniformBuffer::<ViewMatrixBlockData> =
              glium::uniforms::UniformBuffer::new(&display, ViewMatrixBlockData {
		projection: projection_matrix_array,
		modelview: modelview_matrix_array,
	}).unwrap();
	*/

	/*
	let buffer = match glium::uniforms::UniformBuffer::new(&display, 
		ViewMatrixBlockData {
				projection: projection_matrix_array,
				modelview: modelview_matrix_array,
			}
		) {
        Err(_) => return,
        Ok(b) => b
    };
    */
    /*
	    let mut buffer = match glium::uniforms::UniformBuffer::<ViewMatrixBlockData>::empty_unsized(&display, 4 * 32) {
        Err(_) => return,
        Ok(b) => b
    }; */
	    
	
	// 2nd way to map uniform buffer data to local memory
	//let mut mapping = buffer.map();
     //   mapping.projection = projection_matrix_array;   
	//*mapping.modelview[0] = modelview_matrix_array[0]
	
	let uniforms = uniform! {
	    //ViewMatrixBlock: &buffer,
		projection: *perspective_matrix.as_ref(),
        modelview: modelview_matrix_array,
        light_position_worldspace: [2.0, 10.0, 1.0f32],
        diffuse: [1.0f32, 0.0, 0.0]
	};
	

	let mut running = true;
    let mut event_pump = sdl_context.event_pump().unwrap();

    while running {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer,
                  &index_buffer,
                  &program,
                  &uniforms,
                  &Default::default())
            .unwrap();
        target.finish().unwrap();

        // Event loop: includes all windows

        for event in event_pump.poll_iter() {
            use sdl2::event::Event;

            // Stop running if quit event or escape key is pressed
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Option::Some(Keycode::Escape), .. } => {
                    running = false;
                }
                _ => (),
            }
        }
    }
}
