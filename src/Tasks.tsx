import { useEffect, useState } from "react";
import {Task} from "./models";


const Tasks = ({ tasks }: {tasks: Task[] }) => {
    const [editedTasks, setEditedTasks] = useState<Task[]>([]);
    useEffect(() => {
        setEditedTasks(tasks);
    }, [tasks]);

    const handleAddTimeTrack = (taskId: number) => {
        setEditedTasks(editedTasks.map(et => et.id === taskId ? addTimestampToTask(et) : et));       
    };

    const addTimestampToTask = (t: Task): Task => {
        // TODO: actual implementation
        return {
            ...t,
            timeTracks: t.timeTracks.concat(getCurrentUnixTimestampS())
        };
    }

    const getCurrentUnixTimestampS = (): number =>
        Math.floor(new Date().getTime() / 1000);

    return (
        <div>
            <h1>Tasks</h1>
            <ul>
                {editedTasks.map(t => (<li key={t.name}>{t.name} {t.timeTracks} <button onClick={() => handleAddTimeTrack(t.id)}>"⏯"</button></li>))} 
            </ul>
        </div>
    )
}

export default Tasks