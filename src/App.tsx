import Tasks from "./Tasks";
import { useEffect, useState } from "react";
import NewTask from "./NewTask";
import { invoke } from "@tauri-apps/api";
import {Task} from "./models";


function App() {
  const [tasks, setTasks] = useState<Task[]>([]);

  useEffect(() => {
    const fetchTasks = async () => {
      const tasksInDb: Task[] = await invoke("get_all_tasks");
      setTasks(tasksInDb.map(t => t));
    };
    fetchTasks();
  }, []);

         {/* */}

  return (
    <div>
      <NewTask tasks={tasks} setTasks={setTasks}/> 
      <Tasks tasks={tasks}/>
    </div>
  );
}

export default App;
