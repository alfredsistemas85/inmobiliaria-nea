import { fetchApi } from './api';

export const propertiesService = {
  getAll: async () => {
    return fetchApi('/api/properties', {
      method: 'GET',
    });
  },
  getById: async (id: string) => {
    return fetchApi(`/api/properties/${id}`, {
      method: 'GET',
    });
  },
  create: async (data: any) => {
    return fetchApi('/api/properties', {
      method: 'POST',
      body: JSON.stringify(data),
    });
  },
  delete: async (id: string) => {
    return fetchApi(`/api/properties/${id}`, {
      method: 'DELETE',
    });
  }
};
