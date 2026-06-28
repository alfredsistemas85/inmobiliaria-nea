import { LogEvent, LoggerTransport } from '../types';

export class ConsoleTransport implements LoggerTransport {
  log(event: LogEvent): void {
    const { level, timestamp, correlation_id, module, action, metadata } = event;
    const prefix = `[${timestamp}] [${level.toUpperCase()}] [${module}::${action}] [corr: ${correlation_id || 'none'}]`;
    
    switch (level) {
      case 'info':
        console.info(prefix, metadata || '');
        break;
      case 'warn':
        console.warn(prefix, metadata || '');
        break;
      case 'error':
        console.error(prefix, metadata || '');
        break;
      case 'debug':
        console.debug(prefix, metadata || '');
        break;
    }
  }
}
