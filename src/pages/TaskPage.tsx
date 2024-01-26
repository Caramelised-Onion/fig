import { invoke } from "@tauri-apps/api";
import { useEffect } from "react";
import NewTask from "../NewTask";
import Tasks from "../Tasks";
import { Task } from "../models";
import useTasksStore from "../state/tasks";

const TaskPage = () => {
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
      <h1 className={"title"}>FIG</h1>
      <NewTask /> 
      <Tasks />
    </div>
  );
}

export default TaskPage;