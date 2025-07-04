import React, { createContext, useContext, useState, useEffect } from 'react';
import { useAuth } from './AuthContext';
import AsyncStorage from '@react-native-async-storage/async-storage';

export interface Notification {
  id: string;
  userId: string;
  message: string;
  type: 'product_added' | 'product_updated' | 'product_deleted' | 'profile_updated' | "seller_added" | "category_added";
  timestamp: string;
  read: boolean;
}

interface NotifContextType {
  notifications: Notification[];
  addNotification: (message: string, type: Notification['type']) => void;
  markNotificationAsRead: (id: string) => void;
  markAllNotificationsAsRead: () => void;
}

const NotifContext = createContext<NotifContextType | undefined>(undefined);

export function NotifProvider({ children }: { children: React.ReactNode }) {
  const { user } = useAuth();
  const [notifications, setNotifications] = useState<Notification[]>([]);

  // Load persisted data from AsyncStorage on mount
  useEffect(() => {
    loadData();
  }, []);

  // Function to load data from AsyncStorage
  const loadData = async () => {
    try {
      const notificationsData = await AsyncStorage.getItem('notifications');
      if (notificationsData) setNotifications(JSON.parse(notificationsData));
    } catch (error) {
      console.error('Error loading data:', error);
    }
  };

  const saveData = async (key: string, data: any) => {
    try {
      await AsyncStorage.setItem(key, JSON.stringify(data));
    } catch (error) {
      console.error(`Error saving ${key}:`, error);
    }
  };  

  const addNotification = (message: string, type: Notification['type']) => {
    if (!user) return;

    const notification: Notification = {
      id: Date.now().toString(),
      userId: user.id,
      message,
      type,
      timestamp: new Date().toISOString(),
      read: false
    };

    const updatedNotifications = [notification, ...notifications];
    setNotifications(updatedNotifications);
    saveData('notifications', updatedNotifications);
  };

  const markNotificationAsRead = (id: string) => {
    const updatedNotifications = notifications.map(n =>
      n.id === id ? { ...n, read: true } : n
    );
    setNotifications(updatedNotifications);
    saveData('notifications', updatedNotifications);
  };

  const markAllNotificationsAsRead = () => {
    if (!user) return;

    const updatedNotifications = notifications.map(n =>
      n.userId === user.id ? { ...n, read: true } : n
    );
    setNotifications(updatedNotifications);
    saveData('notifications', updatedNotifications);
  };

  return (
    <NotifContext.Provider value={{
      notifications,
      addNotification,
      markNotificationAsRead,
      markAllNotificationsAsRead,
    }}>
      {children}
    </NotifContext.Provider>
  );
}

export function useNotif() {
  const context = useContext(NotifContext);
  if (!context) {
    throw new Error('useData must be used within a DataProvider');
  }
  return context;
}