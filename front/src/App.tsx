import { Route, Routes } from "react-router-dom";
import "./App.css";
import Home from "./pages/Home";
import Post from "./pages/Post";

const App = () => {
  return (
    <Routes>
      <Route path="/" element={<Home />} />
      <Route path="/post/:slug" element={<Post />} />
    </Routes>
  );
};

export default App;
