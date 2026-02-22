<script setup>
import { ref, onMounted, computed } from 'vue'
import { useAuth } from '../../composables/useAuth'
import { useApiBase } from '../../composables/useApi'

const auth = useAuth()
const { adminSkillsUrl, adminCategoriesUrl } = useApiBase()

const skills = ref([])
const categories = ref([])
const loading = ref(true)
const error = ref('')
const formOpen = ref(false)
const editing = ref(null) // { id, category_id, name, level, icon_url }
const saving = ref(false)
const deleteConfirm = ref(null) // id

const formTitle = computed(() => (editing.value ? 'Edit skill' : 'Tambah skill'))

function getAuthHeaders() {
  const t = auth.getToken()
  return t ? { Authorization: `Bearer ${t}` } : {}
}

async function loadCategories() {
  try {
    const r = await fetch(adminCategoriesUrl(), { headers: getAuthHeaders() })
    if (r.ok) {
      const data = await r.json()
      categories.value = data.categories || []
    }
  } catch (_) {}
}

async function load() {
  loading.value = true
  error.value = ''
  try {
    const r = await fetch(adminSkillsUrl(), { headers: getAuthHeaders() })
    if (r.status === 401) {
      auth.logout()
      window.location.href = '/login'
      return
    }
    if (!r.ok) throw new Error('Gagal memuat')
    const data = await r.json()
    skills.value = data.skills || []
  } catch (e) {
    error.value = e.message || 'Koneksi gagal'
  } finally {
    loading.value = false
  }
}

function openCreate() {
  editing.value = null
  formOpen.value = true
}

function openEdit(skill) {
  editing.value = {
    id: skill.id,
    category_id: skill.category_id,
    name: skill.name,
    level: skill.level,
    icon_url: skill.icon_url ?? '',
  }
  formOpen.value = true
}

function closeForm() {
  formOpen.value = false
  editing.value = null
}

async function submitForm(payload) {
  saving.value = true
  error.value = ''
  const body = {
    category_id: payload.category_id,
    name: payload.name,
    level: payload.level,
    icon_url: payload.icon_url || null,
  }
  if (payload.icon_url === '') body.icon_url = null
  try {
    const url = adminSkillsUrl()
    if (payload.id) {
      body.id = payload.id
      const r = await fetch(url, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json', ...getAuthHeaders() },
        body: JSON.stringify(body),
      })
      if (r.status === 401) {
        auth.logout()
        window.location.href = '/login'
        return
      }
      const data = await r.json().catch(() => ({}))
      if (!r.ok) throw new Error(data.error || 'Gagal menyimpan')
    } else {
      const r = await fetch(url, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', ...getAuthHeaders() },
        body: JSON.stringify(body),
      })
      if (r.status === 401) {
        auth.logout()
        window.location.href = '/login'
        return
      }
      const data = await r.json().catch(() => ({}))
      if (!r.ok) throw new Error(data.error || 'Gagal menyimpan')
    }
    closeForm()
    await load()
  } catch (e) {
    error.value = e.message || 'Gagal menyimpan'
  } finally {
    saving.value = false
  }
}

async function doDelete(id) {
  if (!confirm('Hapus skill ini?')) return
  saving.value = true
  error.value = ''
  try {
    const r = await fetch(`${adminSkillsUrl()}?id=${id}`, {
      method: 'DELETE',
      headers: getAuthHeaders(),
    })
    if (r.status === 401) {
      auth.logout()
      window.location.href = '/login'
      return
    }
    if (!r.ok) {
      const data = await r.json().catch(() => ({}))
      throw new Error(data.error || 'Gagal menghapus')
    }
    deleteConfirm.value = null
    await load()
  } catch (e) {
    error.value = e.message || 'Gagal menghapus'
  } finally {
    saving.value = false
  }
}

onMounted(async () => {
  await loadCategories()
  await load()
})
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <h2 class="text-xl font-semibold text-neutral-900 dark:text-white">Skills</h2>
      <button
        type="button"
        @click="openCreate"
        class="px-3 py-1.5 rounded-lg bg-neutral-900 dark:bg-white text-white dark:text-neutral-900 text-sm font-medium hover:opacity-90"
      >
        Tambah skill
      </button>
    </div>
    <p v-if="error" class="text-red-600 dark:text-red-400 text-sm">{{ error }}</p>
    <div v-if="loading" class="py-8 text-neutral-500 dark:text-neutral-400">Memuat…</div>
    <div v-else class="overflow-x-auto rounded-xl border border-neutral-200 dark:border-neutral-800">
      <table class="w-full text-sm">
        <thead class="bg-neutral-100 dark:bg-neutral-800/50">
          <tr>
            <th class="text-left py-3 px-4 font-medium text-neutral-700 dark:text-neutral-300">ID</th>
            <th class="text-left py-3 px-4 font-medium text-neutral-700 dark:text-neutral-300">Kategori</th>
            <th class="text-left py-3 px-4 font-medium text-neutral-700 dark:text-neutral-300">Nama</th>
            <th class="text-left py-3 px-4 font-medium text-neutral-700 dark:text-neutral-300">Level</th>
            <th class="w-24 py-3 px-4"></th>
          </tr>
        </thead>
        <tbody class="divide-y divide-neutral-200 dark:divide-neutral-800">
          <tr v-for="s in skills" :key="s.id" class="text-neutral-600 dark:text-neutral-400">
            <td class="py-3 px-4">{{ s.id }}</td>
            <td class="py-3 px-4">{{ s.category || '—' }}</td>
            <td class="py-3 px-4">{{ s.name }}</td>
            <td class="py-3 px-4">{{ s.level }}</td>
            <td class="py-3 px-4 flex gap-2">
              <button type="button" @click="openEdit(s)" class="text-blue-600 dark:text-blue-400 hover:underline">Edit</button>
              <button type="button" @click="deleteConfirm = s.id" class="text-red-600 dark:text-red-400 hover:underline">Hapus</button>
            </td>
          </tr>
          <tr v-if="!skills.length">
            <td colspan="5" class="py-6 px-4 text-center text-neutral-500 dark:text-neutral-400">Belum ada skill. Buat kategori dulu, lalu tambah skill.</td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Form modal -->
    <div v-if="formOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50" @click.self="closeForm">
      <div class="bg-white dark:bg-neutral-900 rounded-xl shadow-xl max-w-md w-full p-6 border border-neutral-200 dark:border-neutral-700 max-h-[90vh] overflow-y-auto">
        <h3 class="text-lg font-semibold text-neutral-900 dark:text-white mb-4">{{ formTitle }}</h3>
        <form
          @submit.prevent="
            (e) => {
              const fd = new FormData(e.target)
              submitForm({
                id: editing?.id,
                category_id: parseInt(fd.get('category_id'), 10),
                name: fd.get('name'),
                level: fd.get('level'),
                icon_url: fd.get('icon_url') || null,
              })
            }
          "
          class="space-y-4"
        >
          <div>
            <label class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-1">Kategori</label>
            <select
              name="category_id"
              required
              class="w-full px-3 py-2 rounded-lg border border-neutral-300 dark:border-neutral-600 bg-white dark:bg-neutral-800 text-neutral-900 dark:text-white"
            >
              <option value="">— Pilih kategori —</option>
              <option v-for="c in categories" :key="c.id" :value="c.id" :selected="editing?.category_id === c.id">
                {{ c.name }}
              </option>
            </select>
          </div>
          <div>
            <label class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-1">Nama</label>
            <input
              type="text"
              name="name"
              :value="editing?.name"
              required
              class="w-full px-3 py-2 rounded-lg border border-neutral-300 dark:border-neutral-600 bg-white dark:bg-neutral-800 text-neutral-900 dark:text-white"
              placeholder="Contoh: Ansible"
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-1">Level</label>
            <select
              name="level"
              required
              class="w-full px-3 py-2 rounded-lg border border-neutral-300 dark:border-neutral-600 bg-white dark:bg-neutral-800 text-neutral-900 dark:text-white"
            >
              <option value="Fundamental" :selected="editing?.level === 'Fundamental'">Fundamental</option>
              <option value="Advanced" :selected="editing?.level === 'Advanced'">Advanced</option>
              <option value="Expert" :selected="editing?.level === 'Expert'">Expert</option>
            </select>
          </div>
          <div>
            <label class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-1">Icon URL (opsional)</label>
            <input
              type="url"
              name="icon_url"
              :value="editing?.icon_url"
              class="w-full px-3 py-2 rounded-lg border border-neutral-300 dark:border-neutral-600 bg-white dark:bg-neutral-800 text-neutral-900 dark:text-white"
              placeholder="https://..."
            />
          </div>
          <div class="flex gap-2 pt-2">
            <button type="submit" :disabled="saving" class="px-4 py-2 rounded-lg bg-neutral-900 dark:bg-white text-white dark:text-neutral-900 font-medium disabled:opacity-50">
              {{ saving ? 'Menyimpan…' : 'Simpan' }}
            </button>
            <button type="button" @click="closeForm" class="px-4 py-2 rounded-lg border border-neutral-300 dark:border-neutral-600">Batal</button>
          </div>
        </form>
      </div>
    </div>

    <!-- Delete confirm -->
    <div v-if="deleteConfirm" class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50" @click.self="deleteConfirm = null">
      <div class="bg-white dark:bg-neutral-900 rounded-xl shadow-xl max-w-sm w-full p-6 border border-neutral-200 dark:border-neutral-700">
        <p class="text-neutral-700 dark:text-neutral-300 mb-4">Yakin hapus skill ini?</p>
        <div class="flex gap-2">
          <button type="button" @click="doDelete(deleteConfirm)" :disabled="saving" class="px-4 py-2 rounded-lg bg-red-600 text-white font-medium disabled:opacity-50">
            Ya, hapus
          </button>
          <button type="button" @click="deleteConfirm = null" class="px-4 py-2 rounded-lg border border-neutral-300 dark:border-neutral-600">Batal</button>
        </div>
      </div>
    </div>
  </div>
</template>
