import { useState } from 'react'
import { Landmark, CreditCard, BookOpen, Users, Settings } from 'lucide-react'

// Tabs Imports
import { BillingTab } from './financial/BillingTab'
import { TreasuryTab } from './financial/TreasuryTab'
import { AccountingTab } from './financial/AccountingTab'
import { OwnerSettlementsTab } from './financial/OwnerSettlementsTab'
import { IntegrationsTab } from './financial/IntegrationsTab'

type TabKey = 'treasury' | 'billing' | 'accounting' | 'owners' | 'integrations'

export default function Financials() {
  const [activeTab, setActiveTab] = useState<TabKey>('treasury')

  const TABS = [
    { key: 'treasury', label: 'Caja y Tesorería', icon: Landmark },
    { key: 'billing', label: 'Cuentas Corrientes', icon: CreditCard },
    { key: 'accounting', label: 'Contabilidad', icon: BookOpen },
    { key: 'owners', label: 'Liquidaciones', icon: Users },
    { key: 'integrations', label: 'Configuración', icon: Settings },
  ] as const

  return (
    <div className="space-y-6 max-w-[1400px] mx-auto w-full">
      {/* Encabezado */}
      <div>
        <h1 className="text-2xl font-bold tracking-tight text-foreground">ERP Financiero</h1>
        <p className="text-muted-foreground mt-1">
          Gestión centralizada de caja, contabilidad por partida doble, facturación y cobros.
        </p>
      </div>

      {/* Navegación por pestañas */}
      <div className="border-b border-border overflow-x-auto">
        <nav className="flex gap-6 min-w-max" aria-label="Tabs">
          {TABS.map(({ key, label, icon: Icon }) => (
            <button
              key={key}
              onClick={() => setActiveTab(key)}
              className={`
                flex items-center gap-2 py-3 px-1 border-b-2 text-sm font-medium transition-colors
                ${activeTab === key
                  ? 'border-primary text-primary'
                  : 'border-transparent text-muted-foreground hover:text-foreground hover:border-muted'
                }
              `}
            >
              <Icon className="h-4 w-4" />
              {label}
            </button>
          ))}
        </nav>
      </div>

      {/* Contenido de la pestaña */}
      <div className="mt-4">
        {activeTab === 'treasury' && <TreasuryTab />}
        {activeTab === 'billing' && <BillingTab />}
        {activeTab === 'accounting' && <AccountingTab />}
        {activeTab === 'owners' && <OwnerSettlementsTab />}
        {activeTab === 'integrations' && <IntegrationsTab />}
      </div>
    </div>
  )
}
