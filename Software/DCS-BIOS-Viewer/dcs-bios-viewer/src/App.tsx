import "./App.css";
import { Route, Routes } from "react-router-dom";
import { Connect } from "./Connect/Connect.tsx";
import { View } from "./View/View.tsx";

function App() {
    return (
        <Routes>
            <Route path="/" element={<Connect />} />
            <Route path="/view" element={<View />} />
        </Routes>
    );
}

export default App;
