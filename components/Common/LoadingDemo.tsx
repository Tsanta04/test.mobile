import React from 'react';
import { View, Text, TouchableOpacity, StyleSheet, ScrollView } from 'react-native';
import { useLoading } from '@/contexts/LoadingContext';
import { useTheme } from '@/contexts/ThemeContext';
import { useAuth } from '@/contexts/AuthContext';
import { useData } from '@/contexts/DataContext';
import { 
  Play, 
  Pause, 
  RotateCcw, 
  Clock, 
  CheckCircle, 
  AlertCircle,
  Database,
  User,
  Package,
  Trash2,
  Lock
} from 'lucide-react-native';

export function LoadingDemo() {
  const { showLoading, hideLoading, isLoading, loadingText } = useLoading();
  const { colors } = useTheme();
  const { login, register, logout, updateProfile, changePassword } = useAuth();
  const { addProduct, addCategory, addSeller, deleteProduct } = useData();

  // Test functions for different loading scenarios
  const testBasicLoading = () => {
    showLoading('Loading...');
    setTimeout(() => hideLoading(), 2000);
  };

  const testCustomTextLoading = (text: string) => {
    showLoading(text);
    setTimeout(() => hideLoading(), 3000);
  };

  const testAuthLoading = async () => {
    try {
      await login('test@example.com', 'password123');
    } catch (error) {
      console.log('Login test completed');
    }
  };

  const testDataLoading = async () => {
    try {
      await addProduct({
        name: 'Test Product',
        description: 'A test product for demo',
        price: 99.99,
        stock: 10,
        category: 'Electronics',
        seller: 'Test Seller',
        image: 'https://via.placeholder.com/300',
        isActive: true,
      });
    } catch (error) {
      console.log('Product creation test completed');
    }
  };
  console.log("done");
  const testCategoryLoading = async () => {
    try {
      await addCategory('Test Category');
    } catch (error) {
      console.log('Category creation test completed');
    }
  };

  const testSellerLoading = async () => {
    try {
      await addSeller('Test Seller');
    } catch (error) {
      console.log('Seller creation test completed');
    }
  };

  const testProfileLoading = async () => {
    try {
      await updateProfile('Demo User', 'demo@example.com');
    } catch (error) {
      console.log('Profile update test completed');
    }
  };

  const testLogoutLoading = async () => {
    try {
      await logout();
    } catch (error) {
      console.log('Logout test completed');
    }
  };

  const testChangePasswordLoading = async () => {
    try {
      await changePassword('test@example.com', 'newPassword123');
    } catch (error) {
      console.log('Change password test completed');
    }
  };

  const styles = StyleSheet.create({
    container: {
      flex: 1,
      padding: 20,
      backgroundColor: colors.background,
    },
    title: {
      fontSize: 24,
      fontFamily: 'Inter-Bold',
      color: colors.text,
      marginBottom: 8,
      textAlign: 'center',
    },
    subtitle: {
      fontSize: 14,
      fontFamily: 'Inter-Regular',
      color: colors.textSecondary,
      textAlign: 'center',
      marginBottom: 24,
    },
    statusContainer: {
      backgroundColor: colors.surface,
      padding: 16,
      borderRadius: 12,
      marginBottom: 24,
      borderWidth: 1,
      borderColor: colors.border,
    },
    statusTitle: {
      fontSize: 16,
      fontFamily: 'Inter-SemiBold',
      color: colors.text,
      marginBottom: 8,
    },
    statusText: {
      fontSize: 14,
      fontFamily: 'Inter-Regular',
      color: colors.textSecondary,
    },
    sectionTitle: {
      fontSize: 18,
      fontFamily: 'Inter-SemiBold',
      color: colors.text,
      marginBottom: 16,
      marginTop: 8,
    },
    buttonGrid: {
      gap: 12,
    },
    button: {
      backgroundColor: colors.primary,
      paddingVertical: 12,
      paddingHorizontal: 16,
      borderRadius: 8,
      flexDirection: 'row',
      alignItems: 'center',
      justifyContent: 'space-between',
    },
    buttonText: {
      color: colors.background,
      fontSize: 14,
      fontFamily: 'Inter-SemiBold',
      flex: 1,
      marginLeft: 12,
    },
    buttonIcon: {
      marginRight: 8,
    },
    disabledButton: {
      backgroundColor: colors.border,
      opacity: 0.6,
    },
    disabledText: {
      color: colors.textSecondary,
    },
    infoText: {
      fontSize: 12,
      fontFamily: 'Inter-Regular',
      color: colors.textSecondary,
      fontStyle: 'italic',
      marginTop: 8,
    },
  });

  return (
    <ScrollView style={styles.container} showsVerticalScrollIndicator={false}>
      {/* Header */}
      <Text style={styles.title}>Loading Context Demo</Text>
      <Text style={styles.subtitle}>
        Test the centralized loading system with different scenarios
      </Text>

      {/* Status Display */}
      <View style={styles.statusContainer}>
        <Text style={styles.statusTitle}>Current Status</Text>
        <Text style={styles.statusText}>
          Loading: {isLoading ? 'Active' : 'Inactive'}
        </Text>
        {isLoading && loadingText && (
          <Text style={styles.statusText}>
            Text: "{loadingText}"
          </Text>
        )}
      </View>

      {/* Basic Loading Tests */}
      <Text style={styles.sectionTitle}>Basic Loading Tests</Text>
      <View style={styles.buttonGrid}>
        <TouchableOpacity 
          style={[styles.button, isLoading && styles.disabledButton]} 
          onPress={testBasicLoading}
          disabled={isLoading}
        >
          <Play size={16} color={isLoading ? colors.textSecondary : colors.background} />
          <Text style={[styles.buttonText, isLoading && styles.disabledText]}>
            Basic Loading (2s)
          </Text>
        </TouchableOpacity>

        <TouchableOpacity 
          style={[styles.button, isLoading && styles.disabledButton]} 
          onPress={() => testCustomTextLoading('Processing data...')}
          disabled={isLoading}
        >
          <Database size={16} color={isLoading ? colors.textSecondary : colors.background} />
          <Text style={[styles.buttonText, isLoading && styles.disabledText]}>
            Custom Text Loading
          </Text>
        </TouchableOpacity>

        <TouchableOpacity 
          style={[styles.button, isLoading && styles.disabledButton]} 
          onPress={() => testCustomTextLoading('Uploading files...')}
          disabled={isLoading}
        >
          <Package size={16} color={isLoading ? colors.textSecondary : colors.background} />
          <Text style={[styles.buttonText, isLoading && styles.disabledText]}>
            File Upload Simulation
          </Text>
        </TouchableOpacity>
      </View>

      {/* Authentication Tests */}
      <Text style={styles.sectionTitle}>Authentication Tests</Text>
      <View style={styles.buttonGrid}>
        <TouchableOpacity 
          style={[styles.button, isLoading && styles.disabledButton]} 
          onPress={testAuthLoading}
          disabled={isLoading}
        >
          <User size={16} color={isLoading ? colors.textSecondary : colors.background} />
          <Text style={[styles.buttonText, isLoading && styles.disabledText]}>
            Test Login
          </Text>
        </TouchableOpacity>

        <TouchableOpacity 
          style={[styles.button, isLoading && styles.disabledButton]} 
          onPress={testProfileLoading}
          disabled={isLoading}
        >
          <CheckCircle size={16} color={isLoading ? colors.textSecondary : colors.background} />
          <Text style={[styles.buttonText, isLoading && styles.disabledText]}>
            Test Profile Update
          </Text>
        </TouchableOpacity>

        <TouchableOpacity 
          style={[styles.button, isLoading && styles.disabledButton]} 
          onPress={testLogoutLoading}
          disabled={isLoading}
        >
          <RotateCcw size={16} color={isLoading ? colors.textSecondary : colors.background} />
          <Text style={[styles.buttonText, isLoading && styles.disabledText]}>
            Test Logout
          </Text>
        </TouchableOpacity>

        <TouchableOpacity 
          style={[styles.button, isLoading && styles.disabledButton]} 
          onPress={testChangePasswordLoading}
          disabled={isLoading}
        >
          <Lock size={16} color={isLoading ? colors.textSecondary : colors.background} />
          <Text style={[styles.buttonText, isLoading && styles.disabledText]}>
            Test Change Password
          </Text>
        </TouchableOpacity>
      </View>

      {/* Data Management Tests */}
      <Text style={styles.sectionTitle}>Data Management Tests</Text>
      <View style={styles.buttonGrid}>
        <TouchableOpacity 
          style={[styles.button, isLoading && styles.disabledButton]} 
          onPress={testDataLoading}
          disabled={isLoading}
        >
          <Package size={16} color={isLoading ? colors.textSecondary : colors.background} />
          <Text style={[styles.buttonText, isLoading && styles.disabledText]}>
            Test Add Product
          </Text>
        </TouchableOpacity>

        <TouchableOpacity 
          style={[styles.button, isLoading && styles.disabledButton]} 
          onPress={testCategoryLoading}
          disabled={isLoading}
        >
          <Database size={16} color={isLoading ? colors.textSecondary : colors.background} />
          <Text style={[styles.buttonText, isLoading && styles.disabledText]}>
            Test Add Category
          </Text>
        </TouchableOpacity>

        <TouchableOpacity 
          style={[styles.button, isLoading && styles.disabledButton]} 
          onPress={testSellerLoading}
          disabled={isLoading}
        >
          <User size={16} color={isLoading ? colors.textSecondary : colors.background} />
          <Text style={[styles.buttonText, isLoading && styles.disabledText]}>
            Test Add Seller
          </Text>
        </TouchableOpacity>
      </View>

      {/* Information */}
      <View style={styles.statusContainer}>
        <Text style={styles.statusTitle}>How it works</Text>
        <Text style={styles.infoText}>
          • All loading states are managed centrally by LoadingContext{'\n'}
          • Loading text is customizable for each operation{'\n'}
          • Automatic error handling with try/catch/finally{'\n'}
          • Consistent UX across the entire application{'\n'}
          • No manual loading management needed in components
        </Text>
      </View>
    </ScrollView>
  );
} 