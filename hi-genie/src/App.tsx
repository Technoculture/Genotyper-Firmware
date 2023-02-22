import { useState } from "react";
// import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";

function App () {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet () {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <div className="container">
      <div className="text-3xl font-bold underline">Welcome to Tauri!</div>

      <div className="row">
        <form
          onSubmit={ (e) => {
            e.preventDefault();
            greet();
          } }
        >
          <input
            id="greet-input"
            onChange={ (e) => setName(e.currentTarget.value) }
            placeholder="Enter a name..."
          />
          <button type="submit">Greet</button>
        </form>
      </div>
      <p>{ greetMsg }</p>
    </div>
  );
}

export default App;
