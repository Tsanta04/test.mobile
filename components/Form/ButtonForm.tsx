import React from 'react';
import { TouchableOpacity, Text, StyleSheet, ViewStyle, TextStyle, ColorValue } from 'react-native';
import { LinearGradient } from 'expo-linear-gradient';
import { ColorType } from '@/constants/type';

// ButtonForm: A reusable gradient button component with optional icon and loading state
interface GradientButtonProps {
  // Function to call when button is pressed
  onPress: () => void;
  // Whether the button is disabled
  disabled?: boolean;
  // Whether to show loading state
  isLoading?: boolean;
  // Text to show when loading
  loadingText?: string;
  // Button text
  text: string;
  // Gradient colors
  colors: readonly [ColorValue, ColorValue, ...ColorValue[]];
  // Optional icon to display
  icon?: React.ReactNode;
  // Optional style override
  style?: ViewStyle;
}

const ButtonForm: React.FC<GradientButtonProps> = ({
  onPress,
  disabled = false,
  isLoading = false,
  loadingText = 'Loading...',
  text,
  colors,
  icon,
  style
}) => {
  // Styles for the button and its elements
  const styles = StyleSheet.create({
    loginButton: {
        marginTop: 8,
        borderRadius: 12,
        overflow: 'hidden',
      },
      loginGradient: {
        paddingVertical: 16,
        paddingHorizontal: 24,
        flexDirection: 'row',
        alignItems: 'center',
        justifyContent: 'center',
      },
      loginButtonText: {
        fontSize: 18,
        fontFamily: 'Inter-SemiBold',
        color: '#000',
        marginLeft: 8,
      },
  });

  return (
    // Touchable button with gradient background and optional icon
    <TouchableOpacity
      onPress={onPress}
      style={[styles.loginButton, style]}
      disabled={disabled}
    >
      <LinearGradient
        colors={colors}
        style={styles.loginGradient}
      >
        {icon}
        <Text style={styles.loginButtonText}>
          {isLoading ? loadingText : text}
        </Text>
      </LinearGradient>
    </TouchableOpacity>
  );
};

export default ButtonForm;
