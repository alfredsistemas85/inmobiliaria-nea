import { fetchApi } from '@/services/api';

export interface Client {
  id: string
  first_name: string | null
  last_name: string | null
  phone: string
  email: string | null
  notes: string | null
  created_at: string
}

export interface PaginatedResponse<T> {
  data: T[]
  total: number
  limit: number
  offset: number
}

export const clientsService = {
  async getClients(limit = 20, offset = 0, q?: string): Promise<PaginatedResponse<Client>> {
    const params = new URLSearchParams()
    params.append('limit', limit.toString())
    params.append('offset', offset.toString())
    if (q) params.append('q', q)

    return fetchApi(`/clients?${params.toString()}`)
  },

  async createClient(data: Partial<Client>): Promise<Client> {
    return fetchApi('/clients', {
      method: 'POST',
      body: JSON.stringify(data),
    })
  },

  async updateClient(id: string, data: Partial<Client>): Promise<Client> {
    return fetchApi(`/clients/${id}`, {
      method: 'PUT',
      body: JSON.stringify(data),
    })
  },

  async deleteClient(id: string): Promise<void> {
    return fetchApi(`/clients/${id}`, {
      method: 'DELETE',
    })
  },
}
