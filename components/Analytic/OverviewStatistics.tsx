// OverviewStatisticsSection: Displays key statistics about products in a grid layout with cards
import React from 'react';
import { View, Text, ViewStyle, TextStyle, StyleSheet, Dimensions } from 'react-native';
import { Package, ShoppingCart, DollarSign, TrendingUp, Target } from 'lucide-react-native';
import { ColorType } from '@/constants/type';
import StatCard from './StatCard';

// Props for the statistics section component
interface OverviewStatisticsSectionProps {
  // Colors object for theming
  colors: ColorType;
  // Statistical data values
  totalProducts: number;
  totalStock: number;
  totalValue: number;
  averagePrice: number;
  lowestPriced: number;
  highestPriced: number;
  averageStock: number;
}

export default function OverviewStatisticsSection({
  colors,
  totalProducts,
  totalStock,
  totalValue,
  averagePrice,
  lowestPriced,
  highestPriced,
  averageStock
}: OverviewStatisticsSectionProps) {

    const { width } = Dimensions.get('window');
    const styles = StyleSheet.create({
        section: {
          marginBottom: 0,
        },
        sectionTitle: {
          fontSize: 18,
          fontFamily: 'Inter-Bold',
          color: colors.text,
          marginBottom: 16,
        },
        statsGrid: {
          flexDirection: 'row',
          flexWrap: 'wrap',
          gap: 15,
          margin:'auto',
          marginBottom: 20,
        },
        statCardSmall: {
          width: (width - 63) / 2,
        },
        statCardLarge: {
          width: width - 48,
        },
      });
    
    
  return (
    <View style={styles.section}>
      {/* Section title */}
      <Text style={styles.sectionTitle}>Overview Statistics</Text>

      {/* Grid layout for statistics cards */}
      <View style={styles.statsGrid}>
        {/* Total Products card with gradient */}
        <View style={styles.statCardSmall}>
          <StatCard
            title="Total Products"
            value={totalProducts}
            icon={<Package size={16} color={colors.primary} />}
            gradient={true}
          />
        </View>

        {/* Total Stock card */}
        <View style={styles.statCardSmall}>
          <StatCard
            title="Total Stock"
            value={totalStock}
            icon={<ShoppingCart size={16} color={colors.text} />}
          />
        </View>

        {/* Total Value card */}
        <View style={styles.statCardSmall}>
          <StatCard
            title="Total Value"
            value={`$${totalValue.toFixed(0)}`}
            icon={<DollarSign size={16} color={colors.text} />}
          />
        </View>

        {/* Average Price card */}
        <View style={styles.statCardSmall}>
          <StatCard
            title="Avg Price"
            value={`$${averagePrice.toFixed(0)}`}
            icon={<TrendingUp size={16} color={colors.text} />}
          />
        </View>

        {/* Price Range card (larger) with subtitle */}
        <View style={styles.statCardLarge}>
          <StatCard
            title="Price Range"
            value={`$${lowestPriced.toFixed(0)} - $${highestPriced.toFixed(0)}`}
            subtitle={`Average stock per product: ${averageStock.toFixed(1)} items`}
            icon={<Target size={16} color={colors.text} />}
          />
        </View>
      </View>
    </View>
  );
}
