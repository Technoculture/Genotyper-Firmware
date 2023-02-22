import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import * as Popover from '@radix-ui/react-popover';
// import { motion, AnimatePresence } from "framer-motion";

function App () {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet () {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <div className="p-2">
      <div className="text-3xl font-bold">Hi Genie!</div>

      <div>
        <form
          onSubmit={ (e) => {
            e.preventDefault();
            greet();
          } }
          className="flex gap-1"
        >

          <Popover.Root>
            <Popover.Trigger className="bg-green-200 p-2 rounded-md">More info</Popover.Trigger>
            <Popover.Portal>
              <Popover.Content className="bg-green-50 p-2 rounded-md">
                Some more info…
                <Popover.Arrow />
              </Popover.Content>
            </Popover.Portal>
          </Popover.Root>

          <input
            id="greet-input"
            onChange={ (e) => setName(e.currentTarget.value) }
            placeholder="Enter a name..."
            className="bg-red-100 rounded-md p-2"
          />
          <button type="submit">Greet</button>
        </form>
      </div>
      <p>{ greetMsg }</p>
    </div>
  );
}

export default App;
