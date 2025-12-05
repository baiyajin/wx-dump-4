import { createRouter, createWebHistory } from 'vue-router'
import Home from '../views/Home.vue'
import Chat from '../views/Chat.vue'
import Export from '../views/Export.vue'
import Statistics from '../views/Statistics.vue'
import Contacts from '../views/Contacts.vue'
import Favorite from '../views/Favorite.vue'
import Moments from '../views/Moments.vue'
import Cleanup from '../views/Cleanup.vue'
import Tools from '../views/Tools.vue'
import Settings from '../views/Settings.vue'

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home,
  },
  {
    path: '/chat',
    name: 'Chat',
    component: Chat,
  },
  {
    path: '/contacts',
    name: 'Contacts',
    component: Contacts,
  },
  {
    path: '/favorite',
    name: 'Favorite',
    component: Favorite,
  },
  {
    path: '/moments',
    name: 'Moments',
    component: Moments,
  },
  {
    path: '/export',
    name: 'Export',
    component: Export,
  },
  {
    path: '/statistics',
    name: 'Statistics',
    component: Statistics,
  },
  {
    path: '/cleanup',
    name: 'Cleanup',
    component: Cleanup,
  },
  {
    path: '/tools',
    name: 'Tools',
    component: Tools,
  },
  {
    path: '/settings',
    name: 'Settings',
    component: Settings,
  },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router

