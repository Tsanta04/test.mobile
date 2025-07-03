import { useLocalSearchParams } from 'expo-router';
import { StyleSheet, Text, View } from 'react-native';

export default function EditProductScreen() {
    const { id } = useLocalSearchParams<{ id: string }>();    
  return (
    <View style={styles.container}>
      <Text style={styles.title}>MODIFIER LE PRODUIT DE ID {id}</Text>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    alignItems: 'center',
    justifyContent: 'center',
  },
  title: {
    fontSize: 20,
    fontWeight: 'bold',
  },
});
