import React from 'react';
import {
  View,
  Text,
  Image,
  TouchableOpacity,
  StyleSheet,
  Alert,
  Dimensions,
} from 'react-native';
import { useTheme } from '@/contexts/ThemeContext';
import { useAuth } from '@/contexts/AuthContext';
import { useData } from '@/contexts/DataContext';
import { router } from 'expo-router';
import { Eye, CreditCard as Edit3, Trash2 } from 'lucide-react-native';
import { Product } from '@/contexts/DataContext';

// Props type for the component
interface ProductCardProps {
  product: Product;
  viewMode: 'grid' | 'list';
}

// Get device width to calculate grid card size
const { width } = Dimensions.get('window');
const cardWidth = (width - 60) / 2;

export default function ProductCard({ product, viewMode }: ProductCardProps) {
  const { colors } = useTheme();       // Theme colors
  const { user } = useAuth();          // Current authenticated user
  const { deleteProduct } = useData(); // Delete product method

  // Check if the logged-in user is the creator of this product
  const isOwner = product.createdBy === user?.id;

  /**
   * Confirm and delete a product.
   * Only owners can delete.
   */
  const handleDeleteProduct = () => {
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
          onPress: async () => {
            try {
              await deleteProduct(product.id);
            } catch (error) {
              Alert.alert('Error', 'Failed to delete product. Please try again.');
            }
          },
        },
      ]
    );
  };

  /**
   * Navigate to edit screen for the product.
   * Only owners can edit.
   */
  const handleEditProduct = () => {
    if (!isOwner) {
      Alert.alert('Error', 'You can only edit products you created.');
      return;
    }
    router.push(`/product/edit/${product.id}` as any);
  };

  /**
   * Define styles for grid and list views.
   * Dynamically uses theme colors.
   */
  const styles = StyleSheet.create({
    gridCard: {
      width: cardWidth,
      backgroundColor: colors.surface,
      borderRadius: 16,
      borderWidth: 1,
      borderColor: colors.border,
      marginBottom: 16,
      overflow: 'hidden',
    },
    listCard: {
      flexDirection: 'row',
      backgroundColor: colors.surface,
      borderRadius: 12,
      borderWidth: 1,
      borderColor: colors.border,
      marginBottom: 12,
      padding: 12,
      alignItems: 'center',
    },
    gridImage: {
      width: '100%',
      height: 120,
      backgroundColor: colors.border,
    },
    listImage: {
      width: 60,
      height: 60,
      borderRadius: 8,
      backgroundColor: colors.border,
    },
    gridContent: {
      padding: 12,
    },
    listContent: {
      flex: 1,
      marginLeft: 12,
    },
    productName: {
      fontSize: 14,
      fontFamily: 'Inter-SemiBold',
      color: colors.text,
      marginBottom: 4,
    },
    productDescription: {
      fontSize: 12,
      fontFamily: 'Inter-Regular',
      color: colors.textSecondary,
      marginBottom: 8,
      lineHeight: 16,
    },
    priceRow: {
      flexDirection: 'row',
      justifyContent: 'space-between',
      alignItems: 'center',
      marginBottom: 8,
    },
    price: {
      fontSize: 16,
      fontFamily: 'Inter-Bold',
      color: colors.primary,
    },
    stock: {
      fontSize: 11,
      fontFamily: 'Inter-Regular',
      color: colors.textSecondary,
    },
    categoryRow: {
      flexDirection: 'row',
      justifyContent: 'space-between',
      alignItems: 'center',
      marginBottom: 8,
    },
    categoryTag: {
      paddingHorizontal: 6,
      paddingVertical: 2,
      backgroundColor: colors.primary + '20',
      borderRadius: 4,
    },
    categoryText: {
      fontSize: 10,
      fontFamily: 'Inter-SemiBold',
      color: colors.primary,
    },
    seller: {
      fontSize: 10,
      fontFamily: 'Inter-Regular',
      color: colors.textSecondary,
    },
    gridActions: {
      position: 'absolute',
      top: 8,
      right: 8,
      flexDirection: 'row',
      gap: 4,
    },
    listActions: {
      flexDirection: 'row',
      gap: 6,
      marginLeft: 8,
    },
    actionButton: {
      width: 32,
      height: 32,
      borderRadius: 16,
      justifyContent: 'center',
      alignItems: 'center',
      shadowColor: '#000',
      shadowOffset: { width: 0, height: 1 },
      shadowOpacity: 0.2,
      shadowRadius: 2,
      elevation: 2,
    },
    viewButton: {
      backgroundColor: colors.primary,
    },
    editButton: {
      backgroundColor: colors.secondary,
    },
    deleteButton: {
      backgroundColor: colors.error,
    },
  });

  /**
   * Render list view layout for the product card.
   * Includes image, info, and action buttons.
   */
  if (viewMode === 'list') {
    return (
      <View style={styles.listCard}>
        {/* Action button to view product details */}
        <TouchableOpacity onPress={() => router.push(`/product/${product.id}` as any)}>
          <Image source={{ uri: product.image }} style={styles.listImage} />
        </TouchableOpacity>
        
        <View style={styles.listContent}>
          <TouchableOpacity onPress={() => router.push(`/product/${product.id}` as any)}>
            <Text style={styles.productName} numberOfLines={1}>
              {product.name}
            </Text>
            <Text style={styles.productDescription} numberOfLines={2}>
              {product.description}
            </Text>
            <View style={styles.priceRow}>
              <Text style={styles.price}>${product.price.toFixed(2)}</Text>
              <Text style={styles.stock}>Stock: {product.stock}</Text>
            </View>
          </TouchableOpacity>
        </View>
        
        <View style={styles.listActions}>
          {/* Action view detail */}
          <TouchableOpacity
            style={[styles.actionButton, styles.viewButton]}
            onPress={() => router.push(`/product/${product.id}` as any)}
          >
            <Eye size={16} color="#000" />
          </TouchableOpacity>
          {/* Action button to edit product : Only the owner can edit */}
          {
            isOwner && (
              <>
              <TouchableOpacity
                style={[styles.actionButton, styles.editButton]}
                onPress={handleEditProduct}
              >
                <Edit3 size={16} color="#000" />
              </TouchableOpacity>
              <TouchableOpacity
                style={[styles.actionButton, styles.deleteButton]}
                onPress={handleDeleteProduct}
              >
                <Trash2 size={16} color="#FFF" />
              </TouchableOpacity>              
              </>
            )
          }
        </View>
      </View>
    );
  }

  /**
   * Render grid view layout for the product card.
   * Includes image, info, category, seller, and action buttons.
   */
  return (
    <View style={styles.gridCard}>
      {/* Action button to view product details */}
      <TouchableOpacity onPress={() => router.push(`/product/${product.id}` as any)}>
        <Image source={{ uri: product.image }} style={styles.gridImage} />
        <View style={styles.gridContent}>
          <Text style={styles.productName} numberOfLines={2}>
            {product.name}
          </Text>
          <Text style={styles.productDescription} numberOfLines={2}>
            {product.description}
          </Text>
          <View style={styles.priceRow}>
            <Text style={styles.price}>${product.price.toFixed(2)}</Text>
            <Text style={styles.stock}>Stock: {product.stock}</Text>
          </View>
          <View style={styles.categoryRow}>
            <View style={styles.categoryTag}>
              <Text style={styles.categoryText}>{product.category}</Text>
            </View>
            <Text style={styles.seller}>by {product.seller}</Text>
          </View>
        </View>
      </TouchableOpacity>
      {/* Action button to edit product : Only the owner can edit */}
      {
        isOwner && (      
          <View style={styles.gridActions}>
            <TouchableOpacity
              style={[styles.actionButton, styles.editButton]}
              onPress={handleEditProduct}
            >
              <Edit3 size={16} color="#000" />
            </TouchableOpacity>
            <TouchableOpacity
              style={[styles.actionButton, styles.deleteButton]}
              onPress={handleDeleteProduct}
            >
              <Trash2 size={16} color="#FFF" />
            </TouchableOpacity>
          </View>
        )
      }
    </View>
  );
}
