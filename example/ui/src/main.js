import './app.css'
import App from './App.svelte'
import { BoardGameClient } from 'board-game-io-client'

const app = new App({
  target: document.getElementById('app')
})

window.client = new BoardGameClient('ws://localhost:9002')

export default app
