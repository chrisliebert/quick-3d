// Copyright (C) 2016 Chris Liebert

public class Camera extends Quick3DNativeWrapper {

	Camera(float screenWidth, float screenHeight) {
		data = quick3dwrapper.create_camera(screenWidth, screenHeight);
	}
	
	@Override
	public void dispose() {
		quick3dwrapper.free_camera(data);		
	}

	public void moveForward(float amount) {
		data = quick3dwrapper.camera_move_forward(data, amount);
	}

	public void moveBackward(float amount) {
		data = quick3dwrapper.camera_move_backward(data, amount);
	}

	public void moveLeft(float amount) {
		data = quick3dwrapper.camera_move_left(data, amount);
	}

	public void moveRight(float amount) {
		data = quick3dwrapper.camera_move_right(data, amount);
	}

	public void aim(double x, double y) {
		data = quick3dwrapper.camera_aim(data, x, y);	
	}

}
