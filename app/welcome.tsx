import React from 'react';
import {
  View,
  Text,
  TouchableOpacity,
  StyleSheet,
  Dimensions,
} from 'react-native';
import { LinearGradient } from 'expo-linear-gradient';
import { useTheme } from '@/contexts/ThemeContext';
import { router } from 'expo-router';
import { ArrowRight, Package, Star, Users } from 'lucide-react-native';


export default function WelcomeScreen() {
  // Get theme colors from context (light/dark mode support)
  const { colors } = useTheme();

  // Handler to navigate to the login page
  const handleGetStarted = () => {
    router.replace('/(auth)/login' as any);
  };

  /**
   * Styles specific to this WelcomeScreen.
   * Uses theme colors dynamically for dark/light mode support.
   */
  const styles = StyleSheet.create({
    container: {
      flex: 1,
      backgroundColor: colors.background,
    },
    gradient: {
      flex: 1,
    },
    content: {
      flex: 1,
      justifyContent: 'center',
      alignItems: 'center',
      paddingHorizontal: 32,
    },
    logoContainer: {
      width: 120,
      height: 120,
      borderRadius: 60,
      marginBottom: 32,
      overflow: 'hidden',
      shadowColor: '#000',
      shadowOffset: { width: 0, height: 8 },
      shadowOpacity: 0.3,
      shadowRadius: 16,
      elevation: 8,
    },
    logoGradient: {
      flex: 1,
      justifyContent: 'center',
      alignItems: 'center',
    },
    logoIcon: {
      marginBottom: 8,
    },
    logoText: {
      fontSize: 16,
      fontFamily: 'Inter-Bold',
      color: '#000',
    },
    appName: {
      fontSize: 36,
      fontFamily: 'Inter-Bold',
      color: colors.text,
      marginBottom: 16,
      textAlign: 'center',
    },
    slogan: {
      fontSize: 18,
      fontFamily: 'Inter-Regular',
      color: colors.textSecondary,
      textAlign: 'center',
      marginBottom: 48,
      lineHeight: 26,
    },
    featuresContainer: {
      width: '100%',
      alignItems: 'center',
      justifyContent: 'center',      
      marginBottom: 48,
    },
    featureItem: {
      flexDirection: 'row',
      alignItems: 'center',
      marginBottom: 16,
      paddingHorizontal: 16,
    },
    featureIcon: {
      width: 40,
      height: 40,
      borderRadius: 20,
      justifyContent: 'center',
      alignItems: 'center',
      marginRight: 16,
    },
    featureText: {
      fontSize: 16,
      fontFamily: 'Inter-Medium',
      color: colors.text,
      flex: 1,
    },
    getStartedButton: {
      borderRadius: 16,
      overflow: 'hidden',
      shadowColor: '#000',
      shadowOffset: { width: 0, height: 4 },
      shadowOpacity: 0.3,
      shadowRadius: 8,
      elevation: 6,
    },
    getStartedGradient: {
      paddingVertical: 18,
      paddingHorizontal: 48,
      flexDirection: 'row',
      alignItems: 'center',
      justifyContent: 'center',
    },
    getStartedText: {
      fontSize: 18,
      fontFamily: 'Inter-Bold',
      color: '#000',
      marginRight: 12,
    },
    footer: {
      position: 'absolute',
      bottom: 40,
      left: 0,
      right: 0,
      alignItems: 'center',
    },
    footerText: {
      fontSize: 14,
      fontFamily: 'Inter-Regular',
      color: colors.textSecondary,
      textAlign: 'center',
    },
  });

  /**
   * Define features to display in the welcome screen
   * Each feature has an icon, description text, and color
   */
  const features = [
    {
      icon: Package,
      text: 'Manage your product catalog effortlessly',
      color: colors.primary,
    },
    {
      icon: Users,
      text: 'Connect with sellers and expand your reach',
      color: colors.success,
    },
    {
      icon: Star,
      text: 'Track performance with detailed analytics',
      color: colors.secondary,
    },
  ];

  return (
    <View style={styles.container}>

      <LinearGradient
        colors={[colors.background, colors.surface]}
        style={styles.gradient}
      >
        <View style={styles.content}>
          {/* App logo with gradient circle */}
          <View style={styles.logoContainer}>
            <LinearGradient
              colors={[colors.primary, colors.secondary]}
              style={styles.logoGradient}
            >
              <Package size={32} color="#000" style={styles.logoIcon} />
              <Text style={styles.logoText}>PM</Text>
            </LinearGradient>
          </View>

          {/* App name */}
          <Text style={styles.appName}>Product Manager</Text>

          {/* Slogan text */}
          <Text style={styles.slogan}>
            Your complete solution for{'\n'}
            product management and analytics
          </Text>

          {/* Features list */}
          <View style={styles.featuresContainer}>
            {features.map((feature, index) => (
              <View key={index} style={styles.featureItem}>
                <View
                  style={[
                    styles.featureIcon,
                    { backgroundColor: feature.color + '20' },
                  ]}
                >
                  <feature.icon size={20} color={feature.color} />
                </View>
                <Text style={styles.featureText}>{feature.text}</Text>
              </View>
            ))}
          </View>

          {/* Get Started Button */}
          <TouchableOpacity
            style={styles.getStartedButton}
            onPress={handleGetStarted}
          >
            <LinearGradient
              colors={[colors.primary, colors.secondary]}
              style={styles.getStartedGradient}
            >
              <Text style={styles.getStartedText}>Get Started</Text>
              <ArrowRight size={20} color="#000" />
            </LinearGradient>
          </TouchableOpacity>
        </View>

        {/* Footer text */}
        <View style={styles.footer}>
          <Text style={styles.footerText}>
            Start managing your products like a pro
          </Text>
        </View>
      </LinearGradient>
    </View>
  );
}
