<script setup>
import { ref, onMounted } from 'vue'
import { RouterLink, RouterView, useRoute } from 'vue-router'
import { useAuth } from '../composables/useAuth'
import { useApiBase } from '../composables/useApi'

const route = useRoute()
const auth = useAuth()
const { adminResourcesUrl } = useApiBase()

const navItems = ref([
  { path: '/admin', label: 'Overview', desc: 'Ringkasan data', icon: '◉' },
])

const sidebarOpen = ref(false)

function logout() {
  auth.logout()
  window.location.href = '/login'
}

function getAuthHeaders() {
  const t = auth.getToken()
  return t ? { Authorization: `Bearer ${t}` } : {}
}

function closeSidebar() {
  sidebarOpen.value = false
}

onMounted(async () => {
  try {
    const r = await fetch(adminResourcesUrl(), { headers: getAuthHeaders() })
    if (r.ok) {
      const data = await r.json()
      const resources = data.resources || []
      const icons = {
        'skill-categories': '▣',
        'skills': '◆',
        'tools': '◇',
        'tags': '#',
        'projects': '▤',
        'posts': '✎',
      }
      navItems.value = [
        { path: '/admin', label: 'Overview', desc: 'Ringkasan data', icon: '◉' },
        ...resources.map((res) => ({
          path: `/admin/${res.id}`,
          label: res.label,
          desc: `Kelola ${res.label.toLowerCase()}`,
          icon: icons[res.id] || '•',
        })),
      ]
    }
  } catch (_) {}
})
</script>

<template>
  <div class="min-h-screen bg-neutral-100 dark:bg-neutral-950 text-neutral-900 dark:text-neutral-100 flex">
    <!-- Sidebar -->
    <aside
      class="fixed inset-y-0 left-0 z-40 w-64 bg-white dark:bg-neutral-900 border-r border-neutral-200 dark:border-neutral-800 flex flex-col transition-transform duration-200 ease-out lg:translate-x-0"
      :class="sidebarOpen ? 'translate-x-0' : '-translate-x-full'"
    >
      <div class="flex items-center justify-between h-14 px-4 border-b border-neutral-200 dark:border-neutral-800 shrink-0">
        <RouterLink to="/admin" class="font-semibold text-lg tracking-tight text-neutral-900 dark:text-white">
          Dashboard
        </RouterLink>
        <button
          type="button"
          aria-label="Tutup menu"
          class="lg:hidden p-2 rounded-lg hover:bg-neutral-100 dark:hover:bg-neutral-800"
          @click="closeSidebar"
        >
          <span class="text-lg">×</span>
        </button>
      </div>
      <nav class="flex-1 overflow-y-auto p-3 space-y-0.5">
        <RouterLink
          v-for="item in navItems"
          :key="item.path"
          :to="item.path"
          class="flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm font-medium transition-colors"
          :class="route.path === item.path || (item.path !== '/admin' && route.path.startsWith(item.path + '/'))
            ? 'bg-neutral-900 dark:bg-neutral-100 text-white dark:text-neutral-900'
            : 'text-neutral-600 dark:text-neutral-400 hover:bg-neutral-100 dark:hover:bg-neutral-800 hover:text-neutral-900 dark:hover:text-white'"
          @click="closeSidebar"
        >
          <span class="text-base opacity-90" aria-hidden="true">{{ item.icon }}</span>
          <div class="min-w-0">
            <span class="block truncate">{{ item.label }}</span>
            <span v-if="item.desc" class="block text-xs opacity-75 truncate">{{ item.desc }}</span>
          </div>
        </RouterLink>
      </nav>
      <div class="p-3 border-t border-neutral-200 dark:border-neutral-800">
        <RouterLink
          to="/"
          class="flex items-center gap-3 px-3 py-2 rounded-lg text-sm text-neutral-600 dark:text-neutral-400 hover:bg-neutral-100 dark:hover:bg-neutral-800 hover:text-neutral-900 dark:hover:text-white"
        >
          ← Kembali ke situs
        </RouterLink>
      </div>
    </aside>

    <!-- Overlay when sidebar open on mobile -->
    <div
      v-show="sidebarOpen"
      class="fixed inset-0 z-30 bg-black/50 lg:hidden"
      aria-hidden="true"
      @click="closeSidebar"
    />

    <!-- Main content -->
    <div class="flex-1 flex flex-col min-w-0 lg:pl-64">
      <header class="sticky top-0 z-20 h-14 border-b border-neutral-200 dark:border-neutral-800 bg-white/80 dark:bg-neutral-900/80 backdrop-blur flex items-center justify-between px-4 gap-4">
        <button
          type="button"
          aria-label="Buka menu"
          class="lg:hidden p-2 rounded-lg hover:bg-neutral-100 dark:hover:bg-neutral-800"
          @click="sidebarOpen = true"
        >
          <span class="text-lg">☰</span>
        </button>
        <div class="flex-1 min-w-0">
          <h1 class="text-sm font-medium text-neutral-500 dark:text-neutral-400 truncate">
            {{ route.path === '/admin' ? 'Overview' : (navItems.find((n) => route.path.startsWith(n.path) && n.path !== '/admin')?.label || 'Admin') }}
          </h1>
        </div>
        <div class="flex items-center gap-2">
          <span class="text-xs text-neutral-500 dark:text-neutral-400 hidden sm:inline">Admin</span>
          <button
            type="button"
            @click="logout"
            class="px-3 py-1.5 rounded-lg text-sm font-medium border border-neutral-200 dark:border-neutral-700 hover:bg-neutral-100 dark:hover:bg-neutral-800 transition-colors"
          >
            Logout
          </button>
        </div>
      </header>

      <main class="flex-1 p-4 sm:p-6 overflow-auto">
        <RouterView />
      </main>
    </div>
  </div>
</template>
