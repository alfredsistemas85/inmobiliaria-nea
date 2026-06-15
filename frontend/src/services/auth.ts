import { fetchApi } from './api';

export const authService = {
  login: async (credentials: any) => {
    return fetchApi('/api/auth/login', {
      method: 'POST',
      body: JSON.stringify(credentials),
    });
  },
  refresh: async (refresh_token: string) => {
    return fetchApi('/api/auth/refresh', {
      method: 'POST',
      body: JSON.stringify({ refresh_token }),
    });
  },
  me: async () => {
    return fetchApi('/api/auth/me', {
      method: 'GET',
    });
  },
  changePassword: async (data: any) => {
    return fetchApi('/api/auth/change-password', {
      method: 'POST',
      body: JSON.stringify(data),
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
