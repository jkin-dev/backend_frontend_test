import { useEffect, useState } from "react";
import { getUsers, addUser, deleteUser, getMetrics } from "./api";

export default function App() {
  const [users, setUsers] = useState<string[]>([]);
  const [name, setName] = useState("");
  const [metrics, setMetrics] = useState<any>({});

  useEffect(() => {
    refresh();
    getMetrics().then(setMetrics);
  }, []);

  const refresh = () => getUsers().then(setUsers);

  return (
    <div>
      <h1>Users</h1>
      <input value={name} onChange={e => setName(e.target.value)} />
      <button onClick={() => addUser(name).then(refresh)}>Add</button>

      <ul>
        {users.map(u => (
          <li key={u}>
            {u} <button onClick={() => deleteUser(u).then(refresh)}>X</button>
          </li>
        ))}
      </ul>

      <h2>System Metrics</h2>
      <pre>{JSON.stringify(metrics, null, 2)}</pre>
    </div>
  );
}
