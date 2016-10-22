import java.awt.event.WindowEvent;
import java.io.IOException;
import java.util.concurrent.locks.Condition;

public class Quick3DThread implements Runnable {
	private int displayWidth = 640, displayHeight = 480;
	private Display display = null;
	private Renderer renderer = null;
	private Shader shader = null;
	private Camera camera = null;
	private Thread displayThread = null;
	private ShaderDebuggerFrame parent;
	
	public Quick3DThread(ShaderDebuggerFrame parent) {
		this.parent = parent;
		displayThread = new Thread(this);
        displayThread.start();		
	}
	
	@Override
	public void run() {
		float moveSpeed = 0.03f;
		try {
			String dataPath = parent.getDataFilePath();
			parent.getReadyLock().lock();
			Condition ready = parent.getReadyCondition();
			ready.await();
			parent.getReadyLock().unlock();
			display = new Display(displayWidth,  displayHeight,  "Shader Debugger", false);
			parent.btnLoadData.setText("Loading Data");
			renderer = new Renderer(dataPath, display);
			parent.btnLoadData.setText("Data Loaded");
			camera = new Camera((float) displayWidth, (float) displayHeight);
			shader = new Shader(parent.getVertexShaderFilePath(), parent.getFragmentShaderFilePath(), display);
			boolean running = true;
			display.show();
			while(running) {
				// Constantly reload the shader each frame, waiting if there is an error in the shader
				shader.reload();
				if(!shader.isValid()) {
					parent.checkValid();
					parent.updateShaderStatusColor();
					parent.getReadyLock().lock();
					ready.await();			
				}
				
				renderer.render(shader, camera);
				
				EventBuffer events = new EventBuffer(display);
				
				if(events.closed() || events.key_pressed(KeyCode.ESCAPE)) {
					running = false;
				}
				
				if(events.key_pressed(KeyCode.W)) camera.moveForward(moveSpeed);
				if(events.key_pressed(KeyCode.S)) camera.moveBackward(moveSpeed);
				if(events.key_pressed(KeyCode.A)) camera.moveLeft(moveSpeed);
				if(events.key_pressed(KeyCode.D)) camera.moveRight(moveSpeed);
			}
			parent.dispatchEvent(new WindowEvent(parent, WindowEvent.WINDOW_CLOSING));
			displayThread.join();
		} catch (InterruptedException | IOException e) {
			e.printStackTrace();
		}
	}
}
