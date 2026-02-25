const PORTFOLIO_ADMIN_TOKEN = 'portfolio_admin_token';

export function getToken() {
  try {
    return sessionStorage.getItem(PORTFOLIO_ADMIN_TOKEN);
  } catch {
    return null;
  }
}

export function setToken(token) {
  try {
    sessionStorage.setItem(PORTFOLIO_ADMIN_TOKEN, token);
  } catch (_) {}
}

export function clearToken() {
  try {
    sessionStorage.removeItem(PORTFOLIO_ADMIN_TOKEN);
  } catch (_) {}
}
