export type LogLevel = 'info' | 'warn' | 'error' | 'debug';

export interface LogEvent {
  level: LogLevel;
  timestamp: string;
  tenant_id?: string;
  user_id?: string;
  correlation_id?: string;
  module: string;
  action: string;
  metadata?: Record<string, any>;
}

export interface LoggerTransport {
  log(event: LogEvent): void;
}
