from fastapi import FastAPI
from fastapi.responses import RedirectResponse   # add this line
from fastapi.middleware.cors import CORSMiddleware
import psutil

app = FastAPI()
app.add_middleware(
    CORSMiddleware,
    allow_origins=["http://localhost:5173", "https://*.github.dev"],
    allow_methods=["*"],
    allow_headers=["*"],
)
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


