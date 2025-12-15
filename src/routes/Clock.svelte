<script>
  let { hourFormat } = $props();
  let date = $derived(new Intl.DateTimeFormat(undefined, {
    hourCycle: hourFormat,
    hour: "numeric",
    minute: "numeric",
  }));
  let time = $derived(date.format().replaceAll(".", ""));

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
  h1 {
    font-family: "Bagel Fat One", system-ui;
    font-weight: 400;
    font-style: normal;

    text-align: center;
    font-size: 3em;

    text-transform: uppercase;
    -webkit-text-stroke: 2px #0f0f0f;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
/*      background-color: #2f2f2f;*/
    }
  }
</style>
