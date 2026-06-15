import { fetchApi } from './api';

export interface User {
  id: string;
  first_name: string;
  last_name: string;
  email: string;
  role_id: string;
  is_active: boolean;
  tenant_id: string;
}

export interface CreateUserPayload {
  email: string;
  first_name: string;
  last_name: string;
  role_id: string;
  password?: string;
}

export interface UpdateUserPayload {
  email?: string;
  first_name?: string;
  last_name?: string;
  role_id?: string;
  is_active?: boolean;
}

export const usersService = {
  getUsers: async () => {
    const data = await fetchApi('/api/users', { method: 'GET' });
    return data;
  },
  
  getUser: async (id: string) => {
    const data = await fetchApi(`/api/users/${id}`, { method: 'GET' });
    return data;
  },

  createUser: async (payload: CreateUserPayload) => {
    const data = await fetchApi('/api/users', {
      method: 'POST',
      body: JSON.stringify(payload),
    });
    return data;
  },

  updateUser: async (id: string, payload: UpdateUserPayload) => {
    const data = await fetchApi(`/api/users/${id}`, {
      method: 'PUT',
      body: JSON.stringify(payload),
    });
    return data;
  },

  deleteUser: async (id: string) => {
    const data = await fetchApi(`/api/users/${id}`, { method: 'DELETE' });
    return data;
  }
};
