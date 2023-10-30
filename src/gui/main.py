from PySide6 import QtCore, QtWidgets
from bridge import ProcessVideo, ProcessAudio, ProcessCsv

class Video2Anki(QtWidgets.QWidget):
    def __init__(self):
        super().__init__()

        self.setWindowTitle("Video to Anki App")
        self.setGeometry(100, 100, 400, 200)

        layout = QtWidgets.QVBoxLayout(self)
        home_layout = QtWidgets.QFormLayout()
        layout.addLayout(home_layout)

        self.path_video_label = QtWidgets.QLabel("Video Path:")
        self.path_video_edit = QtWidgets.QLineEdit()
        self.browse_video_btn = QtWidgets.QPushButton("Browse")
        home_layout.addRow(self.path_video_label, self.path_video_edit)
        home_layout.addRow(self.browse_video_btn)

        self.path_srt_label = QtWidgets.QLabel("SRT Path:")
        self.path_srt_edit = QtWidgets.QLineEdit()
        self.browse_srt_btn = QtWidgets.QPushButton("Browse")
        home_layout.addRow(self.path_srt_label, self.path_srt_edit)
        home_layout.addRow(self.browse_srt_btn)


        self.output_name_label = QtWidgets.QLabel("Deck name:")
        self.output_name_edit = QtWidgets.QLineEdit("output")
        home_layout.addRow(self.output_name_label, self.output_name_edit)

        self.output_path_label = QtWidgets.QLabel("Output folder:")
        self.output_path_edit = QtWidgets.QLineEdit("output")
        home_layout.addRow(self.output_path_label, self.output_path_edit)

        self.run_button = QtWidgets.QPushButton("Run")
        self.progress_label = QtWidgets.QLabel()

        self.browse_video_btn.clicked.connect(self.browse_video)
        self.browse_srt_btn.clicked.connect(self.browse_srt)
        self.run_button.clicked.connect(self.run_video_to_anki)


        layout.addWidget(self.run_button)
        layout.addWidget(self.progress_label)

    def browse_video(self):
        path, _ = QtWidgets.QFileDialog.getOpenFileName(self, "Choose Video File", "", "Video Files (*.mp4 *.avi *.mkv)")
        if path:
            self.path_video_edit.setText(path)
    def browse_srt(self):
        path, _ = QtWidgets.QFileDialog.getOpenFileName(self, "Choose SRT File", "", "SRT Files (*.srt)")
        if path:
            self.path_srt_edit.setText(path)

    def update_progress(self, done, missing, type):
        self.progress_label.setText(f"{type} - Done: {done}, Missing: {missing}")
        QtCore.QCoreApplication.processEvents() 
        

    def run_video_to_anki(self):
        video_path = self.path_video_edit.text()
        srt_path = self.path_srt_edit.text()
        output_name = self.output_name_edit.text()
        output_path = self.output_path_edit.text()

        if not output_path:
            output_path = "output"
        if not output_name:
            output_name = "output"
        if video_path:
            process_audio = ProcessAudio(video_path, srt_path, output_path, output_name)
            process_video = ProcessVideo(video_path, srt_path, output_path, output_name)
            process_csv = ProcessCsv(srt_path, output_path, output_name)
            process_csv.start()
            process_audio.start(self.update_progress)
            process_video.start(self.update_progress)
            self.progress_label.setText("Done!")
