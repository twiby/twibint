import subprocess

def pytest_configure(config):
    subprocess.run(["maturin", "develop", "-r"])