// TabLayout: Main tab navigator for authenticated users, with notification badge and profile tab
// Handles redirect if user is not authenticated
import { useAuth } from '@/contexts/AuthContext';
import { useNotif } from '@/contexts/NotifContext';
import { useTheme } from '@/contexts/ThemeContext';
import { Redirect, Tabs } from 'expo-router';
import { Chrome as Home, Bell, User } from 'lucide-react-native';
import { View, Text, StyleSheet } from 'react-native';

export default function TabLayout() {
  // Get authentication, notification, and theme context
  const { user, isLoading } = useAuth();
  const { notifications } = useNotif();  
  const { colors } = useTheme();

  if (isLoading) {
    // Optionally show a loading indicator here
    return null;
  }

  // If no user is logged in, redirect to login screen
  if (!user) {
    return <Redirect href="/(auth)/login" />;
  }
  // Count unread notifications for the current user
  const unreadNotifications = notifications.filter(n => n.userId === user.id && !n.read).length;
  
  // Render the tab navigator for authenticated users
  return (
    <Tabs
      screenOptions={{
        headerShown: false,
        tabBarStyle: {
          backgroundColor: colors.surface,
          borderTopColor: colors.border,
        },
        tabBarLabelStyle: {
          fontSize: 12,
          fontFamily: 'Inter-Medium',
          marginTop: 4,
        },
        tabBarActiveTintColor: colors.primary,
        tabBarInactiveTintColor: colors.textSecondary,        
      }}
    >
      {/* Home / Products tab */}
      <Tabs.Screen
        name="index"
        options={{
          title: 'Products',
          tabBarIcon: ({ color, size }) => (
            <Home size={size} color={color} />
          ),
        }}
      />

      {/* Notifications tab with badge for unread notifications */}
      <Tabs.Screen
        name="notifications"
        options={{
          title: 'Notifications',
          tabBarIcon: ({ color, size }) => (
            <View style={styles.notificationIcon}>
              <Bell size={size} color={color} />
              {unreadNotifications > 0 && (
                <View style={[styles.badge, { backgroundColor: colors.error }]}>
                  <Text style={styles.badgeText}>
                    {unreadNotifications > 99 ? '99+' : unreadNotifications}
                  </Text>
                </View>
              )}              
            </View>
          ),
        }}
      />

      {/* User profile tab */}
      <Tabs.Screen
        name="profile"
        options={{
          title: 'Profile',
          tabBarIcon: ({ color, size }) => (
            <User size={size} color={color} />
          ),
        }}
      />
    </Tabs>
  );
}

// Styles for notification icon badge (unused badge styles are ready for future use)
const styles = StyleSheet.create({
  notificationIcon: {
    position: 'relative',
  },
  badge: {
    position: 'absolute',
    top: -8,
    right: -8,
    minWidth: 20,
    height: 20,
    borderRadius: 10,
    justifyContent: 'center',
    alignItems: 'center',
    paddingHorizontal: 4,
  },
  badgeText: {
    color: '#FFFFFF',
    fontSize: 10,
    fontFamily: 'Inter-Bold',
  },
});
