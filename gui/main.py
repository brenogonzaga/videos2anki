from PySide6 import  QtWidgets
import bridge

class Video2Anki(QtWidgets.QWidget):
    def __init__(self):
        super().__init__()

        self.setWindowTitle("Video to Anki App")
        self.setGeometry(100, 100, 400, 200)

        self.path_label = QtWidgets.QLabel("Video Path:")
        self.path_edit = QtWidgets.QLineEdit()
        self.browse_button = QtWidgets.QPushButton("Browse")
        self.run_button = QtWidgets.QPushButton("Run")

        # Connect signals to slots
        self.browse_button.clicked.connect(self.browse_path)
        self.run_button.clicked.connect(self.run_video_to_anki)

        # Layout setup
        layout = QtWidgets.QVBoxLayout(self)
        form_layout = QtWidgets.QFormLayout()
        form_layout.addRow(self.path_label, self.path_edit)
        form_layout.addRow(self.browse_button)
        layout.addLayout(form_layout)
        layout.addWidget(self.run_button)

    def browse_path(self):
        path, _ = QtWidgets.QFileDialog.getOpenFileName(self, "Choose Video File", "", "Video Files (*.mp4 *.avi *.mkv)")
        if path:
            self.path_edit.setText(path)

    def run_video_to_anki(self):
        video_path = self.path_edit.text()
        if video_path:
            # You can replace this line with the actual function call
            bridge.run(video_path)
            # Add any additional logic or display a message as needed
