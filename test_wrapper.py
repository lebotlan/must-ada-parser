# pytest-based tests for python-wrapper functions.
# Tests start a small mock TCP parser server (in-thread) that respects the
# "\n<<END>>\n" terminator and returns a simple JSON response.
#
# Run: pytest python-wrapper/tests -q

import os
import sys
import socket
import threading
import tempfile
import time
import subprocess
import json

# Ensure we can import the wrapper module by adding python-wrapper to sys.path
ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
sys.path.insert(0, ROOT)
import wrapper

HOST = "127.0.0.1"
PORT = 46000
TERMINATOR = b"\n<<END>>\n"


def _mock_server(response: bytes, stop_event: threading.Event):
    srv = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    srv.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    srv.bind((HOST, PORT))
    srv.listen(1)
    srv.settimeout(0.5)
    try:
        while not stop_event.is_set():
            try:
                conn, _ = srv.accept()
            except socket.timeout:
                continue
            data = b""
            while True:
                chunk = conn.recv(4096)
                if not chunk:
                    break
                data += chunk
                if data.endswith(TERMINATOR):
                    break
            # respond with provided response
            conn.sendall(response)
            conn.close()
            # one-shot server for test
            break
    finally:
        srv.close()


def test_send_direct_to_server(tmp_path):
    # Prepare a small .ada-like content file
    ada_file = tmp_path / "sample.ada"
    ada_file.write_text("procedure Test is\nbegin\n null; end Test;")

    # Start mock server
    stop_event = threading.Event()
    response_json = b'{"kind":"Program","children":[]}'
    t = threading.Thread(target=_mock_server, args=(response_json, stop_event))
    t.start()
    time.sleep(0.1)  # allow server to start

    # Use wrapper.send_direct_to_server to send file content to the mock server
    # The function prints the response; capture stdout by invoking the function in a subprocess
    proc = subprocess.run([sys.executable, os.path.join(ROOT, "wrapper.py"), "--file", str(ada_file)], capture_output=True, text=True)
    # stop the server thread
    stop_event.set()
    t.join(timeout=1.0)

    assert proc.returncode == 0
    # verify output contains the JSON response
    assert '{"kind":"Program"' in proc.stdout or '{"kind":"Program"' in proc.stderr


def test_download_to_temp_without_requests(monkeypatch, tmp_path):
    # Test download_to_temp fallback using urllib by pointing to a local file:// URL
    local_file = tmp_path / "example.ada"
    local_file.write_text("dummy")
    url = f"file://{local_file}"

    path = wrapper.download_to_temp(url)
    try:
        with open(path, "rb") as f:
            data = f.read()
        assert b"dummy" in data
    finally:
        # cleanup
        os.unlink(path)