import {Task} from "./models";
import { invoke } from "@tauri-apps/api";
import "./App.css";
import "./bulma-rtl.min.css";
import useTasksStore from "./state/tasks";
import { useState } from "react";

const TaskComponent = ({ task }: { task: Task }) => {
    const [showEditForm, setShowEditForm] = useState<boolean>(false);
    const [taskNameInput, setTaskNameInput] = useState<string>("");
    const tasks = useTasksStore(state => state.tasks);
    const setTasks = useTasksStore(state => state.setTasks);

    const getClassName = (task: Task) => {
        return task.timeTracks.length % 2 === 0 ? "" : " has-background-success"
    }

    const handleDelete = async () => {
        await invoke("delete_task", { id: task.id });
        setTasks(tasks.filter(t => t.id !== task.id));
    }

    const handleAddTimeTrack = async () => {
        const latestTimestamp: number = await invoke("add_time_track", { id: task.id });
        setTasks(tasks.map(t => t.id === task.id ? { ...t, timeTracks: t.timeTracks.concat(latestTimestamp) } : t));   
    }

    const handleSubmit = async (evt: { preventDefault: () => void }) => {
        evt.preventDefault();
        const updatedTask = { ...task, name: taskNameInput }
        await invoke("update_task", { updatedTask })
        setTasks(tasks.map(t => t.id === task.id ? updatedTask : t));
        setTaskNameInput("");
        setShowEditForm(false);
    }
    
    return (
        <div className={"box" + getClassName(task) + " ml-4 is-flex is-flex-direction-row"} style={{ maxWidth: '800px', width: '100%' }}>
            {showEditForm 
                ? 
                    <form onSubmit={handleSubmit}>
                        <input 
                            type="text"
                            value={taskNameInput}
                            onChange={({target}) => setTaskNameInput(target.value)} 
                        />
                    </form>
                : task.name
            }
            <button onClick={() => setShowEditForm(!showEditForm)} className="ml-4">
                Edit task
            </button>
            <button onClick={handleDelete} >Delet</button>
            <button onClick={handleAddTimeTrack}>add time track</button>
        </div>
    )
}

export default TaskComponent