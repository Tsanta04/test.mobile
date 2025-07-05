// SimpleChart: A reusable chart component that renders either bar charts or pie charts based on data
import React from 'react';
import { View, Text, StyleSheet, Dimensions } from 'react-native';
import { useTheme } from '@/contexts/ThemeContext';
import { ChartDataPoint } from '@/constants/type';
import { BarChart3 } from 'lucide-react-native';
import { EmptyDataMessage } from '../Common/EmptyDataMessage';

// Props for the chart component
interface SimpleChartProps {
  // Array of data points to display in the chart
  data: ChartDataPoint[];
  // Type of chart to render ('bar' or 'pie')
  type: 'bar' | 'pie';
  // Title displayed at the top of the chart
  title: string;
}

const { width } = Dimensions.get('window');

export default function SimpleChart({ data, type, title }: SimpleChartProps) {
  const { colors } = useTheme();

  // Calculate the maximum value for percentage calculations
  const maxValue = Math.max(...data.map(item => item.value));
  // Calculate the total sum for pie chart percentages
  const total = data.reduce((sum, item) => sum + item.value, 0);

  // Styles for the chart container and elements
  const styles = StyleSheet.create({
    container: {
      backgroundColor: colors.surface,
      borderRadius: 12,
      padding: 16,
      borderWidth: 1,
      borderColor: colors.border,
      marginBottom: 16,
    },
    title: {
      fontSize: 16,
      fontFamily: 'Inter-SemiBold',
      color: colors.text,
      marginBottom: 16,
      textAlign: 'center',
    },
    // Bar chart specific styles
    barContainer: {
      marginBottom: 12,
    },
    barLabel: {
      fontSize: 12,
      fontFamily: 'Inter-Regular',
      color: colors.text,
      marginBottom: 4,
    },
    barBackground: {
      height: 20,
      backgroundColor: colors.border,
      borderRadius: 10,
      overflow: 'hidden',
    },
    barFill: {
      height: '100%',
      backgroundColor: colors.primary,
      borderRadius: 10,
    },
    barValue: {
      fontSize: 10,
      fontFamily: 'Inter-SemiBold',
      color: colors.textSecondary,
      marginTop: 2,
    },
    // Pie chart specific styles
    pieContainer: {
      alignItems: 'center',
    },
    pieChart: {
      width: 120,
      height: 120,
      borderRadius: 60,
      marginBottom: 16,
      overflow: 'hidden',
    },
    pieSegment: {
      position: 'absolute',
      width: 120,
      height: 120,
      borderRadius: 60,
    },
    legend: {
      flexDirection: 'row',
      flexWrap: 'wrap',
      justifyContent: 'center',
      gap: 12,
    },
    legendItem: {
      flexDirection: 'row',
      alignItems: 'center',
    },
    legendColor: {
      width: 12,
      height: 12,
      borderRadius: 6,
      marginRight: 6,
    },
    legendText: {
      fontSize: 11,
      fontFamily: 'Inter-Regular',
      color: colors.text,
    },
    emptyIcon: {
      marginBottom: 12,
    }
  });

  // Default color palette for chart elements
  const defaultColors = [colors.primary, colors.secondary, colors.success, colors.warning, colors.error];

  // Render bar chart
  if (type === 'bar') {
    return (
      <View style={styles.container}>
        <Text style={styles.title}>{title}</Text>
        {data.length > 0 ? (
          data.map((item, index) => {
            // Calculate percentage for bar width
            const percentage = maxValue > 0 ? (item.value / maxValue) * 100 : 0;
            return (
              <View key={index} style={styles.barContainer}>
                <Text style={styles.barLabel}>{item.label}</Text>
                <View style={styles.barBackground}>
                  <View 
                    style={[
                      styles.barFill, 
                      { 
                        width: `${percentage}%`,
                        backgroundColor: item.color || defaultColors[index % defaultColors.length]
                      }
                    ]} 
                  />
                </View>
                <Text style={styles.barValue}>{item.value}</Text>
              </View>
            );
          })
        ) : (
          <EmptyDataMessage
            message="No entries found"
            subtext="Please add some entries to see them here."
            icon={<BarChart3 size={64} color={colors.textSecondary} style={styles.emptyIcon} />}
            colors={colors}
          />
        )}
      </View>
    );
  }

  // Render pie chart (simplified representation using legend only)
  return (
    <View style={styles.container}>
      <Text style={styles.title}>{title}</Text>
      <View style={styles.pieContainer}>
        <View style={styles.legend}>
          {data.length > 0 ? (
            data.map((item, index) => {
              // Calculate percentage for pie chart
              const percentage = total > 0 ? ((item.value / total) * 100).toFixed(1) : '0';
              return (
                <View key={index} style={styles.legendItem}>
                  <View 
                    style={[
                      styles.legendColor, 
                      { backgroundColor: item.color || defaultColors[index % defaultColors.length] }
                    ]} 
                  />
                  <Text style={styles.legendText}>
                    {item.label}: {percentage}% ({item.value})
                  </Text>
                </View>
              );
            })
          ) : (
            <EmptyDataMessage
              message="No entries found"
              subtext="Please add some entries to see them here."
              icon={<BarChart3 size={64} color={colors.textSecondary} style={styles.emptyIcon} />}
              colors={colors}
            />
          )}
        </View>
      </View>
    </View>
  );
}