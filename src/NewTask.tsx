import { invoke } from "@tauri-apps/api";
import { useState } from "react";
import {Task} from "./models";


type Props = {
    tasks: Task[]
    setTasks: React.Dispatch<React.SetStateAction<Task[]>>
}

const NewTask = ({ tasks, setTasks }: Props) => {
    const [taskInput, setTaskInput] = useState<string>("");

    const handleSubmit = async (evt: { preventDefault: () => void }) => {
        evt.preventDefault()
        const taskName: string = await invoke("create_task", {name: taskInput});
        const givenTask: Task = {
            id: 19, // TODO figure how to generate this
            name: taskName,
            timeTracks: [],
            totalTimeSpent: 0
          }
        setTasks(tasks.concat(givenTask));
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