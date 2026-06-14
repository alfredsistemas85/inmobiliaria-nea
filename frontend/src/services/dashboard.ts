import { fetchApi } from './api'

export interface DashboardStats {
  total_clients: number
  total_properties: number
  new_leads: number
  upcoming_appointments: number
}

export interface DashboardActivity {
  id: string
  title: string
  time: string
  type: string
}

export const dashboardService = {
  async getStats(): Promise<DashboardStats> {
    return fetchApi('/dashboard/stats')
  },

  async getActivity(): Promise<DashboardActivity[]> {
    return fetchApi('/dashboard/activity')
  },
}
