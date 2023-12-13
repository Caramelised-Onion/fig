import { invoke } from "@tauri-apps/api";
import { useState } from "react";

type Props = {
    tasks: string[]
    setTasks: React.Dispatch<React.SetStateAction<string[]>>
}

const NewTask = ({ tasks, setTasks }: Props) => {
    const [taskInput, setTaskInput] = useState<string>("");

    const handleSubmit = async (evt: { preventDefault: () => void }) => {
        evt.preventDefault()
        const taskName: string = await invoke("create_task", {name: taskInput});
        setTasks(tasks.concat(taskName));
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