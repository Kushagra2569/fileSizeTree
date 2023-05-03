import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [disks, setdisks] = useState({});
  const [dirs, setDirs] = useState({});
  const [dMode, setDMode] = useState(false);
  const [loading, setLoading] = useState(false);

  //TODO formatting the data

  async function getDisks(e, key) {
    let res = await invoke("disks", {});
    console.log(res);
    setdisks(res);
    setLoading(false);
  }

  async function getDirs(e, key) {
    setLoading(true);
    let res;
    if (dMode) {
      res = await invoke("dirs", { dir: key });
      console.log(res, dirs[key]);
    } else {
      res = await invoke("dirs", { dir: key });
      console.log(res, disks[key]);
    }
    setDirs(res);
    setDMode(true);
    setLoading(false);
  }

  return (
    <div className="container items-center">
      <div className="row">
        <form
          onSubmit={(e) => {
            e.preventDefault();
            setDMode(false);
            setLoading(true);
            getDisks();
          }}
        >
          <button type="submit">ansh</button>
        </form>
      </div>

      {!dMode &&
        Object.entries(disks).map(([key, value]) => (
          <button onClick={(e) => getDirs(e, key)} entry={key}>
            {key}
            {" : "}
            {value}
          </button>
        ))}
      {dMode &&
        Object.entries(dirs).map(([key, value]) => (
          <button className="" onClick={(e) => getDirs(e, key)} entry={key}>
            {key}
            {" : "}
            {value}
          </button>
        ))}
      {loading && <div className="">loading...</div>}
    </div>
  );
}

export default App;
