import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    const resp = (await invoke("greet", { name }));
    console.log(resp);
  }

  return (
    <div className="container">
      <h1>TAURITOP</h1>

      <p>Klikkaaa tahahahaasldkjalskdjalsd laksdj laskjd alskdj h</p>

      <form
        className="row"
        onSubmit={e => {
          e.preventDefault();
          greet();
        }}
      >
        <button type="submit">Greet</button>
      </form>

    </div>
  );
}

export default App;
