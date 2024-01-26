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

    const handleSubmit = async (evt: { preventDefault: () => void }) => {
        evt.preventDefault();
        const updatedTask = { ...task, name: taskNameInput }
        await invoke("update_task", { updatedTask })
        setTasks(tasks.map(t => t.id === task.id ? updatedTask : t));
        setTaskNameInput("");
    }
    
    return (
        <div className={"box" + getClassName(task)}>
                {task.name} {task.id}
                <button onClick={handleDelete}>Delet</button>
                {/* <button onClick={handleAddTimeTrack}></button> */}
                <button onClick={() => setShowEditForm(!showEditForm)}>
                    Edit task
                </button>
                {
                    showEditForm && 
                    <div>
                        <form onSubmit={handleSubmit}>
                            <input 
                                type="text"
                                value={taskNameInput}
                                onChange={({target}) => setTaskNameInput(target.value)} 
                            />
                            <button type="submit">Update task</button>
                        </form>
                    </div>
                }
        </div>

    )
}

export default TaskComponent