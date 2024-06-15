import useTasksStore from "./state/tasks";
import "./App.css";
import TaskComponent from "./components/TaskComponent";
import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { OngoingTasksUpdate } from "./models";

const Tasks = () => {
    const tasks = useTasksStore(state => state.tasks);
    const updateTasks = useTasksStore(state => state.updateTasks);
    useEffect(() => {
        listen("ongoing_tasks_updated", async evt => {
            const ongoingTasksUpdate = evt.payload as OngoingTasksUpdate;
            console.log("ongoing tasks update", evt.payload);
            updateTasks(ongoingTasksUpdate.updatedTasks);
        })
    }, []);

    return (
        <div>
            <h1>Tasks</h1>
            <div>
                {tasks.map(t => (
                    <TaskComponent key={t.id} task={t} />
                ))} 
            </div>
        </div>
    )
}

export default Tasks