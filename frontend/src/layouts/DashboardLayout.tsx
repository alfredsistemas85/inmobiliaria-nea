import { useState, useEffect, useRef } from 'react'
import { Outlet, Link, useLocation } from 'react-router-dom'
import {
  LayoutDashboard, Home, Users, UserPlus, CalendarDays,
  MessageCircle, Settings, Bell, LogOut, Search, BarChart3, Check, Moon, Sun, UserCog,
  FileText, DollarSign
} from 'lucide-react'
import { cn } from '@/lib/utils'
import { useTheme } from '@/context/ThemeContext'
import { notificationsService, Notification } from '@/services/notifications'

const NAV_ITEMS = [
  { name: 'Dashboard',    path: '/dashboard',    icon: LayoutDashboard },
  { name: 'Propiedades',  path: '/properties',   icon: Home },
  { name: 'Clientes',     path: '/clients',      icon: Users },
  { name: 'Leads',        path: '/leads',        icon: UserPlus },
  { name: 'Citas',        path: '/appointments', icon: CalendarDays },
  { name: 'Contratos',    path: '/contracts',    icon: FileText },
  { name: 'Finanzas',     path: '/financials',   icon: DollarSign },
  { name: 'WhatsApp',     path: '/whatsapp',     icon: MessageCircle },
  { name: 'Reportes',     path: '/reports',      icon: BarChart3 },
  { name: 'Usuarios',     path: '/users',        icon: UserCog, adminOnly: true },
  { name: 'Ajustes',      path: '/settings',     icon: Settings },
]

export function DashboardLayout() {
  const [sidebarOpen, setSidebarOpen] = useState(false)
  const location = useLocation()

  // ── Usuario actual ─────────────────────────────────────────────────────────
  const userStr = localStorage.getItem('user')
  const user = userStr ? JSON.parse(userStr) : null
  const isTenantAdmin = user?.role === 'tenant_admin'

  // ── Notificaciones ─────────────────────────────────────────────────────────
  const [notifications, setNotifications]   = useState<Notification[]>([])
  const [unreadCount, setUnreadCount]       = useState(0)
  const [showNotifications, setShowNotifications] = useState(false)

  // Ref para evitar que se monte más de un interval (React StrictMode monta/desmonta)
  const intervalRef = useRef<ReturnType<typeof setInterval> | null>(null)

  useEffect(() => {
    const fetchNotifications = async () => {
      try {
        const res = await notificationsService.getNotifications()
        // fetchApi devuelve null cuando hay redirect por 401 → ignorar silenciosamente
        if (!res) return
        setNotifications(res.notifications)
        setUnreadCount(res.unread_count)
      } catch {
        // Error de red u otro: no propagar, el redirect lo maneja api.ts
      }
    }

    // Carga inicial
    fetchNotifications()

    // Polling cada 30s — un único interval garantizado por el ref
    if (intervalRef.current) clearInterval(intervalRef.current)
    intervalRef.current = setInterval(fetchNotifications, 30_000)

    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current)
        intervalRef.current = null
      }
    }
  }, [])

  const handleMarkAsRead = async (id: string) => {
    try {
      await notificationsService.markAsRead(id)
      // Actualizar estado local optimistamente: setear read_at a la hora actual
      setNotifications((prev) =>
        prev.map((n) => (n.id === id ? { ...n, read_at: new Date().toISOString() } : n))
      )
      setUnreadCount((prev) => Math.max(0, prev - 1))
    } catch {
      // Silenciar; el backend es la fuente de verdad, el siguiente poll sincronizará
    }
  }

  // ── Tema ───────────────────────────────────────────────────────────────────
  const { theme, setTheme } = useTheme()
  const isDark = theme === 'dark'

  const toggleTheme = () => {
    setTheme(isDark ? 'light' : 'dark')
  }

  return (
    <div className="flex h-screen bg-background text-foreground transition-colors duration-300">
      {/* ── Desktop Sidebar ─────────────────────────────────────────────────── */}
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
              if (item.adminOnly && !isTenantAdmin) return null;
              const isActive = location.pathname.startsWith(item.path)
              return (
                <Link
                  key={item.name}
                  to={item.path}
                  className={cn(
                    'flex items-center gap-3 rounded-md px-3 py-2 text-sm font-medium transition-colors',
                    isActive
                      ? 'bg-primary/10 text-primary'
                      : 'text-muted-foreground hover:bg-muted hover:text-foreground'
                  )}
                >
                  <item.icon
                    className={cn('h-5 w-5', isActive ? 'text-primary' : 'text-muted-foreground')}
                  />
                  {item.name}
                </Link>
              )
            })}
          </nav>
        </div>

        <div className="p-4 border-t border-border">
          <div className="flex items-center gap-3">
            <div className="h-9 w-9 rounded-full bg-primary/10 flex items-center justify-center text-primary font-bold">
              {user?.first_name?.[0]}{user?.last_name?.[0]}
            </div>
            <div className="flex-1 overflow-hidden">
              <p className="truncate text-sm font-medium text-foreground">{user?.first_name} {user?.last_name}</p>
              <p className="truncate text-xs text-muted-foreground">{user?.role === 'tenant_admin' ? 'Administrador' : 'Agente'}</p>
            </div>
            <button
              className="text-muted-foreground hover:text-foreground transition-colors"
              title="Cerrar sesión"
              onClick={() => {
                localStorage.clear()
                window.location.replace('/login')
              }}
            >
              <LogOut className="h-5 w-5" />
            </button>
          </div>
        </div>
      </aside>

      {/* ── Main Content Area ──────────────────────────────────────────────── */}
      <div className="flex flex-1 flex-col overflow-hidden">
        {/* Top Header */}
        <header className="flex h-16 items-center justify-between border-b border-border bg-card px-4 md:px-6">
          {/* Brand (mobile) */}
          <div className="flex items-center gap-4 md:hidden">
            <div className="flex items-center gap-2 font-bold text-lg text-primary">
              <Home className="h-5 w-5" />
              <span>InmobiCRM</span>
            </div>
          </div>

          {/* Search (desktop) */}
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

          {/* Actions */}
          <div className="flex items-center gap-3 ml-auto">
            {/* ── Theme Toggle: Luna / Sol ─────────────────────────────────── */}
            <button
              onClick={toggleTheme}
              aria-label={isDark ? 'Cambiar a modo claro' : 'Cambiar a modo oscuro'}
              title={isDark ? 'Modo claro' : 'Modo oscuro'}
              className="relative flex h-9 w-9 items-center justify-center rounded-md border border-border bg-background text-muted-foreground hover:bg-muted hover:text-foreground transition-colors duration-200"
            >
              {/* Sol e icono de luna con animación de fade */}
              <Sun
                className={cn(
                  'absolute h-4 w-4 transition-all duration-300',
                  isDark ? 'opacity-100 rotate-0 scale-100' : 'opacity-0 rotate-90 scale-75'
                )}
              />
              <Moon
                className={cn(
                  'absolute h-4 w-4 transition-all duration-300',
                  isDark ? 'opacity-0 -rotate-90 scale-75' : 'opacity-100 rotate-0 scale-100'
                )}
              />
            </button>

            {/* ── Notificaciones ───────────────────────────────────────────── */}
            <div className="relative">
              <button
                id="notifications-bell"
                className="relative text-muted-foreground hover:text-foreground transition-colors"
                onClick={() => setShowNotifications((v) => !v)}
                aria-label="Ver notificaciones"
              >
                <Bell className="h-6 w-6" />
                {unreadCount > 0 && (
                  <span className="absolute -top-1 -right-1 flex h-4 w-4 items-center justify-center rounded-full bg-destructive text-[10px] font-bold text-destructive-foreground ring-2 ring-background">
                    {unreadCount > 9 ? '9+' : unreadCount}
                  </span>
                )}
              </button>

              {showNotifications && (
                <div className="absolute right-0 mt-2 w-80 rounded-md bg-card border border-border shadow-lg ring-1 ring-black/5 z-50 animate-in fade-in slide-in-from-top-2 duration-150">
                  <div className="p-3 border-b border-border flex justify-between items-center bg-muted/50 rounded-t-md">
                    <h3 className="font-semibold text-sm text-foreground">Notificaciones</h3>
                    {unreadCount > 0 && (
                      <span className="text-xs text-muted-foreground">{unreadCount} sin leer</span>
                    )}
                  </div>
                  <div className="max-h-[300px] overflow-y-auto">
                    {notifications.length === 0 ? (
                      <div className="p-4 text-center text-sm text-muted-foreground">
                        No hay notificaciones
                      </div>
                    ) : (
                      notifications.map((n) => (
                        <div
                          key={n.id}
                          className={cn(
                            'p-3 border-b border-border hover:bg-muted/50 transition-colors flex gap-3 items-start',
                            !n.read_at && 'bg-primary/5'
                          )}
                        >
                          {/* Indicador de no leído */}
                          <div
                            className={cn(
                              'mt-1 h-2 w-2 rounded-full shrink-0',
                              !n.read_at ? 'bg-primary' : 'bg-transparent'
                            )}
                          />
                          <div className="flex-1 space-y-1 min-w-0">
                            <p className="text-sm font-medium text-foreground leading-tight">
                              {n.title}
                            </p>
                            {/* 'content' es el campo real del backend */}
                            <p className="text-xs text-muted-foreground leading-snug">
                              {n.content}
                            </p>
                            <p className="text-[10px] text-muted-foreground/70">
                              {n.created_at
                                ? new Date(n.created_at).toLocaleString('es-AR')
                                : ''}
                            </p>
                          </div>
                          {!n.read_at && (
                            <button
                              onClick={() => handleMarkAsRead(n.id)}
                              className="text-muted-foreground hover:text-primary p-1 rounded hover:bg-primary/10 transition-colors"
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

            {/* Avatar (mobile) */}
            <div className="md:hidden h-8 w-8 rounded-full bg-primary/10 flex items-center justify-center text-primary font-bold text-sm">
              {user?.first_name?.[0] || 'U'}{user?.last_name?.[0] || ''}
            </div>
          </div>
        </header>

        {/* Main Scrollable Content */}
        <main className="flex-1 overflow-y-auto bg-background p-4 md:p-6 pb-20 md:pb-6">
          <Outlet />
        </main>
      </div>

      {/* ── Mobile Bottom Navigation ───────────────────────────────────────── */}
      <div className="md:hidden fixed bottom-0 left-0 right-0 border-t border-border bg-card px-2 py-2 pb-[max(0.5rem,env(safe-area-inset-bottom))] z-50">
        <nav className="flex items-center justify-between">
          {[NAV_ITEMS[0], NAV_ITEMS[1], NAV_ITEMS[2], NAV_ITEMS[4], NAV_ITEMS[5]].map((item) => {
            const isActive = location.pathname.startsWith(item.path)
            return (
              <Link
                key={item.name}
                to={item.path}
                className={cn(
                  'flex flex-col items-center gap-1 p-2 rounded-md min-w-[64px]',
                  isActive ? 'text-primary' : 'text-muted-foreground hover:text-foreground'
                )}
              >
                <item.icon
                  className={cn('h-5 w-5', isActive ? 'text-primary' : 'text-muted-foreground')}
                />
                <span className="text-[10px] font-medium">{item.name}</span>
              </Link>
            )
          })}
        </nav>
      </div>
    </div>
  )
}
