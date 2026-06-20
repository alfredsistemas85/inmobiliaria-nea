import { useState, useEffect, useRef } from 'react'
import { Card, CardContent } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { MessageCircle, Send, User, Check, CheckCheck, Clock } from 'lucide-react'
import { whatsappService, Conversation, Message } from '@/services/whatsapp'

export default function WhatsApp() {
  const [conversations, setConversations] = useState<Conversation[]>([])
  const [selectedConv, setSelectedConv] = useState<Conversation | null>(null)
  const [messages, setMessages] = useState<Message[]>([])
  const [messageText, setMessageText] = useState('')
  const [sending, setSending] = useState(false)
  
  const messagesEndRef = useRef<HTMLDivElement>(null)
  const isAutoScrollEnabledRef = useRef(true)
  const scrollContainerRef = useRef<HTMLDivElement>(null)

  // Filters
  const [filterTab, setFilterTab] = useState<'ALL' | 'UNREAD' | 'UNASSIGNED' | 'MINE' | 'CLOSED'>('ALL')
  const [searchQuery, setSearchQuery] = useState('')

  // Current user from localStorage
  const currentUser = (() => {
    try {
      const userStr = localStorage.getItem('user')
      return userStr ? JSON.parse(userStr) : null
    } catch {
      return null
    }
  })()

  // Polling variables
  const POLLING_INTERVAL = 5000 // 5 seconds
  const HIDDEN_POLLING_INTERVAL = 30000 // 30 seconds

  // Fetch conversations
  const loadConversations = async () => {
    try {
      const res = await whatsappService.getConversations(1, 50)
      setConversations(res.data)
    } catch (error) {
      console.error('Error loading conversations:', error)
    }
  }

  // Fetch messages for selected conversation
  const loadMessages = async (convId: string, isPolling = false) => {
    try {
      const res = await whatsappService.getMessages(convId, 1, 50)
      setMessages(res.data)
      if (!isPolling || isAutoScrollEnabledRef.current) {
        scrollToBottom()
      }
    } catch (error) {
      console.error('Error loading messages:', error)
    }
  }

  useEffect(() => {
    loadConversations()
  }, [])

  useEffect(() => {
    if (selectedConv) {
      loadMessages(selectedConv.id)
      isAutoScrollEnabledRef.current = true
    } else {
      setMessages([])
    }
  }, [selectedConv])

  // Smart Polling
  useEffect(() => {
    let timeoutId: NodeJS.Timeout

    const poll = async () => {
      // If document is hidden, wait longer, otherwise fast
      const interval = document.hidden ? HIDDEN_POLLING_INTERVAL : POLLING_INTERVAL

      try {
        if (selectedConv) {
          await loadMessages(selectedConv.id, true)
        }
        await loadConversations()
      } finally {
        timeoutId = setTimeout(poll, interval)
      }
    }

    timeoutId = setTimeout(poll, POLLING_INTERVAL)

    return () => clearTimeout(timeoutId)
  }, [selectedConv])

  // Handle scroll events to disable auto-scroll if user scrolls up
  const handleScroll = () => {
    if (!scrollContainerRef.current) return
    const { scrollTop, scrollHeight, clientHeight } = scrollContainerRef.current
    const isAtBottom = scrollHeight - scrollTop - clientHeight < 50
    isAutoScrollEnabledRef.current = isAtBottom
  }

  const scrollToBottom = () => {
    setTimeout(() => {
      messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' })
    }, 100)
  }

  const handleSend = async () => {
    if (!selectedConv || !messageText.trim()) return
    
    setSending(true)
    try {
      const newMessage = await whatsappService.sendMessage(selectedConv.id, messageText)
      setMessages(prev => [...prev, newMessage])
      setMessageText('')
      scrollToBottom()
      loadConversations() // update the left panel
    } catch (error) {
      console.error('Error sending message:', error)
      alert('Error al enviar el mensaje')
    } finally {
      setSending(false)
    }
  }

  const formatTime = (isoString?: string) => {
    if (!isoString) return ''
    const date = new Date(isoString)
    return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
  }

  // Filter conversations
  const filteredConversations = (conversations || []).filter(conv => {
    // 1. Search filter
    if (searchQuery) {
      const q = searchQuery.toLowerCase()
      const matchName = (`${conv.client_first_name || ''} ${conv.client_last_name || ''}`).toLowerCase().includes(q)
      const matchPhone = (conv.client_phone || '').toLowerCase().includes(q)
      const matchText = (conv.last_message_content || '').toLowerCase().includes(q)
      if (!matchName && !matchPhone && !matchText) return false
    }

    // 2. Tab filter
    switch (filterTab) {
      case 'UNREAD':
        return (conv.unread_count || 0) > 0
      case 'UNASSIGNED':
        return !conv.assigned_user_id
      case 'MINE':
        return conv.assigned_user_id === currentUser?.id
      case 'CLOSED':
        return conv.status === 'CLOSED'
      case 'ALL':
      default:
        // By default, maybe we want to show everything or hide closed unless specifically requested
        // Usually 'ALL' means not closed, but we'll include all to be literal
        return true
    }
  })

  return (
    <div className="flex h-[calc(100vh-8rem)] gap-4 w-full">
      {/* Left Panel: Conversations */}
      <Card className="w-1/3 flex flex-col overflow-hidden">
        <div className="p-4 border-b border-border bg-background">
          <div className="flex items-center gap-2 mb-3">
            <MessageCircle className="h-5 w-5 text-green-500" />
            <h2 className="font-semibold text-foreground">Chats</h2>
          </div>
          
          <input 
            type="text" 
            placeholder="Buscar por cliente, teléfono o mensaje..." 
            value={searchQuery}
            onChange={e => setSearchQuery(e.target.value)}
            className="w-full text-sm border border-border rounded-md px-3 py-2 mb-3 outline-none focus:ring-1 focus:ring-green-500"
          />

          <div className="flex gap-1 overflow-x-auto pb-1 hide-scrollbar">
            {['ALL', 'UNREAD', 'UNASSIGNED', 'MINE', 'CLOSED'].map((tab) => (
              <button
                key={tab}
                onClick={() => setFilterTab(tab as 'ALL' | 'UNREAD' | 'UNASSIGNED' | 'MINE' | 'CLOSED')}
                className={`text-xs px-3 py-1.5 rounded-full whitespace-nowrap transition-colors ${
                  filterTab === tab ? 'bg-slate-900 text-white' : 'bg-card text-muted-foreground hover:bg-muted border border-border'
                }`}
              >
                {tab === 'ALL' ? 'Todas' : tab === 'UNREAD' ? 'No Leídas' : tab === 'UNASSIGNED' ? 'Sin Asignar' : tab === 'MINE' ? 'Mías' : 'Cerradas'}
              </button>
            ))}
          </div>
        </div>
        <div className="flex-1 overflow-y-auto">
          {filteredConversations.length === 0 ? (
            <div className="p-4 text-center text-muted-foreground text-sm">No hay conversaciones</div>
          ) : (
            filteredConversations.map(conv => (
              <div 
                key={conv.id}
                onClick={() => setSelectedConv(conv)}
                className={`p-4 border-b border-border cursor-pointer hover:bg-background transition-colors ${selectedConv?.id === conv.id ? 'bg-green-50/50 hover:bg-green-50/50' : ''}`}
              >
                <div className="flex justify-between items-start mb-1">
                  <h3 className="font-medium text-foreground truncate pr-2">
                    {conv.client_first_name || 'Cliente'} {conv.client_last_name || ''}
                  </h3>
                  <span className="text-xs text-muted-foreground whitespace-nowrap">
                    {formatTime(conv.last_message_at)}
                  </span>
                </div>
                <div className="flex items-center gap-1 mb-1">
                  {conv.assigned_user_id ? (
                    <span className="text-[10px] bg-blue-50 text-blue-600 px-1.5 py-0.5 rounded border border-blue-100 flex items-center gap-1">
                      <User className="h-3 w-3" /> Asignado
                    </span>
                  ) : (
                    <span className="text-[10px] bg-amber-50 text-amber-600 px-1.5 py-0.5 rounded border border-amber-100">
                      Sin asignar
                    </span>
                  )}
                  {conv.status === 'CLOSED' && (
                    <span className="text-[10px] bg-muted text-muted-foreground px-1.5 py-0.5 rounded border border-border">
                      Cerrado
                    </span>
                  )}
                </div>
                <div className="flex justify-between items-center">
                  <p className="text-sm text-muted-foreground truncate flex-1 pr-2">
                    {conv.last_message_direction === 'outbound' && <span className="text-muted-foreground mr-1">Tú:</span>}
                    {conv.last_message_content || 'Inicia la conversación'}
                  </p>
                  {(conv.unread_count ?? 0) > 0 && (
                    <span className="bg-green-500 text-white text-xs font-bold px-2 py-0.5 rounded-full">
                      {conv.unread_count}
                    </span>
                  )}
                </div>
              </div>
            ))
          )}
        </div>
      </Card>

      {/* Right Panel: Messages */}
      <Card className="flex-1 flex flex-col overflow-hidden bg-background/50">
        {selectedConv ? (
          <>
            {/* Chat Header */}
            <div className="p-4 border-b border-border bg-card flex justify-between items-center shrink-0 shadow-sm z-10">
              <div className="flex items-center gap-3">
                <div className="h-10 w-10 bg-muted rounded-full flex items-center justify-center text-muted-foreground">
                  <User className="h-5 w-5" />
                </div>
                <div>
                  <h2 className="font-semibold text-foreground">
                    {selectedConv.client_first_name || 'Cliente'} {selectedConv.client_last_name || ''}
                  </h2>
                  <p className="text-xs text-muted-foreground">{selectedConv.client_phone}</p>
                </div>
              </div>
            </div>

            {/* Chat Messages */}
            <div 
              className="flex-1 overflow-y-auto p-4 space-y-4"
              ref={scrollContainerRef}
              onScroll={handleScroll}
            >
              {messages.length === 0 ? (
                <div className="h-full flex items-center justify-center text-muted-foreground text-sm">
                  Sin mensajes. Envía un mensaje para comenzar.
                </div>
              ) : (
                messages.map(msg => {
                  const isOutbound = msg.direction === 'outbound' || msg.sender_type === 'agent' || msg.sender_type === 'bot'
                  
                  return (
                    <div key={msg.id} className={`flex ${isOutbound ? 'justify-end' : 'justify-start'}`}>
                      <div 
                        className={`max-w-[75%] rounded-2xl px-4 py-2 relative shadow-sm ${
                          isOutbound 
                            ? 'bg-green-500 text-white rounded-tr-sm' 
                            : 'bg-card text-foreground rounded-tl-sm border border-border'
                        }`}
                      >
                        <p className="text-sm whitespace-pre-wrap break-words">{msg.content}</p>
                        <div className={`flex items-center justify-end gap-1 mt-1 text-[10px] ${isOutbound ? 'text-green-100' : 'text-muted-foreground'}`}>
                          <span>{formatTime(msg.created_at)}</span>
                          {isOutbound && (
                            msg.status === 'read' ? <CheckCheck className="h-3 w-3 text-blue-300" /> :
                            msg.status === 'delivered' ? <CheckCheck className="h-3 w-3" /> :
                            msg.status === 'sent' ? <Check className="h-3 w-3" /> :
                            <Clock className="h-3 w-3" />
                          )}
                        </div>
                      </div>
                    </div>
                  )
                })
              )}
              <div ref={messagesEndRef} />
            </div>

            {/* Chat Input */}
            <div className="p-4 bg-card border-t border-border shrink-0">
              <div className="flex gap-2">
                <input
                  type="text"
                  value={messageText}
                  onChange={(e) => setMessageText(e.target.value)}
                  onKeyDown={(e) => e.key === 'Enter' && !e.shiftKey && handleSend()}
                  placeholder="Escribe un mensaje..."
                  className="flex-1 h-12 rounded-full border border-border bg-background px-4 text-sm focus:outline-none focus:ring-2 focus:ring-green-500 focus:bg-card transition-colors"
                />
                <Button 
                  onClick={handleSend} 
                  disabled={!messageText.trim() || sending}
                  className="h-12 w-12 rounded-full bg-green-500 hover:bg-green-600 p-0 flex items-center justify-center shadow-sm"
                >
                  <Send className="h-5 w-5 ml-1" />
                </Button>
              </div>
            </div>
          </>
        ) : (
          <div className="h-full flex flex-col items-center justify-center text-muted-foreground p-8 text-center">
            <div className="h-20 w-20 bg-muted rounded-full flex items-center justify-center mb-4">
              <MessageCircle className="h-10 w-10 text-muted-foreground" />
            </div>
            <h3 className="text-lg font-medium text-foreground mb-2">WhatsApp Integrado</h3>
            <p className="max-w-xs text-sm">Selecciona una conversación del panel izquierdo para comenzar a chatear con tus clientes.</p>
          </div>
        )}
      </Card>
    </div>
  )
}
