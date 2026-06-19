import { createApp } from 'vue'
import App from './App.vue'
import './styles.css'

createApp(App).mount('#app')
requestAnimationFrame(() => document.body.classList.remove('app-loading'))
