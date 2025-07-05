import React, { createContext, useContext, useState, useEffect } from 'react';
import AsyncStorage from '@react-native-async-storage/async-storage';
import { useLoading } from './LoadingContext';
import usersData from '../data/users.json';                                   // Import local JSON data representing initial users
import bcrypt from 'bcryptjs';

// Define the User interface
export interface User {
  id: string;
  email: string;
  name: string;
  password: string;
}

// Define the context shape
interface AuthContextType {
  user: User | null;
  isLoading: boolean;
  login: (email: string, password: string) => Promise<boolean>;
  register: (name: string, email: string, password: string) => Promise<boolean>;
  logout: () => Promise<void>;
  changePassword: (email: string, newPassword: string) => Promise<boolean>;
  updateProfile: (name: string, email: string) => Promise<boolean>;
}

// Generate crypt random
bcrypt.setRandomFallback((len) => {
  const buf = [];
  for (let i = 0; i < len; i++) {
    // Remplace ça par une vraie source de random si tu veux
    buf.push(Math.floor(Math.random() * 256));
  }
  return buf;
});

// Create the AuthContext with the defined type
const AuthContext = createContext<AuthContextType | undefined>(undefined);

// AuthProvider component to wrap the app and provide authentication state
export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [user, setUser] = useState<User | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [users, setUsers] = useState<User[]>(usersData as User[]);
  const { showLoading, hideLoading } = useLoading();

  useEffect(() => {
    loadUser();
  }, []);

  /**
   * Loads user data from AsyncStorage if present
   */
  const loadUser = async () => {
    showLoading('Loading user session...');
    try {
      const userData = await AsyncStorage.getItem('user');
      if (userData) {
        setUser(JSON.parse(userData));
      }
    } catch (error) {
      console.error('Error loading user:', error);
    } finally {
      setIsLoading(false);
      hideLoading();
    }
  };

  /**
   * Login function: finds a user matching the given credentials
   * and stores them in AsyncStorage
   */
  const login = async (email: string, password: string): Promise<boolean> => {
    showLoading('Signing in...');
    try {
      const foundUser = users.find(u => u.email === email);
      if (!foundUser) return false;
  

      const isPasswordValid = await bcrypt.compare(password, foundUser.password);
      if (!isPasswordValid) return false;
  
      const userWithoutPassword = { ...foundUser, password: '' };
      setUser(userWithoutPassword);
      await AsyncStorage.setItem('user', JSON.stringify(userWithoutPassword));
      return true;
    } catch (error) {
      console.error('Error during login:', error);
      return false;
    } finally {
      hideLoading();
    }
  };

  /**
   * Register function: creates a new user if email is unique
   * Adds the user to the local list
   */
  const register = async (name: string, email: string, password: string): Promise<boolean> => {
    showLoading('Creating account...');
    try {
      // Check for existing email
      if (users.some(u => u.email === email)) {
        return false;
      }

      const hashedPassword = await bcrypt.hash(password, 10);
      
      // Create new user
      const newUser: User = {
        id: Date.now().toString(),
        name,
        email,
        password: hashedPassword
      };

      // Add new user to local users list
      setUsers(prev => [...prev, newUser]);
      return true;
    } catch (error) {
      console.error('Error during registration:', error);
      return false;
    } finally {
      hideLoading();
    }
  };

  /**
   * Logout function: clears the user state and AsyncStorage
   */
  const logout = async () => {
    showLoading('Signing out...');
    try {
      setUser(null);
      await AsyncStorage.removeItem('user');
    } catch (error) {
      console.error('Error during logout:', error);
    } finally {
      hideLoading();
    }
  };

  /**
   * Updates the user's profile (name and email)
   * Checks for email uniqueness
   */
  const updateProfile = async (name: string, email: string): Promise<boolean> => {
    if (!user) return false;

    showLoading('Updating profile...');
    try {
      // Ensure the new email is not taken by another user
      if (users.some(u => u.email === email && u.id !== user.id)) {
        return false;
      }

      // Update user info in state and storage
      const updatedUser = { ...user, name, email };
      setUser(updatedUser);
      setUsers(prev => prev.map(u => u.id === user.id ? { ...u, name, email } : u));
      await AsyncStorage.setItem('user', JSON.stringify(updatedUser));
      return true;
    } catch (error) {
      console.error('Error updating profile:', error);
      return false;
    } finally {
      hideLoading();
    }
  };

  const changePassword = async (email: string, newPassword: string): Promise<boolean> => {
    const foundUser = users.find(u => u.email === email);

    showLoading('Updating password...');
    try {
      // If there is no user with this email
      if(!foundUser) return false;

      // Hash the new password
      console.log(newPassword);
      const hashedPassword = await bcrypt.hash(newPassword, 10);      
      console.log(hashedPassword);
            
      // Update user info in state and storage
      const updatedUser = { ...foundUser, password: hashedPassword };
      setUsers(prev => prev.map(u => u.id === foundUser.id ? { ...u, password: hashedPassword } : u));
      
      // Only update AsyncStorage if this is the currently logged in user
      if (user && user.id === foundUser.id) {
        const userWithoutPassword = { ...updatedUser, password: '' };
        setUser(userWithoutPassword);
        await AsyncStorage.setItem('user', JSON.stringify(userWithoutPassword));
      }
      
      return true;
    } catch (error) {
      console.error('Error updating password:', error);
      return false;
    } finally {
      hideLoading();
    }
  };  

  // Provide the context to children components
  return (
    <AuthContext.Provider value={{
      user,
      isLoading,
      login,
      register,
      logout,
      changePassword,
      updateProfile
    }}>
      {children}
    </AuthContext.Provider>
  );
}

/**
 * useAuth custom hook to consume AuthContext
 * Throws an error if used outside AuthProvider
 */
export function useAuth() {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
}
