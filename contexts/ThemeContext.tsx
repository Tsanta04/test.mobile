import React, { createContext, useContext, useState, useEffect } from 'react';
import AsyncStorage from '@react-native-async-storage/async-storage';           // pour sauvegarder le thème entre les sessions

type Theme = 'light' | 'dark';

// La forme du contexte de thème
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

// Palette de couleurs pour le mode clair
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

// Palette de couleurs pour le mode sombre
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

// Création du contexte avec undefined par défaut
const ThemeContext = createContext<ThemeContextType | undefined>(undefined);

/**
 * Composant ThemeProvider
 * - Englobe l'application pour fournir le contexte de thème
 */
export function ThemeProvider({ children }: { children: React.ReactNode }) {
  const [theme, setTheme] = useState<Theme>('light'); // Thème par défaut : clair

  // Chargement du thème sauvegardé au premier rendu
  useEffect(() => {
    loadTheme();
  }, []);

  // Fonction pour charger la valeur du thème depuis AsyncStorage
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

  // Fonction pour basculer entre les thèmes et sauvegarder la sélection
  const toggleTheme = async () => {
    const newTheme = theme === 'light' ? 'dark' : 'light';
    setTheme(newTheme);
    try {
      await AsyncStorage.setItem('theme', newTheme);
    } catch (error) {
      console.error('Erreur lors de la sauvegarde du thème :', error);
    }
  };

  // Choix de la palette de couleurs en fonction du thème actif
  const colors = theme === 'light' ? lightColors : darkColors;

  return (
    <ThemeContext.Provider value={{ theme, toggleTheme, colors }}>
      {children}
    </ThemeContext.Provider>
  );
}

/**
 * Hook personnalisé useTheme
 * - Permet à n'importe quel composant d'accéder au contexte du thème
 * - Lève une erreur si utilisé en dehors du ThemeProvider
 */
export function useTheme() {
  const context = useContext(ThemeContext);
  if (!context) {
    throw new Error('useTheme doit être utilisé dans un ThemeProvider');
  }
  return context;
}
