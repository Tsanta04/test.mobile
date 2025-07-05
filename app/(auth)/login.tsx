import React, { useState } from 'react';
import {
  View,
  Text,
  TouchableOpacity,
  ScrollView,
  StyleSheet,
  KeyboardAvoidingView,
  Platform,
  Alert
} from 'react-native';
import { LinearGradient } from 'expo-linear-gradient';
import { useTheme } from '@/contexts/ThemeContext';
import { router } from 'expo-router';
import { Eye, EyeOff, Mail, Lock, LogIn } from 'lucide-react-native';
import { useAuth } from '@/contexts/AuthContext';
import { useLoading } from '@/contexts/LoadingContext';
import InputForm from '@/components/Form/InputForm';
import ButtonForm from '@/components/Form/ButtonForm';

export default function LoginScreen() {
  // Access the current theme colors (light/dark)
  const { colors } = useTheme();
  const { login } = useAuth();
  const { isLoading } = useLoading();

  // Form state
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [showPassword, setShowPassword] = useState(false);
  const [errors, setErrors] = useState<{ email?: string; password?: string }>({});


  // Form validation logic
  const validateForm = () => {
    const newErrors: { email?: string; password?: string } = {};

    // Validate email field
    if (!email) {
      newErrors.email = 'Email is required';
    } else if (!/\S+@\S+\.\S+/.test(email)) {
      newErrors.email = 'Please enter a valid email';
    }

    // Validate password field
    if (!password) {
      newErrors.password = 'Password is required';
    } else if (password.length < 6) {
      newErrors.password = 'Password must be at least 6 characters';
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };


  // Simulated login handler
  const handleLogin = async () => {
    if (!validateForm()) return;
    console.log("Form validated");

    //Do login
    try {
      const success = await login(email, password);
      if (success) {
        router.replace('/(tabs)');
      } else {
        Alert.alert('Login Failed', 'Invalid email or password. Please try again.');
      }
    } catch (error) {
      Alert.alert('Error', 'An unexpected error occurred. Please try again.');
    }
  };


  // Stylesheet definition
  const styles = StyleSheet.create({
    container: {
      flex: 1,
      backgroundColor: colors.background,
    },
    gradient: {
      flex: 1,
    },
    scrollContainer: {
      flex: 1,
      justifyContent: 'center',
      paddingHorizontal: 40,
      paddingVertical: 60,
    },
    header: {
      alignItems: 'center',
      marginBottom: 48,
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
    footer: {
      marginTop: 32,
      alignItems: 'center',
    },
    footerText: {
      fontSize: 16,
      fontFamily: 'Inter-Regular',
      color: colors.textSecondary,
    },
    signUpButton: {
      marginTop: 8,
    },
    signUpText: {
      fontSize: 16,
      fontFamily: 'Inter-SemiBold',
      color: colors.primary,
    },
    demoCredentials: {
      marginTop: 24,
      padding: 16,
      backgroundColor: colors.surface,
      borderRadius: 12,
      borderWidth: 1,
      borderColor: colors.border,
    },
    demoTitle: {
      fontSize: 14,
      fontFamily: 'Inter-SemiBold',
      color: colors.text,
      marginBottom: 8,
    },
    demoText: {
      fontSize: 12,
      fontFamily: 'Inter-Regular',
      color: colors.textSecondary,
      lineHeight: 16,
    },
  });

  
  return (
    <KeyboardAvoidingView
      style={styles.container}
      behavior={Platform.OS === 'ios' ? 'padding' : 'height'}
    >
      <LinearGradient
        colors={[colors.background, colors.surface]}
        style={styles.gradient}
      >
        <ScrollView
          contentContainerStyle={styles.scrollContainer}
          keyboardShouldPersistTaps="handled"
        >
          {/* Header with title and subtitle */}
          <View style={styles.header}>
            <Text style={styles.title}>Welcome Back</Text>
            <Text style={styles.subtitle}>
              Sign in to your account to continue
            </Text>
          </View>

          {/* Login form */}
          <View style={styles.form}>
            {/* Email input */}
            <InputForm
              label="Email"
              value={email}
              onChangeText={setEmail}
              placeholder="Enter your email"
              error={errors.email}
              colors={colors}
              keyboardType="email-address"
              icon={<Mail size={20} color={colors.textSecondary} style={styles.icon} />}
              requiredSign={false}
            />

            <InputForm
              label="Password"
              value={password}
              onChangeText={setPassword}
              placeholder="Enter your password"
              error={errors.password}
              colors={colors}
              secureTextEntry={!showPassword}
              autoCapitalize="none"
              autoCorrect={false}
              icon={<Lock size={20} color={colors.textSecondary} style={styles.icon} />}
              requiredSign={false}
              rightElement={
                <TouchableOpacity onPress={() => setShowPassword(!showPassword)} style={styles.eyeIcon}>
                  {showPassword ? (
                    <EyeOff size={20} color={colors.textSecondary} />
                  ) : (
                    <Eye size={20} color={colors.textSecondary} />
                  )}
                </TouchableOpacity>
              }
            />

            {/* Login button */}
            <ButtonForm
              onPress={handleLogin}
              disabled={isLoading}
              isLoading={isLoading}
              loadingText="Signing In..."
              text="Sign In"
              colors={[colors.primary, colors.secondary]}
              icon={<LogIn size={20} color="#000" />}
            />
          </View>

          {/* Demo credentials section */}
          <View style={styles.demoCredentials}>
            <Text style={styles.demoTitle}>Demo credentials (from data/users.json):</Text>
            <Text style={styles.demoText}>
              Email: test@example.com{'\n'}
              Password: password123{'\n\n'}
              Or create a new account below
            </Text>
          </View>

          {/* Link to registration page */}
          <View style={styles.footer}>
            <Text style={styles.footerText}>Don't have an account?</Text>
            <TouchableOpacity
              style={styles.signUpButton}
              onPress={() => router.push('/(auth)/register')}
            >
              <Text style={styles.signUpText}>Sign Up</Text>
            </TouchableOpacity>
          </View>
        </ScrollView>
      </LinearGradient>
    </KeyboardAvoidingView>
  );
}
