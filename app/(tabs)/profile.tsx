import React, { useState } from 'react';
import {
  View,
  Text,
  TouchableOpacity,
  ScrollView,
  TextInput,
  Alert,
  StyleSheet,
  Dimensions,
} from 'react-native';
import { LinearGradient } from 'expo-linear-gradient';
import { useAuth } from '@/contexts/AuthContext';
import { useData } from '@/contexts/DataContext';
import { useTheme } from '@/contexts/ThemeContext';
import { router } from 'expo-router';
import { User, Mail, Package, Settings, LogOut, CreditCard as Edit3, Save, X, Moon, Sun, DollarSign, TrendingUp, ShoppingCart, ChartBar as BarChart3 } from 'lucide-react-native';
import Header from '@/components/Header';
import ButtonForm from '@/components/Form/ButtonForm';

const { width } = Dimensions.get('window');

export default function ProfileScreen() {
  const { user, logout, updateProfile } = useAuth();
  const { getUserProducts } = useData();
  const { colors, theme, toggleTheme } = useTheme();
  
  const handleLogout = () => {
    Alert.alert(
      'Logout',
      'Are you sure you want to logout?',
      [
        { text: 'Cancel', style: 'cancel' },
        { text: 'Logout', style: 'destructive', onPress: logout },
      ]
    );
  };

  if (!user) {
    return null;
  }

  const styles = StyleSheet.create({
    container: {
      flex: 1,
      backgroundColor: colors.background,
    },
    scrollContent: {
      flex: 1,
      paddingHorizontal: 24,
    },
    profileCard: {
      backgroundColor: colors.surface,
      borderRadius: 24,
      padding: 24,
      marginBottom: 24,
      alignItems: 'center',
      shadowColor: '#000',
      shadowOpacity: 0.1,
      shadowOffset: { width: 0, height: 2 },
      shadowRadius: 8,
      elevation: 4,
    },
    avatarContainer: {
      width: 80,
      height: 80,
      borderRadius: 40,
      marginBottom: 16,
      overflow: 'hidden',
    },
    avatarGradient: {
      flex: 1,
      justifyContent: 'center',
      alignItems: 'center',
    },
    avatarText: {
      fontSize: 32,
      fontFamily: 'Inter-Bold',
      color: '#000',
    },
    profileInfo: {
      alignItems: 'center',
      width: '100%',
    },
    profileName: {
      fontSize: 22,
      fontFamily: 'Inter-Bold',
      color: colors.text,
      marginBottom: 4,
    },
    profileEmail: {
      fontSize: 14,
      fontFamily: 'Inter-Regular',
      color: colors.textSecondary,
      marginBottom: 20,
    },
    startEditButton: {
      borderRadius: 16,
      overflow: 'hidden',
      width: '60%',
    },
    startEditGradient: {
      paddingVertical: 12,
      paddingHorizontal: 20,
      flexDirection: 'row',
      alignItems: 'center',
      justifyContent: 'center',
    },
    startEditButtonText: {
      fontSize: 14,
      fontFamily: 'Inter-SemiBold',
      color: '#000',
      marginLeft: 8,
    },
    settingsCard: {
      backgroundColor: colors.surface,
      borderRadius: 24,
      borderWidth: 1,
      borderColor: colors.border,
      overflow: 'hidden',
      marginBottom: 24,
      shadowColor: '#000',
      shadowOpacity: 0.05,
      shadowOffset: { width: 0, height: 2 },
      shadowRadius: 6,
      elevation: 2,
    },
    settingsItem: {
      flexDirection: 'row',
      alignItems: 'center',
      paddingVertical: 18,
      paddingHorizontal: 20,
      borderBottomWidth: 1,
      borderBottomColor: colors.border,
    },
    settingsItemLast: {
      borderBottomWidth: 0,
    },
    settingsIcon: {
      marginRight: 16,
    },
    settingsText: {
      flex: 1,
      fontSize: 15,
      fontFamily: 'Inter-Medium',
      color: colors.text,
    }
  });
  

  return (
    <View style={styles.container}>
      <Header
        title="Profile"
        theme={theme}
        toggleTheme={toggleTheme}
        colors={colors}
      />

      <ScrollView style={styles.scrollContent}>
        <View style={styles.profileCard}>
          <View style={styles.avatarContainer}>
            <LinearGradient
              colors={[colors.primary, colors.secondary]}
              style={styles.avatarGradient}
            >
              <Text style={styles.avatarText}>
                {user.name.charAt(0).toUpperCase()}
              </Text>
            </LinearGradient>
          </View>

          <View style={styles.profileInfo}>
            <Text style={styles.profileName}>{user.name}</Text>
            <Text style={styles.profileEmail}>{user.email}</Text>
            <TouchableOpacity
              style={styles.startEditButton}
              onPress={() => console.log("Edit true")}
            >
              <LinearGradient
                colors={[colors.primary, colors.secondary]}
                style={styles.startEditGradient}
              >
                <Edit3 size={16} color="#000" />
                <Text style={styles.startEditButtonText}>Edit Profile</Text>
              </LinearGradient>
            </TouchableOpacity>
          </View>
        </View>

        <View style={styles.settingsCard}>
          <TouchableOpacity style={styles.settingsItem}>
            <Settings size={18} color={colors.textSecondary} style={styles.settingsIcon} />
            <Text style={styles.settingsText}>Settings</Text>
          </TouchableOpacity>
          <TouchableOpacity style={[styles.settingsItem, styles.settingsItemLast]} onPress={toggleTheme}>
            {theme === 'light' ? (
              <Moon size={18} color={colors.textSecondary} style={styles.settingsIcon} />
            ) : (
              <Sun size={18} color={colors.textSecondary} style={styles.settingsIcon} />
            )}
            <Text style={styles.settingsText}>
              Switch to {theme === 'light' ? 'Dark' : 'Light'} Mode
            </Text>
          </TouchableOpacity>
        </View>
        <ButtonForm
          onPress={handleLogout}
          text="Logout"
          colors={[colors.error, '#FE4848']}
          icon={<LogOut size={18} color="#FFF" />}
        />
      </ScrollView>
    </View>
  );
}