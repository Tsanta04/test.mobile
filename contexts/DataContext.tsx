import React, { createContext, useContext, useState, useEffect } from 'react';
import AsyncStorage from '@react-native-async-storage/async-storage';
import { useAuth } from './AuthContext';

// Import JSON data as initial seed
import productsData from '../data/products.json';
import categoriesData from '../data/categories.json';
import sellersData from '../data/sellers.json';


// Define data types / interfaces for type safety
export interface Product {
  id: string;
  name: string;
  description: string;
  price: number;
  stock: number;
  category: string;
  seller: string;
  image: string;
  isActive: boolean;
  createdBy: string;
  createdAt: string;
}

export interface Category {
  id: string;
  name: string;
}

export interface Seller {
  id: string;
  name: string;
}

export interface Notification {
  id: string;
  userId: string;
  message: string;
  type: 'product_added' | 'product_updated' | 'product_deleted' | 'profile_updated';
  timestamp: string;
  read: boolean;
}


// Context interface describing all available methods and data
interface DataContextType {
  products: Product[];
  categories: Category[];
  sellers: Seller[];
  notifications: Notification[];
  addCategory: (name: string) => void;
  addSeller: (name: string) => void;  
  addProduct: (product: Omit<Product, 'id' | 'createdBy' | 'createdAt'>) => void;
  updateProduct: (id: string, product: Omit<Product, 'id' | 'createdBy' | 'createdAt'>) => void;
  deleteProduct: (id: string) => void;
  getUserProducts: () => Product[];
}


// Create Context
const DataContext = createContext<DataContextType | undefined>(undefined);

// DataProvider component to wrap app and provide state
export function DataProvider({ children }: { children: React.ReactNode }) {
  const { user } = useAuth();

  // States for products, categories, sellers, notifications
  const [products, setProducts] = useState<Product[]>(productsData as Product[]);
  const [categories, setCategories] = useState<Category[]>(categoriesData as Category[]);
  const [sellers, setSellers] = useState<Seller[]>(sellersData as Seller[]);
  const [notifications, setNotifications] = useState<Notification[]>([]);

  // Load persisted data from AsyncStorage on mount
  useEffect(() => {
    loadData();
  }, []);

  // Function to load data from AsyncStorage
  const loadData = async () => {
    try {
      const [storedProducts, storedCategories, storedSellers, notificationsData] = await Promise.all([
        AsyncStorage.getItem('products'),
        AsyncStorage.getItem('categories'),
        AsyncStorage.getItem('sellers'),
        AsyncStorage.getItem('notifications')
      ]);

      if (storedProducts) setProducts(JSON.parse(storedProducts));
      if (storedCategories) setCategories(JSON.parse(storedCategories));
      if (storedSellers) setSellers(JSON.parse(storedSellers));
      if (notificationsData) setNotifications(JSON.parse(notificationsData));
    } catch (error) {
      console.error('Error loading data:', error);
    }
  };

  // Helper to save data to AsyncStorage
  const saveData = async (key: string, data: any) => {
    try {
      await AsyncStorage.setItem(key, JSON.stringify(data));
    } catch (error) {
      console.error(`Error saving ${key}:`, error);
    }
  };

  // Add new product (createdBy current user)
  const addProduct = (productData: Omit<Product, 'id' | 'createdBy' | 'createdAt'>) => {
    if (!user) return;

    const newProduct: Product = {
      ...productData,
      id: Date.now().toString(),
      createdBy: user.id,
      createdAt: new Date().toISOString()
    };

    const updatedProducts = [...products, newProduct];
    setProducts(updatedProducts);
    saveData('products', updatedProducts);

  };

  // Update existing product by ID
  const updateProduct = (id: string, productData: Omit<Product, 'id' | 'createdBy' | 'createdAt'>) => {
    if (!user) return;

    const updatedProducts = products.map(p => 
      p.id === id ? { ...p, ...productData } : p
    );
    setProducts(updatedProducts);
    saveData('products', updatedProducts);

  };

  // Delete product by ID
  const deleteProduct = (id: string) => {
    const product = products.find(p => p.id === id);
    if (!product) return;

    const updatedProducts = products.filter(p => p.id !== id);
    setProducts(updatedProducts);
    saveData('products', updatedProducts);

  };

  // Add new category  
  const addCategory = (name: string) => {
    const newCategory: Category = {
      id: Date.now().toString(),
      name
    };
    const updatedCategories = [...categories, newCategory];
    setCategories(updatedCategories);
    saveData('categories', updatedCategories);
  };

  // Add new seller  
  const addSeller = (name: string) => {
    const newSeller: Seller = {
      id: Date.now().toString(),
      name
    };
    const updatedSellers = [...sellers, newSeller];
    setSellers(updatedSellers);
    saveData('sellers', updatedSellers);
  };


  // Get all products created by the current user
  const getUserProducts = () => {
    if (!user) return [];
    return products.filter(p => p.createdBy === user.id);
  };

  return (
    <DataContext.Provider value={{
      products,
      categories,
      sellers,
      notifications,
      addProduct,
      updateProduct,
      deleteProduct,
      addCategory,
      addSeller,
      getUserProducts
    }}>
      {children}
    </DataContext.Provider>
  );
}

// Custom hook to access DataContext
export function useData() {
  const context = useContext(DataContext);
  if (!context) {
    throw new Error('useData must be used within a DataProvider');
  }
  return context;
}
