// import { createApp } from 'vue';
// import App from './App.vue';

// const app = createApp(App);
// app.mount('#app');

import { createApp } from 'vue'
import App from './App.vue'
import router from './router'

createApp(App)
  .use(router)
  .mount('#app')


import iconUrl from '@assets/app.svg';
import metaFileRaw from '@assets/meta.yml?raw';
import { parse } from 'yaml';

const meta = parse(metaFileRaw);

const title = `${meta.title} — ${meta.version} ${meta.status}`;
const favicon = { rel: 'icon', type: 'image/svg+xml', href: iconUrl };

const headTitle = document.querySelector('title');
if (headTitle) {
	headTitle.textContent = title;
}

const headFavicon = document.querySelector('link[rel="icon"]');
if (headFavicon) {
	headFavicon.setAttribute('href', favicon.href);
}
