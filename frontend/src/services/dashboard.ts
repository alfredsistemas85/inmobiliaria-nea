import { fetchApi } from './api'

export interface DashboardStats {
  total_clients: number
  total_properties: number
  new_leads: number
  upcoming_appointments: number
  active_whatsapp_conversations: number
  leads_this_month: number
  conversions_this_month: number
  
  // New fields for charts
  leads_by_status: { status: string; count: number }[]
  conversations_by_agent: { agent_name: string; count: number }[]
  conversions_by_month: { month: string; count: number }[]
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
