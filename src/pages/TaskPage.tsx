import { invoke } from "@tauri-apps/api";
import { useEffect } from "react";
import NewTask from "../NewTask";
import Tasks from "../Tasks";
import { Task } from "../models";
import useTasksStore from "../state/tasks";

const TaskPage = () => {
    const setTasks = useTasksStore(state => state.setTasks);
    const tasks = useTasksStore(state => state.tasks);
    useEffect(() => {
        const fetchTasks = async () => {
            const tasksInDb: Task[] = await invoke("get_all_tasks");
            console.log(`Found ${tasksInDb.length} tasks in the db`);
            setTasks(tasksInDb.map(t => t));
        };
        fetchTasks();
        invoke("poll_for_ongoing_task_updates");
    }, []);

  return (
    <div>
      <h1 className={"title"}>FIG</h1>
      <NewTask />
      {tasks.length && <Tasks />}
    </div>
  );
}

export default TaskPage;