// ProductForm component: Handles the creation and editing of product data, including image selection, validation, and form submission.
import React, { useState, useEffect } from 'react';
import {
  View,
  Text,
  TouchableOpacity,
  ScrollView,
  Image,
  Alert,
  StyleSheet,
  KeyboardAvoidingView,
  Platform,
} from 'react-native';
import { LinearGradient } from 'expo-linear-gradient';
import { router } from 'expo-router';
import { useData } from '@/contexts/DataContext';
import { useTheme } from '@/contexts/ThemeContext';
import * as ImagePicker from 'expo-image-picker';
import {
  ArrowLeft,
  Save,
  Camera,
  Image as ImageIcon,
  Link,
  Plus,
  Package,
  DollarSign,
  Hash,
  FileText,
  Tag,
  User,
} from 'lucide-react-native';
import { ProductFormProps } from '@/constants/type';
import SelectionModal from '../SelectionModal';
import SelectForm from './SelectForm';
import ImageUrlModal from './ImageUrlModal';
import InputForm from './InputForm';

export default function ProductForm(
{ 
    handleSave, 
    handleAddCategory, 
    handleAddSeller, 
    formData, 
    setFormData,
    title 
}: ProductFormProps) {

  // Get categories and sellers from context
  const { categories, sellers } = useData();
  // Get theme colors
  const { colors } = useTheme();
  // State for camera, modals, loading, errors, etc.
  const [showImageOptions, setShowImageOptions] = useState(false);
  const [showCategoryModal, setShowCategoryModal] = useState(false);
  const [showSellerModal, setShowSellerModal] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [errors, setErrors] = useState<Record<string, string>>({});
  const [showImageModal,setShowImageModal] = useState<boolean>(false);

  // Styles for the form and its elements
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
    headerTitle: {
      fontSize: 18,
      fontFamily: 'Inter-Bold',
      color: colors.text,
    },
    saveButton: {
      borderRadius: 12,
      overflow: 'hidden',
    },
    saveGradient: {
      paddingHorizontal: 16,
      paddingVertical: 12,
      flexDirection: 'row',
      alignItems: 'center',
    },
    saveButtonText: {
      fontSize: 14,
      fontFamily: 'Inter-SemiBold',
      color: '#000',
      marginLeft: 8,
    },
    scrollContent: {
      flex: 1,
      paddingHorizontal: 20,
    },
    form: {
      gap: 20,
      paddingBottom: 40,
    },
    label: {
      fontSize: 14,
      fontFamily: 'Inter-Medium',
      color: colors.text,
    },
    required: {
      color: colors.error,
    },
    icon: {
      marginRight: 12,
    },
    imageContainer: {
      gap: 8,
    },
    imagePreview: {
      width: '100%',
      height: 200,
      borderRadius: 12,
      backgroundColor: colors.surface,
      borderWidth: 1,
      borderColor: colors.border,
      marginBottom: 12,
    },
    imageButton: {
      backgroundColor: colors.surface,
      borderRadius: 12,
      borderWidth: 1,
      borderColor: colors.border,
      paddingVertical: 16,
      alignItems: 'center',
      justifyContent: 'center',
    },    
    imageButtonText: {
      fontSize: 14,
      fontFamily: 'Inter-SemiBold',
      color: colors.primary,
      marginTop: 8,
    },
    imageOptions: {
      position: 'absolute',
      top: 0,
      left: 0,
      right: 0,
      bottom: 0,
      backgroundColor: 'rgba(0, 0, 0, 0.5)',
      justifyContent: 'center',
      alignItems: 'center',
    },
    imageOptionsCard: {
      backgroundColor: colors.surface,
      borderRadius: 16,
      padding: 24,
      margin: 20,
      gap: 16,
    },
    imageOption: {
      flexDirection: 'row',
      alignItems: 'center',
      paddingVertical: 12,
      paddingHorizontal: 16,
      backgroundColor: colors.background,
      borderRadius: 12,
    },
    imageOptionText: {
      fontSize: 14,
      fontFamily: 'Inter-SemiBold',
      color: colors.text,
      marginLeft: 12,
    },
    cameraOverlay: {
      flex: 1,
      justifyContent: 'space-between',
      alignItems: 'center',
      padding: 40,
    },
    cameraButton: {
      width: 50,
      height: 50,
      borderRadius: 25,
      justifyContent: 'center',
      alignItems: 'center',
      alignSelf: 'flex-start',
    },
    captureButton: {
      width: 80,
      height: 80,
      borderRadius: 40,
      justifyContent: 'center',
      alignItems: 'center',
      alignSelf: 'center',
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
    }    
  });

  // Validate the form fields and set error messages if needed
  const validateForm = () => {
    const newErrors: Record<string, string> = {};

    if (!formData.name.trim()) newErrors.name = 'Product name is required';
    if (!formData.description.trim()) newErrors.description = 'Description is required';
    if (!formData.price || parseFloat(formData.price) <= 0) newErrors.price = 'Valid price is required';
    if (!formData.stock || parseInt(formData.stock) <= 0) newErrors.stock = 'Valid stock quantity is required';
    if (!formData.category) newErrors.category = 'Category is required';
    if (!formData.seller) newErrors.seller = 'Seller is required';
    if (!formData.image) newErrors.image = 'Product image is required';

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  // Handle image selection from camera
  const handleImageFromCamera = async () => {
    const { status } = await ImagePicker.requestCameraPermissionsAsync();
    if (status !== 'granted') {
      Alert.alert('Permission Required', 'Camera permission is required to take photos.');
      return;
    }  
    const result = await ImagePicker.launchCameraAsync({
      mediaTypes: 'images',
      allowsEditing: true,
      aspect: [1, 1],
      quality: 0.7,
    });
    if (!result.canceled && result.assets && result.assets.length > 0) {
      const uri = result.assets[0].uri;
      setFormData({ ...formData, image: uri });
    }
    setShowImageOptions(false);
  };

  // Handle image selection from the gallery
  const handleImageFromGallery = async () => {
    setShowImageOptions(false);
    try {
      const result = await ImagePicker.launchImageLibraryAsync({
        mediaTypes: 'images',
        allowsEditing: true,
        aspect: [4, 3],
        quality: 1,
      });

      if (!result.canceled) {
        setFormData({ ...formData, image: result.assets[0].uri });
      }
    } catch (error) {
      Alert.alert('Error', 'Failed to pick image from gallery.');
    }
  };

  // Show the modal for entering an image URL
  const handleImageFromURL = () => {
    setShowImageOptions(false);
    setShowImageModal(true);
  };

  const save = ()=>{
    if(!validateForm())return;    
    Alert.alert(
      title,
        'Are you sure to pursue this action?',
      [
        { text: 'Cancel', style: 'cancel' },
        { text: 'Confirm', style: 'default', onPress: () => {handleSave()} },
      ]
    );
  }

  // Main render: form layout, modals, and image options
  return (
    <KeyboardAvoidingView
      style={styles.container}
      behavior={Platform.OS === 'ios' ? 'padding' : 'height'}
    >
      {/* Header with back button and save button */}
      <View style={styles.header}>
        <TouchableOpacity
          style={styles.backButton}
          onPress={() => router.back()}
        >
          <ArrowLeft size={20} color={colors.text} />
        </TouchableOpacity>

        <Text style={styles.headerTitle}>{title}</Text>

        <TouchableOpacity
          style={styles.saveButton}
          onPress={save}
          disabled={isLoading}
        >
          <LinearGradient
            colors={[colors.success, '#059669']}
            style={styles.saveGradient}
          >
            <Save size={16} color="#FFF" />
            <Text style={[styles.saveButtonText, { color: '#FFF' }]}> 
              {isLoading ? 'Saving...' : 'Save'}
            </Text>
          </LinearGradient>
        </TouchableOpacity>
      </View>

      {/* Scrollable form content */}
      <ScrollView style={styles.scrollContent} keyboardShouldPersistTaps="handled">
        <View style={styles.form}>
          {/* Product name input */}
          <InputForm
            label="Product Name"
            value={formData.name}
            onChangeText={(text:string) => setFormData({ ...formData, name: text })}
            placeholder="Enter product name"
            error={errors.name}
            colors={colors}
            icon={<Package size={18} color={colors.textSecondary} style={styles.icon} />}
            requiredSign={true}
          />

          {/* Product description input */}
          <InputForm
            label="Description"
            value={formData.description}
            onChangeText={(text:string) => setFormData({ ...formData, description: text })}
            placeholder="Enter product description"
            error={errors.description}
            colors={colors}
            icon={<FileText size={18} color={colors.textSecondary} style={styles.icon} />}
            multiline={true}
            numberOfLines={4}
            requiredSign={true}
          />

          {/* Product price input */}
          <InputForm
            label="Price"
            value={formData.price}
            onChangeText={(text:string) => setFormData({ ...formData, price: text })}
            placeholder="0.00"
            error={errors.price}
            colors={colors}
            icon={<DollarSign size={18} color={colors.textSecondary} style={styles.icon} />}
            keyboardType="decimal-pad"
            requiredSign={true}            
          />

          {/* Product stock input */}
          <InputForm
            label="Stock Quantity"
            value={formData.stock}
            onChangeText={(text:string) => setFormData({ ...formData, stock: text })}
            placeholder="0"
            error={errors.stock}
            colors={colors}
            icon={<Hash size={18} color={colors.textSecondary} style={styles.icon} />}
            keyboardType="number-pad"
            requiredSign={true}            
          />

          {/* Category select button */}
          <SelectForm
            label="Category"
            value={formData.category}
            onPress={() => setShowCategoryModal(true)}
            error={errors.category}
            colors={colors}
            placeholder="Select Category"
            icon={<Tag size={16} color={colors.textSecondary} style={styles.icon} />}
          />

          {/* Seller select button */}
          <SelectForm
            label="Seller"
            value={formData.seller}
            onPress={() => setShowSellerModal(true)}
            error={errors.seller}
            colors={colors}
            placeholder="Select Seller"
            icon={<User size={16} color={colors.textSecondary} style={styles.icon} />}
          />

          {/* Product image section: preview or add button */}
          <View style={styles.imageContainer}>
            <Text style={styles.label}>
              Product Image <Text style={styles.required}>*</Text>
            </Text>
            
            {formData.image ? (
              <TouchableOpacity onPress={() => setShowImageOptions(true)}>
                <Image source={{ uri: formData.image }} style={styles.imagePreview} />
              </TouchableOpacity>
            ) : (
              <TouchableOpacity
                style={styles.imageButton}
                onPress={() => setShowImageOptions(true)}
              >
                <ImageIcon size={32} color={colors.textSecondary} />
                <Text style={styles.imageButtonText}>Add Product Image</Text>
              </TouchableOpacity>
            )}
            {errors.image && <Text style={styles.errorText}>{errors.image}</Text>}
          </View>
        </View>
      </ScrollView>

      {/* Category selection modal with add option */}
      <SelectionModal 
        visible={showCategoryModal} 
        title='Select Category' 
        data={categories} 
        selected={formData.category}
        onSelect={(item) => {
          setFormData({ ...formData, category: item.name });
          setShowCategoryModal(false);
        }}
        onClose={()=>{
          setShowCategoryModal(false);
        }}
        colors={colors}
        enableToAdd={true}
        handleAddNew={handleAddCategory}
      />    

      {/* Seller selection modal with add option */}
      <SelectionModal 
        visible={showSellerModal} 
        title='Select Seller' 
        data={sellers} 
        selected={formData.seller}
        onSelect={(item) => {
          setFormData({ ...formData, seller: item.name });
          setShowSellerModal(false);
        }}
        onClose={()=>{
          setShowSellerModal(false);
        }}
        colors={colors}
        enableToAdd={true}
        handleAddNew={handleAddSeller}
      />    

      {/* Image options modal: choose camera, gallery, or URL */}
      {showImageOptions && (
        <View style={styles.imageOptions}>
          <View style={styles.imageOptionsCard}>
            <TouchableOpacity style={styles.imageOption} onPress={handleImageFromCamera}>
              <Camera size={20} color={colors.primary} />
              <Text style={styles.imageOptionText}>Take Photo</Text>
            </TouchableOpacity>
            
            <TouchableOpacity style={styles.imageOption} onPress={handleImageFromGallery}>
              <ImageIcon size={20} color={colors.secondary} />
              <Text style={styles.imageOptionText}>Choose from Gallery</Text>
            </TouchableOpacity>
            
            <TouchableOpacity style={styles.imageOption} onPress={handleImageFromURL}>
              <Link size={20} color={colors.warning} />
              <Text style={styles.imageOptionText}>Enter URL</Text>
            </TouchableOpacity>
            
            <TouchableOpacity
              style={styles.imageOption}
              onPress={() => setShowImageOptions(false)}
            >
              <Text style={[styles.imageOptionText, { color: colors.error }]}>Cancel</Text>
            </TouchableOpacity>
          </View>
        </View>
      )}

      {/* Modal for entering and validating an image URL */}
      <ImageUrlModal
        visible={showImageModal}
        initialUrl={formData.image}
        colors={colors}
        onClose={() => setShowImageModal(false)}
        onValidate={(url:string) => setFormData({ ...formData, image: url })}
      />      
    </KeyboardAvoidingView>
    
  );
}