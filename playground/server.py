"""A stdlib-only development server for the rust2cython playground."""
import json
import os
import subprocess
import tempfile
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

ROOT = Path(__file__).resolve().parent
BINARY = os.environ.get("RUST2CYTHON_BIN", "rust2cython")

def sections(stdout, name):
    markers = {key: f"=== {name}.{key} ===" for key in ("pxd", "pyx", "h")}
    result = {}
    for key, marker in markers.items():
        start = stdout.find(marker)
        if start < 0:
            raise ValueError(f"Generator did not produce {name}.{key}")
        start += len(marker)
        next_markers = [stdout.find(value, start) for value in markers.values()]
        ffi = stdout.find(f"=== {name}_ffi.rs ===", start)
        stops = [pos for pos in next_markers + [ffi] if pos >= 0]
        end = min(stops) if stops else len(stdout)
        result[key] = stdout[start:end].strip() + "\n"
    return result

class Handler(BaseHTTPRequestHandler):
    def headers(self):
        self.send_header("Access-Control-Allow-Origin", "*")
        self.send_header("Access-Control-Allow-Headers", "Content-Type")
        self.send_header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")

    def reply(self, status, payload):
        body = json.dumps(payload).encode()
        self.send_response(status)
        self.headers()
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def do_OPTIONS(self):
        self.send_response(204); self.headers(); self.end_headers()

    def do_GET(self):
        if self.path not in ("/", "/index.html"):
            self.send_error(404); return
        content = (ROOT / "index.html").read_bytes()
        self.send_response(200); self.headers(); self.send_header("Content-Type", "text/html; charset=utf-8"); self.end_headers(); self.wfile.write(content)

    def do_POST(self):
        if self.path != "/generate": self.reply(404, {"error": "Not found"}); return
        try:
            length = int(self.headers.get("Content-Length", "0"))
            request = json.loads(self.rfile.read(length))
            code, name = request["code"], request.get("name", "mylib")
            if not isinstance(code, str) or not name.replace("_", "a").isalnum() or not name:
                raise ValueError("Provide Rust source and a valid library name")
            with tempfile.TemporaryDirectory() as directory:
                source = Path(directory) / "input.rs"; source.write_text(code, encoding="utf-8")
                run = subprocess.run([BINARY, str(source), "--dry-run", "-n", name], capture_output=True, text=True, timeout=10)
            if run.returncode: raise RuntimeError(run.stderr.strip() or run.stdout.strip())
            self.reply(200, {**sections(run.stdout, name), "error": None})
        except subprocess.TimeoutExpired:
            self.reply(504, {"error": "Generation timed out after 10 seconds"})
        except Exception as error:
            self.reply(400, {"error": str(error)})

if __name__ == "__main__":
    port = int(os.environ.get("PORT", "7331"))
    print(f"rust2cython playground listening on http://localhost:{port}")
    ThreadingHTTPServer(("0.0.0.0", port), Handler).serve_forever()
