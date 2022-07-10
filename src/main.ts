import { createApp } from 'vue'
import App from './App.vue'
import i18n from './i18n'
import PrimeVue from 'primevue/config'
import 'primevue/resources/primevue.min.css'
import 'primevue/resources/themes/mdc-dark-indigo/theme.css'
import 'primeicons/primeicons.css'

const app = createApp(App);

app.use(i18n);
app.use(PrimeVue);
app.mount('#app');