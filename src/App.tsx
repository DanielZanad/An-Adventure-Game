import { FormEvent, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [input, setInput] = useState("");

  async function read_input(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();

    setInput((input) => input.trim().toLowerCase());

    if (input === "olharemvolta") {
      console.log(await invoke("look_around"));
    }
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    console.log(await invoke("read_input", { input }));
  }

  return (
    <main className="text-gray-500 bg-black">
      <div className="flex flex-col justify-between h-dvh w-full border items-center ">
        <div className="border border-amber-400 w-10/12 h-5/12 mt-12"></div>
        <div className="border border-amber-400 w-10/12 h-5/12 mb-6 flex flex-col">
          <div className="w-full h-8/12 border border-amber-300"></div>
          <form
            className="flex flex-row items-center m-auto justify-between gap-40"
            onSubmit={read_input}
          >
            <input
              type="text"
              className="border w-96"
              onChange={(e) => setInput(e.target.value)}
            />
            <button
              type="submit"
              className="border border-amber-400 cursor-pointer p-4"
            >
              Enviar
            </button>
          </form>
        </div>
      </div>
    </main>
  );
}

export default App;
