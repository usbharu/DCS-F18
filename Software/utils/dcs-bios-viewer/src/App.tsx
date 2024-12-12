import "./App.css";
import { Route, Routes } from "react-router-dom";
import { Connect } from "./Connect";
import { MetricsView } from "./MetricsView";

function App() {
  return (
    <Routes>
      <Route path="/" Component={Connect}></Route>
      <Route path="/view" Component={MetricsView}></Route>
    </Routes>
  );
}

export default App;
