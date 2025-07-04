import React from 'react';
import { View, Text, TextInput, TouchableOpacity, TextStyle, ViewStyle, StyleSheet } from 'react-native';
import { LinearGradient } from 'expo-linear-gradient';
import { User, Mail, X, Save } from 'lucide-react-native';
import { ColorType } from '@/constants/type';

interface EditProfileFormProps {
  editName: string;
  setEditName: (name: string) => void;
  editEmail: string;
  setEditEmail: (email: string) => void;
  handleCancelEdit: () => void;
  handleSaveProfile: () => void;
  isLoading: boolean;
  colors: ColorType;
}

export default function EditProfileForm({
  editName,
  setEditName,
  editEmail,
  setEditEmail,
  handleCancelEdit,
  handleSaveProfile,
  isLoading,
  colors,
}: EditProfileFormProps) {

    const styles = StyleSheet.create({
        editForm: {
            width: '100%',
            gap: 16,
        },
        inputContainer: {
            gap: 6,
        },
        label: {
            fontSize: 14,
            fontFamily: 'Inter-Medium',
            color: colors.text,
        },
        inputWrapper: {
            flexDirection: 'row',
            alignItems: 'center',
            backgroundColor: colors.background,
            borderRadius: 12,
            borderWidth: 1,
            borderColor: colors.border,
            paddingHorizontal: 16,
            paddingVertical: 12,
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
        editActions: {
            flexDirection: 'row',
            gap: 12,
            marginTop: 8,
        },
        editButton: {
            flex: 1,
            borderRadius: 12,
            overflow: 'hidden',
        },
        editGradient: {
            paddingVertical: 12,
            paddingHorizontal: 16,
            flexDirection: 'row',
            alignItems: 'center',
            justifyContent: 'center',
        },
        editButtonText: {
            fontSize: 14,
            fontFamily: 'Inter-SemiBold',
            color: '#000',
            marginLeft: 8,
        },
        cancelButton: {
            flex: 1,
            backgroundColor: colors.surface,
            borderRadius: 12,
            borderWidth: 1,
            borderColor: colors.border,
            paddingVertical: 12,
            paddingHorizontal: 16,
            flexDirection: 'row',
            alignItems: 'center',
            justifyContent: 'center',
        },
        cancelButtonText: {
            fontSize: 14,
            fontFamily: 'Inter-SemiBold',
            color: colors.text,
            marginLeft: 8,
        }
        });    

  return (
    <View style={styles.editForm}>
      <View style={styles.inputContainer}>
        <Text style={styles.label}>Full Name</Text>
        <View style={styles.inputWrapper}>
          <User size={18} color={colors.textSecondary} style={styles.icon} />
          <TextInput
            style={styles.input}
            value={editName}
            onChangeText={setEditName}
            placeholder="Enter your name"
            placeholderTextColor={colors.textSecondary}
            autoCapitalize="words"
          />
        </View>
      </View>

      <View style={styles.inputContainer}>
        <Text style={styles.label}>Email</Text>
        <View style={styles.inputWrapper}>
          <Mail size={18} color={colors.textSecondary} style={styles.icon} />
          <TextInput
            style={styles.input}
            value={editEmail}
            onChangeText={setEditEmail}
            placeholder="Enter your email"
            placeholderTextColor={colors.textSecondary}
            keyboardType="email-address"
            autoCapitalize="none"
          />
        </View>
      </View>

      <View style={styles.editActions}>
        <TouchableOpacity
          style={styles.cancelButton}
          onPress={handleCancelEdit}
        >
          <X size={16} color={colors.text} />
          <Text style={styles.cancelButtonText}>Cancel</Text>
        </TouchableOpacity>

        <TouchableOpacity
          style={styles.editButton}
          onPress={handleSaveProfile}
          disabled={isLoading}
        >
          <LinearGradient
            colors={[colors.success, '#059669']}
            style={styles.editGradient}
          >
            <Save size={16} color="#FFF" />
            <Text style={[styles.editButtonText, { color: '#FFF' }]}>
              {isLoading ? 'Saving...' : 'Save'}
            </Text>
          </LinearGradient>
        </TouchableOpacity>
      </View>
    </View>
  );
}
