<script>
import BoardGameUi from './BoardGameUi.svelte'
import Config from './Config.svelte'
import GameView from './GameView.svelte'
</script>

<BoardGameUi game_name="Example">
  <Config slot="config"
          let:config={config} config={config}
          let:readonly={readonly} readonly={readonly}
          let:config_handler={config_handler} on:config_change={config_handler}></Config>

  <GameView slot="game"
            let:view={view} view={view}
            let:do_action={do_action} on:do_action={do_action}/>
</BoardGameUi>
