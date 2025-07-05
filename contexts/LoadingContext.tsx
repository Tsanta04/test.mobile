import React, { createContext, useContext, useState, ReactNode } from 'react';

/**
 * Interface defining the structure of the Loading Context
 * Provides global loading state management across the application
 */
interface LoadingContextType {
  isLoading: boolean;
  loadingText?: string;
  /** Function to show loading spinner with optional text */
  showLoading: (text?: string) => void;
  /** Function to hide loading spinner and clear text */
  hideLoading: () => void;
  /** Function to update loading text without changing loading state */
  setLoadingText: (text: string) => void;
}

/**
 * Create the Loading Context with undefined as default value
 * This ensures TypeScript safety when using the context
 */
const LoadingContext = createContext<LoadingContextType | undefined>(undefined);

/**
 * Props interface for the LoadingProvider component
 */
interface LoadingProviderProps {
  /** React children components that will have access to loading context */
  children: ReactNode;
}

/**
 * LoadingProvider component that wraps the application to provide global loading state
 * 
 * This provider manages:
 * - Global loading state (isLoading)
 * - Loading text display (loadingText)
 * - Functions to control loading behavior
 * 
 * Usage:
 * <LoadingProvider>
 *   <App />
 * </LoadingProvider>
 */
export function LoadingProvider({ children }: LoadingProviderProps) {
  // State to track whether loading is currently active
  const [isLoading, setIsLoading] = useState(false);
  
  // State to store the text to display during loading
  const [loadingText, setLoadingTextState] = useState<string | undefined>(undefined);

  /**
   * Shows the loading spinner and optionally sets loading text
   * @param text - Optional text to display during loading
   */
  const showLoading = (text?: string) => {
    setIsLoading(true);
    if (text) {
      setLoadingTextState(text);
    }
  };

  /**
   * Hides the loading spinner and clears any loading text
   * This function should be called when the loading operation is complete
   */
  const hideLoading = () => {
    setIsLoading(false);
    setLoadingTextState(undefined);
  };

  /**
   * Updates the loading text without changing the loading state
   * Useful for updating progress messages during long operations
   * @param text - New text to display during loading
   */
  const setLoadingText = (text: string) => {
    setLoadingTextState(text);
  };

  // Create the context value object with all loading-related state and functions
  const value: LoadingContextType = {
    isLoading,
    loadingText,
    showLoading,
    hideLoading,
    setLoadingText,
  };

  return (
    <LoadingContext.Provider value={value}>
      {children}
    </LoadingContext.Provider>
  );
}

/**
 * Custom hook to access the Loading Context
 * 
 * This hook provides a safe way to access loading state and functions
 * from any component within the LoadingProvider
 * 
 * @returns LoadingContextType - The loading context with state and functions
 * @throws Error if used outside of LoadingProvider
 * 
 * Usage:
 * const { isLoading, showLoading, hideLoading } = useLoading();
 */
export function useLoading() {
  const context = useContext(LoadingContext);
  if (!context) {
    throw new Error('useLoading must be used within a LoadingProvider');
  }
  return context;
} 