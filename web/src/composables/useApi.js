/**
 * Base URL untuk panggilan API backend.
 * Dev: proxy Vite mengirim /api ke backend (lihat vite.config.js).
 * Build: set VITE_API_URL (mis. https://api.example.com) bila frontend beda origin.
 */
export function useApiBase() {
  const base = import.meta.env.VITE_API_URL ?? ''
  const root = base ? base.replace(/\/$/, '') : ''

  /** Path absolut: root + path atau path saja bila root kosong. */
  function apiPath(path) {
    return root ? `${root}${path}` : path
  }

  /** Path dengan slug (encode URI). */
  function apiPathSlug(prefix, slug) {
    return apiPath(`${prefix}/${encodeURIComponent(slug)}`)
  }

  /** URL admin per resource (CMS dinamis). path = skill-categories | skills | tools | tags | projects | posts */
  function adminResourceUrl(path) {
    return apiPath(`/api/admin/${path}`)
  }

  return {
    apiBase: base,
    healthUrl: () => apiPath('/api/health'),
    statusUrl: () => apiPath('/api/status'),
    skillsUrl: () => apiPath('/api/skills'),
    projectsUrl: () => apiPath('/api/projects'),
    projectBySlugUrl: (slug) => apiPathSlug('/api/projects', slug),
    postsUrl: () => apiPath('/api/posts'),
    postBySlugUrl: (slug) => apiPathSlug('/api/posts', slug),
    loginUrl: () => apiPath('/api/login'),
    adminUrl: () => apiPath('/api/admin'),
    adminResourcesUrl: () => apiPath('/api/admin/resources'),
    adminCategoriesUrl: () => apiPath('/api/admin/skill-categories'),
    adminSkillsUrl: () => apiPath('/api/admin/skills'),
    adminResourceUrl,
  }
}

/**
 * Panggil API dengan fetch + JSON. Header Authorization opsional.
 * @param {string} url - URL lengkap (dari useApiBase)
 * @param {{ method?: string, body?: object, token?: string }} options
 * @returns {Promise<object>} Response body (parsed JSON)
 * @throws {Error} Jika !response.ok; message dari body.error atau statusText
 */
export async function apiFetch(url, { method = 'GET', body, token } = {}) {
  const headers = {}
  if (body !== undefined) headers['Content-Type'] = 'application/json'
  if (token) headers['Authorization'] = `Bearer ${token}`

  const res = await fetch(url, {
    method,
    headers,
    body: body !== undefined ? JSON.stringify(body) : undefined,
  })

  let data = {}
  try {
    const text = await res.text()
    if (text) data = JSON.parse(text)
  } catch (_) {}

  if (!res.ok) {
    const msg = data.error || data.message || res.statusText
    const err = new Error(msg)
    err.status = res.status
    err.data = data
    throw err
  }
  return data
}
