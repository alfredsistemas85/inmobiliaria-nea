import { fetchApi } from '@/services/api';
import { PaginatedResponse } from './clients'

export interface Lead {
  id: string
  tenant_id: string
  client_id: string
  property_id: string | null
  status: string | null
  source: string | null
  assigned_to: string | null
  created_at: string | null
  updated_at: string | null
  deleted_at: string | null
}

export const leadsService = {
  async getLeads(limit = 100, offset = 0, q?: string): Promise<PaginatedResponse<Lead>> {
    const params = new URLSearchParams()
    params.append('limit', limit.toString())
    params.append('offset', offset.toString())
    if (q) params.append('q', q)

    return fetchApi(`/leads?${params.toString()}`)
  },

  async createLead(data: Partial<Lead>): Promise<Lead> {
    return fetchApi('/leads', {
      method: 'POST',
      body: JSON.stringify(data),
    })
  },

  async updateLead(id: string, data: Partial<Lead>): Promise<Lead> {
    return fetchApi(`/leads/${id}`, {
      method: 'PUT',
      body: JSON.stringify(data),
    })
  },

  async convertLead(id: string): Promise<Lead> {
    return fetchApi(`/leads/${id}/convert`, {
      method: 'POST',
    })
  },

  async deleteLead(id: string): Promise<void> {
    return fetchApi(`/leads/${id}`, {
      method: 'DELETE',
    })
  },
}
