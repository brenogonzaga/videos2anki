from PySide6 import QtCore, QtWidgets
from bridge import ProcessVideo, ProcessAudio, ProcessCsv

class Video2Anki(QtWidgets.QWidget):
    def __init__(self):
        super().__init__()

        self.setWindowTitle("Video to Anki App")
        self.setGeometry(100, 100, 400, 200)

        self.path_label = QtWidgets.QLabel("Video Path:")
        self.path_edit = QtWidgets.QLineEdit()
        self.browse_button = QtWidgets.QPushButton("Browse")
        self.output_name_label = QtWidgets.QLabel("Output Name:")
        self.output_name_edit = QtWidgets.QLineEdit()
        self.output_path_label = QtWidgets.QLabel("Output Path:")
        self.output_path_edit = QtWidgets.QLineEdit()
        self.run_button = QtWidgets.QPushButton("Run")
        self.progress_label = QtWidgets.QLabel()

        self.browse_button.clicked.connect(self.browse_path)
        self.run_button.clicked.connect(self.run_video_to_anki)

        layout = QtWidgets.QVBoxLayout(self)
        form_layout = QtWidgets.QFormLayout()
        form_layout.addRow(self.path_label, self.path_edit)
        form_layout.addRow(self.browse_button)
        form_layout.addRow(self.output_name_label, self.output_name_edit)
        form_layout.addRow(self.output_path_label, self.output_path_edit)
        layout.addLayout(form_layout)
        layout.addWidget(self.run_button)
        layout.addWidget(self.progress_label)

    def output_name(self):
        return self.output_name_edit.text()

    def browse_path(self):
        path, _ = QtWidgets.QFileDialog.getOpenFileName(self, "Choose Video File", "", "Video Files (*.mp4 *.avi *.mkv)")
        if path:
            self.path_edit.setText(path)

    def update_progress(self, done, missing, type):
        self.progress_label.setText(f"{type} - Done: {done}, Missing: {missing}")
        QtCore.QCoreApplication.processEvents() 
        

    def run_video_to_anki(self):
        video_path = self.path_edit.text()
        output_name = self.output_name_edit.text()
        output_path = self.output_path_edit.text()
        if not output_path:
            output_path = "output"
        if not output_name:
            output_name = "output"
        if video_path:
            srt_path = video_path.replace(".mkv", ".srt")
            process_audio = ProcessAudio(video_path, srt_path, output_path, output_name)
            process_video = ProcessVideo(video_path, srt_path, output_path, output_name)
            process_csv = ProcessCsv(srt_path, output_path, output_name)
            process_csv.start()
            process_audio.start(self.update_progress)
            process_video.start(self.update_progress)
            self.progress_label.setText("Done!")
