<script setup>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { RouterLink } from 'vue-router'
import { useAuth } from '../../composables/useAuth'
import { useApiBase } from '../../composables/useApi'

const router = useRouter()
const auth = useAuth()
const { adminUrl, adminResourcesUrl, adminResourceUrl } = useApiBase()

const LIST_KEYS = {
  'skill-categories': 'categories',
  'skills': 'skills',
  'tools': 'tools',
  'tags': 'tags',
  'projects': 'projects',
  'posts': 'posts',
}

const message = ref('')
const loading = ref(true)
const error = ref('')
const resources = ref([])
const counts = ref({})

function getAuthHeaders() {
  const t = auth.getToken()
  return t ? { Authorization: `Bearer ${t}` } : {}
}

onMounted(async () => {
  const token = auth.getToken()
  if (!token) {
    router.replace('/login')
    return
  }
  try {
    const rAdmin = await fetch(adminUrl(), { headers: getAuthHeaders() })
    if (rAdmin.status === 401) {
      auth.logout()
      router.replace('/login')
      return
    }
    if (rAdmin.ok) {
      const data = await rAdmin.json()
      message.value = data.message || 'Admin area'
    } else {
      error.value = 'Gagal memuat data admin.'
    }

    const rRes = await fetch(adminResourcesUrl(), { headers: getAuthHeaders() })
    if (rRes.ok) {
      const data = await rRes.json()
      resources.value = data.resources || []
      await Promise.all(
        resources.value.map(async (res) => {
          try {
            const r = await fetch(adminResourceUrl(res.id), { headers: getAuthHeaders() })
            if (r.ok) {
              const d = await r.json()
              const key = LIST_KEYS[res.id]
              counts.value[res.id] = (d[key] || []).length
            } else {
              counts.value[res.id] = 0
            }
          } catch (_) {
            counts.value[res.id] = 0
          }
        })
      )
    }
  } catch (_) {
    error.value = 'Koneksi gagal.'
  } finally {
    loading.value = false
  }
})
</script>

<template>
  <div class="space-y-8">
    <!-- Welcome / Page title -->
    <div>
      <h2 class="text-2xl font-bold text-neutral-900 dark:text-white tracking-tight">Overview</h2>
      <p class="mt-1 text-sm text-neutral-500 dark:text-neutral-400">
        Ringkasan konten dan akses cepat ke resource CMS.
      </p>
    </div>

    <p v-if="error" class="text-sm text-red-600 dark:text-red-400">{{ error }}</p>
    <div v-else-if="loading" class="flex items-center gap-2 text-neutral-500 dark:text-neutral-400">
      <span class="inline-block w-5 h-5 border-2 border-neutral-300 dark:border-neutral-600 border-t-neutral-900 dark:border-t-white rounded-full animate-spin" />
      Memuat…
    </div>

    <template v-else>
      <!-- Welcome card -->
      <div class="rounded-xl border border-neutral-200 dark:border-neutral-800 bg-white dark:bg-neutral-900/50 p-6 shadow-sm">
        <p class="text-neutral-700 dark:text-neutral-300">{{ message }}</p>
        <p class="mt-2 text-sm text-neutral-500 dark:text-neutral-400">
          Kelola konten secara dinamis berdasarkan model database (CMS). Pilih resource di bawah atau dari menu samping.
        </p>
      </div>

      <!-- Stats / resource cards (dashboard style) -->
      <div>
        <h3 class="text-sm font-semibold text-neutral-700 dark:text-neutral-300 uppercase tracking-wider mb-4">
          Resource
        </h3>
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
          <RouterLink
            v-for="res in resources"
            :key="res.id"
            :to="`/admin/${res.id}`"
            class="group block rounded-xl border border-neutral-200 dark:border-neutral-800 bg-white dark:bg-neutral-900/50 p-5 shadow-sm hover:shadow-md hover:border-neutral-300 dark:hover:border-neutral-600 transition-all duration-200"
          >
            <div class="flex items-start justify-between">
              <div>
                <h4 class="font-semibold text-neutral-900 dark:text-white group-hover:text-neutral-700 dark:group-hover:text-neutral-200">
                  {{ res.label }}
                </h4>
                <p class="mt-1 text-2xl font-bold text-neutral-800 dark:text-neutral-200 tabular-nums">
                  {{ counts[res.id] ?? '—' }}
                </p>
                <p class="mt-1 text-xs text-neutral-500 dark:text-neutral-400">
                  Tambah / edit / hapus
                </p>
              </div>
              <span class="text-neutral-300 dark:text-neutral-600 group-hover:text-neutral-500 dark:group-hover:text-neutral-400 transition-colors">→</span>
            </div>
          </RouterLink>
        </div>
      </div>
    </template>
  </div>
</template>
