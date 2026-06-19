import React, { Component, ErrorInfo, ReactNode } from 'react';
import { AlertTriangle, RefreshCw } from 'lucide-react';
import { Button } from '@/components/ui/button';

interface Props {
  children?: ReactNode;
  fallback?: ReactNode;
}

interface State {
  hasError: boolean;
  error: Error | null;
  errorInfo: ErrorInfo | null;
}

export class ErrorBoundary extends Component<Props, State> {
  public state: State = {
    hasError: false,
    error: null,
    errorInfo: null,
  };

  public static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error, errorInfo: null };
  }

  public componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error('Uncaught error in ErrorBoundary:', error, errorInfo);
    this.setState({ errorInfo });
  }

  private handleReset = () => {
    this.setState({ hasError: false, error: null, errorInfo: null });
    window.location.reload();
  };

  public render() {
    if (this.state.hasError) {
      if (this.props.fallback) {
        return this.props.fallback;
      }

      return (
        <div className="min-h-[400px] flex flex-col items-center justify-center p-8 text-center bg-background rounded-lg border border-border m-4 shadow-sm">
          <div className="h-16 w-16 bg-red-100 text-red-600 rounded-full flex items-center justify-center mb-6">
            <AlertTriangle className="h-8 w-8" />
          </div>
          <h2 className="text-2xl font-bold text-foreground mb-2">Algo salió mal</h2>
          <p className="text-muted-foreground max-w-md mb-6">
            Ha ocurrido un error inesperado al cargar esta sección. Hemos registrado el problema para solucionarlo.
          </p>
          
          {this.state.error && (
            <div className="w-full max-w-2xl bg-muted p-4 rounded-md text-left overflow-auto mb-6 border border-border">
              <p className="font-mono text-sm text-red-500 font-semibold mb-2">{this.state.error.toString()}</p>
              {this.state.errorInfo && (
                <pre className="font-mono text-xs text-muted-foreground whitespace-pre-wrap">
                  {this.state.errorInfo.componentStack}
                </pre>
              )}
            </div>
          )}

          <Button onClick={this.handleReset} className="gap-2">
            <RefreshCw className="h-4 w-4" />
            Recargar aplicación
          </Button>
        </div>
      );
    }

    return this.props.children;
  }
}
