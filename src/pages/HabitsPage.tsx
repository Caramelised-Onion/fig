import { invoke } from "@tauri-apps/api";
import { useEffect } from "react";
import NewHabit from "../NewHabit"
import { Habit } from "../models";
import useHabitsStore from "../state/habits";
import Habits from "./Habits";

const HabitsPage = () => {
    const setHabits = useHabitsStore(state => state.setHabits);
    useEffect(() => {
        const fetchHabits = async () => {
            const habitsInDb: Habit[] = await invoke("get_all_habits");
            setHabits(habitsInDb);
        };
        fetchHabits();
    }, []);

    return (
        <>
            <NewHabit />
            <Habits />
        </>
        
    )
}

export default HabitsPage;