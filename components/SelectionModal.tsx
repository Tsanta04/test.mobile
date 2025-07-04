import { ColorType } from '@/constants/type';
import React from 'react';
import {
  Modal,
  View,
  Text,
  FlatList,
  TouchableOpacity,
  StyleSheet,
} from 'react-native';

// Define the shape of an item in the selection list
type Item = {
  id: string;
  name: string;
};

// Define the props interface for the modal component
interface SelectionModalProps {
  visible: boolean;         
  title: string;           
  data: Item[];              
  onSelect: (item: Item) => void; 
  onClose: () => void;         
  colors: ColorType;           
}

/**
 * SelectionModal Component
 * Displays a modal with a selectable list of items.
 * Useful for choosing categories, sellers, etc.
 */
const SelectionModal: React.FC<SelectionModalProps> = ({
  visible,
  title,
  data,
  onSelect,
  onClose,
  colors
}) => {

  // Define styles for the modal using the theme colors
  const styles = StyleSheet.create({
    modalOverlay: {
        flex: 1,
        backgroundColor: 'rgba(0, 0, 0, 0.5)', // Semi-transparent backdrop
        justifyContent: 'center',
        alignItems: 'center',
    },
    modalContent: {
        backgroundColor: colors.surface,
        borderRadius: 16,
        padding: 20,
        margin: 20,
        maxHeight: '60%',
        width: '80%',
    },
    modalTitle: {
        fontSize: 18,
        fontFamily: 'Inter-Bold',
        color: colors.text,
        marginBottom: 16,
        textAlign: 'center',
    },
    optionItem: {
        paddingVertical: 12,
        paddingHorizontal: 16,
        backgroundColor: colors.background,
        borderRadius: 8,
        marginBottom: 8,
    },
    optionText: {
        fontSize: 14,
        fontFamily: 'Inter-Regular',
        color: colors.text,
    },
    modalActions: {
        flexDirection: 'row',
        gap: 12,
        marginTop: 16,
    },
    modalButton: {
        flex: 1,
        paddingVertical: 12,
        borderRadius: 8,
        alignItems: 'center',
        backgroundColor: colors.primary,
    },
    modalButtonText: {
        fontSize: 14,
        fontFamily: 'Inter-SemiBold',
        color: '#000',
    },
  });

  return (
    <Modal visible={visible} transparent animationType="fade">
      <View style={styles.modalOverlay}>
        <View style={styles.modalContent}>
          
          {/* Modal title */}
          <Text style={styles.modalTitle}>{title}</Text>

          {/* List of selectable items */}
          <FlatList
            data={data}
            keyExtractor={(item) => item.id}
            renderItem={({ item }) => (
              <TouchableOpacity
                style={styles.optionItem}
                onPress={() => onSelect(item)} // Call onSelect when tapped
              >
                <Text style={styles.optionText}>{item.name}</Text>
              </TouchableOpacity>
            )}
          />

          {/* Modal close button */}
          <View style={styles.modalActions}>
            <TouchableOpacity style={styles.modalButton} onPress={onClose}>
              <Text style={styles.modalButtonText}>Close</Text>
            </TouchableOpacity>
          </View>
          
        </View>
      </View>
    </Modal>
  );
};

export default SelectionModal;
