import { invoke } from "@tauri-apps/api";
import { useState } from "react";
import {Task} from "./models";
import useTasksStore from "./state/tasks";

const NewTask = () => {
    const [taskInput, setTaskInput] = useState<string>("");

    const tasks = useTasksStore(state => state.tasks);
    const setTasks = useTasksStore(state => state.setTasks);
    
    const handleSubmit = async (evt: { preventDefault: () => void }) => {
        evt.preventDefault()
        const taskId: number = await invoke("create_task", {name: taskInput});
        const givenTask: Task = {
            id: taskId,
            name: taskInput,
            timeTracks: [],
            totalTimeSpent: 0
        }
        setTasks(tasks.concat(givenTask));
        setTaskInput("");
    }

    return (
        <div>
            <h2>Create a New Task</h2>
            <form onSubmit={handleSubmit}>
                <label>name
                    <input 
                        type="text" 
                        value={taskInput}
                        onChange={({target}) => setTaskInput(target.value)} 
                    />
                </label>
                <button type="submit">Save Task</button>
            </form>
        </div>
    )
}

export default NewTask