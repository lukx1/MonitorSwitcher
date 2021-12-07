#!/usr/bin/env python3

import socket, sys
from colorama import init, Fore,Back,Style
HOST = "127.0.0.1"
PORT = 7443

msg = "read"

# if len(sys.argv) > 1:
#     msg = sys.argv[1]

init()

print(f"{Fore.YELLOW}Starting python test client {HOST}:{PORT}")

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    print("Connecting...")
    s.connect((HOST,PORT))
    print("Connected, ready")
    while True:
        print(Fore.CYAN)
        text = input()
        s.sendall(text.encode())
        s.sendall(b"\n")
        resp = s.recv(1024)
        print(f"{Fore.RESET}Printing response{Fore.GREEN}")
        print(resp.decode("utf-8"))
# with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
#     print("Connecting")
#     s.connect((HOST,PORT))
#     print(f"Sending {msg}")
#     s.sendall(msg.encode())
#     print(f"Sending \\n")
#     s.sendall(b"\n")
#     print("Reading")
    resp = s.recv(1024)
    print("Printing response")
    print(resp.decode("utf-8"))
#     print("Closing stream")
#     s.close()
