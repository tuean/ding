import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
import router from './router/index.js'

const app = createApp(App)

app.directive('myfocus', {
    updated: (el, binding) => {
        // console.log('myfocus', el, binding)
        if (binding.value === true) {
            el.focus()
            console.log('myfocus focus called')
        }
    } 
})

app.use(router).mount('#app')
