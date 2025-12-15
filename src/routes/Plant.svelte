<script>
  let pot = $state("pot1.png");
  let plant = $state("plant1.png");
</script>

{#if pot !== "" && plant !== ""}
  <img id="plant" src="/{plant}" alt="A growing plant."/>
  <img id="pot" src="/{pot}" alt="A plant pot."/>
{/if}

<style>
  #plant {
    position: fixed;
    top: calc(55vh - 25vh);
    left: calc(50% - 30vh / 2);

    height: 30vh;
    width: 30vh;
  }

  #pot {
    position: fixed;
    top: 55vh;
    left: calc(50% - 30vh / 2);

    height: 30vh;
    width: 30vh;
  }
</style>