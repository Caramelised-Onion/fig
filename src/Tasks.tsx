const Tasks = ({ tasks }: {tasks: string[] }) => {
    return (
        <div>
            <h1>Tasks</h1>
            <ul>
                {tasks.map(t => (<li key={t}>{t}</li>))} 
            </ul>
        </div>
    )
}

export default Tasks