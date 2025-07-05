import React from 'react';
import {
  View,
  Text,
  StyleSheet,
  ActivityIndicator,
  Dimensions,
} from 'react-native';
import { useTheme } from '@/contexts/ThemeContext';

interface LoadingSpinnerProps {
  size?: 'sm' | 'md' | 'lg';
  text?: string;
  fullScreen?: boolean;
}

export function LoadingSpinner({
  size = 'md',
  text,
  fullScreen = false,
}: LoadingSpinnerProps) {
  const { colors } = useTheme();
  
  const spinnerSize = {
    sm: 20,
    md: 40,
    lg: 60,
  }[size];

  const textSize = {
    sm: 12,
    md: 16,
    lg: 20,
  }[size];

  const content = (
    <View style={styles.content}>
      <ActivityIndicator
        size="large"
        color={colors.secondary}
        style={{ width: spinnerSize, height: spinnerSize }}
      />
      {text && (
        <Text style={[styles.text, { fontSize: textSize, color: colors.textSecondary }]}>
          {text}
        </Text>
      )}
    </View>
  );

  if (fullScreen) {
    return (
      <View style={[styles.fullScreen, { backgroundColor: `rgba(${colors.background === '#FFFFFF' ? '255, 255, 255' : '15, 23, 42'}, 0.8)` }]}>
        {content}
      </View>
    );
  }

  return content;
}

const styles = StyleSheet.create({
  fullScreen: {
    position: 'absolute',
    top: 0,
    left: 0,
    width: Dimensions.get('window').width,
    height: Dimensions.get('window').height,
    justifyContent: 'center',
    alignItems: 'center',
    zIndex: 999,
  },
  content: {
    alignItems: 'center',
    justifyContent: 'center',
    gap: 12,
  },
  text: {
    opacity: 0.7,
  },
});
