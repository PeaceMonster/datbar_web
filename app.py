from flask import Flask, send_from_directory

app = Flask(__name__)

@app.route("/")
def std_path():
    return send_from_directory('frontend/dist/', 'index.html')

@app.route("/<path:path>")
def serve_static(path):
    return send_from_directory('frontend/dist/', path)