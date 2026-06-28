import React, { Component, ReactNode } from 'react';
import { logger } from '@/lib/logger';
import { Button } from '@/components/ui/button';

interface Props {
  children: ReactNode;
  onReset?: () => void;
  onBackToDashboard?: () => void;
}

interface State {
  hasError: boolean;
  error: Error | null;
}

export class ContractWizardErrorBoundary extends Component<Props, State> {
  public state: State = {
    hasError: false,
    error: null
  };

  public static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  public componentDidCatch(error: Error, errorInfo: React.ErrorInfo) {
    logger.error('WizardErrorBoundary', 'render_error', { 
      message: error.message, 
      stack: error.stack,
      componentStack: errorInfo.componentStack 
    });
  }

  public render() {
    if (this.state.hasError) {
      return (
        <div className="flex flex-col items-center justify-center p-8 bg-zinc-900 border border-zinc-800 rounded-xl space-y-6 text-center">
          <div className="text-red-400">
            <svg className="w-12 h-12 mx-auto mb-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
            </svg>
          </div>
          <h2 className="text-xl font-bold text-white">Ocurrió un error inesperado en el Wizard</h2>
          <p className="text-zinc-400 max-w-md">
            No se pudo completar la operación debido a un fallo en el componente. Puedes intentar reiniciar el wizard o volver al dashboard.
          </p>
          <div className="flex space-x-4">
            <Button variant="outline" onClick={() => {
              this.setState({ hasError: false, error: null });
              this.props.onReset && this.props.onReset();
            }}>
              Reiniciar Wizard
            </Button>
            <Button onClick={() => this.props.onBackToDashboard && this.props.onBackToDashboard()}>
              Volver al Dashboard
            </Button>
          </div>
        </div>
      );
    }

    return this.props.children;
  }
}
