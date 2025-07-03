import { ColorType } from '@/constants/type';
import React from 'react';
import { View, Text, TouchableOpacity, StyleSheet } from 'react-native';

// Define the props interface for the pagination component
interface PaginationControlsProps {
  currentPage: number;                 
  totalPages: number;                  
  onPageChange: (newPage: number) => void; 
  colors: ColorType;                 
}

/**
 * PaginationControls Component
 * Displays pagination navigation with "Previous" and "Next" buttons.
 * Disabled when on the first or last page.
 */
const PaginationControls: React.FC<PaginationControlsProps> = ({
  currentPage,
  totalPages,
  onPageChange,
  colors
}) => {

  if (totalPages <= 1) return null;

  // Define component styles dynamically with theme colors
  const styles = StyleSheet.create({
    pagination: {
      flexDirection: 'row',
      justifyContent: 'center',
      alignItems: 'center',
      paddingVertical: 20,
      gap: 16,
    },
    paginationButton: {
      paddingHorizontal: 16,
      paddingVertical: 8,
      backgroundColor: colors.surface,
      borderRadius: 8,
      borderWidth: 1,
      borderColor: colors.border,
    },
    paginationText: {
      fontSize: 12,
      fontFamily: 'Inter-SemiBold',
      color: colors.text,
    }
  });

  return (
    <View style={styles.pagination}>
      
      <TouchableOpacity
        style={styles.paginationButton}
        onPress={() => onPageChange(Math.max(1, currentPage - 1))}
        disabled={currentPage === 1}
      >
        <Text
          style={[
            styles.paginationText,
            { opacity: currentPage === 1 ? 0.5 : 1 }, // Visual feedback for disabled state
          ]}
        >
          Previous
        </Text>
      </TouchableOpacity>

      {/* Display current page and total pages */}
      <Text style={styles.paginationText}>
        {currentPage} of {totalPages}
      </Text>

      <TouchableOpacity
        style={styles.paginationButton}
        onPress={() => onPageChange(Math.min(totalPages, currentPage + 1))}
        disabled={currentPage === totalPages}
      >
        <Text
          style={[
            styles.paginationText,
            { opacity: currentPage === totalPages ? 0.5 : 1 }, // Visual feedback for disabled state
          ]}
        >
          Next
        </Text>
      </TouchableOpacity>
    </View>
  );
};

export default PaginationControls;
