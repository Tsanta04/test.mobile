import React from 'react';
import {
  View,
  Text,
  Image,
  TouchableOpacity,
  ScrollView,
  Alert,
  StyleSheet,
} from 'react-native';
import { useLocalSearchParams, router } from 'expo-router';
import { LinearGradient } from 'expo-linear-gradient';
import { useData } from '@/contexts/DataContext';
import { useAuth } from '@/contexts/AuthContext';
import { useTheme } from '@/contexts/ThemeContext';
import { ArrowLeft, CreditCard as Edit3, Trash2, Package, DollarSign, User as UserIcon, Tag, Calendar } from 'lucide-react-native';
import ButtonForm from '@/components/Form/ButtonForm';

/**
 * ProductDetailScreen - Displays detailed information about a specific product
 * Shows product image, details, price, stock, and owner actions (edit/delete)
 */
export default function ProductDetailScreen() {
  // Get product ID from URL parameters
  const { id } = useLocalSearchParams<{ id: string }>();
  
  // Access data context for products and delete functionality
  const { products, deleteProduct } = useData();
  
  // Access authentication context to get current user
  const { user } = useAuth();
  
  // Access theme context for dynamic colors
  const { colors } = useTheme();

  // Find the specific product by ID
  const product = products.find(p => p.id === id);

  /**
   * Dynamic styles that adapt to the current theme
   * All colors are pulled from the theme context for consistency
   */
  const styles = StyleSheet.create({
    // Main container with theme background
    container: {
      flex: 1,
      backgroundColor: colors.background,
    },
    
    // Header section with back button and action buttons
    header: {
      flexDirection: 'row',
      alignItems: 'center',
      justifyContent: 'space-between',
      paddingTop: 60, // Account for status bar
      paddingHorizontal: 20,
      paddingBottom: 16,
    },
    
    // Circular back button with theme styling
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
    
    // Container for edit/delete action buttons (owner only)
    headerActions: {
      flexDirection: 'row',
      gap: 12,
    },
    
    // Circular action buttons (edit/delete)
    actionButton: {
      width: 40,
      height: 40,
      borderRadius: 20,
      justifyContent: 'center',
      alignItems: 'center',
    },
    
    // Scrollable content area
    scrollContent: {
      flex: 1,
    },
    
    // Product image container with rounded corners
    imageContainer: {
      height: 300,
      marginHorizontal: 20,
      borderRadius: 16,
      overflow: 'hidden',
      marginBottom: 24,
    },
    
    // Product image styling
    productImage: {
      width: '100%',
      height: '100%',
    },
    
    // Main content container with padding
    contentContainer: {
      paddingHorizontal: 20,
      paddingBottom: 40,
    },
    
    // Product title and description section
    productHeader: {
      marginBottom: 24,
    },
    
    // Large product name with bold font
    productName: {
      fontSize: 28,
      fontFamily: 'Inter-Bold',
      color: colors.text,
      marginBottom: 8,
    },
    
    // Product description with secondary text color
    productDescription: {
      fontSize: 16,
      fontFamily: 'Inter-Regular',
      color: colors.textSecondary,
      lineHeight: 24,
    },
    
    // Price and stock information card
    priceSection: {
      flexDirection: 'row',
      justifyContent: 'space-between',
      alignItems: 'center',
      backgroundColor: colors.surface,
      borderRadius: 16,
      padding: 20,
      marginBottom: 24,
      borderWidth: 1,
      borderColor: colors.border,
    },
    
    // Left side of price section (price info)
    priceContainer: {
      flex: 1,
    },
    
    // Price label text
    priceLabel: {
      fontSize: 14,
      fontFamily: 'Inter-Regular',
      color: colors.textSecondary,
      marginBottom: 4,
    },
    
    // Large price value with primary color
    priceValue: {
      fontSize: 32,
      fontFamily: 'Inter-Bold',
      color: colors.primary,
    },
    
    // Right side of price section (stock info)
    stockContainer: {
      alignItems: 'flex-end',
    },
    
    // Stock label text
    stockLabel: {
      fontSize: 14,
      fontFamily: 'Inter-Regular',
      color: colors.textSecondary,
      marginBottom: 4,
    },
    
    // Stock value with success color
    stockValue: {
      fontSize: 24,
      fontFamily: 'Inter-Bold',
      color: colors.success,
    },
    
    // Product details card (category, seller, date)
    detailsSection: {
      backgroundColor: colors.surface,
      borderRadius: 16,
      padding: 20,
      marginBottom: 24,
      borderWidth: 1,
      borderColor: colors.border,
    },
    
    // Section title for product details
    sectionTitle: {
      fontSize: 20,
      fontFamily: 'Inter-Bold',
      color: colors.text,
      marginBottom: 16,
    },
    
    // Individual detail item row
    detailItem: {
      flexDirection: 'row',
      alignItems: 'center',
      paddingVertical: 12,
      borderBottomWidth: 1,
      borderBottomColor: colors.border,
    },
    
    // Last detail item (no bottom border)
    detailItemLast: {
      borderBottomWidth: 0,
    },
    
    // Icon container for detail items
    detailIcon: {
      marginRight: 16,
    },
    
    // Content area for detail items
    detailContent: {
      flex: 1,
    },
    
    // Label text for detail items
    detailLabel: {
      fontSize: 14,
      fontFamily: 'Inter-Regular',
      color: colors.textSecondary,
      marginBottom: 2,
    },
    
    // Value text for detail items
    detailValue: {
      fontSize: 16,
      fontFamily: 'Inter-SemiBold',
      color: colors.text,
    },
    
    // Container for owner action buttons
    ownerActions: {
      gap: 12,
    },
    
    // Edit button with gradient background
    editButton: {
      borderRadius: 12,
      overflow: 'hidden',
    },
    
    // Gradient background for edit button
    editGradient: {
      paddingVertical: 16,
      paddingHorizontal: 24,
      flexDirection: 'row',
      alignItems: 'center',
      justifyContent: 'center',
    },
    
    // Edit button text
    editButtonText: {
      fontSize: 16,
      fontFamily: 'Inter-SemiBold',
      color: '#000',
      marginLeft: 8,
    },
    
    // Delete button with gradient background
    deleteButton: {
      borderRadius: 12,
      overflow: 'hidden',
    },
    
    // Gradient background for delete button
    deleteGradient: {
      paddingVertical: 16,
      paddingHorizontal: 24,
      flexDirection: 'row',
      alignItems: 'center',
      justifyContent: 'center',
    },
    
    // Delete button text
    deleteButtonText: {
      fontSize: 16,
      fontFamily: 'Inter-SemiBold',
      color: '#FFF',
      marginLeft: 8,
    },
    
    // Error state container (product not found)
    errorContainer: {
      flex: 1,
      justifyContent: 'center',
      alignItems: 'center',
      padding: 20,
    },
    
    // Error message text
    errorText: {
      fontSize: 18,
      fontFamily: 'Inter-SemiBold',
      marginBottom: 16,
      textAlign: 'center',
    },
    
    // Back button text styling
    backButtonText: {
      fontSize: 16,
      fontFamily: 'Inter-SemiBold',
    },
  });

  /**
   * Error state: Product not found
   * Shows when the product ID doesn't match any existing product
   */
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

  // Check if current user is the owner of this product
  const isOwner = product.createdBy === user?.id;

  /**
   * Handle edit button press
   * Only allows product owners to edit their products
   */
  const handleEdit = () => {
    // Only allow product owners to edit their products: error management
    if (!isOwner) {
      Alert.alert('Error', 'You can only edit products you created.');
      return;
    }
    router.push(`/product/edit/${product.id}` as any);
  };

  /**
   * Handle delete button press
   * Shows confirmation dialog before deleting
   * Only allows product owners to delete their products
   */
  const handleDelete = () => {
    // Only allow product owners to delete their products: error management
    if (!isOwner) {
      Alert.alert('Error', 'You can only delete products you created.');
      return;
    }

    Alert.alert(
      'Delete Product',
      `Are you sure you want to delete "${product.name}"?`,
      [
        { text: 'Cancel', style: 'cancel' },
        {
          text: 'Delete',
          style: 'destructive',
          onPress: () => {
            deleteProduct(product.id);
            router.back();
          },
        },
      ]
    );
  };

  /**
   * Format date string to readable format
   * Converts ISO date string to localized date format
   */
  const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    return date.toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
    });
  };

  return (
    <View style={styles.container}>
      {/* Header with back button and owner actions */}
      <View style={styles.header}>
        <TouchableOpacity
          style={styles.backButton}
          onPress={() => router.back()}
        >
          <ArrowLeft size={20} color={colors.text} />
        </TouchableOpacity>

        {/* Show edit/delete buttons only for product owner */}
        {isOwner && (
          <View style={styles.headerActions}>
            <TouchableOpacity
              style={[styles.actionButton, { backgroundColor: colors.secondary }]}
              onPress={handleEdit}
            >
              <Edit3 size={20} color="#000" />
            </TouchableOpacity>
            <TouchableOpacity
              style={[styles.actionButton, { backgroundColor: colors.error }]}
              onPress={handleDelete}
            >
              <Trash2 size={20} color="#FFF" />
            </TouchableOpacity>
          </View>
        )}
      </View>

      {/* Scrollable content area */}
      <ScrollView style={styles.scrollContent}>
        {/* Product image */}
        <View style={styles.imageContainer}>
          <Image source={{ uri: product.image }} style={styles.productImage} />
        </View>

        {/* Product information container */}
        <View style={styles.contentContainer}>
          {/* Product title and description */}
          <View style={styles.productHeader}>
            <Text style={styles.productName}>{product.name}</Text>
            <Text style={styles.productDescription}>{product.description}</Text>
          </View>

          {/* Price and stock information */}
          <View style={styles.priceSection}>
            <View style={styles.priceContainer}>
              <Text style={styles.priceLabel}>Price</Text>
              <Text style={styles.priceValue}>${product.price.toFixed(2)}</Text>
            </View>
            <View style={styles.stockContainer}>
              <Text style={styles.stockLabel}>In Stock</Text>
              <Text style={styles.stockValue}>{product.stock}</Text>
            </View>
          </View>

          {/* Product details section */}
          <View style={styles.detailsSection}>
            <Text style={styles.sectionTitle}>Product Details</Text>
            
            {/* Category information */}
            <View style={styles.detailItem}>
              <Tag size={20} color={colors.primary} style={styles.detailIcon} />
              <View style={styles.detailContent}>
                <Text style={styles.detailLabel}>Category</Text>
                <Text style={styles.detailValue}>{product.category}</Text>
              </View>
            </View>

            {/* Seller information */}
            <View style={styles.detailItem}>
              <UserIcon size={20} color={colors.secondary} style={styles.detailIcon} />
              <View style={styles.detailContent}>
                <Text style={styles.detailLabel}>Seller</Text>
                <Text style={styles.detailValue}>{product.seller}</Text>
              </View>
            </View>

            {/* Creation date */}
            <View style={[styles.detailItem, styles.detailItemLast]}>
              <Calendar size={20} color={colors.textSecondary} style={styles.detailIcon} />
              <View style={styles.detailContent}>
                <Text style={styles.detailLabel}>Created</Text>
                <Text style={styles.detailValue}>{formatDate(product.createdAt)}</Text>
              </View>
            </View>
          </View>

          {/* Owner action buttons (edit/delete) */}
          {isOwner && (
            <View style={styles.ownerActions}>
              {/* Edit product button */}
              <ButtonForm
                onPress={handleEdit}
                text="Edit Product"
                colors={[colors.primary, colors.secondary]}
                icon={<Edit3 size={20} color="#000" />}
              />

              {/* Delete product button */}
              <ButtonForm
                onPress={handleDelete}
                text="Delete Product"
                colors={[colors.error, '#FE4848']}
                icon={<Trash2 size={20} color="#FFF" />}
              />

            </View>
          )}
        </View>
      </ScrollView>
    </View>
  );
}