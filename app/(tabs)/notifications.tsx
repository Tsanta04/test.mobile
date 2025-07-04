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
import { Notification } from '@/contexts/DataContext';
import Header from '@/components/Header';
import NotificationItem from '@/components/Notification/NotificationItem';

const getNotificationIcon = (type: Notification['type']) => {
  switch (type) {
    case 'product_added':
      return Package;
    case 'product_updated':
      return Edit;
    case 'product_deleted':
      return Trash2;
    case 'profile_updated':
      return User;
    default:
      return Bell;
  }
};

const getNotificationColor = (type: Notification['type'], colors: any) => {
  switch (type) {
    case 'product_added':
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

export default function NotificationsScreen() {
  const { notifications } = useData();
  const { user } = useAuth();
  const { colors, theme, toggleTheme } = useTheme();
  const [refreshing, setRefreshing] = React.useState(false);

  const userNotifications = notifications.filter(n => n.userId === user?.id);
  const unreadCount = userNotifications.filter(n => !n.read).length;

  const onRefresh = () => {
    setRefreshing(true);
    setTimeout(() => setRefreshing(false), 1000);
  };

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

  const renderNotification = ({ item: notification }: { item: Notification }) => {
    const IconComponent = getNotificationIcon(notification.type);
    const iconColor = getNotificationColor(notification.type, colors);

    return (
      <NotificationItem
        notification={notification}
        onPress={() => console.log("Hello")}
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
    emptyContainer: {
      flex: 1,
      justifyContent: 'center',
      alignItems: 'center',
      paddingVertical: 60,
    },
    emptyIcon: {
      marginBottom: 16,
      opacity: 0.5,
    },
    emptyText: {
      fontSize: 18,
      fontFamily: 'Inter-SemiBold',
      color: colors.textSecondary,
      textAlign: 'center',
      marginBottom: 8,
    },
    emptySubtext: {
      fontSize: 14,
      fontFamily: 'Inter-Regular',
      color: colors.textSecondary,
      textAlign: 'center',
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
                onPress={()=>{console.log("Clicked");
                }}
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
        <View style={styles.emptyContainer}>
          <Bell size={64} color={colors.textSecondary} style={styles.emptyIcon} />
          <Text style={styles.emptyText}>No notifications yet</Text>
          <Text style={styles.emptySubtext}>
            When you perform actions like adding or editing products,{'\n'}
            you'll see notifications here.
          </Text>
        </View>
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