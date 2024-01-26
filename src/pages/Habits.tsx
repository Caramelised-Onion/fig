import HabitComponent from "../HabitComponent";
import useHabitsStore from "../state/habits";

const Habits = () => {
    const habits = useHabitsStore(state => state.habits);

    return (
        <div>
            <h1>Habits</h1>
            <div>
                {habits.map(h => (
                    <HabitComponent key={h.id} habit={h} />
                ))} 
            </div>
        </div>
    )
}

export default Habits