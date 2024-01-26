import {
  BrowserRouter as Router,
  Routes, Route, Link
} from 'react-router-dom'

import TaskPage from "./pages/TaskPage";
import HabitsPage from './pages/HabitsPage';

const App = () => {
  const padding = {
    padding: 5
  }

  return (
    <Router>
      <div>
        <Link style={padding} to="/">Tasks</Link>
        <Link style={padding} to="/habits">habits</Link>
      </div>

      <Routes>
        <Route path="/" element={<TaskPage />} />
        <Route path="/habits" element={<HabitsPage />} />
      </Routes>
    </Router>
  )
}

export default App;
