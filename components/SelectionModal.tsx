import { ColorType } from '@/constants/type';
import { Check, Plus, Tag } from 'lucide-react-native';
import React, { useState } from 'react';
import {
  Modal,
  View,
  Text,
  FlatList,
  TouchableOpacity,
  StyleSheet,
  TextInput,
} from 'react-native';

// Define the shape of an item in the selection list
type Item = {
  id: string;
  name: string;
};

// Define the props interface for the modal component
// Use conditional types to make handleAddCategory required when enableToAdd is true
interface BaseSelectionModalProps {
  visible: boolean;         
  title: string;           
  data: Item[];              
  selected: string;
  onSelect: (item: Item) => void; 
  onClose: () => void;         
  colors: ColorType;           
  enableToAdd: boolean;
}

// When enableToAdd is true, handleAddCategory is required
interface SelectionModalPropsWithAdd extends BaseSelectionModalProps {
  enableToAdd: true;
  handleAddNew: (newName: string) => void;
}

// When enableToAdd is false, handleAddCategory is optional
interface SelectionModalPropsWithoutAdd extends BaseSelectionModalProps {
  enableToAdd: false;
  handleAddNew?: (newName: string) => void;
}

// Union type for the complete interface
type SelectionModalProps = SelectionModalPropsWithAdd | SelectionModalPropsWithoutAdd;

/**
 * SelectionModal Component
 * Displays a modal with a selectable list of items.
 * Useful for choosing categories, sellers, etc.
 */
const SelectionModal: React.FC<SelectionModalProps> = ({
  visible,
  title,
  data,
  selected,
  onSelect,
  onClose,
  colors,
  handleAddNew,
  enableToAdd
}) => {

  const [inputMode, setInputMode] = useState(!enableToAdd);
  const [newName, setNewName] = useState('');

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
        flexDirection: 'row',
        alignItems: 'center',        
        borderRadius: 8,
        marginBottom: 8,
    },
    optionText: {
        fontSize: 14,
        paddingHorizontal:10,
        fontFamily: 'Inter-Regular',
        color: colors.text,
    },
    addNewOption: {
      backgroundColor: colors.primary + '20',
      borderWidth: 1,
      borderColor: colors.primary,
    },
    addNewText: {
      color: colors.primary,
      fontFamily: 'Inter-SemiBold',
    },
    inputMode: {
      gap: 12,
    },
    inputModeWrapper: {
      flexDirection: 'row',
      gap: 12,
    },
    inputModeInput: {
      flex: 1,
      backgroundColor: colors.background,
      borderRadius: 12,
      borderWidth: 1,
      borderColor: colors.border,
      paddingHorizontal: 16,
      paddingVertical: 12,
      fontSize: 14,
      fontFamily: 'Inter-Regular',
      color: colors.text,
    },
    inputModeButton: {
      paddingHorizontal: 16,
      paddingVertical: 12,
      borderRadius: 12,
      justifyContent: 'center',
      alignItems: 'center',
    },
    inputModeButtonText: {
      fontSize: 14,
      fontFamily: 'Inter-SemiBold',
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
    optionItemSelected: {
      backgroundColor: colors.primary + '20',
      borderWidth: 1,
      borderColor: colors.primary,
    }    
  });

  return (
    <Modal visible={visible} transparent animationType="fade">
      <View style={styles.modalOverlay}>
        <View style={styles.modalContent}>
          
          {/* Modal title */}
          <Text style={styles.modalTitle}>{title}</Text>

          {/* List of selectable items */}
          {inputMode ? (
            <View style={styles.inputMode}>
              <View style={styles.inputModeWrapper}>
                <TextInput
                  style={styles.inputModeInput}
                  placeholder="Enter new category name"
                  placeholderTextColor={colors.textSecondary}
                  value={newName}
                  onChangeText={setNewName}
                  autoFocus
                />
                <TouchableOpacity
                  style={[styles.inputModeButton, { backgroundColor: colors.success }]}
                  onPress={() => {
                    if (handleAddNew && newName.trim()) {
                      handleAddNew(newName.trim());
                      setNewName('');
                      setInputMode(false);
                    }
                  }}
                >
                  <Text style={[styles.inputModeButtonText, { color: '#FFF' }]}>Add</Text>
                </TouchableOpacity>
              </View>
              <TouchableOpacity
                style={[styles.inputModeButton, { backgroundColor: colors.surface, borderWidth: 1, borderColor: colors.border }]}
                onPress={() => {
                  setInputMode(false);
                  setNewName('');
                }}
              >
                <Text style={[styles.inputModeButtonText, { color: colors.text }]}>Cancel</Text>
              </TouchableOpacity>
            </View>
          ) : (
            <FlatList
              data={[...data]}
              keyExtractor={(item) => item.id}
              renderItem={({ item }) => (
                <TouchableOpacity
                  style={[styles.optionItem, selected === item.name && styles.optionItemSelected]}
                  onPress={() => {
                    onSelect(item)
                  }}
                >
                  <Tag size={16} color={selected === item.name ? colors.primary : colors.textSecondary} />
                  <Text style={styles.optionText}>{item.name}</Text>
                  {selected === item.name && <Check size={16} color={colors.primary} />}
                </TouchableOpacity>
              )}
              ListFooterComponent={() => (
                  enableToAdd &&
                    <TouchableOpacity
                      style={[styles.optionItem, styles.addNewOption]}
                      onPress={() => setInputMode(true)}
                    >
                      <Plus size={16} color={colors.primary} />
                      <Text style={[styles.optionText, styles.addNewText]}>Add New Category</Text>
                    </TouchableOpacity>
              )}
            />
          )}

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
