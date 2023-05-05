import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
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

function Sysinfo() {
  const [cpuUsage, setCpuUsage] = useState([]);
  const [chartData, setChartData] = useState([]); //cpu

  const [memoryUsage, setMemoryUsage] = useState([]);
  const [chartData2, setChartData2] = useState([]); //memory

  const columns = [
    {
      Header: "pid",
      accessorKey: "pid",
    },
    {
      Header: "Name",
      accessorKey: "name",
    },
    {
      Header: "CPU Usage %",
      accessorKey: "cpu_usage",
    },
  ];

  const columns2 = [
    {
      Header: "pid",
      accessorKey: "pid",
    },
    {
      Header: "Name",
      accessorKey: "name",
    },
    {
      Header: "Memory Usage %",
      accessorKey: "memory_usage",
    },
  ];

  const [data, setData] = useState([]); //cpu table
  const [data2, setData2] = useState([]); //memory table

  async function getSortedCpu() {
    setData(await invoke("getSortedCpuUsage"));
  }

  async function getSortedMemory() {
    setData2(await invoke("getSortedMemUsage"));
  }

  useEffect(() => {
    let interval = setInterval(getSortedMemory, 1000);

    return () => {
      clearInterval(interval);
    };
  }, [data2]);

  useEffect(() => {
    let interval = setInterval(getSortedCpu, 1000);

    return () => {
      clearInterval(interval);
    };
  }, [data]);

  async function getCPU() {
    setCpuUsage(await invoke("getCPU"));
  }

  async function getMemory() {
    setMemoryUsage(await invoke("getMemory"));
  }

  useEffect(() => {
    let interval = setInterval(getCPU, 2000);
    return () => {
      clearInterval(interval);
    };
  }, [cpuUsage]);

  useEffect(() => {
    let interval = setInterval(getMemory, 1000);
    return () => {
      clearInterval(interval);
    };
  }, [memoryUsage]);

  useEffect(() => {
    const interval = setInterval(() => {
      setChartData((chartData) => [
        ...chartData,
        {
          name: "",
          CPU1: cpuUsage[0],
          CPU2: cpuUsage[1],
          CPU3: cpuUsage[2],
          CPU4: cpuUsage[3],
        },
      ]);
    }, 2000);
    return () => clearInterval(interval);
  }, [chartData, cpuUsage]);

  useEffect(() => {
    const interval = setInterval(() => {
      setChartData2((chartData2) => [
        ...chartData2,
        {
          name: "",
          Memory: memoryUsage,
        },
      ]);
    }, 1000);
    return () => clearInterval(interval);
  }, [chartData2, memoryUsage]);

  return (
    <div>
     <h2> System Info</h2>
      <div className="title1">
        <h1>CPU Usage:</h1>
      </div>
      <div className="cpuDiv">
        <ResponsiveContainer width={500} height={400}>
          <LineChart data={chartData}>
            <XAxis dataKey="name" />
            <YAxis type="number" domain={[0, 100]} />
            <Legend />
            <Line
              type="monotone"
              stroke="blue"
              dataKey="CPU1"
              isAnimationActive={true}
              animationDuration={50}
            />
            <Line
              type="monotone"
              stroke="red"
              dataKey="CPU2"
              isAnimationActive={true}
              animationDuration={50}
            />
            <Line
              type="monotone"
              stroke="green"
              dataKey="CPU3"
              isAnimationActive={true}
              animationDuration={50}
            />
            <Line
              type="monotone"
              stroke="purple"
              dataKey="CPU4"
              isAnimationActive={true}
              animationDuration={50}
            />
          </LineChart>
        </ResponsiveContainer>

        <MaterialReactTable
          columns={columns}
          data={data}
          initialState={{
            density: "compact",
          }}
          enablePagination={false}
          enableDensityToggle={false}
          enableColumnActions={false}
          enableClickToCopy={false}
          enableColumnFilterModes={false}
          enableRowSelection={false}
          enableBottomToolbar={false}
          enableSearch={false}
          enableColumnVisibilityToggle={false}
          enableColumnReordering={false}
          enableSettings={false}
          enableFullScreenToggle={false}
          enableSort={false}
          enableFilters={false}
          enableGrouping={false}
          enableSelection={false}
          enableMultiRowSelection={false}
          enableMultiRowExpansion={false}
          size="small"
        />
      </div>
      <div>
        <h1>Memory Usage:</h1>{" "}
      </div>
      <div className="memDiv">
        <ResponsiveContainer width={500} height={400}>
          <AreaChart data={chartData2}>
            <XAxis dataKey="name" />
            <YAxis type="number" domain={[0, 100]} />
            <Legend />
            <Area
              type="monotone"
              stroke="blue"
              dataKey="Memory"
              isAnimationActive={true}
              animationDuration={50}
            />
          </AreaChart>
        </ResponsiveContainer>
        <MaterialReactTable
          columns={columns2}
          data={data2}
          initialState={{
            density: "compact",
          }}
          enablePagination={false}
          enableDensityToggle={false}
          enableColumnActions={false}
          enableClickToCopy={false}
          enableColumnFilterModes={false}
          enableRowSelection={false}
          enableBottomToolbar={false}
          enableSearch={false}
          enableColumnVisibilityToggle={false}
          enableColumnReordering={false}
          enableSettings={false}
          enableFullScreenToggle={false}
          enableSort={false}
          enableFilters={false}
          enableGrouping={false}
          enableSelection={false}
          enableMultiRowSelection={false}
          enableMultiRowExpansion={false}
          size="small"
        />
      </div>
    </div>
  );
}

export default Sysinfo;
