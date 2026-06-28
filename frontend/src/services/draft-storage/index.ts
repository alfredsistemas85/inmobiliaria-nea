export interface DraftData {
  version: string;
  timestamp: string;
  tenant: string;
  usuario: string;
  property_id: string | null;
  payload: any;
  correlation_id: string;
}

export class DraftStorage {
  private static DRAFT_PREFIX = 'contract_wizard_draft_';

  private static getKey(tenant: string, property: string | null, user: string): string {
    const propStr = property || 'manual';
    return `${this.DRAFT_PREFIX}${tenant}_${propStr}_${user}`;
  }

  static save(tenant: string, property: string | null, user: string, payload: any, correlation_id: string): void {
    const key = this.getKey(tenant, property, user);
    const draft: DraftData = {
      version: '1.0',
      timestamp: new Date().toISOString(),
      tenant,
      usuario: user,
      property_id: property,
      payload,
      correlation_id
    };
    try {
      localStorage.setItem(key, JSON.stringify(draft));
    } catch (e) {
      console.error('Failed to save draft', e);
    }
  }

  static load(tenant: string, property: string | null, user: string): DraftData | null {
    const key = this.getKey(tenant, property, user);
    try {
      const data = localStorage.getItem(key);
      if (data) {
        return JSON.parse(data) as DraftData;
      }
    } catch (e) {
      console.error('Failed to parse draft', e);
    }
    return null;
  }

  static remove(tenant: string, property: string | null, user: string): void {
    const key = this.getKey(tenant, property, user);
    localStorage.removeItem(key);
  }

  static exists(tenant: string, property: string | null, user: string): boolean {
    const key = this.getKey(tenant, property, user);
    return localStorage.getItem(key) !== null;
  }

  static clearExpired(maxAgeMs = 24 * 60 * 60 * 1000): void {
    const now = Date.now();
    for (let i = 0; i < localStorage.length; i++) {
      const key = localStorage.key(i);
      if (key && key.startsWith(this.DRAFT_PREFIX)) {
        try {
          const item = JSON.parse(localStorage.getItem(key)!);
          if (item && item.timestamp) {
            const age = now - new Date(item.timestamp).getTime();
            if (age > maxAgeMs) {
              localStorage.removeItem(key);
            }
          }
        } catch {
          localStorage.removeItem(key); // clear corrupted
        }
      }
    }
  }
}
