// ImageUrlModal component: A modal dialog for entering an image URL, validating it, and previewing the image before confirming
import { ColorType } from '@/constants/type';
import React, { useState } from 'react';
import { Modal, View, Text, TextInput, TouchableOpacity, Image, StyleSheet } from 'react-native';

// Props for the modal, including visibility, initial URL, callbacks, and theming
interface ImageUrlModalProps {
  visible: boolean;
  initialUrl?: string;
  onClose: () => void;
  onValidate: (url: string) => void;
  colors: ColorType
}

const ImageUrlModal: React.FC<ImageUrlModalProps> = ({ visible, initialUrl = '', onClose, onValidate, colors }) => {
  // State for the input URL, validation status, and check trigger
  const [url, setUrl] = useState(initialUrl);
  const [checked, setChecked] = useState(false);
  const [valid, setValid] = useState<boolean | null>(null);

  // Styles for the modal and its elements
  const styles = StyleSheet.create({
    modalOverlay: {
      flex: 1,
      backgroundColor: 'rgba(0, 0, 0, 0.5)',
      justifyContent: 'center',
      alignItems: 'center',
    },
    modalContent: {
      backgroundColor: colors.surface,
      borderRadius: 16,
      padding: 20,
      margin: 20,
      width: 320,
      alignItems: 'center',
    },
    modalTitle: {
      fontSize: 18,
      fontFamily: 'Inter-Bold',
      color: colors.text,
      marginBottom: 16,
      textAlign: 'center',
    },
    input: {
      width: '100%',
      backgroundColor: colors.background,
      borderRadius: 12,
      borderWidth: 1,
      borderColor: colors.border,
      paddingHorizontal: 16,
      paddingVertical: 12,
      fontSize: 14,
      fontFamily: 'Inter-Regular',
      color: colors.text,
      marginBottom: 12,
    },
    buttonRow: {
      flexDirection: 'row',
      justifyContent: 'space-between',
      width: '100%',
      marginBottom: 12,
      gap: 12,
    },
    button: {
      flex: 1,
      alignItems: 'center',
      paddingVertical: 12,
      borderRadius: 8,
      backgroundColor: colors.primary,
    },
    buttonText: {
      fontSize: 14,
      fontFamily: 'Inter-SemiBold',
      color: '#000',
    },
    previewContainer: {
      alignItems: 'center',
      marginTop: 12,
    },
    imagePreview: {
      width: 120,
      height: 120,
      borderRadius: 8,
      marginBottom: 8,
      backgroundColor: colors.background,
    },
    confirmButton: {
      backgroundColor: colors.success,
      borderRadius: 8,
      paddingVertical: 8,
      paddingHorizontal: 16,
      marginTop: 8,
    },
    confirmText: {
      color: '#FFF',
      fontWeight: 'bold',
      fontSize: 14,
    },
    notFound: {
      color: colors.error,
      marginTop: 8,
      fontSize: 14,
      fontFamily: 'Inter-SemiBold',
    },
  });

  // Validate the image URL by trying to get its size
  const handleValidate = () => {
    setChecked(true);
    if (!url.trim()) {
      setValid(false);
      return;
    }
    Image.getSize(
      url,
      () => setValid(true),
      () => setValid(false)
    );
  };

  // Confirm the selected image URL and close the modal
  const handleConfirm = () => {
    if (valid) {
      onValidate(url);
      onClose();
      setChecked(false);
      setValid(null);
    }
  };

  // Reset state when modal is closed or opened
  React.useEffect(() => {
    if (visible) {
      setUrl(initialUrl);
      setChecked(false);
      setValid(null);
    }
  }, [visible, initialUrl]);

  return (
    <Modal visible={visible} transparent animationType="fade">
      <View style={styles.modalOverlay}>
        <View style={styles.modalContent}>
          {/* Modal title and input field for the image URL */}
          <Text style={styles.modalTitle}>Enter Image URL</Text>
          <TextInput
            style={styles.input}
            placeholder="https://example.com/image.jpg"
            placeholderTextColor={colors.textSecondary}
            value={url}
            onChangeText={text => {
              setUrl(text);
              setChecked(false);
              setValid(null);
            }}
            autoCapitalize="none"
            autoCorrect={false}
          />
          {/* Row of Cancel and Check buttons */}
          <View style={styles.buttonRow}>
            <TouchableOpacity style={styles.button} onPress={onClose}>
              <Text style={styles.buttonText}>Cancel</Text>
            </TouchableOpacity>
            <TouchableOpacity style={styles.button} onPress={handleValidate}>
              <Text style={styles.buttonText}>Check</Text>
            </TouchableOpacity>
          </View>
          {/* Conditional rendering: show image preview or error message after checking */}
          {checked && (
            valid ? (
              <View style={styles.previewContainer}>
                <Image source={{ uri: url }} style={styles.imagePreview} />
                <TouchableOpacity style={styles.confirmButton} onPress={handleConfirm}>
                  <Text style={styles.confirmText}>Use this image</Text>
                </TouchableOpacity>
              </View>
            ) : (
              <Text style={styles.notFound}>Image not found or invalid URL</Text>
            )
          )}
        </View>
      </View>
    </Modal>
  );
};

export default ImageUrlModal;