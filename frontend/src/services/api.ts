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

export const fetchApi = async (endpoint: string, options: RequestInit = {}): Promise<any> => {
  const token = localStorage.getItem('token');
  
  const headers = {
    'Content-Type': 'application/json',
    ...(token ? { Authorization: `Bearer ${token}` } : {}),
    ...options.headers,
  };

  const response = await fetch(`${API_URL}${endpoint}`, {
    ...options,
    headers,
  });

  if (!response.ok) {
    if (response.status === 401 && endpoint !== '/api/auth/login' && endpoint !== '/api/auth/refresh') {
      const refreshToken = localStorage.getItem('refresh_token');
      if (!refreshToken) {
        localStorage.removeItem('token');
        window.location.href = '/';
        return null;
      }

      if (!isRefreshing) {
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
            onRefreshed(data.access_token);
            isRefreshing = false;
          } else {
            localStorage.clear();
            window.location.href = '/';
            return null;
          }
        } catch (error) {
          localStorage.clear();
          window.location.href = '/';
          return null;
        }
      }

      return new Promise((resolve) => {
        subscribeTokenRefresh((newToken) => {
          const newHeaders = { ...headers, Authorization: `Bearer ${newToken}` };
          resolve(fetch(`${API_URL}${endpoint}`, { ...options, headers: newHeaders }).then(r => r.json().catch(() => null)));
        });
      });
    }

    const errorData = await response.json().catch(() => null);
    throw new Error(errorData?.error || errorData?.message || 'Error en la petición');
  }

  if (response.status === 204) return null;
  
  return response.json();
};
