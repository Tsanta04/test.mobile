import React, { useState, useMemo } from 'react';
import {
  View,
  Text,
  FlatList,
  TouchableOpacity,
  TextInput,
  StyleSheet,
  RefreshControl,
} from 'react-native';
import { LinearGradient } from 'expo-linear-gradient';
import { useData } from '@/contexts/DataContext';
import { useTheme } from '@/contexts/ThemeContext';
import { router, Link } from 'expo-router';
import { Search, Plus, Filter, Grid2x2 as Grid, List, X } from 'lucide-react-native';
import ProductCard from '@/components/ProductList/ProductCard';
import Header from '@/components/Header';
import FilterPanel from '@/components/ProductList/FilterPanels';
import PaginationControls from '@/components/ProductList/Pagination';
import SelectionModal from '@/components/SelectionModal';

// Number of items per page for pagination
const ITEMS_PER_PAGE = 10;

export default function ProductsScreen() {
  const { products, categories, sellers } = useData();
  const { colors, theme, toggleTheme } = useTheme();

  // State variables for filters, modals, view mode, pagination, etc.
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedCategory, setSelectedCategory] = useState('All');
  const [selectedSeller, setSelectedSeller] = useState('All');
  const [priceRange, setPriceRange] = useState({ min: '', max: '' });
  const [showFilters, setShowFilters] = useState(false);
  const [showCategoryModal, setShowCategoryModal] = useState(false);
  const [showSellerModal, setShowSellerModal] = useState(false);
  const [viewMode, setViewMode] = useState<'grid' | 'list'>('grid');
  const [currentPage, setCurrentPage] = useState(1);
  const [refreshing, setRefreshing] = useState(false);


  /**
   * Filter products based on search, category, seller, and price range.
   * useMemo optimizes re-calculation only when dependencies change.
   */
  const filteredProducts = useMemo(() => {
    let filtered = products.filter(product => {
      const matchesSearch = product.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
                           product.description.toLowerCase().includes(searchQuery.toLowerCase());
      const matchesCategory = selectedCategory === 'All' || product.category === selectedCategory;
      const matchesSeller = selectedSeller === 'All' || product.seller === selectedSeller;
      let matchesPrice = true;
      if (priceRange.min) {
        matchesPrice = matchesPrice && product.price >= parseFloat(priceRange.min);
      }
      if (priceRange.max) {
        matchesPrice = matchesPrice && product.price <= parseFloat(priceRange.max);
      }

      return matchesSearch && matchesCategory && matchesSeller && matchesPrice;
    });

    return filtered;
  }, [products, searchQuery, selectedCategory, selectedSeller, priceRange]);


  /**
   * Paginate the filtered products.
   */
  const paginatedProducts = useMemo(() => {
    const startIndex = (currentPage - 1) * ITEMS_PER_PAGE;
    return filteredProducts.slice(startIndex, startIndex + ITEMS_PER_PAGE);
  }, [filteredProducts, currentPage]);


  // Total number of pages for pagination
  const totalPages = Math.ceil(filteredProducts.length / ITEMS_PER_PAGE);


  /**
   * Refresh handler (e.g. pull-to-refresh)
   */
  const onRefresh = () => {
    setRefreshing(true);
    setTimeout(() => setRefreshing(false), 1000);
  };


  /**
   * Reset all filters to default values
   */
  const resetFilters = () => {
    setSearchQuery('');
    setSelectedCategory('All');
    setSelectedSeller('All');
    setPriceRange({ min: '', max: '' });
    setCurrentPage(1);
  };


  /**
   * Define the styles (using dynamic theme colors)
   */
  const styles = StyleSheet.create({
    container: {
      flex: 1,
      backgroundColor: colors.background,
    },
    searchContainer: {
      paddingHorizontal: 20,
      paddingBottom: 16,
    },
    searchWrapper: {
      flexDirection: 'row',
      alignItems: 'center',
      backgroundColor: colors.surface,
      borderRadius: 12,
      borderWidth: 1,
      borderColor: colors.border,
      paddingHorizontal: 16,
      paddingVertical: 12,
    },
    searchInput: {
      flex: 1,
      fontSize: 14,
      fontFamily: 'Inter-Regular',
      color: colors.text,
      marginLeft: 12,
    },
    controlsContainer: {
      flexDirection: 'row',
      justifyContent: 'space-between',
      alignItems: 'center',
      paddingHorizontal: 20,
      paddingBottom: 16,
    },
    viewModeButtons: {
      flexDirection: 'row',
      backgroundColor: colors.surface,
      borderRadius: 8,
      borderWidth: 1,
      borderColor: colors.border,
      overflow: 'hidden',
    },
    viewModeButton: {
      paddingHorizontal: 12,
      paddingVertical: 8,
    },
    viewModeButtonActive: {
      backgroundColor: colors.primary,
    },
    addButton: {
      borderRadius: 12,
      overflow: 'hidden',
    },
    addGradient: {
      paddingHorizontal: 16,
      paddingVertical: 12,
      flexDirection: 'row',
      alignItems: 'center',
    },
    addButtonText: {
      fontSize: 14,
      fontFamily: 'Inter-SemiBold',
      color: '#000',
      marginLeft: 8,
    },
    productsContainer: {
      flex: 1,
      paddingHorizontal: 20,
    },
    productsList: {
      paddingBottom: 20,
    },
    emptyContainer: {
      flex: 1,
      justifyContent: 'center',
      alignItems: 'center',
      paddingVertical: 60,
    },
    emptyText: {
      fontSize: 16,
      fontFamily: 'Inter-SemiBold',
      color: colors.textSecondary,
      textAlign: 'center',
    },
    emptySubtext: {
      fontSize: 12,
      fontFamily: 'Inter-Regular',
      color: colors.textSecondary,
      textAlign: 'center',
      marginTop: 8,
    }
  });


  /**
   * Render a single product card
   */
  const renderProduct = ({ item }: { item: any }) => (
    <ProductCard product={item} viewMode={viewMode} />
  );
  

  return (
    <View style={styles.container}>
      {/* App header with title, theme toggle, filter button */}
      <Header
        title="Products"
        theme={theme}
        toggleTheme={toggleTheme}
        onToggleFilters={() => setShowFilters(!showFilters)}
        colors={colors}
      />

      {/* Search input bar */}
      <View style={styles.searchContainer}>
        <View style={styles.searchWrapper}>
          <Search size={18} color={colors.textSecondary} />
          <TextInput
            style={styles.searchInput}
            placeholder="Search products..."
            placeholderTextColor={colors.textSecondary}
            value={searchQuery}
            onChangeText={(text) => {
              setSearchQuery(text);
              setCurrentPage(1);
            }}
          />
          {searchQuery.length > 0 && (
            <TouchableOpacity onPress={() => {
              setSearchQuery('');
              setCurrentPage(1);
            }}>
              <X size={18} color={colors.textSecondary} />
            </TouchableOpacity>
          )}
        </View>
      </View>

      {/* Filter panel (shown when toggled) */}
      {showFilters && (
        <FilterPanel
          selectedCategory={selectedCategory}
          selectedSeller={selectedSeller}
          priceRange={priceRange}
          setShowCategoryModal={setShowCategoryModal}
          setShowSellerModal={setShowSellerModal}
          setPriceRange={setPriceRange}
          setCurrentPage={setCurrentPage}
          resetFilters={resetFilters}
          colors={colors}
        />
      )}

      {/* View mode buttons and Add Product button */}
      <View style={styles.controlsContainer}>
        <View style={styles.viewModeButtons}>
          <TouchableOpacity
            style={[styles.viewModeButton, viewMode === 'grid' && styles.viewModeButtonActive]}
            onPress={() => setViewMode('grid')}
          >
            <Grid size={18} color={viewMode === 'grid' ? '#000' : colors.textSecondary} />
          </TouchableOpacity>
          <TouchableOpacity
            style={[styles.viewModeButton, viewMode === 'list' && styles.viewModeButtonActive]}
            onPress={() => setViewMode('list')}
          >
            <List size={18} color={viewMode === 'list' ? '#000' : colors.textSecondary} />
          </TouchableOpacity>
        </View>

        <TouchableOpacity
          style={styles.addButton}
          onPress={() => router.push('/product/add' as any)}
        >
          <LinearGradient
            colors={[colors.primary, colors.secondary]}
            style={styles.addGradient}
          >
            <Plus size={18} color="#000" />
            <Text style={styles.addButtonText}>Add Product</Text>
          </LinearGradient>
        </TouchableOpacity>
      </View>

      {/* Product list or empty state */}
      <View style={styles.productsContainer}>
        {paginatedProducts.length === 0 ? (
          <View style={styles.emptyContainer}>
            <Text style={styles.emptyText}>No products found</Text>
            <Text style={styles.emptySubtext}>
              {filteredProducts.length === 0 ? 'Try adjusting your search or filters' : 'Go to page 1 to see results'}
            </Text>
          </View>
        ) : (
          <FlatList
            data={paginatedProducts}
            renderItem={renderProduct}
            keyExtractor={(item) => item.id}
            style={styles.productsList}
            refreshControl={
              <RefreshControl
                refreshing={refreshing}
                onRefresh={onRefresh}
                tintColor={colors.primary}
              />
            }
            numColumns={viewMode === 'grid' ? 2 : 1}
            key={viewMode}
            columnWrapperStyle={viewMode === 'grid' ? { justifyContent: 'space-between' } : undefined}
          />
        )}
      </View>

      {/* Pagination controls */}
      <PaginationControls
        currentPage={currentPage}
        totalPages={totalPages}
        onPageChange={setCurrentPage}
        colors={colors}
      />

      {/* Category selection modal */}
      <SelectionModal
        visible={showCategoryModal} 
        title='Select Category' 
        data={categories} 
        selected={selectedCategory}
        onSelect={(item) => {
          setSelectedCategory(item.name);
          setShowCategoryModal(false);
          setCurrentPage(1);
        }}
        onClose={()=>{setShowCategoryModal(false)}}
        colors={colors}
        enableToAdd={false}
      />

      {/* Seller selection modal */}
      <SelectionModal 
        visible={showSellerModal} 
        title='Select Seller' 
        data={sellers} 
        selected={selectedSeller}
        onSelect={(item) => {
          setSelectedSeller(item.name);
          setShowSellerModal(false);
          setCurrentPage(1);
        }}
        onClose={()=>{setShowSellerModal(false)}}
        colors={colors}
        enableToAdd={false}
      />

    </View>
  );
}
