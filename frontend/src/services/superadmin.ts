import { fetchApi } from '@/services/api';

export const superadminService = {
  // Dashboard
  getStats: async () => {
    return fetchApi('/superadmin/dashboard/stats');
  },

  // Tenants
  getTenants: async () => {
    return fetchApi('/superadmin/tenants');
  },
  
  getTenant: async (id: string) => {
    return fetchApi(`/superadmin/tenants/${id}`);
  },

  createTenant: async (data: any) => {
    return fetchApi('/superadmin/tenants', {
      method: 'POST',
      body: JSON.stringify(data),
    });
  },

  updateTenantStatus: async (id: string, status: string) => {
    return fetchApi(`/superadmin/tenants/${id}/status`, {
      method: 'PUT',
      body: JSON.stringify({ status })
    });
  },

  // Monitoring
  getSystemErrors: async () => {
    return fetchApi('/superadmin/monitoring/errors');
  },

  // Support
  getTickets: async () => {
    return fetchApi('/superadmin/support');
  },

  // Scheduler
  triggerAdjustments: async () => {
    return fetchApi('/superadmin/scheduler/trigger-adjustments', {
      method: 'POST',
    });
  }
};
