import sys
import random
from PySide6 import QtCore, QtWidgets, QtGui
import sync_scribe

class MyWidget(QtWidgets.QWidget):
    def __init__(self):
        super().__init__()

        self.line_edit = QtWidgets.QLineEdit()
        self.button = QtWidgets.QPushButton("Click me!")
        self.text = QtWidgets.QLabel("Hello World",
                                     alignment=QtCore.Qt.AlignCenter)

        self.layout = QtWidgets.QVBoxLayout(self)
        self.layout.addWidget(self.line_edit)
        self.layout.addWidget(self.text)
        self.layout.addWidget(self.button)

        self.button.clicked.connect(self.magic)

    @QtCore.Slot()
    def magic(self):
        input = self.line_edit.text()
        sync_scribe.run(input)
        self.text.setText(self.line_edit.text())