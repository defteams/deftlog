import { useEffect, useRef, useState } from "react";
import { AgGridReact } from "@ag-grid-community/react";
import { listen } from "@tauri-apps/api/event";
import { ColDef } from "@ag-grid-community/core";
import { ClientSideRowModelModule } from "@ag-grid-community/client-side-row-model";
import { MasterDetailModule } from "@ag-grid-enterprise/master-detail";
import LogDetail from "./components/LogDetail";

// Row Data Interface
interface IRow {
  channel: string;
  level_name: string;
  context: any;
  datetime: string;
}

const colDefs: ColDef[] = [
  { field: "channel", filter: true },
  { headerName: "Level", field: "level_name", filter: true, width: 100 },
  {
    field: "context",
    cellRenderer: "agGroupCellRenderer",
    valueGetter: (params) => params.data.message,
    flex: 1,
    filter: true,
  },
  { field: "datetime", filter: true },
];

export default function Application() {
  const gridRef = useRef(null);
  // Row Data: The data to be displayed.
  const [rowData, setRowData] = useState<IRow[]>([]);

  useEffect(() => {
    const unlisten = listen<string>("log_entry", (event) => {
      const newEntry = JSON.parse(event.payload);
      setRowData((logs) => [...logs, newEntry]);
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  // Container: Defines the grid's theme & dimensions.
  return (
    <div className="ag-theme-quartz-dark w-full h-screen">
      {/* <pre>{JSON.stringify(rowData, null, 2)}</pre> */}
      <AgGridReact
        ref={gridRef}
        rowData={rowData}
        columnDefs={colDefs}
        modules={[ClientSideRowModelModule, MasterDetailModule]}
        masterDetail
        detailCellRendererParams={{ dasd: "xxxx" }}
        detailCellRenderer={LogDetail}
        detailRowAutoHeight
        embedFullWidthRows
        className="w-auto text-sm"
      />
    </div>
  );
}
