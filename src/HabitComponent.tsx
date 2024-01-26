import { Habit } from "./models"

const HabitComponent = ({ habit }: { habit: Habit }) => {
    return (
        <div>{habit.name}</div>
    )
}

export default HabitComponent