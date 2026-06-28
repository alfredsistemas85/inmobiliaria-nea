import { LogEvent, LoggerTransport, LogLevel } from './types';
import { ConsoleTransport } from './transports/console.transport';

class Logger {
  private transports: LoggerTransport[] = [];
  private static instance: Logger;

  private constructor() {
    this.addTransport(new ConsoleTransport());
    // Se puede habilitar en el futuro: this.addTransport(new HttpTransport());
  }

  public static getInstance(): Logger {
    if (!Logger.instance) {
      Logger.instance = new Logger();
    }
    return Logger.instance;
  }

  public addTransport(transport: LoggerTransport) {
    this.transports.push(transport);
  }

  private dispatch(
    level: LogLevel,
    module: string,
    action: string,
    metadata?: Record<string, any>
  ) {
    // Obtener correlation_id desde algun lugar global si aplica, o pasarlo en metadata
    // Aquí usamos window.correlation_id u otra forma segura.
    // Para simplificar, la inyección del correlation id se asume gestionada desde la vista, o un global.
    const correlation_id = (window as any).__CORRELATION_ID__ || metadata?.correlation_id;

    const event: LogEvent = {
      level,
      timestamp: new Date().toISOString(),
      module,
      action,
      correlation_id,
      metadata
    };

    this.transports.forEach(t => t.log(event));
  }

  info(module: string, action: string, metadata?: Record<string, any>) {
    this.dispatch('info', module, action, metadata);
  }

  warn(module: string, action: string, metadata?: Record<string, any>) {
    this.dispatch('warn', module, action, metadata);
  }

  error(module: string, action: string, metadata?: Record<string, any>) {
    this.dispatch('error', module, action, metadata);
  }

  debug(module: string, action: string, metadata?: Record<string, any>) {
    this.dispatch('debug', module, action, metadata);
  }
}

export const logger = Logger.getInstance();
