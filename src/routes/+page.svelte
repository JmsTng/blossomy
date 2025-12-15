<script lang="ts">
  import Clock from "./Clock.svelte";
  import Settings from "./Settings.svelte";
  import Plant from "./Plant.svelte";

  let hourFormat = $state("h12");
  let page = $state("home");

  $inspect(page);
</script>

<main class="container">
  {#if $state.snapshot(page) === "home"}
    <Clock {hourFormat}/>
    <Plant/>
  {:else}
    <Settings bind:format={hourFormat}/>
  {/if}

  <span id="switcher">
    <a href="/" onclick={ () => {page = "home"} }>home</a>
    <a href="/" onclick={ () => {page = "settings"} }>settings</a>
  </span>
</main>

<style>
  @import url('https://fonts.googleapis.com/css2?family=Bagel+Fat+One&display=swap');

  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-image: url("/bk1.png");
    background-position: center;
    background-repeat: repeat;
    background-size: calc(128px * 0.25);

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;

    overflow: hidden;
  }

  #switcher {
    position: fixed;
    top: calc(90vh - 1em);

    width: 100vw;

    text-align: center;
  }

  .container {
    margin: 0;
    padding-top: 5vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  a {
    color: #0f0f0f;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
/*      background-color: #2f2f2f;*/
    }

    a {
      color: #f6f6f6;
    }
  }
</style>
