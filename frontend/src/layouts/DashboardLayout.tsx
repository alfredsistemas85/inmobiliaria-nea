import { useState } from 'react'
import { Outlet, Link, useLocation } from 'react-router-dom'
import { LayoutDashboard, Home, Users, UserPlus, CalendarDays, MessageCircle, Settings, Menu, Bell, LogOut, Search } from 'lucide-react'
import { cn } from '@/lib/utils'

const NAV_ITEMS = [
  { name: 'Dashboard', path: '/dashboard', icon: LayoutDashboard },
  { name: 'Propiedades', path: '/properties', icon: Home },
  { name: 'Clientes', path: '/clients', icon: Users },
  { name: 'Leads', path: '/leads', icon: UserPlus },
  { name: 'Citas', path: '/appointments', icon: CalendarDays },
  { name: 'WhatsApp', path: '/whatsapp', icon: MessageCircle },
  { name: 'Ajustes', path: '/settings', icon: Settings },
]

export function DashboardLayout() {
  const [sidebarOpen, setSidebarOpen] = useState(false)
  const location = useLocation()

  return (
    <div className="flex h-screen bg-slate-50 text-slate-900">
      {/* Desktop Sidebar */}
      <aside className="hidden w-64 flex-col border-r border-slate-200 bg-white md:flex">
        <div className="flex h-16 items-center px-6 border-b border-slate-200">
          <div className="flex items-center gap-2 font-bold text-xl text-blue-600">
            <Home className="h-6 w-6" />
            <span>InmobiCRM</span>
          </div>
        </div>
        
        <div className="flex-1 overflow-y-auto py-4">
          <nav className="space-y-1 px-3">
            {NAV_ITEMS.map((item) => {
              const isActive = location.pathname.startsWith(item.path)
              return (
                <Link
                  key={item.name}
                  to={item.path}
                  className={cn(
                    "flex items-center gap-3 rounded-md px-3 py-2 text-sm font-medium transition-colors",
                    isActive 
                      ? "bg-blue-50 text-blue-600" 
                      : "text-slate-600 hover:bg-slate-100 hover:text-slate-900"
                  )}
                >
                  <item.icon className={cn("h-5 w-5", isActive ? "text-blue-600" : "text-slate-400")} />
                  {item.name}
                </Link>
              )
            })}
          </nav>
        </div>

        <div className="p-4 border-t border-slate-200">
          <div className="flex items-center gap-3">
            <div className="h-9 w-9 rounded-full bg-blue-100 flex items-center justify-center text-blue-700 font-bold">
              JD
            </div>
            <div className="flex-1 overflow-hidden">
              <p className="truncate text-sm font-medium">John Doe</p>
              <p className="truncate text-xs text-slate-500">Agente Inmobiliario</p>
            </div>
            <button className="text-slate-400 hover:text-slate-600">
              <LogOut className="h-5 w-5" />
            </button>
          </div>
        </div>
      </aside>

      {/* Main Content Area */}
      <div className="flex flex-1 flex-col overflow-hidden">
        {/* Top Header */}
        <header className="flex h-16 items-center justify-between border-b border-slate-200 bg-white px-4 md:px-6">
          <div className="flex items-center gap-4 md:hidden">
            <div className="flex items-center gap-2 font-bold text-lg text-blue-600">
              <Home className="h-5 w-5" />
              <span>InmobiCRM</span>
            </div>
          </div>
          
          <div className="hidden md:flex flex-1 items-center max-w-md">
            <div className="relative w-full">
              <Search className="absolute left-2.5 top-2.5 h-4 w-4 text-slate-400" />
              <input
                type="text"
                placeholder="Buscar propiedades, clientes, leads..."
                className="w-full rounded-md border border-slate-200 bg-slate-50 pl-9 pr-4 py-2 text-sm outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
              />
            </div>
          </div>

          <div className="flex items-center gap-4 ml-auto">
            <button className="relative text-slate-400 hover:text-slate-600">
              <Bell className="h-6 w-6" />
              <span className="absolute top-0 right-0 h-2 w-2 rounded-full bg-red-500 ring-2 ring-white"></span>
            </button>
            <div className="md:hidden h-8 w-8 rounded-full bg-blue-100 flex items-center justify-center text-blue-700 font-bold text-sm">
              JD
            </div>
          </div>
        </header>

        {/* Main Scrollable Content */}
        <main className="flex-1 overflow-y-auto bg-slate-50/50 p-4 md:p-6 pb-20 md:pb-6">
          <Outlet />
        </main>
      </div>

      {/* Mobile Bottom Navigation */}
      <div className="md:hidden fixed bottom-0 left-0 right-0 border-t border-slate-200 bg-white px-2 py-2 pb-[max(0.5rem,env(safe-area-inset-bottom))] z-50">
        <nav className="flex items-center justify-between">
          {[NAV_ITEMS[0], NAV_ITEMS[1], NAV_ITEMS[2], NAV_ITEMS[4], NAV_ITEMS[5]].map((item) => {
            const isActive = location.pathname.startsWith(item.path)
            return (
              <Link
                key={item.name}
                to={item.path}
                className={cn(
                  "flex flex-col items-center gap-1 p-2 rounded-md min-w-[64px]",
                  isActive ? "text-blue-600" : "text-slate-500 hover:text-slate-900"
                )}
              >
                <item.icon className={cn("h-5 w-5", isActive ? "text-blue-600" : "text-slate-400")} />
                <span className="text-[10px] font-medium">{item.name}</span>
              </Link>
            )
          })}
        </nav>
      </div>
    </div>
  )
}
