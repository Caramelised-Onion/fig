import useTasksStore from "./state/tasks";
import "./App.css";
import TaskComponent from "./components/TaskComponent";

const Tasks = () => {
    const tasks = useTasksStore(state => state.tasks);

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