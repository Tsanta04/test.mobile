// SelectForm component: A stylized select button for choosing an option from a list (e.g., category, seller)
import React from 'react';
import { View, Text, TouchableOpacity, StyleSheet } from 'react-native';
import { ChevronDown } from 'lucide-react-native';
import { ColorType } from '@/constants/type';

// Props for the select button, including label, value, placeholder, and error handling
interface LabeledSelectButtonProps {
  label: string;
  value: string;
  placeholder?: string;
  onPress: () => void;
  error?: string;
  colors: ColorType;
  icon?: React.ReactNode;
}

const SelectForm: React.FC<LabeledSelectButtonProps> = ({
  label,
  value,
  placeholder,
  onPress,
  error,
  colors,
  icon,
}) => {
  // Styles for the select button and its states
  const styles = StyleSheet.create({
    errorText: {
        fontSize: 12,
        fontFamily: 'Inter-Regular',
        color: colors.error,
        marginTop: 4,
    },
    selectContainer: {
        gap: 8,
    },
    label: {
        fontSize: 14,
        fontFamily: 'Inter-Medium',
        color: colors.text,
    },
    required: {
        color: colors.error,
    },
    selectButton: {
        flex: 1,
        backgroundColor: colors.surface,
        borderRadius: 12,
        borderWidth: 1,
        borderColor: colors.border,
        paddingHorizontal: 16,
        paddingVertical: 12,
        flexDirection: 'row',
        alignItems: 'center',
        justifyContent: 'space-between',
    },
    selectButtonActive: {
        borderColor: colors.primary,
        backgroundColor: colors.primary + '10',
    },
    inputWrapperError: {
        borderColor: colors.error,
    },    
    selectText: {
        fontSize: 14,
        fontFamily: 'Inter-Regular',
        color: colors.textSecondary,
    },
    selectTextActive: {
        color: colors.text,
        fontFamily: 'Inter-SemiBold',
    },        
 });

  return (
    <View style={styles.selectContainer}>
      {/* Label for the select field */}
      <Text style={styles.label}>
        {label} <Text style={styles.required}>*</Text>
      </Text>
      {/* Touchable select button with icon and value/placeholder */}
      <TouchableOpacity
        style={[
          styles.selectButton,
          value && styles.selectButtonActive,
          error && styles.inputWrapperError,
        ]}
        onPress={onPress}
      >
        <View style={{ flexDirection: 'row', alignItems: 'center' }}>
          {icon}
          <Text style={[styles.selectText, value && styles.selectTextActive]}>
            {value || placeholder}
          </Text>
        </View>
        <ChevronDown size={16} color={colors.textSecondary} />
      </TouchableOpacity>
      {/* Error message if present */}
      {error && <Text style={styles.errorText}>{error}</Text>}
    </View>
  );
};

export default SelectForm;
