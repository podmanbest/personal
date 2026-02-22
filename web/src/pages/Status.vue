<script setup>
import { ref, onMounted } from 'vue'
import { useApiBase, apiFetch } from '../composables/useApi'

const { healthUrl, statusUrl } = useApiBase()
const health = ref(null)
const status = ref(null)
const error = ref(null)

onMounted(async () => {
  try {
    const [healthData, statusData] = await Promise.all([
      apiFetch(healthUrl()).catch(() => null),
      apiFetch(statusUrl()).catch((e) => { throw e }),
    ])
    health.value = healthData
    status.value = statusData
  } catch (e) {
    error.value = e.message || 'Gagal memuat status API'
    status.value = null
  }
})
</script>

<template>
  <div class="space-y-8">
    <h1 class="text-3xl font-bold text-neutral-900 dark:text-white">Status</h1>
    <p class="text-neutral-600 dark:text-neutral-400">
      Uptime dan kesehatan layanan (Phase 5 — dogfooding).
    </p>
    <div
      v-if="error"
      class="p-4 rounded-lg bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 text-red-700 dark:text-red-300"
    >
      {{ error }}
    </div>
    <div
      v-else-if="status"
      class="space-y-6"
    >
      <div class="p-6 rounded-xl border border-neutral-200 dark:border-neutral-800 bg-white dark:bg-neutral-900/50 space-y-4">
        <div class="flex items-center gap-3">
          <span
            class="w-3 h-3 rounded-full"
            :class="status.status === 'ok' ? 'bg-green-500 animate-pulse' : 'bg-amber-500'"
          />
          <span class="font-mono text-lg">{{ status.status === 'ok' ? 'Operational' : status.status }}</span>
        </div>
        <dl class="grid grid-cols-1 sm:grid-cols-2 gap-3 text-sm">
          <div>
            <dt class="text-neutral-500 dark:text-neutral-400">Uptime (seconds)</dt>
            <dd class="font-mono text-neutral-900 dark:text-white">{{ status.uptime_seconds ?? 0 }}</dd>
          </div>
          <div>
            <dt class="text-neutral-500 dark:text-neutral-400">Database</dt>
            <dd class="font-mono text-neutral-900 dark:text-white">{{ status.database ?? '—' }}</dd>
          </div>
        </dl>
      </div>
      <div
        v-if="health"
        class="p-4 rounded-lg border border-neutral-200 dark:border-neutral-800 bg-white dark:bg-neutral-900/50 text-sm"
      >
        <span class="text-neutral-500 dark:text-neutral-400">Health:</span>
        <span class="font-mono text-neutral-900 dark:text-white ml-2">{{ health.status ?? '—' }}</span>
      </div>
    </div>
    <div v-else class="p-6 rounded-xl border border-neutral-200 dark:border-neutral-800 animate-pulse">
      Loading…
    </div>
  </div>
</template>
