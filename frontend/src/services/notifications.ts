import { fetchApi } from '@/services/api';

/**
 * Modelo de notificación que refleja exactamente la respuesta del backend.
 *
 * Backend (notification.rs):
 *   id: Uuid, tenant_id: Uuid, user_id: Option<Uuid>,
 *   type: String, title: String, content: String,
 *   read_at: Option<DateTime<Utc>>, created_at: Option<DateTime<Utc>>
 */
export interface Notification {
  id: string
  tenant_id: string
  user_id: string | null     // Option<Uuid> en el backend
  type: string               // 'NEW_LEAD' | 'NEW_MESSAGE' | 'ASSIGNED' | 'APPOINTMENT_CREATED'
  title: string
  content: string            // ← era 'message' (incorrecto), el backend devuelve 'content'
  read_at: string | null     // ← era 'is_read: boolean' (incorrecto), el backend devuelve 'read_at'
  created_at: string | null
}

/**
 * Respuesta real del endpoint GET /api/notifications.
 * El backend devuelve { unread_count, notifications }, NO { data }.
 */
export interface NotificationListResponse {
  unread_count: number
  notifications: Notification[]
}

export const notificationsService = {
  getNotifications: async (): Promise<NotificationListResponse> => {
    return fetchApi('/notifications')
  },

  markAsRead: async (id: string): Promise<void> => {
    return fetchApi(`/notifications/${id}/read`, { method: 'POST' })
  },
}
