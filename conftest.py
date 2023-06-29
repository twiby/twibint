import sys
import subprocess

import pathlib
directory = str(pathlib.Path(__file__).parent.resolve())

def pytest_configure(config):
    subprocess.run([
        sys.executable, 
        "-m", 
        "maturin", 
        "develop", 
        "-r"], 
        cwd = directory)