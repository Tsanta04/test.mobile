import React, { useState } from 'react';
import { ProductType } from '@/constants/type';
import ProductForm from '@/components/Form/ProductForm';
import { Alert } from 'react-native';
import { router } from 'expo-router';
import { useData } from '@/contexts/DataContext';

export default function AddProductScreen() {
  const {addProduct, addCategory, addSeller} = useData();
  const [formData, setFormData] = useState<ProductType>({
    name: '',
    description: '',
    price: '0',
    stock: '0',
    category: '',
    seller: '',
    image: '',
  });

  const handleSave = async () => {
    console.log("Form is valid: ", formData);
    try {
      await addProduct({
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
        `Product "${formData.name}" has been added successfully!`,
        [{ text: 'OK', onPress: () => router.back() }]
      );
    } catch (error) {
      Alert.alert('Error', 'Failed to add product. Please try again.');
    }    
  };

  const handleAddCategory = async (newCategoryName: string) => {
    if (newCategoryName.trim()) {
      try {
        console.log("Adding category: ", newCategoryName);      
        await addCategory(newCategoryName.trim());      
        setFormData({ ...formData, category: newCategoryName });
      } catch (error) {
        Alert.alert('Error', 'Failed to add category. Please try again.');
      }
    }
  };

  const handleAddSeller = async (newSellerName: string) => {
    if (newSellerName.trim()) {
      try {
        console.log("Adding seller: ", newSellerName);
        await addSeller(newSellerName.trim());
        setFormData({ ...formData, seller: newSellerName });
      } catch (error) {
        Alert.alert('Error', 'Failed to add seller. Please try again.');
      }
    }
  };

  return (
    <ProductForm 
      handleSave={handleSave} 
      handleAddCategory={handleAddCategory} 
      handleAddSeller={handleAddSeller} 
      formData={formData} 
      setFormData={setFormData} 
      title='Add Product'
    />
  );
}