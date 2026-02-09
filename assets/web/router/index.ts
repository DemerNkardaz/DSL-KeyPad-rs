import { createRouter, createWebHistory } from 'vue-router'
import MainWindow from '@windows/main/Window.vue'
import SettingsWindow from '@windows/settings/Window.vue'
import TooltipWindow from '@windows/tooltip/Window.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'main',
      component: MainWindow,
    },
    {
      path: '/settings',
      name: 'settings',
      component: SettingsWindow,
		},
		{
			path: '/tooltip',
			name: 'tooltip',
			component: TooltipWindow,
		}
    // {
    //   path: '/:pathMatch(.*)*',
    //   name: 'not-found',
    //   component: TooltipWindow,
    // },
  ],
})

export default router
