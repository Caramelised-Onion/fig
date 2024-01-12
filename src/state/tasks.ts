import { create } from "zustand";
import { Task } from "../models"

type TasksState = {
    tasks: Task[],
    setTasks: (newTasks: Task[]) => void
}

const useTasksStore = create<TasksState>()(
    set => ({
        tasks: [],
        setTasks: (newTasks: Task[]) => set(_ => ({ tasks: newTasks }))
    })
);

export default useTasksStore;