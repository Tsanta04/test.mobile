import React, { useEffect, useState } from 'react';
import { ProductType } from '@/constants/type';
import ProductForm from '@/components/Form/ProductForm';
import { router, useLocalSearchParams } from 'expo-router';
import { useData } from '@/contexts/DataContext';
import { useTheme } from '@/contexts/ThemeContext';
import { useAuth } from '@/contexts/AuthContext';
import { View } from 'lucide-react-native';
import { Alert, StyleSheet, Text, TouchableOpacity } from 'react-native';

export default function AddProductScreen() {
  const { id } = useLocalSearchParams<{ id: string }>();
  const { products, updateProduct, addCategory, addSeller } = useData();
  const [ isLoading, setIsLoading ] = useState(false);
  const { colors } = useTheme();
  const { user } = useAuth();

  const [formData, setFormData] = useState<ProductType>({
    name: '',
    description: '',
    price: '0',
    stock: '0',
    category: '',
    seller: '',
    image: '',
  });

  const product = products.find(p => p.id === id);

  useEffect(() => {
    if (product) {
      setFormData({
        name: product.name,
        description: product.description,
        price: product.price.toString(),
        stock: product.stock.toString(),
        category: product.category,
        seller: product.seller,
        image: product.image,
      });
    }
  }, [product]);

  const handleUpdate = async () => {
    console.log("Form is valid: ", formData);
    setIsLoading(true);
    try {
      updateProduct(id!, {
        name: formData.name.trim(),
        description: formData.description.trim(),
        price: parseFloat(formData.price),
        stock: parseInt(formData.stock),
        category:formData.category,
        seller:formData.seller,
        image:formData.image,
        isActive: true,
      });
      Alert.alert(
        'Success',
        `Product "${formData.name}" has been updated successfully!`,
        [{ text: 'OK', onPress: () => router.back() }]
      );
    } catch (error) {
      console.log(error);      
      Alert.alert('Error', 'Failed to update product. Please try again.');
    } finally {
      setIsLoading(false);
    }
  };

  const handleAddCategory = (newCategoryName: string) => {
    if (newCategoryName.trim()) {
      console.log("Adding category: ", newCategoryName);      
      addCategory(newCategoryName.trim());      
      setFormData({ ...formData, category: newCategoryName });
    }
  };

  const handleAddSeller = (newSellerName: string) => {
    if (newSellerName.trim()) {
      console.log("Adding seller: ", newSellerName);
      addSeller(newSellerName.trim());
      setFormData({ ...formData, seller: newSellerName });
    }
  };

  const styles = StyleSheet.create({
    container: {
      flex: 1,
      backgroundColor: colors.background,
    },
    header: {
      flexDirection: 'row',
      alignItems: 'center',
      justifyContent: 'space-between',
      paddingTop: 60,
      paddingHorizontal: 20,
      paddingBottom: 16,
    },
    backButton: {
      width: 40,
      height: 40,
      borderRadius: 20,
      backgroundColor: colors.surface,
      justifyContent: 'center',
      alignItems: 'center',
      borderWidth: 1,
      borderColor: colors.border,
    },
    errorContainer: {
      flex: 1,
      justifyContent: 'center',
      alignItems: 'center',
      padding: 20,
    },
    backButtonText: {
      fontSize: 16,
      fontFamily: 'Inter-SemiBold',
    },
    errorText: {
      fontSize: 12,
      fontFamily: 'Inter-Regular',
      color: colors.error,
      marginTop: 4,
    },
  });  

  if (!product) {
    return (
      <View style={[styles.container, { backgroundColor: colors.background }]}>
        <View style={styles.errorContainer}>
          <Text style={[styles.errorText, { color: colors.text }]}>Product not found</Text>
          <TouchableOpacity
            style={styles.backButton}
            onPress={() => router.back()}
          >
            <Text style={[styles.backButtonText, { color: colors.primary }]}>Go Back</Text>
          </TouchableOpacity>
        </View>
      </View>
    );
  }

  if (product.createdBy !== user?.id) {
    return (
      <View style={[styles.container, { backgroundColor: colors.background }]}>
        <View style={styles.errorContainer}>
          <Text style={[styles.errorText, { color: colors.text }]}>You can only edit products you created</Text>
          <TouchableOpacity
            style={styles.backButton}
            onPress={() => router.back()}
          >
            <Text style={[styles.backButtonText, { color: colors.primary }]}>Go Back</Text>
          </TouchableOpacity>
        </View>
      </View>
    );
  }

  return (
    <ProductForm 
      handleSave={handleUpdate} 
      handleAddCategory={handleAddCategory} 
      handleAddSeller={handleAddSeller} 
      formData={formData} 
      setFormData={setFormData} 
      title='Edit Product'
    />
  );
}