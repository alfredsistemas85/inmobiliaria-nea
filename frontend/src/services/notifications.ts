import { fetchApi } from './api'

export interface Notification {
  id: string
  tenant_id: string
  user_id: string
  type: string
  title: string
  message: string
  is_read: boolean
  created_at: string
}

export const notificationsService = {
  getNotifications: async (): Promise<{ data: Notification[] }> => {
    return fetchApi('/notifications')
  },

  markAsRead: async (id: string): Promise<void> => {
    return fetchApi(`/notifications/${id}/read`, { method: 'POST' })
  }
}
