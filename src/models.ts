export type Task = {
    id: number,
    name: string,
    timeTracks: number[],
    totalTimeSpent: number
}

export type Habit = {
    id: number,
    name: string,
    streak: number,
    timeIntervalS: number,
    freqInInterval: number
}