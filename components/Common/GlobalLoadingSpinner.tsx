import React from 'react';
import { LoadingSpinner } from './LoadingSpinner';
import { useLoading } from '@/contexts/LoadingContext';

export function GlobalLoadingSpinner() {
  const { isLoading, loadingText } = useLoading();

  if (!isLoading) {
    return null;
  }

  return (
    <LoadingSpinner
      fullScreen={true}
      text={loadingText}
      size="lg"
    />
  );
} 