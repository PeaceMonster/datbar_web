from flask import Flask, send_from_directory

app = Flask(__name__)

@app.route("/")
def std_path():
    return send_from_directory('frontend/dist/', 'index.html')

@app.route("/<path:path>")
def alternate_path(path):
    print(path)
    return send_from_directory('frontend/dist/', 'index.html')

@app.route("/admin")
def admin_pth():
    return send_from_directory('frontend/dist/', 'admin.html')



@app.route("/assets/<path:path>")
def serve_static(path):
    return send_from_directory('frontend/dist/assets', path)

