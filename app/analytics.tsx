// AnalyticsScreen: Displays comprehensive analytics and statistics for the user's products
import DistributionChartsSection from '@/components/Analytic/DistributionChart';
import OverviewStatisticsSection from '@/components/Analytic/OverviewStatistics';
import { useData } from '@/contexts/DataContext';
import { useTheme } from '@/contexts/ThemeContext';
import { router } from 'expo-router';
import { ArrowLeft } from 'lucide-react-native';
import { ScrollView, StyleSheet, Text, TouchableOpacity, View } from 'react-native';

export default function AnalyticsScreen() {
  // Get statistics data from the data context
  const { getStatProducts } = useData();
  const { colors } = useTheme();
  
  // Extract all statistics from the data context
  const { 
    totalProducts, 
    totalValue, 
    totalStock, 
    averagePrice, 
    highestPriced, 
    lowestPriced, 
    averageStock, 
    categoryData,
    sellerData,
    priceRangeData, 
    stockRangeData 
  } = getStatProducts();

  // Styles for the analytics screen layout
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
      fontSize: 20,
      fontFamily: 'Inter-Bold',
      color: colors.text,
    },
    placeholder: {
      width: 40,
    },
    scrollContent: {
      flex: 1,
      paddingHorizontal: 20,
    }
  });

  return (
    <View style={styles.container}>
      {/* Header with back button and title */}
      <View style={styles.header}>
        <TouchableOpacity
          style={styles.backButton}
          onPress={() => router.back()}
        >
          <ArrowLeft size={20} color={colors.text} />
        </TouchableOpacity>
        <Text style={styles.headerTitle}>Detailed Analytics</Text>
        <View style={styles.placeholder} />
      </View>

      {/* Scrollable content containing statistics and charts */}
      <ScrollView style={styles.scrollContent}>
        {/* Overview statistics section with key metrics */}
        <OverviewStatisticsSection
          colors={colors}
          totalProducts={totalProducts}
          totalStock={totalStock}
          totalValue={totalValue}
          averagePrice={averagePrice}
          lowestPriced={lowestPriced}
          highestPriced={highestPriced}
          averageStock={averageStock}
        />

        {/* Distribution charts section with category, price, and stock data */}
        <DistributionChartsSection
          colors={colors}
          categoryData={categoryData}
          sellerData={sellerData}
          priceRangeData={priceRangeData}
          stockRangeData={stockRangeData}
        />

      </ScrollView>
    </View>
  );
}
