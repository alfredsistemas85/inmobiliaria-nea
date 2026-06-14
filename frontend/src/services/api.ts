export const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000';

let isRefreshing = false;
let refreshSubscribers: ((token: string | null) => void)[] = [];

const subscribeTokenRefresh = (cb: (token: string | null) => void) => {
  refreshSubscribers.push(cb);
};

const onRefreshed = (token: string | null) => {
  refreshSubscribers.forEach((cb) => cb(token));
  refreshSubscribers = [];
};

/**
 * Limpia TODA la sesión y redirige al login.
 * Se asegura de que isRefreshing quede en false y los subscribers vacíos
 * ANTES de redirigir, para que no queden estados zombi si el redirect es lento.
 */
const handleAuthFailure = () => {
  isRefreshing = false;
  onRefreshed(null);           // resuelve todos los subscribers pendientes con null
  refreshSubscribers = [];
  localStorage.removeItem('token');
  localStorage.removeItem('refresh_token');
  localStorage.removeItem('tenant_id');
  localStorage.removeItem('user');
  // Usar replace para no dejar historial del estado roto
  window.location.replace('/login');
};

export const fetchApi = async (endpoint: string, options: RequestInit = {}): Promise<any> => {
  const token = localStorage.getItem('token');

  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
    ...(token ? { Authorization: `Bearer ${token}` } : {}),
    ...(options.headers as Record<string, string>),
  };

  const formattedEndpoint = endpoint.startsWith('/api')
    ? endpoint
    : endpoint.startsWith('/')
    ? `/api${endpoint}`
    : `/api/${endpoint}`;

  let response: Response;
  try {
    response = await fetch(`${API_URL}${formattedEndpoint}`, {
      ...options,
      headers,
    });
  } catch {
    // Error de red puro (sin conexión, CORS, etc.)
    throw new Error('Sin conexión con el servidor');
  }

  // ── 401: lógica de refresh ────────────────────────────────────────────────
  if (
    response.status === 401 &&
    formattedEndpoint !== '/api/auth/login' &&
    formattedEndpoint !== '/api/auth/refresh'
  ) {
    const refreshToken = localStorage.getItem('refresh_token');

    // Sin refresh token → sesión definitivamente inválida
    if (!refreshToken) {
      handleAuthFailure();
      return null;
    }

    // Si ya hay un refresh en curso, encolar esta request y esperar el resultado
    if (isRefreshing) {
      return new Promise<any>((resolve) => {
        subscribeTokenRefresh((newToken) => {
          if (!newToken) {
            // El refresh falló mientras esperábamos → devolver null (el redirect ya ocurrió)
            resolve(null);
            return;
          }
          const retryHeaders = { ...headers, Authorization: `Bearer ${newToken}` };
          fetch(`${API_URL}${formattedEndpoint}`, { ...options, headers: retryHeaders })
            .then((r) => {
              if (!r.ok) return null;
              if (r.status === 204) return null;
              return r.json().catch(() => null);
            })
            .then(resolve)
            .catch(() => resolve(null));
        });
      });
    }

    // Primera request en recibir 401: iniciar el refresh
    isRefreshing = true;

    try {
      const refreshResponse = await fetch(`${API_URL}/api/auth/refresh`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ refresh_token: refreshToken }),
      });

      if (!refreshResponse.ok) {
        handleAuthFailure();
        return null;
      }

      const data = await refreshResponse.json();
      const newAccessToken: string = data.access_token;
      const newRefreshToken: string = data.refresh_token;

      localStorage.setItem('token', newAccessToken);
      localStorage.setItem('refresh_token', newRefreshToken);

      // Notificar a requests encoladas ANTES de hacer el retry propio
      isRefreshing = false;
      onRefreshed(newAccessToken);

      // Reintentar la request original con el token nuevo
      const retryHeaders = { ...headers, Authorization: `Bearer ${newAccessToken}` };
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
    } catch (err) {
      // Si el error viene del throw de arriba (retry fallido), re-lanzar
      if (err instanceof Error && err.message !== 'Sin conexión con el servidor') {
        // Solo hacer auth failure si fue un problema de auth, no de red del retry
        if (!isRefreshing) throw err; // ya lo manejamos
      }
      handleAuthFailure();
      return null;
    }
  }

  // ── Otros errores HTTP ────────────────────────────────────────────────────
  if (!response.ok) {
    const errorData = await response.json().catch(() => null);
    throw new Error(errorData?.error || errorData?.message || 'Error en la petición');
  }

  if (response.status === 204) return null;
  return response.json();
};
