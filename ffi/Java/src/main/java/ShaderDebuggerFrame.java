import java.awt.Color;
import java.awt.EventQueue;

import javax.swing.JFrame;
import javax.swing.JPanel;
import javax.swing.UnsupportedLookAndFeelException;
import javax.swing.border.EmptyBorder;
import javax.swing.filechooser.FileNameExtensionFilter;
import javax.swing.JButton;
import javax.swing.JFileChooser;

import java.awt.event.ActionListener;
import java.io.BufferedReader;
import java.io.File;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.util.concurrent.locks.Condition;
import java.util.concurrent.locks.ReentrantLock;
import java.awt.event.ActionEvent;
import javax.swing.JLabel;
import javax.swing.JOptionPane;

import java.awt.GridLayout;
import java.awt.event.MouseAdapter;
import java.awt.event.MouseEvent;

public class ShaderDebuggerFrame extends JFrame implements ActionListener {
	private static final long serialVersionUID = 1L;

	private String glslangValidatorProgram = "glslangValidator";

	Quick3DThread quick3dThread = null;
	ReentrantLock lock = null;
	Condition ready = null;
	
	private boolean vertexShaderValid = false, fragmentShaderValid = false;
	
	private JPanel contentPane;
	JLabel lblLoadedData, lblVertexShader, lblFragmentShader;
	JButton btnLoadData, btnSetVertexShader, btnSetFragmentShader;
	private final JFileChooser fcData = new JFileChooser();
	private final JFileChooser fcVertexShader = new JFileChooser();
	private final JFileChooser fcFragmentShader = new JFileChooser();
	private JPanel panelData;
	private JPanel panelVertexShader;
	
	public JPanel getPanelVertexShader() {
		return panelVertexShader;
	}

	public JPanel getPanelFragmentShader() {
		return panelFragmentShader;
	}

	private JPanel panelFragmentShader;
	private String dataFilePath = null, vertexShaderFilePath = null, fragmentShaderFilePath = null;
	
	/**
	 * Launch the application.
	 */
	public static void main(String[] args) {
		EventQueue.invokeLater(new Runnable() {
			public void run() {
				try {
					ShaderDebuggerFrame frame = new ShaderDebuggerFrame();
					frame.setVisible(true);
				} catch (Exception e) {
					e.printStackTrace();
				}
			}
		});
	}

	public boolean hasFilesSelected() {
		boolean result =  (
			panelData.getBackground().equals(Color.GREEN)
			&&
			panelVertexShader.getBackground().equals(Color.GREEN)
			&&
			panelFragmentShader.getBackground().equals(Color.GREEN)
			);
		return result;
	}
	
	/**
	 * Create the frame.
	 */
	public ShaderDebuggerFrame() {
		for (javax.swing.UIManager.LookAndFeelInfo info : javax.swing.UIManager.getInstalledLookAndFeels()) {
            if ("Nimbus".equals(info.getName())) {
                try {
					javax.swing.UIManager.setLookAndFeel(info.getClassName());
				} catch (ClassNotFoundException | InstantiationException | IllegalAccessException
						| UnsupportedLookAndFeelException e) {
					System.err.println("Unable to set look and feel");
					e.printStackTrace();
				}
                break;
            }
		}
		lock = new ReentrantLock();
		ready = lock.newCondition();
		
		fcData.setFileFilter(new FileNameExtensionFilter("3D Data (.bin.gz)", "gz"));
		fcVertexShader.setFileFilter(new FileNameExtensionFilter("GLSL Vertex Shader (.vert)", "vert"));
		fcFragmentShader.setFileFilter(new FileNameExtensionFilter("GLSL Fragment Shader (.frag)", "frag"));
		
		setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
		setBounds(100, 100, 600, 145);
		contentPane = new JPanel();
		contentPane.setBorder(new EmptyBorder(5, 5, 5, 5));
		setContentPane(contentPane);
		
		contentPane.setLayout(new GridLayout(0, 2, 0, 0));
		
		panelData = new JPanel();
		contentPane.add(panelData);
		panelData.setLayout(null);
		
		lblLoadedData = new JLabel("No data loaded");
		lblLoadedData.setBounds(10, 11, 267, 14);
		panelData.add(lblLoadedData);
		
		btnLoadData = new JButton("Load Data");
		btnLoadData.addActionListener(this);
		contentPane.add(btnLoadData);
		
		btnSetVertexShader = new JButton("Set Vertex Shader");
		btnSetVertexShader.addActionListener(this);
		
		panelVertexShader = new JPanel();
		panelVertexShader.addMouseListener(new MouseAdapter() {
			@Override
			public void mouseClicked(MouseEvent arg0) {
				if(null != getVertexShaderFilePath()) {
					setVertexShader(getVertexShaderFilePath());
					checkReady();
				}
			}
		});
		contentPane.add(panelVertexShader);
		panelVertexShader.setLayout(null);
		
		lblVertexShader = new JLabel("No vertex shader loaded");
		lblVertexShader.setBounds(10, 11, 267, 14);
		panelVertexShader.add(lblVertexShader);
		contentPane.add(btnSetVertexShader);
		
		btnSetFragmentShader = new JButton("Set Fragment Shader");
		btnSetFragmentShader.addActionListener(this);
		
		panelFragmentShader = new JPanel();
		panelFragmentShader.addMouseListener(new MouseAdapter() {
			@Override
			public void mouseClicked(MouseEvent arg0) {
				if(null != getFragmentShaderFilePath()) {
					setFragmentShader(getFragmentShaderFilePath());
					checkReady();
				}
			}
		});
		contentPane.add(panelFragmentShader);
		panelFragmentShader.setLayout(null);
		
		lblFragmentShader = new JLabel("No fragment shader loaded");
		lblFragmentShader.setBounds(10, 11, 267, 14);
		panelFragmentShader.add(lblFragmentShader);
		contentPane.add(btnSetFragmentShader);
	}

	protected void checkReady() {
		if(vertexShaderValid && fragmentShaderValid && hasFilesSelected()) {
			lock.lock();
			ready.signal();
			lock.unlock();
		}
	}

	@Override
	public void actionPerformed(ActionEvent e) {
		Object eventSource = e.getSource();
        if (eventSource == btnLoadData) {
            int returnVal = fcData.showOpenDialog(ShaderDebuggerFrame.this);
            if (returnVal == JFileChooser.APPROVE_OPTION) {
                File file = fcData.getSelectedFile();
                lblLoadedData.setText(file.getAbsolutePath());
                setDataFilePath(file.getAbsolutePath());
                panelData.setBackground(Color.GREEN);
                quick3dThread = new Quick3DThread(this);
                btnLoadData.setEnabled(false);
				checkReady();
            } else {
                System.err.println("Open command cancelled by user.");
            }
        } else if (eventSource == btnSetVertexShader) {
            int returnVal = fcVertexShader.showOpenDialog(ShaderDebuggerFrame.this);
            if (returnVal == JFileChooser.APPROVE_OPTION) {
                setVertexShader(fcVertexShader.getSelectedFile().getAbsolutePath());
                checkReady();
            } else {
                System.err.println("Open command cancelled by user.");
            }
        } else if (eventSource == btnSetFragmentShader) {
            int returnVal = fcFragmentShader.showOpenDialog(ShaderDebuggerFrame.this);
            if (returnVal == JFileChooser.APPROVE_OPTION) {
                setFragmentShader(fcFragmentShader.getSelectedFile().getAbsolutePath());
                checkReady();
            } else {
                System.err.println("Open command cancelled by user.");
            }
        }
    }

	public boolean validateShaderSource(String path) {
		// Execute the glslangValidator program
		return validateShaderSource(path, glslangValidatorProgram);
	}
	
	public boolean validateShaderSource(String path, String glslangValidator) {
		// Execute the glslangValidator program
		String args[] = {glslangValidator, path};
		ProcessBuilder builder = new ProcessBuilder(args);
	    Process process;
		try {
			process = builder.start();
			InputStream is = process.getInputStream();
		    InputStreamReader isr = new InputStreamReader(is);
		    BufferedReader br = new BufferedReader(isr);
		    String line;
		    boolean noOutput = true;
		    while ((line = br.readLine()) != null) {
		      System.err.println(line);
		      noOutput = false;
		    }
		    if(noOutput) {
		    	return true;
		    } else {
		    	return false;
		    }
		} catch (IOException e) {
			JOptionPane.showMessageDialog(null,
					"glslangValidator not found in PATH: Please locate the executable",
					"Error",
					JOptionPane.ERROR_MESSAGE
			);
			JFileChooser glslangProgramChooser = new JFileChooser();
			int returnVal = glslangProgramChooser.showOpenDialog(null);
            if (returnVal == JFileChooser.APPROVE_OPTION) {
                File file = glslangProgramChooser.getSelectedFile();
                glslangValidatorProgram = file.getAbsolutePath();
                return validateShaderSource(path);
            } else {
            	System.exit(1);
            }
			return false;
		}
	}
	
	public void updateShaderStatusColor() {
		if(vertexShaderValid) {
        	panelVertexShader.setBackground(Color.GREEN);
        } else {
        	panelVertexShader.setBackground(Color.RED);
        }
		
		if(fragmentShaderValid) {
        	panelFragmentShader.setBackground(Color.GREEN);
        } else {
        	panelFragmentShader.setBackground(Color.RED);
        }
	}
	
	public void setVertexShader(String absolutePath) {
		setVertexShaderFilePath(absolutePath);
        lblVertexShader.setText(absolutePath);
        vertexShaderValid = validateShaderSource(absolutePath);
		if(vertexShaderValid) {
        	panelVertexShader.setBackground(Color.GREEN);
        } else {
        	panelVertexShader.setBackground(Color.RED);
        }
	}

	public void setFragmentShader(String absolutePath) {
        setFragmentShaderFilePath(absolutePath);
        lblFragmentShader.setText(absolutePath);
        fragmentShaderValid = validateShaderSource(absolutePath);
        if(fragmentShaderValid) {
        	panelFragmentShader.setBackground(Color.GREEN);
        } else {
        	panelFragmentShader.setBackground(Color.RED);
        }
	}

	public String getVertexShaderFilePath() {
		return vertexShaderFilePath;
	}

	public void setVertexShaderFilePath(String vertexShaderFilePath) {
		this.vertexShaderFilePath = vertexShaderFilePath;
	}

	public String getFragmentShaderFilePath() {
		return fragmentShaderFilePath;
	}

	public void setFragmentShaderFilePath(String fragmentShaderFilePath) {
		this.fragmentShaderFilePath = fragmentShaderFilePath;
	}

	public String getDataFilePath() {
		return dataFilePath;
	}

	public void setDataFilePath(String dataFilePath) {
		this.dataFilePath = dataFilePath;
	}

	public Condition getReadyCondition() {
		return ready;
	}

	public ReentrantLock getReadyLock() {
		return lock;
	}

	public void checkValid() {
		vertexShaderValid = validateShaderSource(vertexShaderFilePath);
		fragmentShaderValid = validateShaderSource(fragmentShaderFilePath);
	}
}
