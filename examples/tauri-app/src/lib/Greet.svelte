<script>
  import { get, set } from 'tauri-plugin-settings-api';

  let new_name = '';
  let name = '';

  async function load_name() {
    name = await get('name');
  }

  async function set_name() {
    await set('name', new_name)

    await load_name()
  }

  load_name()
</script>

<div>
  <div class="row">
    <p>{name === "" ? "Loading name..." : `Hello ${name}!`}</p>

    <input id="greet-input" placeholder="Enter a name..." bind:value={new_name} />
    <button on:click={set_name}>
      Greet
    </button>
  </div>
</div>

