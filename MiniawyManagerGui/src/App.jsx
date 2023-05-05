import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import {
  BrowserRouter as Router,
  Routes,
  Route,
  NavLink,
} from "react-router-dom";
import React, { useMemo } from "react";
import MaterialReactTable from "material-react-table";
import {
  LineChart,
  Line,
  CartesianGrid,
  XAxis,
  YAxis,
  ResponsiveContainer,
  AreaChart,
  Area,
  Legend,
} from "recharts";
import {
  Box,
  Button,
  ListItemIcon,
  MenuItem,
  Typography,
  TextField,
} from "@mui/material";

import Proctable from "./Proctable";
import Sysinfo from "./Sysinfo";

function App() {
  return (
    <div>
      <Router>
        <div className="navBar">
          <NavLink to="/">Processes</NavLink>
          <NavLink to="/sysinfo">System Info</NavLink>
        </div>
        <Routes>
          <Route path="/" element={<Proctable />} />
          <Route path="/sysinfo" element={<Sysinfo />} />
        </Routes>
      </Router>
    </div>
  );
}

export default App;
