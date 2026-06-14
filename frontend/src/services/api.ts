export const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000';

let isRefreshing = false;
let refreshSubscribers: ((token: string) => void)[] = [];

const subscribeTokenRefresh = (cb: (token: string) => void) => {
  refreshSubscribers.push(cb);
};

const onRefreshed = (token: string) => {
  refreshSubscribers.forEach((cb) => cb(token));
  refreshSubscribers = [];
};

/**
 * Limpia el estado del refresh y redirige al login.
 * Centraliza todos los caminos de fallo para evitar estados inconsistentes.
 */
const handleAuthFailure = () => {
  isRefreshing = false;
  refreshSubscribers = [];
  localStorage.clear();
  window.location.href = '/login';
};

export const fetchApi = async (endpoint: string, options: RequestInit = {}): Promise<any> => {
  const token = localStorage.getItem('token');

  const headers = {
    'Content-Type': 'application/json',
    ...(token ? { Authorization: `Bearer ${token}` } : {}),
    ...options.headers,
  };

  const formattedEndpoint = endpoint.startsWith('/api')
    ? endpoint
    : endpoint.startsWith('/')
    ? `/api${endpoint}`
    : `/api/${endpoint}`;

  const response = await fetch(`${API_URL}${formattedEndpoint}`, {
    ...options,
    headers,
  });

  if (!response.ok) {
    // ── 401: intento de refresh ──────────────────────────────────────────────
    if (
      response.status === 401 &&
      formattedEndpoint !== '/api/auth/login' &&
      formattedEndpoint !== '/api/auth/refresh'
    ) {
      const refreshToken = localStorage.getItem('refresh_token');

      // Sin refresh token → redirect inmediato, sin Promise pendiente
      if (!refreshToken) {
        handleAuthFailure();
        return null;
      }

      // Si ya hay un refresh en curso, encolar esta request
      if (isRefreshing) {
        return new Promise<any>((resolve) => {
          subscribeTokenRefresh((newToken) => {
            const newHeaders = { ...headers, Authorization: `Bearer ${newToken}` };
            resolve(
              fetch(`${API_URL}${formattedEndpoint}`, { ...options, headers: newHeaders })
                .then((r) => {
                  if (!r.ok) return null;
                  if (r.status === 204) return null;
                  return r.json().catch(() => null);
                })
            );
          });
        });
      }

      // Primera request en 401: iniciar refresh
      isRefreshing = true;

      try {
        const res = await fetch(`${API_URL}/api/auth/refresh`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ refresh_token: refreshToken }),
        });

        if (res.ok) {
          const data = await res.json();
          localStorage.setItem('token', data.access_token);
          localStorage.setItem('refresh_token', data.refresh_token);
          isRefreshing = false;       // ← reset antes de notificar
          onRefreshed(data.access_token);

          // Reintentar la request original con el nuevo token
          const retryHeaders = { ...headers, Authorization: `Bearer ${data.access_token}` };
          const retryResponse = await fetch(`${API_URL}${formattedEndpoint}`, {
            ...options,
            headers: retryHeaders,
          });

          if (!retryResponse.ok) {
            const errData = await retryResponse.json().catch(() => null);
            throw new Error(errData?.error || errData?.message || 'Error en la petición');
          }
          if (retryResponse.status === 204) return null;
          return retryResponse.json();
        } else {
          // Refresh falló (token inválido/expirado)
          handleAuthFailure();   // ← resetea isRefreshing + limpia storage
          return null;
        }
      } catch {
        // Error de red durante el refresh
        handleAuthFailure();     // ← resetea isRefreshing + limpia storage
        return null;
      }
    }

    // ── Cualquier otro error HTTP ────────────────────────────────────────────
    const errorData = await response.json().catch(() => null);
    throw new Error(errorData?.error || errorData?.message || 'Error en la petición');
  }

  if (response.status === 204) return null;

  return response.json();
};
