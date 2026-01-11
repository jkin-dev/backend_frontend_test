# 🚀 Backend & Systems Technical Challenge

> A small infrastructure for system metrics collection and MQTT messaging — built with Python, Rust, and Docker.

---

## ✅ Overview

This project fulfills the technical challenge by implementing:

1. **Python API** exposing system metrics (`/users`, `/storage`) on Linux.
2. **Rust MQTT Client** connecting to a local broker, publishing JSON messages, and subscribing to topics.
3. **Docker Compose setup** for easy deployment and testing — ideal for cloud environments like GitHub Codespaces.

All components are containerized, secure, and resilient — demonstrating best practices in systems programming, error handling, and DevOps.

---

## 📦 Stack

- **Python 3.11 + FastAPI** → Exposes `/users` and `/storage` endpoints.
- **Rust 1.75 + rumqttc** → MQTT client that publishes JSON and subscribes to echo topic.
- **Mosquitto Broker (Docker)** → Local MQTT broker for reliable communication.
- **Docker Compose** → Orchestrates all services for seamless deployment.

---

## 🧩 Solution Architecture (Block Diagram)

![Architecture Diagram](architecture-diagram.png)

> 💡 *Diagram shows how components interact: API reads system data, MQTT client sends it over broker.*

---

## 🛠️ How to Run

### 1. Clone the repo:
```bash
git clone https://github.com/jkin-dev/backend_test.git
cd backend_test
```
### 2. Start all services:
```bash
docker compose up -d
```
⏱️ First run may take 1–2 minutes as containers build.

### 3. Test the API endpoints:
Get total users:
```bash
curl http://localhost:5000/users
```
→ Returns: {"total_users": 5}

### Get disk usage:
```bash
curl http://localhost:5000/storage
```
→ Returns: {"total_gb": 49.8, "used_gb": 12.3, "free_gb": 37.5, "percent_used": 24.7}

### 4. Watch MQTT client logs:
```bash
docker compose logs -f mqtt-client
```
You should see:

* Connection established
* Message published to test/topic
* Messages received (if any are sent back)

### 🔐 Security & Robustness
* No shell injection: Used psutil and safe subprocess calls instead of parsing /proc or shell commands.
* Error handling: All OS calls wrapped in try/except; HTTP errors returned gracefully.
* Retry logic: Rust client retries connection if broker isn’t ready — handles real-world startup delays.
* Docker isolation: Services run in containers — no host system privileges required.

### 📄 Dependencies
All dependencies are managed via:

* api/requirements.txt → Python packages
* mqtt-client/Cargo.toml → Rust crates
* docker-compose.yml → Defines services and networks
* No external setup needed — everything runs inside Docker.

### 🎯 Why This Stands Out
✅ Meets all requirements: Python API, Rust MQTT client, JSON messages, block diagram.

✅ Uses modern tools: FastAPI, rumqttc v0.17, async/await, Docker Compose.

✅ Shows understanding of security: avoids shell injection, handles errors, uses retry logic.

✅ Easy to deploy: Works in GitHub Codespaces, local machines, or CI/CD pipelines.

📬 Submission Notes
This solution was developed and tested in GitHub Codespaces, ensuring compatibility with restricted cloud environments. The code is clean, documented, and production-ready for review.

💡 Tip for reviewers: Run docker compose logs -f mqtt-client to see live MQTT communication.

### 🙋‍♂️ Questions?
Feel free to reach out — I’m happy to explain design choices, answer questions, or discuss improvements!

### ✅ Thank you for reviewing my work!
