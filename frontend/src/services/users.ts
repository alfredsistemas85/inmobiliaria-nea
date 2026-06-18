import { fetchApi } from './api';

export interface User {
  id: string;
  first_name: string;
  last_name: string;
  email: string;
  role: string;
  is_active: boolean;
  tenant_id: string;
}

export interface CreateUserPayload {
  email: string;
  first_name: string;
  last_name: string;
  role: string;
  password?: string;
}

export interface UpdateUserPayload {
  email?: string;
  first_name?: string;
  last_name?: string;
  role?: string;
  is_active?: boolean;
}

export const usersService = {
  // NOTA: fetchApi ya agrega /api automáticamente. No incluir /api aquí.
  getUsers: async () => {
    return fetchApi('/users', { method: 'GET' });
  },

  getUser: async (id: string) => {
    return fetchApi(`/users/${id}`, { method: 'GET' });
  },

  createUser: async (payload: CreateUserPayload) => {
    return fetchApi('/users', {
      method: 'POST',
      body: JSON.stringify(payload),
    });
  },

  updateUser: async (id: string, payload: UpdateUserPayload) => {
    return fetchApi(`/users/${id}`, {
      method: 'PUT',
      body: JSON.stringify(payload),
    });
  },

  deleteUser: async (id: string) => {
    return fetchApi(`/users/${id}`, { method: 'DELETE' });
  },
};
