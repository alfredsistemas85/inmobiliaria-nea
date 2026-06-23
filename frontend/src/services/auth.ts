import { fetchApi } from '@/services/api';

export const authService = {
  // INC-014: Unified paths — fetchApi auto-prepends /api
  login: async (credentials: any) => {
    return fetchApi('/auth/login', {
      method: 'POST',
      body: JSON.stringify(credentials),
    });
  },
  refresh: async (refresh_token: string) => {
    return fetchApi('/auth/refresh', {
      method: 'POST',
      body: JSON.stringify({ refresh_token }),
    });
  },
  me: async () => {
    return fetchApi('/auth/me', {
      method: 'GET',
    });
  },
  changePassword: async (data: any) => {
    return fetchApi('/auth/change-password', {
      method: 'POST',
      body: JSON.stringify(data),
    });
  },
  verifyEmail: async (token: string) => {
    return fetchApi('/auth/verify-email', {
      method: 'POST',
      body: JSON.stringify({ token }),
    });
  },
  setupPassword: async (token: string, password: string) => {
    return fetchApi('/auth/setup-password', {
      method: 'POST',
      body: JSON.stringify({ token, password }),
    });
  },
  logout: () => {
    localStorage.removeItem('token');
    localStorage.removeItem('refresh_token');
    localStorage.removeItem('tenant_id');
    localStorage.removeItem('user');
    window.location.href = '/';
  }
};
