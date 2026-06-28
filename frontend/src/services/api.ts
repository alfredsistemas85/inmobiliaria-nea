export const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000';

export class ApiError extends Error {
  public status?: number;
  public code?: string;
  public correlation_id?: string;
  
  constructor(message: string, status?: number, code?: string, correlation_id?: string) {
    super(message);
    this.name = 'ApiError';
    this.status = status;
    this.code = code;
    this.correlation_id = correlation_id;
  }
}

let isRefreshing = false;
let refreshSubscribers: ((token: string | null) => void)[] = [];

const subscribeTokenRefresh = (cb: (token: string | null) => void) => {
  refreshSubscribers.push(cb);
};

const onRefreshed = (token: string | null) => {
  refreshSubscribers.forEach((cb) => cb(token));
  refreshSubscribers = [];
};

const handleAuthFailure = () => {
  isRefreshing = false;
  onRefreshed(null);
  refreshSubscribers = [];
  localStorage.removeItem('token');
  localStorage.removeItem('refresh_token');
  localStorage.removeItem('tenant_id');
  localStorage.removeItem('user');
  window.location.replace('/login');
};

const MAX_RETRIES = 3;
const BASE_DELAY_MS = 500;

async function fetchWithRetry(url: string, options: RequestInit): Promise<Response> {
  let attempt = 0;
  
  while (attempt <= MAX_RETRIES) {
    try {
      // Configuramos un timeout usando AbortController (ej: 10s)
      const controller = new AbortController();
      const timeoutId = setTimeout(() => controller.abort(), 10000);
      
      const response = await fetch(url, { ...options, signal: controller.signal });
      clearTimeout(timeoutId);
      
      // Si la respuesta es exitosa o un error "lógico" (400-500), no reintentamos (se maneja fuera).
      // Salvo que sea 502, 503, 504 que son errores de red/gateway.
      if (response.ok || (response.status >= 400 && response.status < 500) || response.status === 500) {
         return response;
      }
      
      // Si es 502/503/504, lo tratamos como error reintentable
      if (response.status === 502 || response.status === 503 || response.status === 504) {
         throw new Error(`Server network error: ${response.status}`);
      }
      
      return response;
    } catch (err: any) {
      // err puede ser de fetch fallido (network error) o abortado (timeout)
      const isRetryable = err.name === 'AbortError' || err.message.includes('network') || err.message.includes('fetch');
      
      if (!isRetryable || attempt === MAX_RETRIES) {
        throw new Error(err.name === 'AbortError' ? 'El servidor no respondió (Timeout)' : 'Sin conexión con el servidor');
      }
      
      attempt++;
      const delay = BASE_DELAY_MS * (2 ** (attempt - 1));
      await new Promise(res => setTimeout(res, delay));
    }
  }
  throw new Error('Sin conexión con el servidor');
}

export const fetchApi = async (endpoint: string, options: RequestInit = {}): Promise<any> => {
  const token = localStorage.getItem('token');
  const correlationId = (window as any).__CORRELATION_ID__;

  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
    ...(token ? { Authorization: `Bearer ${token}` } : {}),
    ...(correlationId ? { 'X-Correlation-ID': correlationId } : {}),
    ...(options.headers as Record<string, string>),
  };

  const formattedEndpoint = endpoint.startsWith('/api')
    ? endpoint
    : endpoint.startsWith('/')
    ? `/api${endpoint}`
    : `/api/${endpoint}`;

  let response: Response;
  try {
    response = await fetchWithRetry(`${API_URL}${formattedEndpoint}`, {
      ...options,
      headers,
    });
  } catch (err: any) {
    throw new ApiError(err.message, 0); // 0 status indicates network/timeout
  }

  // ── 401: lógica de refresh ────────────────────────────────────────────────
  if (
    response.status === 401 &&
    formattedEndpoint !== '/api/auth/login' &&
    formattedEndpoint !== '/api/auth/refresh'
  ) {
    const refreshToken = localStorage.getItem('refresh_token');

    if (!refreshToken) {
      handleAuthFailure();
      return null;
    }

    if (isRefreshing) {
      return new Promise<any>((resolve) => {
        subscribeTokenRefresh((newToken) => {
          if (!newToken) {
            resolve(null);
            return;
          }
          const retryHeaders = { ...headers, Authorization: `Bearer ${newToken}` };
          fetchWithRetry(`${API_URL}${formattedEndpoint}`, { ...options, headers: retryHeaders })
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

    isRefreshing = true;

    try {
      const refreshResponse = await fetchWithRetry(`${API_URL}/api/auth/refresh`, {
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

      isRefreshing = false;
      onRefreshed(newAccessToken);

      const retryHeaders = { ...headers, Authorization: `Bearer ${newAccessToken}` };
      const retryResponse = await fetchWithRetry(`${API_URL}${formattedEndpoint}`, {
        ...options,
        headers: retryHeaders,
      });

      if (!retryResponse.ok) {
        const errData = await retryResponse.json().catch(() => null);
        throw new ApiError(
          errData?.message || errData?.error || 'Error en la petición',
          retryResponse.status,
          errData?.code,
          errData?.correlation_id
        );
      }
      if (retryResponse.status === 204) return null;
      return retryResponse.json();
    } catch (err) {
      if (err instanceof ApiError && err.status !== 0) {
        if (!isRefreshing) throw err; 
      }
      handleAuthFailure();
      return null;
    }
  }

  // ── Otros errores HTTP ────────────────────────────────────────────────────
  if (!response.ok) {
    const errorData = await response.json().catch(() => null);
    throw new ApiError(
      errorData?.message || errorData?.error || 'Error en la petición',
      response.status,
      errorData?.code,
      errorData?.correlation_id
    );
  }

  if (response.status === 204) return null;
  
  const text = await response.text();
  return text ? JSON.parse(text) : null;
};
