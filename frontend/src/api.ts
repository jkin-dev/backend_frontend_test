const API = "http://localhost:5000";

export const getUsers = () => fetch(`${API}/users`).then(r => r.json());
export const addUser = (name: string) =>
  fetch(`${API}/users/${name}`, { method: "POST" });

export const deleteUser = (name: string) =>
  fetch(`${API}/users/${name}`, { method: "DELETE" });

export const getMetrics = () =>
  fetch(`${API}/metrics`).then(r => r.json());
