import { invoke } from "@tauri-apps/api";
import useTasksStore from "./state/tasks";
import "./App.css";
import TaskComponent from "./TaskComponent";

const Tasks = () => {
    const tasks = useTasksStore(state => state.tasks);
    const setTasks = useTasksStore(state => state.setTasks);

    const handleAddTimeTrack = async (taskId: number) => {
        const latestTimestamp: number = await invoke("add_time_track", { id: taskId });
        setTasks(tasks.map(t => t.id === taskId ? { ...t, timeTracks: t.timeTracks.concat(latestTimestamp) } : t));       
    };

    return (
        <div>
            <h1>Tasks</h1>
            <div>
                {tasks.map(t => (
                    <div key={t.id}>
                        <button onClick={() => handleAddTimeTrack(t.id)}>"⏯"</button>
                        <TaskComponent task={t} />
                    </div>
                ))} 
            </div>
        </div>
    )
}

export default Tasks