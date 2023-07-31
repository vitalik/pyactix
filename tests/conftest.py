import pytest
import sys
import os
import pathlib
import signal
import socket
import subprocess
import time
import platform
import requests


TESTS_DIR = pathlib.Path(__file__).parent.resolve()
ROOT = TESTS_DIR.parent.resolve()
print(ROOT)


sys.path.insert(0, ROOT)


def spawn_process(command: list[str]) -> subprocess.Popen:
    if platform.system() == "Windows":
        process = subprocess.Popen(command, shell=True, creationflags=subprocess.CREATE_NEW_PROCESS_GROUP)
        return process
    process = subprocess.Popen(command, preexec_fn=os.setsid)
    return process


def kill_process(process: subprocess.Popen) -> None:
    if platform.system() == "Windows":
        process.send_signal(signal.CTRL_BREAK_EVENT)
        process.kill()
        return

    try:
        os.killpg(os.getpgid(process.pid), signal.SIGKILL)
    except ProcessLookupError:
        pass


def start_server(host: str = '127.0.0.1', port=8080) -> subprocess.Popen:
    """
    Call this method to wait for the server to start
    """
    # Start the server

    python_executable = "python"
    if 'VIRTUAL_ENV' in os.environ:
        python_executable = os.path.join(os.environ['VIRTUAL_ENV'], 'bin', 'python')
    print("Using python:", python_executable)

    base_routes = os.path.join(TESTS_DIR, "./application.py")
    command = [python_executable, base_routes]
    process = spawn_process(command)

    # Wait for the server to be reachable
    timeout = 5  # The maximum time we will wait for an answer
    start_time = time.monotonic()
    while True:
        if time.monotonic() - start_time > timeout:
            kill_process(process)
            raise ConnectionError(f"Could not reach server at {host}:{port}")
        try:
            sock = socket.create_connection((host, port), timeout=5)
            sock.close()
            break  # We were able to reach the server, exit the loop
        except Exception:
            pass
    return process


class Server:
    def __init__(self, url):
        self.url = url

    def __repr__(self) -> str:
        return f"<Server {self.url}>"


class Client(requests.Session):
    def __init__(self, base_url: str):
        super().__init__()
        self.base_url = base_url.rstrip('/')

    def request(self, method: str, url: str, **kwargs):
        url = f"{self.base_url}/{url.lstrip('/')}"
        return super().request(method, url, **kwargs)


@pytest.fixture(scope="session")
def server():
    process = start_server()
    yield Server('http://127.0.0.1:8080')
    kill_process(process)


@pytest.fixture
def client(server):
    return Client(server.url)
