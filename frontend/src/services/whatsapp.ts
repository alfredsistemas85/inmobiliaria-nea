import { fetchApi } from '@/services/api';

export interface Conversation {
  id: string;
  client_id: string;
  client_first_name?: string;
  client_last_name?: string;
  client_phone: string;
  status?: string;
  last_message_at?: string;
  unread_count?: number;
  assigned_user_id?: string;
  last_message_content?: string;
  last_message_direction?: string;
}

export interface Message {
  id: string;
  tenant_id: string;
  conversation_id: string;
  sender_type: string;
  content?: string;
  media_url?: string;
  media_type?: string;
  is_read?: boolean;
  created_at?: string;
  external_id?: string;
  direction?: string;
  status?: string;
  is_ai_generated?: boolean;
}

export interface PaginatedResponse<T> {
  data: T[];
  total: number;
  limit: number;
  offset: number;
}

export const whatsappService = {
  getConversations: async (page = 1, limit = 50): Promise<PaginatedResponse<Conversation>> => {
    return fetchApi(`/whatsapp/conversations?page=${page}&limit=${limit}`);
  },

  getMessages: async (conversationId: string, page = 1, limit = 50): Promise<PaginatedResponse<Message>> => {
    return fetchApi(`/whatsapp/conversations/${conversationId}/messages?page=${page}&limit=${limit}`);
  },

  sendMessage: async (conversationId: string, content: string): Promise<Message> => {
    return fetchApi(`/whatsapp/conversations/${conversationId}/messages`, {
      method: 'POST',
      body: JSON.stringify({ content }),
    });
  },

  getInstanceStatus: async () => {
    return fetchApi('/whatsapp/instance', {
      method: 'GET',
    });
  },

  createInstance: async (instanceName: string) => {
    return fetchApi('/whatsapp/instance', {
      method: 'POST',
      body: JSON.stringify({ instance_name: instanceName }),
    });
  },

  getQr: async () => {
    return fetchApi('/whatsapp/instance/qr', {
      method: 'GET',
    });
  },

  logoutInstance: async () => {
    return fetchApi('/whatsapp/instance/logout', {
      method: 'POST',
    });
  }
};
