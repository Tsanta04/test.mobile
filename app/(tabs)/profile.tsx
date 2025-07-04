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
import { useTheme } from '@/contexts/ThemeContext';
import { Settings, LogOut, CreditCard as Edit3, Moon, Sun, ChartBar as BarChart3 } from 'lucide-react-native';
import Header from '@/components/Header';
import ButtonForm from '@/components/Form/ButtonForm';
import EditProfileForm from '@/components/Profil/EditProfil';
import { useNotif } from '@/contexts/NotifContext';
import OverviewStatisticsSection from '@/components/Analytic/OverviewStatistics';
import { useData } from '@/contexts/DataContext';
import { router } from 'expo-router';

// ProfileScreen: Displays and allows editing of the user's profile, with logout and theme toggle functionality
export default function ProfileScreen() {
  // Get user, logout, and updateProfile from context
  const { user, logout, updateProfile } = useAuth();
  const { addNotification } = useNotif();
  const {getStatProducts} = useData();
  const { totalProducts, totalValue, totalStock, averagePrice, highestPriced, lowestPriced, averageStock } = getStatProducts();
  const { colors, theme, toggleTheme } = useTheme();
  
  // State for editing mode, form fields, and loading
  const [isEditing, setIsEditing] = useState(false);
  const [editName, setEditName] = useState(user?.name || '');
  const [editEmail, setEditEmail] = useState(user?.email || '');
  const [isLoading, setIsLoading] = useState(false);

  // Save profile changes with validation and notification
  const saveProfile = async() =>{
    if (!editName.trim() || !editEmail.trim()) {
      Alert.alert('Error', 'Please fill in all fields');
      return;
    }

    if (!/\S+@\S+\.\S+/.test(editEmail)) {
      Alert.alert('Error', 'Please enter a valid email address');
      return;
    }

    setIsLoading(true);
    try {
      const success = await updateProfile(editName.trim(), editEmail);
      if (success) {
        setIsEditing(false);
        addNotification('Profile updated successfully', 'profile_updated');
        Alert.alert('Success', 'Profile updated successfully');
      } else {
        Alert.alert('Error', 'This email is already in use');
      }
    } catch (error) {
      Alert.alert('Error', 'Failed to update profile');
    } finally {
      setIsLoading(false);
    }    
  }

  // Confirm before saving profile changes
  const handleSaveProfile = async () => {
    console.log("Save Profile...");
    Alert.alert(
      'Editing profile',
      `Are you sure to edit your profile ?`,
      [
        { text: 'Cancel', style: 'cancel' },
        {
          text: 'Edit',
          style: 'destructive',
          onPress: saveProfile,
        },
      ]
    );    
  };

  // Cancel editing and reset form fields
  const handleCancelEdit = () => {
    setIsEditing(false);
    setEditName(user?.name || '');
    setEditEmail(user?.email || '');
  };

  // Confirm before logging out
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

  const handleStatDetail = () => {
    router.push('/analytics');
  }

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
      marginTop:24,
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
            {isEditing ? (
              <EditProfileForm
                editName={editName}
                setEditName={setEditName}
                editEmail={editEmail}
                setEditEmail={setEditEmail}
                handleCancelEdit={handleCancelEdit}
                handleSaveProfile={handleSaveProfile}
                isLoading={isLoading}
                colors={colors}
              />
              ):(
              <>
                <Text style={styles.profileName}>{user.name}</Text>
                <Text style={styles.profileEmail}>{user.email}</Text>
                <TouchableOpacity
                  style={styles.startEditButton}
                  onPress={() => setIsEditing(true)}
                >
                  <LinearGradient
                    colors={[colors.primary, colors.secondary]}
                    style={styles.startEditGradient}
                  >
                    <Edit3 size={16} color="#000" />
                    <Text style={styles.startEditButtonText}>Edit Profile</Text>
                  </LinearGradient>
                </TouchableOpacity>
              </>
            )}
          </View>
        </View>
        <OverviewStatisticsSection
          colors={colors}
          totalProducts={totalProducts}
          totalStock={totalStock}
          totalValue={totalValue}
          averagePrice={averagePrice}
          lowestPriced={lowestPriced}
          highestPriced={highestPriced}
          averageStock={averageStock}
        />
        <ButtonForm
          onPress={handleStatDetail}
          text="See more details"
          colors={[colors.primary, colors.secondary]}
          icon={<Edit3 size={20} color="#000" />}
        />        
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