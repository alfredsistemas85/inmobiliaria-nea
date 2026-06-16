import api from './api';

export const superadminService = {
  // Dashboard
  getStats: async () => {
    const response = await api.get('/superadmin/dashboard/stats');
    return response.data;
  },

  // Tenants
  getTenants: async () => {
    const response = await api.get('/superadmin/tenants');
    return response.data;
  },
  
  getTenant: async (id: string) => {
    const response = await api.get(`/superadmin/tenants/${id}`);
    return response.data;
  },

  createTenant: async (data: any) => {
    const response = await api.post('/superadmin/tenants', data);
    return response.data;
  },

  updateTenantStatus: async (id: string, status: string) => {
    const response = await api.put(`/superadmin/tenants/${id}/status`, { status });
    return response.data;
  },

  // Monitoring
  getSystemErrors: async () => {
    const response = await api.get('/superadmin/monitoring/errors');
    return response.data;
  },

  // Support
  getTickets: async () => {
    const response = await api.get('/superadmin/support');
    return response.data;
  }
};
