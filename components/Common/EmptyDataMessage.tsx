import React, { ReactNode } from 'react';
import { View, Text, StyleSheet, ViewStyle, TextStyle } from 'react-native';
import { useTheme } from '@react-navigation/native';
import { ColorType } from '@/constants/type';

type Props = {
  message?: string;
  subtext?: string;
  icon?:ReactNode  ;
  colors:ColorType
};

export const EmptyDataMessage: React.FC<Props> = ({
  message,
  subtext,
  icon,
  colors
}) => {
  const styles = StyleSheet.create({
    emptyContainer: {
      flex: 1,
      justifyContent: 'center',
      alignItems: 'center',
      paddingVertical: 60,
    },
    emptyIcon: {
      marginBottom: 16,
      opacity: 0.5,
    },
    emptyText: {
      fontSize: 18,
      fontFamily: 'Inter-SemiBold',
      color: colors.textSecondary,
      textAlign: 'center',
      marginBottom: 8,
    },
    emptySubtext: {
      fontSize: 14,
      fontFamily: 'Inter-Regular',
      color: colors.textSecondary,
      textAlign: 'center',
    },
  });

  return (
    <View style={styles.emptyContainer}>
      {icon}
      <Text style={styles.emptyText}>{message}</Text>
      <Text style={styles.emptySubtext}>{subtext}</Text>
    </View>
  );
};