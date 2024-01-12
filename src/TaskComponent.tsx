import {Task} from "./models";
import { invoke } from "@tauri-apps/api";
import "./App.css";
import useTasksStore from "./state/tasks";

const TaskComponent = ({ task }: { task: Task }) => {
    const tasks = useTasksStore(state => state.tasks);
    const setTasks = useTasksStore(state => state.setTasks);

    const getClassName = (task: Task) => {
        return task.timeTracks.length % 2 === 0 ? "" : "inprogress"
    }

    const handleDelete = async () => {
        await invoke("delete_task", { id: task.id });
        setTasks(tasks.filter(t => t.id !== task.id));
    }
    
    return (
        <div className={getClassName(task)}>
            {task.name} {task.timeTracks.join(", ")} {task.id}
            <button onClick={handleDelete}>Delet</button>
        </div>
    )
}

export default TaskComponent