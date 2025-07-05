import React from 'react';
import {
  View,
  Text,
  FlatList,
  TouchableOpacity,
  StyleSheet,
  RefreshControl,
} from 'react-native';
import { LinearGradient } from 'expo-linear-gradient';
import { useData } from '@/contexts/DataContext';
import { useAuth } from '@/contexts/AuthContext';
import { useTheme } from '@/contexts/ThemeContext';
import { Bell, Package, CreditCard as Edit, Trash2, User, Check, CheckCheck, Moon, Sun } from 'lucide-react-native';
import Header from '@/components/Header';
import NotificationItem from '@/components/Notification/NotificationItem';
import { Notification, useNotif } from '@/contexts/NotifContext';
import { EmptyDataMessage } from '@/components/Common/EmptyDataMessage';

// NotificationsScreen: Displays a list of notifications for the current user, with mark-as-read and mark-all-read functionality
export default function NotificationsScreen() {
  // Get notifications, user, and theme from context
  const { notifications, markNotificationAsRead, markAllNotificationsAsRead } = useNotif();
  const { user } = useAuth();
  const { colors, theme, toggleTheme } = useTheme();
  const [refreshing, setRefreshing] = React.useState(false);

  // Filter notifications for the current user
  const userNotifications = notifications.filter(n => n.userId === user?.id);
  // Count unread notifications
  const unreadCount = userNotifications.filter(n => !n.read).length;

  // Handler for pull-to-refresh
  const onRefresh = () => {
    setRefreshing(true);
    setTimeout(() => setRefreshing(false), 1000);
  };

  // Format the notification timestamp as a relative time string
  const formatTime = (timestamp: string) => {
    const date = new Date(timestamp);
    const now = new Date();
    const diffInHours = Math.floor((now.getTime() - date.getTime()) / (1000 * 60 * 60));
    
    if (diffInHours < 1) {
      const diffInMinutes = Math.floor((now.getTime() - date.getTime()) / (1000 * 60));
      return diffInMinutes < 1 ? 'Just now' : `${diffInMinutes}m ago`;
    } else if (diffInHours < 24) {
      return `${diffInHours}h ago`;
    } else {
      const diffInDays = Math.floor(diffInHours / 24);
      return `${diffInDays}d ago`;
    }
  };

  // Render a single notification item
  const renderNotification = ({ item: notification }: { item: Notification }) => {
    const IconComponent = getNotificationIcon(notification.type);
    const iconColor = getNotificationColor(notification.type, colors);

    return (
      <NotificationItem
        notification={notification}
        onPress={() => markNotificationAsRead(notification.id)}
        IconComponent={IconComponent}
        iconColor={iconColor}
        colors={colors}
        formatTime={formatTime}
      />
    );
  };

  const styles = StyleSheet.create({
    container: {
      flex: 1,
      backgroundColor: colors.background,
    },
    statsContainer: {
      paddingHorizontal: 20,
      paddingBottom: 16,
    },
    statsCard: {
      borderRadius: 12,
      borderWidth: 1,
      borderColor: colors.border,
      padding: 16,
      flexDirection: 'row',
      justifyContent: 'space-between',
      alignItems: 'center',
    },
    statsText: {
      fontSize: 16,
      fontFamily: 'Inter-SemiBold',
      color: colors.text,
    },
    markAllButton: {
      borderRadius: 8,
      overflow: 'hidden',
    },
    markAllGradient: {
      paddingHorizontal: 12,
      paddingVertical: 8,
      flexDirection: 'row',
      alignItems: 'center',
    },
    markAllText: {
      fontSize: 14,
      fontFamily: 'Inter-SemiBold',
      color: '#000',
      marginLeft: 4,
    },
    notificationsList: {
      flex: 1,
      paddingHorizontal: 20,
    },
    emptyIcon: {
      marginBottom: 16,
      opacity: 0.5,
    },
  });

  return (
    <View style={styles.container}>
      <Header
        title="Notifications"
        theme={theme}
        toggleTheme={toggleTheme}
        colors={colors}
      />

      {userNotifications.length > 0 && (
        <View style={styles.statsContainer}>
          <View style={[styles.statsCard, { backgroundColor: colors.surface }]}>
            <Text style={styles.statsText}>
              {unreadCount} unread of {userNotifications.length} total
            </Text>
            {unreadCount > 0 && (
              <TouchableOpacity
                style={styles.markAllButton}
                onPress={markAllNotificationsAsRead}
              >
                <LinearGradient
                  colors={[colors.primary, colors.secondary]}
                  style={styles.markAllGradient}
                >
                  <CheckCheck size={16} color="#000" />
                  <Text style={styles.markAllText}>Mark All Read</Text>
                </LinearGradient>
              </TouchableOpacity>
            )}
          </View>
        </View>
      )}

      {userNotifications.length === 0 ? (
        <EmptyDataMessage
          message="No notifications yet"
          subtext="When you perform actions like adding or editing products,you'll see notifications here."
          icon = { <Bell size={64} color={colors.textSecondary} style={styles.emptyIcon} /> }
          colors={colors}
        />
      ) : (
        <FlatList
          data={userNotifications}
          renderItem={renderNotification}
          keyExtractor={(item) => item.id}
          style={styles.notificationsList}
          refreshControl={
            <RefreshControl
              refreshing={refreshing}
              onRefresh={onRefresh}
              tintColor={colors.primary}
            />
          }
        />
      )}
    </View>
  );
}

// Helper to get the correct icon for each notification type
const getNotificationIcon = (type: Notification['type']) => {
  switch (type) {
    case 'product_added':
      return Package;
    case 'product_updated':
      return Edit;
    case 'category_added':
      return Package;
    case 'seller_added':
      return Package;
    case 'product_deleted':      
      return Trash2;
    case 'profile_updated':
      return User;
    default:
      return Bell;
  }
};

// Helper to get the correct color for each notification type
const getNotificationColor = (type: Notification['type'], colors: any) => {
  switch (type) {
    case 'product_added':
      return colors.success;
    case 'category_added':
      return colors.success;      
    case 'seller_added':
      return colors.success;
    case 'product_updated':
      return colors.warning;
    case 'product_deleted':
      return colors.error;
    case 'profile_updated':
      return colors.primary;
    default:
      return colors.textSecondary;
  }
};