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
  update: async (id: string, data: any) => {
    return fetchApi(`/api/properties/${id}`, {
      method: 'PUT',
      body: JSON.stringify(data),
    });
  },
  delete: async (id: string) => {
    return fetchApi(`/api/properties/${id}`, {
      method: 'DELETE',
    });
  },
  uploadImage: async (id: string, file: File) => {
    const formData = new FormData();
    formData.append('file', file);
    
    // fetchApi envía Content-Type application/json por defecto si es objeto.
    // Para FormData, tenemos que dejar que el browser asigne el boundary multipart,
    // así que no seteamos Content-Type.
    const token = localStorage.getItem('token');
    const headers: any = {};
    if (token) headers['Authorization'] = `Bearer ${token}`;

    const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000';
    const response = await fetch(`${API_URL}/api/properties/${id}/images`, {
      method: 'POST',
      headers,
      body: formData,
    });
    if (!response.ok) throw new Error('Error al subir imagen');
    return response;
  },
  uploadDocument: async (id: string, file: File) => {
    const formData = new FormData();
    formData.append('file', file);
    
    const token = localStorage.getItem('token');
    const headers: any = {};
    if (token) headers['Authorization'] = `Bearer ${token}`;

    const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000';
    const response = await fetch(`${API_URL}/api/properties/${id}/documents`, {
      method: 'POST',
      headers,
      body: formData,
    });
    if (!response.ok) throw new Error('Error al subir documento');
    return response;
  }
};
