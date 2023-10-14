from PySide6 import QtWidgets
from gui.main import MyWidget

import sys

if __name__ == "__main__":
    app = QtWidgets.QApplication([])
    widget = MyWidget()
    widget.resize(800, 600)
    widget.show()
    sys.exit(app.exec())
