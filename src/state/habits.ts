import { create } from "zustand";
import { Habit } from "../models"

type HabitsState = {
    habits: Habit[],
    setHabits: (newHabit: Habit[]) => void
}

const useHabitsStore = create<HabitsState>()(
    set => ({
        habits: [],
        setHabits: (newHabits: Habit[]) => set(_ => ({ habits: newHabits }))
    })
);

export default useHabitsStore;