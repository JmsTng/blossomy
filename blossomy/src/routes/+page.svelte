<script lang="ts">
  import { SvelteDate } from "svelte/reactivity";
  import { onMount } from "svelte";

  const date = new SvelteDate();
  let hours = $state(0);
  let minutes = $state(0);

  const formatter = new Intl.DateTimeFormat(undefined, {
    hour: "2-digit",
    minute: "2-digit"
  });

  $effect(() => {
    const interval = setInterval(() => {
      date.setTime(Date.now());

      hours = date.getHours();
      minutes = date.getMinutes();
    }, 1000);

    return () => {
      clearInterval(interval);
    }
  });
</script>

<main class="container">
  <h1>{`${hours}:${minutes.toString().padStart(2, "0")} ${hours < 12 ? 'AM' : 'PM'}`}</h1>
</main>

<style>
@import url('https://fonts.googleapis.com/css2?family=Bagel+Fat+One&display=swap');

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

h1 {
  font-family: "Bagel Fat One", system-ui;
  font-weight: 400;
  font-style: normal;

  text-align: center;
  font-size: 4em;

  text-transform: uppercase;
  /* -webkit-text-stroke: #0f0f0f 3px; */
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }
}

</style>
