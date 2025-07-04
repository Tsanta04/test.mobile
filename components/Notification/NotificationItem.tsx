import React from 'react';
import { View, Text, TouchableOpacity, StyleSheet, StyleProp, ViewStyle, TextStyle } from 'react-native';
import { ColorType } from '@/constants/type';

interface Notification {
  id: string;
  message: string;
  timestamp: string;
  read: boolean;
}

// NotificationItem: Displays a single notification with icon, message, time, and read/unread state
interface NotificationItemProps {
  notification: Notification;
  onPress: () => void;
  IconComponent: React.ElementType;
  iconColor: string;
  colors: ColorType;
  formatTime: (timestamp: string) => string;
  style?: StyleProp<ViewStyle>;
}

const NotificationItem: React.FC<NotificationItemProps> = ({
  notification,
  onPress,
  IconComponent,
  iconColor,
  colors,
  formatTime,
  style,
}) => {

    // Styles for the notification item and its elements
    const styles = StyleSheet.create({
        notificationItem: {
            flexDirection: 'row',
            padding: 16,
            borderRadius: 12,
            borderWidth: 1,
            marginBottom: 12,
            alignItems: 'flex-start',
        },
        iconContainer: {
            width: 40,
            height: 40,
            borderRadius: 20,
            justifyContent: 'center',
            alignItems: 'center',
            marginRight: 12,
        },
        notificationContent: {
            flex: 1,
        },
        notificationMessage: {
            fontSize: 16,
            lineHeight: 22,
            marginBottom: 4,
        },
        notificationTime: {
            fontSize: 12,
            fontFamily: 'Inter-Regular',
        },
        unreadDot: {
            width: 8,
            height: 8,
            borderRadius: 4,
            marginLeft: 8,
            marginTop: 8,
        }
    });
    

  return (
    // Touchable notification item with icon, message, time, and unread dot
    <TouchableOpacity
      style={[
        styles.notificationItem,
        {
          backgroundColor: notification.read ? colors.surface : colors.primary + '10',
          borderColor: colors.border,
        },
        style
      ]}
      onPress={onPress}
    >
      <View style={[styles.iconContainer, { backgroundColor: iconColor + '20' }]}>
        <IconComponent size={20} color={iconColor} />
      </View>

      <View style={styles.notificationContent}>
        <Text
          style={[
            styles.notificationMessage,
            {
              color: colors.text,
              fontFamily: notification.read ? 'Inter-Regular' : 'Inter-SemiBold',
            },
          ]}
        >
          {notification.message}
        </Text>
        <Text style={[styles.notificationTime, { color: colors.textSecondary }]}>
          {formatTime(notification.timestamp)}
        </Text>
      </View>

      {!notification.read && (
        <View style={[styles.unreadDot, { backgroundColor: colors.primary }]} />
      )}
    </TouchableOpacity>
  );
};

export default NotificationItem;
