import { LogEvent, LoggerTransport } from '../types';

export class HttpTransport implements LoggerTransport {
  log(event: LogEvent): void {
    // Implementación futura: enviar a un endpoint de logs
    // fetch('/api/logs', { method: 'POST', body: JSON.stringify(event) });
  }
}
