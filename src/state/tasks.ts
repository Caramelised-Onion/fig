import { create } from "zustand";
import { Task } from "../models"

type TasksState = {
    tasks: Task[],
    setTasks: (newTasks: Task[]) => void,
    updateTasks: (updatedTasks: Task[]) => void,
}

const useTasksStore = create<TasksState>()(
    set => ({
        tasks: [],
        setTasks: (newTasks: Task[]) => set(_ => ({ tasks: newTasks })),
        updateTasks: (updatedTasks: Task[]) => set(state => ({ tasks: state.tasks.map(
            t => {
                const updated = updatedTasks.find(ut => ut.id === t.id);
                if (updated) {
                    return updated;
                }
                return t;
            }
        )}))
    })
);

export default useTasksStore;