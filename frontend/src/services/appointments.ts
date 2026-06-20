import { fetchApi } from '@/services/api';
import { PaginatedResponse } from './clients'

export interface Appointment {
  id: string
  tenant_id: string
  client_id: string
  property_id: string | null
  user_id: string | null
  scheduled_at: string
  status: string | null
  notes: string | null
  confirmed_at: string | null
  cancelled_at: string | null
  confirmation_sent_at: string | null
  created_at: string | null
  updated_at: string | null
  deleted_at: string | null
}

export const appointmentsService = {
  async getAppointments(limit = 20, offset = 0, q?: string): Promise<PaginatedResponse<Appointment>> {
    const params = new URLSearchParams()
    params.append('limit', limit.toString())
    params.append('offset', offset.toString())
    if (q) params.append('q', q)

    return fetchApi(`/appointments?${params.toString()}`)
  },

  async createAppointment(data: Partial<Appointment>): Promise<Appointment> {
    return fetchApi('/appointments', {
      method: 'POST',
      body: JSON.stringify(data),
    })
  },

  async updateAppointment(id: string, data: Partial<Appointment>): Promise<Appointment> {
    return fetchApi(`/appointments/${id}`, {
      method: 'PUT',
      body: JSON.stringify(data),
    })
  },

  async deleteAppointment(id: string): Promise<void> {
    return fetchApi(`/appointments/${id}`, {
      method: 'DELETE',
    })
  },
}
