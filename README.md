# Linux Admin Dashboard – Hiring Test Solution

Full-stack demo that exposes Linux user & system metrics through a React UI and an MQTT data-stream.

## Quick start (one command)
```bash
git clone https://github.com/jkin-dev/backend_frontend_test.git
cd backend_frontend_test
make demo        # builds & starts everything
# open http://localhost:5173
```

## What you will see
* List of local Linux users with add / delete buttons
* Live CPU, memory, disk widget (refreshes every 2 s)
* Rust MQTT client publishes JSON metrics to test/metrics
* Everything containerised – no local tools required

## Stack

| Layer       | Tech                  | Port | Notes                 |
| ----------- | --------------------- | ---- | --------------------- |
| Front       | React + Vite          | 5173 | SPA, consumes FastAPI |
| API         | FastAPI (Python)      | 5000 | Auto-docs at /docs    |
| MQTT broker | eclipse-mosquitto     | 1883 | No auth, demo only    |
| MQTT client | Rust (rumqttc, tokio) | —    | One-shot, 5 s loop    |

## Architecture diagram

![Alt text](diagram.png)

## Security choices
API & front run as non-root user inside containers
Read-only mount for /etc/passwd (user list)
MQTT open by design (local demo) – would add TLS + auth in prod
## Clean-up
make stop
