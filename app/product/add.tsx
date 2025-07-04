import React, { useState } from 'react';
import { ProductType } from '@/constants/type';
import ProductForm from '@/components/Form/ProductForm';
import { Alert } from 'react-native';
import { router } from 'expo-router';
import { useData } from '@/contexts/DataContext';

export default function AddProductScreen() {
  const [isLoading, setIsLoading] = useState(false);
  const {addProduct} = useData();
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
    setIsLoading(true);
    try {
      addProduct({
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
    } finally {
      setIsLoading(false);
    }    
  };

  const handleAddCategory = (newCategoryName: string) => {
    if (newCategoryName.trim()) {
      console.log("Adding category: ", newCategoryName);      
      setFormData({ ...formData, category: newCategoryName });
    }
  };

  const handleAddSeller = (newSellerName: string) => {
    if (newSellerName.trim()) {
      console.log("Adding seller: ", newSellerName);
      setFormData({ ...formData, seller: newSellerName });
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