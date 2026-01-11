from fastapi import FastAPI, RedirectResponse
import psutil

app = FastAPI()

users = ["alice", "bob"]

@app.get("/users")
def get_users():
    return users

@app.get("/metrics")
def metrics():
    return {
        "cpu_percent": psutil.cpu_percent(),
        "memory_percent": psutil.virtual_memory().percent,
        "disk_percent": psutil.disk_usage("/").percent,
    }

@app.get("/")
def root():
    return RedirectResponse("/docs")

@app.post("/users/{name}")
def add_user(name: str):
    if name not in users:
        users.append(name)
    return users

@app.delete("/users/{name}")
def delete_user(name: str):
    if name in users:
        users.remove(name)
    return users


