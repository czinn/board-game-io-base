<script>
import BoardGameUi from './BoardGameUi.svelte'
import Config from './Config.svelte'
</script>

<BoardGameUi game_name="Example">
  <Config slot="config"
          let:config={config} config={config}
          let:readonly={readonly} readonly={readonly}
          let:config_handler={config_handler} on:config_change={config_handler} />
</BoardGameUi>

<style>
</style>
