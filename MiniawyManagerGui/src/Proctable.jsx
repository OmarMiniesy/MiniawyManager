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

function Proctable() {
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
      Header: "Status",
      accessorKey: "state",
    },
    {
      Header: "Memory KB",
      accessorKey: "memory_usage",
    },
    {
      Header: "CPU %",
      accessorKey: "cpu_usage",
    },
    {
      Header: "CPU Time",
      accessorKey: "cpu_time",
    },
    {
      Header: "User",
      accessorKey: "user_name",
    },
    {
      Header: "Priority",
      accessorKey: "priority",
    },
    {
      Header: "Files Opened",
      accessorKey: "files_opened",
    },
  ];

  const [data, setData] = useState([]);

  async function getProcesses() {
    setData(await invoke("hashmapFill"));
  }

  useEffect(() => {
    let interval = setInterval(getProcesses, 5000);

    return () => {
      clearInterval(interval);
    };
  }, [data]);

  const [canKill, setCanKill] = useState();

  useEffect(() => {
    let interval = setInterval(setCanKill, 5000);

    return () => {
      clearInterval(interval);
    };
  }, true);

  return (
    <div>
      <Button onClick={getProcesses}> Quick Start</Button>
      <MaterialReactTable
        columns={columns}
        data={data}
        enableColumnFilterModes
        enableRowSelection
        enableSorting={false}
        enablePagination={false}
        initialState={{
          density: "compact",
          pagination: { pageSize: 100 },
        }}
        enableDensityToggle={false}
        enableColumnActions={false}
        renderTopToolbarCustomActions={({ table }) => {
          const handleKill = () => {
            table.getSelectedRowModel().flatRows.map((row) => {
              setCanKill(invoke("kill", { pid: row.original.pid }));
            });
          };

          return (
            <div style={{ display: "flex", gap: "0.5rem" }}>
              <Button
                color="error"
                disabled={!table.getIsSomeRowsSelected()}
                onClick={handleKill}
                variant="contained"
              >
                Kill
              </Button>

              <Button className="permission" disabled={!canKill}>
                NO PERMISSIONS
              </Button>
            </div>
          );
        }}
      />
    </div>
  );
}

export default Proctable;
