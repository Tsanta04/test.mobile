// StatCard: A reusable card component for displaying statistics with optional gradient background and icon
import React from 'react';
import { View, Text, StyleSheet } from 'react-native';
import { LinearGradient } from 'expo-linear-gradient';
import { useTheme } from '@/contexts/ThemeContext';

// Props for the statistics card component
interface StatCardProps {
  // Title displayed above the value
  title: string;
  // Main value to display (can be string or number)
  value: string | number;
  // Optional subtitle displayed below the value
  subtitle?: string;
  // Optional icon displayed next to the title
  icon?: React.ReactNode;
  // Whether to use gradient background instead of solid color
  gradient?: boolean;
}

export default function StatCard({ title, value, subtitle, icon, gradient = false }: StatCardProps) {
  const { colors } = useTheme();

  // Styles for the card and its elements
  const styles = StyleSheet.create({
    // Container for regular (non-gradient) cards
    container: {
      borderRadius: 12,
      padding: 16,
      borderWidth: 1,
      borderColor: colors.border,
      minHeight: 80,
      justifyContent: 'center',
    },
    // Container for gradient cards
    gradientContainer: {
      borderRadius: 12,
      padding: 16,
      minHeight: 80,
      justifyContent: 'center',
    },
    // Header section containing icon and title
    header: {
      flexDirection: 'row',
      alignItems: 'center',
      marginBottom: 8,
    },
    // Title text with conditional styling based on gradient
    title: {
      fontSize: 12,
      fontFamily: 'Inter-Regular',
      color: gradient ? '#000' : colors.textSecondary,
      marginLeft: icon ? 8 : 0,
    },
    // Main value text with conditional styling
    value: {
      fontSize: 24,
      fontFamily: 'Inter-Bold',
      color: gradient ? '#000' : colors.text,
      marginBottom: 4,
    },
    // Subtitle text with conditional styling
    subtitle: {
      fontSize: 10,
      fontFamily: 'Inter-Regular',
      color: gradient ? 'rgba(0,0,0,0.7)' : colors.textSecondary,
    },
  });

  // Render gradient version of the card
  if (gradient) {
    return (
      <LinearGradient
        colors={[colors.primary, colors.secondary]}
        style={styles.gradientContainer}
      >
        <View style={styles.header}>
          {icon}
          <Text style={styles.title}>{title}</Text>
        </View>
        <Text style={styles.value}>{value}</Text>
        {subtitle && <Text style={styles.subtitle}>{subtitle}</Text>}
      </LinearGradient>
    );
  }

  // Render regular version of the card
  return (
    <View style={[styles.container, { backgroundColor: colors.surface }]}>
      <View style={styles.header}>
        {icon}
        <Text style={styles.title}>{title}</Text>
      </View>
      <Text style={styles.value}>{value}</Text>
      {subtitle && <Text style={styles.subtitle}>{subtitle}</Text>}
    </View>
  );
}