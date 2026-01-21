#!/usr/bin/env python3
# Simple mock parser server used by the C client test harness.
# Listens on 127.0.0.1:46000, reads until "\n<<END>>\n" then returns a fixed JSON.

import socket
import threading

HOST = "127.0.0.1"
PORT = 46000
TERMINATOR = b"\n<<END>>\n"
RESPONSE = b'{"kind":"Program","children":[]}'

def run_server_once():
    srv = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    srv.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    srv.bind((HOST, PORT))
    srv.listen(1)
    conn, addr = srv.accept()
    data = b""
    while True:
        chunk = conn.recv(4096)
        if not chunk:
            break
        data += chunk
        if data.endswith(TERMINATOR):
            break
    conn.sendall(RESPONSE)
    conn.close()
    srv.close()

if __name__ == "__main__":
    run_server_once()