<script setup>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useAuth } from '../../composables/useAuth'
import { useApiBase } from '../../composables/useApi'

const router = useRouter()
const auth = useAuth()
const { adminUrl } = useApiBase()

const message = ref('')
const loading = ref(true)
const error = ref('')

onMounted(async () => {
  const token = auth.getToken()
  if (!token) {
    router.replace('/login')
    return
  }
  try {
    const r = await fetch(adminUrl(), {
      headers: { Authorization: `Bearer ${token}` },
    })
    if (r.status === 401) {
      auth.logout()
      router.replace('/login')
      return
    }
    if (!r.ok) {
      error.value = 'Gagal memuat data admin.'
      return
    }
    const data = await r.json()
    message.value = data.message || 'Admin area'
  } catch (_) {
    error.value = 'Koneksi gagal.'
  } finally {
    loading.value = false
  }
})
</script>

<template>
  <div class="space-y-6">
    <h2 class="text-xl font-semibold text-neutral-900 dark:text-white">Overview</h2>
    <p v-if="error" class="text-red-600 dark:text-red-400">{{ error }}</p>
    <div v-else-if="loading" class="text-neutral-500 dark:text-neutral-400">Memuat…</div>
    <div v-else class="p-6 rounded-xl border border-neutral-200 dark:border-neutral-800 bg-white dark:bg-neutral-900/50 space-y-4">
      <p class="text-neutral-700 dark:text-neutral-300">{{ message }}</p>
      <p class="text-sm text-neutral-500 dark:text-neutral-400">
        Gunakan menu samping untuk mengelola <strong>Kategori Skill</strong> dan <strong>Skills</strong> (tabel di database).
      </p>
    </div>
  </div>
</template>
