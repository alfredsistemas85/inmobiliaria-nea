import { useState, useEffect } from 'react'
import { Outlet, Link, useLocation } from 'react-router-dom'
import { LayoutDashboard, Home, Users, UserPlus, CalendarDays, MessageCircle, Settings, Menu, Bell, LogOut, Search, BarChart3, Check } from 'lucide-react'
import { cn } from '@/lib/utils'
import { ThemeToggle } from '@/components/ThemeToggle'
import { notificationsService, Notification } from '@/services/notifications'

const NAV_ITEMS = [
  { name: 'Dashboard', path: '/dashboard', icon: LayoutDashboard },
  { name: 'Propiedades', path: '/properties', icon: Home },
  { name: 'Clientes', path: '/clients', icon: Users },
  { name: 'Leads', path: '/leads', icon: UserPlus },
  { name: 'Citas', path: '/appointments', icon: CalendarDays },
  { name: 'WhatsApp', path: '/whatsapp', icon: MessageCircle },
  { name: 'Reportes', path: '/reports', icon: BarChart3 },
  { name: 'Ajustes', path: '/settings', icon: Settings },
]

export function DashboardLayout() {
  const [sidebarOpen, setSidebarOpen] = useState(false)
  const location = useLocation()
  
  const [notifications, setNotifications] = useState<Notification[]>([])
  const [showNotifications, setShowNotifications] = useState(false)

  const unreadCount = notifications.filter(n => !n.is_read).length

  useEffect(() => {
    const fetchNotifications = async () => {
      try {
        const res = await notificationsService.getNotifications()
        setNotifications(res.data)
      } catch (error) {
        console.error('Error fetching notifications:', error)
      }
    }

    fetchNotifications()
    const interval = setInterval(fetchNotifications, 30000) // Poll every 30s
    return () => clearInterval(interval)
  }, [])

  const handleMarkAsRead = async (id: string) => {
    try {
      await notificationsService.markAsRead(id)
      setNotifications(prev => prev.map(n => n.id === id ? { ...n, is_read: true } : n))
    } catch (error) {
      console.error('Error marking as read:', error)
    }
  }

  return (
    <div className="flex h-screen bg-background text-foreground transition-colors">
      {/* Desktop Sidebar */}
      <aside className="hidden w-64 flex-col border-r border-border bg-card md:flex">
        <div className="flex h-16 items-center px-6 border-b border-border">
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
                      ? "bg-primary/10 text-primary" 
                      : "text-muted-foreground hover:bg-muted hover:text-foreground"
                  )}
                >
                  <item.icon className={cn("h-5 w-5", isActive ? "text-primary" : "text-muted-foreground")} />
                  {item.name}
                </Link>
              )
            })}
          </nav>
        </div>

        <div className="p-4 border-t border-border">
          <div className="flex items-center gap-3">
            <div className="h-9 w-9 rounded-full bg-primary/10 flex items-center justify-center text-primary font-bold">
              JD
            </div>
            <div className="flex-1 overflow-hidden">
              <p className="truncate text-sm font-medium text-foreground">John Doe</p>
              <p className="truncate text-xs text-muted-foreground">Agente Inmobiliario</p>
            </div>
            <button className="text-muted-foreground hover:text-foreground">
              <LogOut className="h-5 w-5" />
            </button>
          </div>
        </div>
      </aside>

      {/* Main Content Area */}
      <div className="flex flex-1 flex-col overflow-hidden">
        {/* Top Header */}
        <header className="flex h-16 items-center justify-between border-b border-border bg-card px-4 md:px-6">
          <div className="flex items-center gap-4 md:hidden">
            <div className="flex items-center gap-2 font-bold text-lg text-primary">
              <Home className="h-5 w-5" />
              <span>InmobiCRM</span>
            </div>
          </div>
          
          <div className="hidden md:flex flex-1 items-center max-w-md">
            <div className="relative w-full">
              <Search className="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
              <input
                type="text"
                placeholder="Buscar propiedades, clientes, leads..."
                className="w-full rounded-md border border-input bg-background pl-9 pr-4 py-2 text-sm outline-none focus:border-primary focus:ring-1 focus:ring-primary text-foreground"
              />
            </div>
          </div>

          <div className="flex items-center gap-4 ml-auto">
            <ThemeToggle />
            
            <div className="relative">
              <button 
                className="relative text-muted-foreground hover:text-foreground"
                onClick={() => setShowNotifications(!showNotifications)}
              >
                <Bell className="h-6 w-6" />
                {unreadCount > 0 && (
                  <span className="absolute -top-1 -right-1 flex h-4 w-4 items-center justify-center rounded-full bg-destructive text-[10px] font-bold text-destructive-foreground ring-2 ring-background">
                    {unreadCount > 9 ? '9+' : unreadCount}
                  </span>
                )}
              </button>
              
              {showNotifications && (
                <div className="absolute right-0 mt-2 w-80 rounded-md bg-card border border-border shadow-lg ring-1 ring-black ring-opacity-5 z-50">
                  <div className="p-3 border-b border-border flex justify-between items-center bg-muted/50 rounded-t-md">
                    <h3 className="font-semibold text-sm text-foreground">Notificaciones</h3>
                  </div>
                  <div className="max-h-[300px] overflow-y-auto">
                    {notifications.length === 0 ? (
                      <div className="p-4 text-center text-sm text-muted-foreground">No hay notificaciones</div>
                    ) : (
                      notifications.map(n => (
                        <div 
                          key={n.id} 
                          className={`p-3 border-b border-border hover:bg-muted/50 transition-colors flex gap-3 items-start ${!n.is_read ? 'bg-primary/5' : ''}`}
                        >
                          <div className={`mt-1 h-2 w-2 rounded-full shrink-0 ${!n.is_read ? 'bg-primary' : 'bg-transparent'}`} />
                          <div className="flex-1 space-y-1">
                            <p className="text-sm font-medium text-foreground leading-tight">{n.title}</p>
                            <p className="text-xs text-muted-foreground leading-snug">{n.message}</p>
                            <p className="text-[10px] text-muted-foreground/70">{new Date(n.created_at).toLocaleString()}</p>
                          </div>
                          {!n.is_read && (
                            <button 
                              onClick={() => handleMarkAsRead(n.id)}
                              className="text-muted-foreground hover:text-primary p-1 rounded hover:bg-primary/10"
                              title="Marcar como leída"
                            >
                              <Check className="h-4 w-4" />
                            </button>
                          )}
                        </div>
                      ))
                    )}
                  </div>
                </div>
              )}
            </div>
            <div className="md:hidden h-8 w-8 rounded-full bg-primary/10 flex items-center justify-center text-primary font-bold text-sm">
              JD
            </div>
          </div>
        </header>

        {/* Main Scrollable Content */}
        <main className="flex-1 overflow-y-auto bg-background p-4 md:p-6 pb-20 md:pb-6">
          <Outlet />
        </main>
      </div>

      {/* Mobile Bottom Navigation */}
      <div className="md:hidden fixed bottom-0 left-0 right-0 border-t border-border bg-card px-2 py-2 pb-[max(0.5rem,env(safe-area-inset-bottom))] z-50">
        <nav className="flex items-center justify-between">
          {[NAV_ITEMS[0], NAV_ITEMS[1], NAV_ITEMS[2], NAV_ITEMS[4], NAV_ITEMS[5]].map((item) => {
            const isActive = location.pathname.startsWith(item.path)
            return (
              <Link
                key={item.name}
                to={item.path}
                className={cn(
                  "flex flex-col items-center gap-1 p-2 rounded-md min-w-[64px]",
                  isActive ? "text-primary" : "text-muted-foreground hover:text-foreground"
                )}
              >
                <item.icon className={cn("h-5 w-5", isActive ? "text-primary" : "text-muted-foreground")} />
                <span className="text-[10px] font-medium">{item.name}</span>
              </Link>
            )
          })}
        </nav>
      </div>
    </div>
  )
}
