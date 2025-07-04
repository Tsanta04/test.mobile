// DistributionChartsSection: Displays various distribution charts for products data analysis
import React from 'react';
import { View, Text, StyleSheet } from 'react-native';
import SimpleChart from './SimpleChart';
import { ChartDataPoint, ColorType } from '@/constants/type';


// Props for the distribution charts section component
interface DistributionChartsSectionProps {
  categoryData?: ChartDataPoint[];
  sellerData?: ChartDataPoint[];  
  priceRangeData?: ChartDataPoint[];
  stockRangeData?: ChartDataPoint[];
  colors: ColorType;
}

export default function DistributionChartsSection({
  categoryData = [],
  sellerData = [],
  priceRangeData = [],
  stockRangeData = [],
  colors
}: DistributionChartsSectionProps) {

    // Styles for the charts section
    const styles = StyleSheet.create({
        section: {
          marginBottom: 0,
        },
        sectionTitle: {
          fontSize: 18,
          fontFamily: 'Inter-Bold',
          color: colors.text,
          marginBottom: 16,
        }
    });

  return (
    <View style={styles.section}>
      {/* Section title */}
      <Text style={styles.sectionTitle}>Distribution Charts</Text>

      {/* Category distribution chart - only render if data exists */}
      {categoryData.length > 0 && (
        <SimpleChart
          data={categoryData}
          type="bar"
          title="Products by Category"
        />
      )}

      {/* Seller distribution chart - only render if data exists */}
      {sellerData.length > 0 && (
        <SimpleChart
          data={sellerData}
          type="bar"
          title="Products by Seller"
        />
      )}      

      {/* Price range distribution chart - only render if data exists */}
      {priceRangeData.length > 0 && (
        <SimpleChart
          data={priceRangeData}
          type="pie"
          title="Price Distribution"
        />
      )}

      {/* Stock level distribution chart - only render if data exists */}
      {stockRangeData.length > 0 && (
        <SimpleChart
          data={stockRangeData}
          type="bar"
          title="Stock Level Distribution"
        />
      )}
    </View>
  );
}
