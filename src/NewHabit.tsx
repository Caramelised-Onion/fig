import { invoke } from "@tauri-apps/api";
import { useState } from "react";
import useHabitsStore from "./state/habits";

const NewHabit = () => {
    const [nameInput, setNameInput] = useState<string>("");
    const [intervalInput, setIntervalInput] = useState<number>(1);
    const [frequencyInput, setFrequencyInput] = useState<number>(1);

    const habits = useHabitsStore(state => state.habits);
    const setHabits = useHabitsStore(state => state.setHabits);

    const handleSubmit = async (evt: { preventDefault: () => void }) => {
        evt.preventDefault();
        const intervalS = intervalInput * 24 * 60 *60;
        const habitId: number = await invoke("create_habit", {
            name: nameInput,
            timeIntervalS: intervalS,
            freqInInterval: frequencyInput
        });
        const givenHabit = {
            id: habitId,
            name: nameInput,
            streak: 0,
            timeIntervalS: intervalS,
            freqInInterval: frequencyInput
        }
        setHabits(habits.concat(givenHabit));
        setNameInput("");
        setIntervalInput(1);
        setFrequencyInput(1);
    }

    return (
        <div>
            <h2>Create a New habit</h2>
            <form onSubmit={handleSubmit}>
                <div>
                    <label>name
                        <input 
                            type="text" 
                            value={nameInput}
                            onChange={({target}) => setNameInput(target.value)} 
                        />
                    </label>
                </div>
                <div>
                    <label> interval (in days for now)
                        <input 
                            type="number" 
                            min={1}
                            value={intervalInput}
                            onChange={({target}) => setIntervalInput(Number(target.value))} 
                        />
                    </label>
                </div>
                <div>
                    <label>frequency in interval
                        <input 
                            type="number" 
                            min={1}
                            value={frequencyInput}
                            onChange={({target}) => setFrequencyInput(Number(target.value))} 
                        />
                    </label>
                </div>
                <button type="submit">Save habit</button>
            </form>
        </div>
    )
}

export default NewHabit