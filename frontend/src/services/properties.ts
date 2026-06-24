import { fetchApi } from '@/services/api';

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
    const token = localStorage.getItem('token');
    const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000';

    // 1. Obtener URL firmada
    const res = await fetch(`${API_URL}/api/documents/upload-url`, {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${token}`,
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        file_name: file.name.replace(/[^a-zA-Z0-9.-]/g, '_'),
        file_size: file.size,
        mime_type: file.type || 'image/jpeg',
        entity_type: 'property',
        entity_id: id
      })
    });

    if (!res.ok) throw new Error('Error al generar url de subida de imagen');
    const { upload_url } = await res.json();

    // 2. Subir directo a Supabase
    const uploadRes = await fetch(upload_url, {
      method: 'PUT',
      headers: { 'Content-Type': file.type || 'image/jpeg' },
      body: file
    });

    if (!uploadRes.ok) throw new Error('Error al subir imagen a storage');
    return { success: true };
  },
  uploadDocument: async (id: string, file: File) => {
    const token = localStorage.getItem('token');
    const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000';

    // 1. Obtener URL firmada
    const res = await fetch(`${API_URL}/api/documents/upload-url`, {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${token}`,
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        file_name: file.name.replace(/[^a-zA-Z0-9.-]/g, '_'),
        file_size: file.size,
        mime_type: file.type || 'application/octet-stream',
        entity_type: 'property',
        entity_id: id
      })
    });

    if (!res.ok) throw new Error('Error al generar url de subida de documento');
    const { upload_url } = await res.json();

    // 2. Subir directo a Supabase
    const uploadRes = await fetch(upload_url, {
      method: 'PUT',
      headers: { 'Content-Type': file.type || 'application/octet-stream' },
      body: file
    });

    if (!uploadRes.ok) throw new Error('Error al subir documento a storage');
    return { success: true };
  },
};
