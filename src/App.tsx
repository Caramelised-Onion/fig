import Tasks from "./Tasks";
import { useEffect } from "react";
import NewTask from "./NewTask";
import { invoke } from "@tauri-apps/api";
import {Task} from "./models";
import useTasksStore from "./state/tasks";


function App() {
  const setTasks = useTasksStore(state => state.setTasks);
  useEffect(() => {
    const fetchTasks = async () => {
      const tasksInDb: Task[] = await invoke("get_all_tasks");
      setTasks(tasksInDb.map(t => t));
    };
    fetchTasks();
  }, []);

  return (
    <div>
      <NewTask /> 
      <Tasks />
    </div>
  );
}

export default App;
