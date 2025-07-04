import React from 'react';
import {
  View,
  Text,
  TouchableOpacity,
  TextInput,
  StyleSheet,
} from 'react-native';
import { ChevronDown } from 'lucide-react-native';
import { ColorType } from '@/constants/type';

// Define the shape of price range state
interface PriceRange {
  min: string;
  max: string;
}

// Props definition for the FilterPanel component
interface FilterPanelProps {
  selectedCategory: string;
  selectedSeller: string;
  priceRange: PriceRange;
  setShowCategoryModal: (value: boolean) => void;
  setShowSellerModal: (value: boolean) => void;
  setPriceRange: React.Dispatch<React.SetStateAction<PriceRange>>;
  setCurrentPage: (page: number) => void;
  resetFilters: () => void;
  colors: ColorType;
}

/**
 * FilterPanel
 * Renders the UI for filtering products by category, seller, and price range.
 * Includes buttons to open selection modals and clear all filters.
 */
const FilterPanel: React.FC<FilterPanelProps> = ({
  colors,
  selectedCategory,
  selectedSeller,
  priceRange,
  setShowCategoryModal,
  setShowSellerModal,
  setPriceRange,
  setCurrentPage,
  resetFilters,
}) => {
  
  /**
   * Define the component styles
   * Uses dynamic theme colors passed as props
   */
  const styles = StyleSheet.create({
    filtersContainer: {
      paddingHorizontal: 20,
      paddingBottom: 16,
      backgroundColor: colors.surface,
      marginHorizontal: 20,
      borderRadius: 12,
      borderWidth: 1,
      borderColor: colors.border,
      marginBottom: 16,
    },
    filtersHeader: {
      flexDirection: 'row',
      justifyContent: 'space-between',
      alignItems: 'center',
      paddingVertical: 16,
    },
    filtersTitle: {
      fontSize: 16,
      fontFamily: 'Inter-SemiBold',
      color: colors.text,
    },
    clearFiltersButton: {
      paddingHorizontal: 12,
      paddingVertical: 6,
      backgroundColor: colors.error,
      borderRadius: 6,
    },
    clearFiltersText: {
      color: '#FFF',
      fontSize: 11,
      fontFamily: 'Inter-SemiBold',
    },
    filterRow: {
      flexDirection: 'row',
      gap: 12,
      marginBottom: 12,
    },
    filterSelect: {
      flex: 1,
      backgroundColor: colors.background,
      borderRadius: 8,
      borderWidth: 1,
      borderColor: colors.border,
      paddingHorizontal: 12,
      paddingVertical: 8,
      flexDirection: 'row',
      justifyContent: 'space-between',
      alignItems: 'center',
    },
    filterSelectText: {
      fontSize: 12,
      fontFamily: 'Inter-Regular',
      color: colors.text,
    },
    priceInputs: {
      flexDirection: 'row',
      gap: 12,
      marginBottom: 16,
    },
    priceInput: {
      flex: 1,
      backgroundColor: colors.background,
      borderRadius: 8,
      borderWidth: 1,
      borderColor: colors.border,
      paddingHorizontal: 12,
      paddingVertical: 8,
      fontSize: 12,
      fontFamily: 'Inter-Regular',
      color: colors.text,
    },
  });

  return (
    <View style={styles.filtersContainer}>
      {/* Header with title and Clear All button */}
      <View style={styles.filtersHeader}>
        <Text style={styles.filtersTitle}>Filters</Text>
        <TouchableOpacity
          style={styles.clearFiltersButton}
          onPress={resetFilters}
        >
          <Text style={styles.clearFiltersText}>Clear All</Text>
        </TouchableOpacity>
      </View>
      
      {/* Row with Category and Seller selectors */}
      <View style={styles.filterRow}>
        <TouchableOpacity
          style={styles.filterSelect}
          onPress={() => setShowCategoryModal(true)}
        >
          <Text style={styles.filterSelectText}>
            Category: {selectedCategory}
          </Text>
          <ChevronDown size={16} color={colors.textSecondary} />
        </TouchableOpacity>
        
        <TouchableOpacity
          style={styles.filterSelect}
          onPress={() => setShowSellerModal(true)}
        >
          <Text style={styles.filterSelectText}>
            Seller: {selectedSeller}
          </Text>
          <ChevronDown size={16} color={colors.textSecondary} />
        </TouchableOpacity>
      </View>

      {/* Row with Min and Max price inputs */}
      <View style={styles.priceInputs}>
        <TextInput
          style={styles.priceInput}
          placeholder="Min Price"
          placeholderTextColor={colors.textSecondary}
          value={priceRange.min}
          onChangeText={(text) => {
            setPriceRange(prev => ({ ...prev, min: text }));
            setCurrentPage(1);
          }}
          keyboardType="numeric"
        />
        <TextInput
          style={styles.priceInput}
          placeholder="Max Price"
          placeholderTextColor={colors.textSecondary}
          value={priceRange.max}
          onChangeText={(text) => {
            setPriceRange(prev => ({ ...prev, max: text }));
            setCurrentPage(1);
          }}
          keyboardType="numeric"
        />
      </View>
    </View>
  );
};

export default FilterPanel;
