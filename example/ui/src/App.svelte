<script>
import BoardGameUi from './BoardGameUi.svelte'
import Config from './Config.svelte'
</script>

<BoardGameUi game_name="Example">
  <Config slot="config"
          let:config={config} config={config}
          let:readonly={readonly} readonly={readonly} />
</BoardGameUi>

<style>
</style>
