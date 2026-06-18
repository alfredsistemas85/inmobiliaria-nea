import { fetchApi } from './api';

export interface Property {
  id: string;
  tenant_id: string;
  title: string;
  description: string;
  property_type: string;
  operation_type: string;
  price: number;
  currency: string;
  status: string;
  location?: string;
  bedrooms?: number;
  bathrooms?: number;
  area_sqm?: number;
  views?: number;
  images?: { url: string }[];
  created_at: string;
}

export const propertiesService = {
  // NOTA: fetchApi ya agrega /api automáticamente. No incluir /api aquí.
  getAll: async (limit: number = 20, offset: number = 0) => {
    return fetchApi(`/properties?limit=${limit}&offset=${offset}`, {
      method: 'GET',
    });
  },
  getById: async (id: string) => {
    return fetchApi(`/properties/${id}`, {
      method: 'GET',
    });
  },
  create: async (data: any) => {
    return fetchApi('/properties', {
      method: 'POST',
      body: JSON.stringify(data),
    });
  },
  update: async (id: string, data: any) => {
    return fetchApi(`/properties/${id}`, {
      method: 'PUT',
      body: JSON.stringify(data),
    });
  },
  delete: async (id: string) => {
    return fetchApi(`/properties/${id}`, {
      method: 'DELETE',
    });
  },
  uploadImage: async (id: string, file: File) => {
    const formData = new FormData();
    formData.append('file', file);

    // Para FormData, el browser asigna el Content-Type multipart/form-data con boundary.
    // No seteamos Content-Type manualmente.
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
    return response.json().catch(() => null);
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
    return response.json().catch(() => null);
  },
};
