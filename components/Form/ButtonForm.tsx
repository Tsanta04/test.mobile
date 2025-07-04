import React from 'react';
import { TouchableOpacity, Text, StyleSheet, ViewStyle, TextStyle, ColorValue } from 'react-native';
import { LinearGradient } from 'expo-linear-gradient';
import { ColorType } from '@/constants/type';

interface GradientButtonProps {
  onPress: () => void;
  disabled?: boolean;
  isLoading?: boolean;
  loadingText?: string;
  text: string;
  colors: readonly [ColorValue, ColorValue, ...ColorValue[]];
  icon?: React.ReactNode;
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
