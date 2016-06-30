// Copyright(C) 2016 Chris Liebert
extern crate glium;
extern crate nalgebra;

use common::Vertex8f32;
implement_vertex!(Vertex8f32, position, normal, texcoord);


// use self::glium::{Surface, Display, IndexBuffer, Program, VertexBuffer};
// use glium::uniforms::UniformsStorage;

// use self::nalgebra::{Matrix4, PerspectiveMatrix3};

// ::VertexBuffer;
// use glium::IndexBuffer;



// trait Renderer {
// fn new(display: Display) -> Renderer;
// fn buffer_to_gpu();
// fn render();
// }
//
// pub struct SceneRenderer {
// display: Display,
// vertexBuffer: VertexBuffer,
// indices: raw,
// program: raw,
// }
//
//
// pub fn new(display: Display) -> Renderer {
// Renderer { display: display }
// Set up camera
//    let perspective = PerspectiveMatrix3::new(screen_width as f32 / screen_height as f32,
//                                              45.0,
//                                              0.1,
//                                              1000.0);
//    let perspective_matrix: Matrix4<f32> = perspective.to_matrix();
//
//
//    let modelview_matrix_array = [[1.0f32, -0.0f32, -0.0f32, 0.0f32],
//                                  [0.0f32, 1.0f32, -0.0f32, 0.0f32],
//                                  [0.0f32, 0.0f32, 1.0f32, 0.0f32],
//                                  [-5.0f32, -3.0f32, -12.0f32, 1.0f32]];
//
//    let uniforms = uniform! {
// 	        projection: *perspective_matrix.as_ref(),
// 	        modelview: modelview_matrix_array,
// 	        light_position_worldspace: [2.0, 10.0, 1.0f32],
// 	        diffuse: [0.5f32, 0.5, 0.5]
// 	    };
// }
//

// pub fn render(display: Display,
// let mut target = display.draw();
//        target.clear_color(0.0, 0.0, 0.0, 1.0);
//        target.draw(&vertex_buffer,
//                  &index_buffer,
//                  &program,
//                  &uniforms,
//                  &Default::default())
//            .unwrap();
//        target.finish().unwrap();
// }
//



// Uniform buffers 
	/* camera modelview matrix used for testing
	
    
	let projection_matrix_array = [
		[1.19506f32, 0.0f32, 0.0f32, 0.0f32],
        [0.0f32, 1.79259, 0.0f32, 0.0f32],
        [ 0.0f32, 0.0f32,-1.0f32,-1.0f32],
        [0.0f32, 0.0f32, -0.2f32, 0.0f32] 
   ];
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