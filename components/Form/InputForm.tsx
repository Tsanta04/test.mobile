// InputForm component: A reusable input field for forms, supporting label, error, icon, and theming
import React, { ReactNode } from 'react';
import { View, Text, TextInput, StyleSheet, TextStyle, ViewStyle, KeyboardTypeOptions, TextInputProps } from 'react-native';
import { ColorType } from '@/constants/type';

// Props for the input field, including label, value, error, icon, and theming
interface InputFormProps {
    label: string;
    value: string;
    requiredSign:boolean;    
    onChangeText: (text: string) => void;
    placeholder?: string;
    error?: string;
    colors: ColorType;
    multiline?: boolean;
    numberOfLines?: number;
    keyboardType?: TextInputProps['keyboardType'];    
    icon?: ReactNode;    
    secureTextEntry?:boolean;
    autoCapitalize?:TextInputProps['autoCapitalize'];
    autoCorrect?:boolean;
    rightElement?: ReactNode;
}
  
const InputForm: React.FC<InputFormProps> = ({
    label,
    requiredSign,
    error,
    colors,
    icon,
    ...props
}) => {
  // Styles for the input field and its states
  const styles = StyleSheet.create({
    inputContainer: {
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
      inputWrapper: {
        flexDirection: 'row',
        alignItems: 'center',
        backgroundColor: colors.surface,
        borderRadius: 12,
        borderWidth: 1,
        borderColor: colors.border,
        paddingHorizontal: 16,
        paddingVertical: 12,
      },
      inputWrapperError: {
        borderColor: colors.error,
      },
      icon: {
        marginRight: 12,
      },
      input: {
        flex: 1,
        fontSize: 14,
        fontFamily: 'Inter-Regular',
        color: colors.text,
      },
      textArea: {
        minHeight: 80,
        textAlignVertical: 'top',
      },
      errorText: {
        fontSize: 12,
        fontFamily: 'Inter-Regular',
        color: colors.error,
        marginTop: 4,
      }
  });

  return (
    <View style={styles.inputContainer}>
      {/* Label for the input field */}
      <Text style={styles.label}>
        {label} {requiredSign && <Text style={styles.required}>*</Text>}
      </Text>
      {/* Input field with optional icon and error styling */}
      <View style={[styles.inputWrapper, error && styles.inputWrapperError]}>
        {icon}        
        <TextInput
            style={[styles.input]}
            placeholderTextColor={colors.textSecondary}
            {...props}
        />
        {props.rightElement}
      </View>
      {/* Error message if present */}
      {error && <Text style={styles.errorText}>{error}</Text>}
    </View>
  );
}

export default InputForm;