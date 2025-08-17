<script>
  let { hourFormat } = $props();
  let date = new Intl.DateTimeFormat(undefined, {
    hourCycle: hourFormat,
    hour: "numeric",
    minute: "numeric",
  });
  let time = $state("");

  $effect(() => {
    const interval = setInterval(() => {
      time = date.format().replaceAll(".", "");
    }, 1000);

    return () => {
      clearInterval(interval);
    };
  });
</script>

<h1>{time}</h1>

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
