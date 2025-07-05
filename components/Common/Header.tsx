import React from 'react';
import { View, Text, TouchableOpacity, StyleSheet } from 'react-native';
import { LinearGradient } from 'expo-linear-gradient';
import { Moon, Sun, Filter } from 'lucide-react-native';

// Custom type for theme colors
import { ColorType } from '@/constants/type';

// Define props for the Header component
interface HeaderProps {
  title: string;                 
  theme: 'light' | 'dark';     
  toggleTheme: () => void;      
  onToggleFilters?: () => void; 
  colors: ColorType;            
}

/**
 * Header Component
 * A customizable header with a title, theme toggle button, and optional filter button.
 */
const Header: React.FC<HeaderProps> = ({
  title,
  theme,
  toggleTheme,
  onToggleFilters,
  colors
}) => {

  // Define dynamic styles using the theme colors
  const styles = StyleSheet.create({
    container: {
      marginHorizontal: 10,
      marginTop: 48,
      marginBottom: 20,
      backgroundColor: colors.background,
    },
    header: {
      paddingTop: 60,
      paddingHorizontal: 20,
      paddingBottom: 16,
    },
    headerGradient: {
      paddingVertical: 20,
      paddingHorizontal: 20,
      flexDirection: 'row',
      justifyContent: 'space-between',
      alignItems: 'center',
      borderRadius: 16,
    },
    headerTitle: {
      fontSize: 24,
      fontFamily: 'Inter-Bold',
      color: '#000',
    },
    headerActions: {
      flexDirection: 'row',
      gap: 12,
    },
    headerButton: {
      width: 36,
      height: 36,
      borderRadius: 18,
      backgroundColor: 'rgba(0, 0, 0, 0.1)',
      justifyContent: 'center',
      alignItems: 'center',
    },
  });

  return (
    <View style={styles.container}>
      <LinearGradient
        colors={[colors.primary, colors.secondary]}
        style={styles.headerGradient}
      >
        {/* Header title */}
        <Text style={styles.headerTitle}>{title}</Text>
        <View style={styles.headerActions}>
          <TouchableOpacity
            style={styles.headerButton}
            onPress={toggleTheme}
          >
            {theme === 'light' ? (
              <Moon size={18} color="#000" />
            ) : (
              <Sun size={18} color="#000" />
            )}
          </TouchableOpacity>

          {/* Show filter button only for 'Products' screen and if handler exists */}
          {title === 'Products' && onToggleFilters && (
            <TouchableOpacity
              style={styles.headerButton}
              onPress={onToggleFilters}
            >
              <Filter size={18} color="#000" />
            </TouchableOpacity>
          )}
        </View>
      </LinearGradient>
    </View>
  );
};

export default Header;
