import React, { useState } from 'react';
import { ProductType } from '@/constants/type';
import ProductForm from '@/components/Form/ProductForm';

export default function AddProductScreen() {
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