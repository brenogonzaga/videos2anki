from PySide6 import QtWidgets
from gui.main import Video2Anki

import sys

if __name__ == "__main__":
    app = QtWidgets.QApplication(sys.argv)
    window = Video2Anki()
    window.show()
    sys.exit(app.exec())
