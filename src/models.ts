export type Interval = {
    start_time: Date,
    end_time: Date | undefined,
}

export type Task = {
    id: number,
    name: string,
    intervals: Interval[],
    totalTimeSpent: number
}

export type Habit = {
    id: number,
    name: string,
    streak: number,
    timeIntervalS: number,
    freqInInterval: number
}