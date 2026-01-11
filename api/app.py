import logging
import os
from flask import Flask, jsonify
import psutil

logging.basicConfig(
    level=logging.INFO,
    format="%(levelname)s:%(name)s:%(message)s"
)
logger = logging.getLogger("system-api")

app = Flask(__name__)

@app.route("/users", methods=["GET"])
def users():
    try:
        with open("/etc/passwd") as f:
            count = sum(1 for _ in f)
        logger.info("Users endpoint -> %s users", count)
        return jsonify({"total_users": count}), 200
    except Exception as exc:
        logger.exception("Failed to read /etc/passwd")
        return jsonify({"error": "unable to compute user count"}), 500

@app.route("/storage", methods=["GET"])
def storage():
    try:
        du = psutil.disk_usage("/")
        percent = du.percent
        logger.info("Storage endpoint -> %.1f%% used", percent)
        return jsonify({"disk_usage_percent": percent}), 200
    except Exception as exc:
        logger.exception("psutil disk_usage failed")
        return jsonify({"error": "unable to obtain disk usage"}), 503

@app.errorhandler(404)
def not_found(e):
    return jsonify({"error": "endpoint not found"}), 404

if __name__ == "__main__":
    # gunicorn will override this when used in prod
    app.run(host="0.0.0.0", port=5000, debug=False)
