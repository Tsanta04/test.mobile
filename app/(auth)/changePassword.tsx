import React, { useState } from 'react';
import {
  View,
  Text,
  TextInput,
  TouchableOpacity,
  Alert,
  ScrollView,
  StyleSheet,
  KeyboardAvoidingView,
  Platform
} from 'react-native';
import { LinearGradient } from 'expo-linear-gradient';
import { useTheme } from '@/contexts/ThemeContext';
import { router } from 'expo-router';
import { Eye, EyeOff, Mail, Lock, User, UserPlus, ArrowLeft } from 'lucide-react-native';
import { useAuth } from '@/contexts/AuthContext';
import { useLoading } from '@/contexts/LoadingContext';
import InputForm from '@/components/Form/InputForm';
import ButtonForm from '@/components/Form/ButtonForm';

// Registration screen component
export default function ChangePasswordScreen() {
  // Theme colors from context
  const { colors } = useTheme();
  const { changePassword } = useAuth();
  const { isLoading } = useLoading();

  // Form state variables
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [showPassword, setShowPassword] = useState(false);
  const [showConfirmPassword, setShowConfirmPassword] = useState(false);


  // Validation errors state
  const [errors, setErrors] = useState<{
    name?: string;
    email?: string;
    password?: string;
    confirmPassword?: string;
  }>({});


  /**
   * Validate the registration form
   * Returns true if the form is valid
   */
  const validateForm = () => {
    const newErrors: {
      name?: string;
      email?: string;
      password?: string;
      confirmPassword?: string;
    } = {};

    // Email validation
    if (!email) {
      newErrors.email = 'Email is required';
    } else if (!/\S+@\S+\.\S+/.test(email)) {
      newErrors.email = 'Please enter a valid email';
    }

    // Password validation
    if (!password) {
      newErrors.password = 'Password is required';
    } else if (password.length < 6) {
      newErrors.password = 'Password must be at least 6 characters';
    } else if (!/(?=.*[a-z])(?=.*[A-Z])(?=.*\d)/.test(password)) {
      newErrors.password = 'Password must contain uppercase, lowercase, and number';
    }

    // Confirm password validation
    if (!confirmPassword) {
      newErrors.confirmPassword = 'Please confirm your password';
    } else if (password !== confirmPassword) {
      newErrors.confirmPassword = 'Passwords do not match';
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };


  /**
   * Handle form submission
   * Called when the user presses the "Create Account" button
   */
  const handleUpdate = async () => {
    if (!validateForm()) return;
    console.log("Form submitted");
    try {
      const success = await changePassword(email, password);

      if (success) {
        Alert.alert(
          'Changing password Successful',
          'Your password has been changed successfully. Please log in.',
          [{ text: 'OK', onPress: () => router.replace('/(auth)/login') }]
        );
      } else {
        Alert.alert('Updating Failed', 'There is an error. Please reverify your information.');
      }
    } catch (error) {
      Alert.alert('Error', 'An unexpected error occurred. Please try again.');
    }
  };


  // Define styles using the current theme
  const styles = StyleSheet.create({
    container: {
      flex: 1,
      backgroundColor: colors.background,
    },
    gradient: {
      flex: 1,
    },
    scrollContainer: {
      flexGrow: 1,
      justifyContent: 'center',
      paddingHorizontal: 40,
      paddingVertical: 60,
    },
    backButton: {
      position: 'absolute',
      top: 60,
      left: 40,
      width: 40,
      height: 40,
      borderRadius: 20,
      backgroundColor: colors.surface,
      justifyContent: 'center',
      alignItems: 'center',
      borderWidth: 1,
      borderColor: colors.border,
    },
    header: {
      alignItems: 'center',
      marginBottom: 48,
      marginTop: 60,
    },
    title: {
      fontSize: 32,
      fontFamily: 'Inter-Bold',
      color: colors.text,
      marginBottom: 8,
    },
    subtitle: {
      fontSize: 16,
      fontFamily: 'Inter-Regular',
      color: colors.textSecondary,
      textAlign: 'center',
    },
    form: {
      gap: 20,
    },
    icon: {
      marginRight: 12,
    },
    eyeIcon: {
      marginLeft: 12,
    },
    errorText: {
      fontSize: 14,
      fontFamily: 'Inter-Regular',
      color: colors.error,
      marginTop: 4,
    },
    footer: {
      marginTop: 32,
      alignItems: 'center',
    },
    footerText: {
      fontSize: 16,
      fontFamily: 'Inter-Regular',
      color: colors.textSecondary,
    },
    signInButton: {
      marginTop: 8,
    },
    signInText: {
      fontSize: 16,
      fontFamily: 'Inter-SemiBold',
      color: colors.primary,
    },
  });

  
  // Render the registration screen UI
  return (
    <KeyboardAvoidingView
      style={styles.container}
      behavior={Platform.OS === 'ios' ? 'padding' : 'height'}
    >
      <LinearGradient
        colors={[colors.background, colors.surface]}
        style={styles.gradient}
      >
        {/* Back button to navigate to the previous screen */}
        <TouchableOpacity
          style={styles.backButton}
          onPress={() => router.back()}
        >
          <ArrowLeft size={20} color={colors.text} />
        </TouchableOpacity>

        <ScrollView
          contentContainerStyle={styles.scrollContainer}
          keyboardShouldPersistTaps="handled"
        >
          {/* Header section with title and subtitle */}
          <View style={styles.header}>
            <Text style={styles.title}>Change Password</Text>
            <Text style={styles.subtitle}>
                Change your password to secure your account
            </Text>
          </View>

          {/* Registration form */}
          <View style={styles.form}>

            <InputForm
              label="Email"
              value={email}
              onChangeText={setEmail}
              placeholder="Enter your email"
              error={errors.email}
              colors={colors}
              keyboardType="email-address"
              autoCapitalize="none"
              autoCorrect={false}
              requiredSign={false}
              icon={<Mail size={20} color={colors.textSecondary} style={styles.icon} />}
            />

            <InputForm
              label="Password"
              value={password}
              onChangeText={setPassword}
              placeholder="Create a password"
              error={errors.password}
              colors={colors}
              secureTextEntry={!showPassword}
              autoCapitalize="none"
              autoCorrect={false}
              requiredSign={false}
              icon={<Lock size={20} color={colors.textSecondary} style={styles.icon} />}
              rightElement={
                <TouchableOpacity
                  onPress={() => setShowPassword(!showPassword)}
                  style={styles.eyeIcon}
                >
                  {showPassword ? (
                    <EyeOff size={20} color={colors.textSecondary} />
                  ) : (
                    <Eye size={20} color={colors.textSecondary} />
                  )}
                </TouchableOpacity>
              }
            />

            <InputForm
              label="Confirm Password"
              value={confirmPassword}
              onChangeText={setConfirmPassword}
              placeholder="Confirm your password"
              error={errors.confirmPassword}
              colors={colors}
              secureTextEntry={!showConfirmPassword}
              autoCapitalize="none"
              autoCorrect={false}
              requiredSign={false}
              icon={<Lock size={20} color={colors.textSecondary} style={styles.icon} />}
              rightElement={
                <TouchableOpacity
                  onPress={() => setShowConfirmPassword(!showConfirmPassword)}
                  style={styles.eyeIcon}
                >
                  {showConfirmPassword ? (
                    <EyeOff size={20} color={colors.textSecondary} />
                  ) : (
                    <Eye size={20} color={colors.textSecondary} />
                  )}
                </TouchableOpacity>
              }
            />

            {/* Submit button */}
            <ButtonForm
              onPress={handleUpdate}
              disabled={isLoading}
              isLoading={isLoading}
              loadingText="Update Account..."
              text="Change Password"
              colors={[colors.primary, colors.secondary]}
              icon={<UserPlus size={20} color="#000" />}
            />
          </View>

          {/* Footer with link to Sign In */}
          <View style={styles.footer}>
            <Text style={styles.footerText}>Not forgot password?</Text>
            <TouchableOpacity
              style={styles.signInButton}
              onPress={() => router.replace('/(auth)/login')}
            >
              <Text style={styles.signInText}>Sign In</Text>
            </TouchableOpacity>
          </View>
        </ScrollView>
      </LinearGradient>
    </KeyboardAvoidingView>
  );
}
