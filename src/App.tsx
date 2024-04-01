import { useEffect, useState } from "react";
import { Event, listen } from '@tauri-apps/api/event';
import Log from './components/Log';

export default function App() {
  const [logs, setLogs] = useState<string[]>([]);
  useEffect(() => {
    listen('event-name', (event: Event<string>) => {
      const newEntry = JSON.parse(event.payload);
      console.log('event', event.payload)
      setLogs(logs => [...logs, newEntry]);
      // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
      // event.payload is the payload object
    });

    // return () => {
    //   console.log('removed');
    //   unlisten();
    // }
  }, []);

  return (
    <div className="w-full p-2">
      { logs.length ? logs.map((log, key) => <Log key={key} collapsed={Boolean(logs.length > 1 && key < logs.length - 1)} log={ log } />) : null }
    </div>

  )
}