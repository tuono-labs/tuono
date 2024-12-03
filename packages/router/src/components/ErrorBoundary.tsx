import React, { Component, ErrorInfo, ReactNode } from 'react';

interface ErrorBoundaryProps {
  children: ReactNode;
  errorMessage?: string;
  fallbackComponent?: ReactNode;
  onError?: (error: Error, errorInfo: ErrorInfo) => void;
  resetKeys?: any[]; // Allow resetting error state when specific props change
}

interface ErrorBoundaryState {
  hasError: boolean;
  error?: Error;
  errorInfo?: ErrorInfo;
}

class ErrorBoundary extends Component<ErrorBoundaryProps, ErrorBoundaryState> {
  constructor(props: ErrorBoundaryProps) {
    super(props);
    this.state = { 
      hasError: false,
      error: undefined,
      errorInfo: undefined
    };
  }

  // Static method to derive state from an error
  static getDerivedStateFromError(error: Error): ErrorBoundaryState {
    // Update state to display the fallback UI
    return { 
      hasError: true,
      error 
    };
  }

  // Lifecycle method to catch errors
  componentDidCatch(error: Error, errorInfo: ErrorInfo): void {
    // Log the error
    console.error('ErrorBoundary caught an error', error, errorInfo);

    // Call external error handler if provided
    if (this.props.onError) {
      this.props.onError(error, errorInfo);
    }

    // Store full error details in state for more comprehensive error reporting
    this.setState({ 
      error, 
      errorInfo 
    });
  }

  // Check if we should reset the error state based on props changes
  componentDidUpdate(prevProps: ErrorBoundaryProps): void {
    if (this.props.resetKeys && 
        this.state.hasError && 
        this.props.resetKeys.some((key, index) => 
          key !== prevProps.resetKeys?.[index]
        )) {
      this.resetErrorBoundary();
    }
  }

  // Method to reset error state
  resetErrorBoundary = () => {
    this.setState({ 
      hasError: false,
      error: undefined,
      errorInfo: undefined 
    });
  };

  // Render method with improved error handling
  render(): ReactNode {
    if (this.state.hasError) {
      // Priority: 
      // 1. Custom fallback component
      // 2. Custom error message
      // 3. Default error message
      if (this.props.fallbackComponent) {
        return this.props.fallbackComponent;
      }

      return (
        <div role="alert" className="error-boundary">
          <h1>{this.props.errorMessage || 'Something went wrong.'}</h1>
          <details style={{ whiteSpace: 'pre-wrap' }}>
            {this.state.error && <summary>Error Details</summary>}
            {this.state.error && <p>{this.state.error.toString()}</p>}
            {this.state.errorInfo?.componentStack && (
              <pre>{this.state.errorInfo.componentStack}</pre>
            )}
          </details>
          <button onClick={this.resetErrorBoundary}>
            Reset
          </button>
        </div>
      );
    }

    return this.props.children;
  }
}

export default ErrorBoundary;