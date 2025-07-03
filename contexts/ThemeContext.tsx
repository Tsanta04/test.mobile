import React, { createContext, useContext, useState, useEffect } from 'react';
import AsyncStorage from '@react-native-async-storage/async-storage';           // to persist theme selection across app launches

type Theme = 'light' | 'dark';

// Define allowed theme types
interface ThemeContextType {
  theme: Theme;                   
  toggleTheme: () => void;
  colors: {                       
    background: string;
    surface: string;
    primary: string;
    secondary: string;
    text: string;
    textSecondary: string;
    border: string;
    error: string;
    success: string;
    warning: string;
  };
}

// Define color palette for Light Mode
const lightColors = {
  background: '#FFFFFF',      
  surface: '#E2E8F0',         
  primary: '#FFD700',         
  secondary: '#FACC15',       
  text: '#0F172A',            
  textSecondary: '#64748B',   
  border: '#CBD5E1',          
  error: '#EF4444',           
  success: '#10B981',         
  warning: '#F59E0B',         
};

// Define color palette for Dark Mode
const darkColors = {
  background: '#0F172A',      
  surface: '#1E293B',         
  primary: '#FFD700',         
  secondary: '#FACC15',       
  text: '#FFFFFF',            
  textSecondary: '#94A3B8',  
  border: '#334155',         
  error: '#F87171',          
  success: '#34D399',         
  warning: '#FBBF24',      
};

// Create a context with undefined as default value
const ThemeContext = createContext<ThemeContextType | undefined>(undefined);

/**
 * ThemeProvider component wraps the entire app to provide theme context
 */
export function ThemeProvider({ children }: { children: React.ReactNode }) {
  const [theme, setTheme] = useState<Theme>('light'); // Thème par défaut : clair

  // Load saved theme from AsyncStorage on first render
  useEffect(() => {
    loadTheme();
  }, []);

  // Async function to load persisted theme value
  const loadTheme = async () => {
    try {
      const savedTheme = await AsyncStorage.getItem('theme');
      if (savedTheme && (savedTheme === 'light' || savedTheme === 'dark')) {
        setTheme(savedTheme);
      }
    } catch (error) {
      console.error('Erreur lors du chargement du thème :', error);
    }
  };

  // Toggle between light and dark themes, and persist selection
  const toggleTheme = async () => {
    const newTheme = theme === 'light' ? 'dark' : 'light';
    setTheme(newTheme);
    try {
      await AsyncStorage.setItem('theme', newTheme);
    } catch (error) {
      console.error('Erreur lors de la sauvegarde du thème :', error);
    }
  };

  // Determine the color palette based on current theme
  const colors = theme === 'light' ? lightColors : darkColors;

  return (
    <ThemeContext.Provider value={{ theme, toggleTheme, colors }}>
      {children}
    </ThemeContext.Provider>
  );
}

/**
 * useTheme custom hook
 * - Allows any component to access the theme context
 * - Throws an error if used outside of ThemeProvider
 */
export function useTheme() {
  const context = useContext(ThemeContext);
  if (!context) {
    throw new Error('useTheme doit être utilisé dans un ThemeProvider');
  }
  return context;
}
