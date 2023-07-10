<script>
  import { invoke } from "@tauri-apps/api/tauri"

  let greetMsg = "loading name...";

  function change_name() {
    invoke('plugin:settings|get', { key: 'name' }).then(console.log).catch(console.log)
  }

  let new_name = "";
  function set_name() {
    invoke('plugin:settings|set', { key: 'name', value: new_name }).then(console.log).catch(console.log)
  }

  change_name()
  console.log('testaddo')
</script>

<div>
  <p>{greetMsg}</p>

  <div class="row">
    <input id="greet-input" placeholder="Enter a name..." bind:value={new_name} />
    <button on:click={set_name}>
      Greet
    </button>
  </div>
</div>

