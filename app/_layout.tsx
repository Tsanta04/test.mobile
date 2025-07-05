import { useEffect } from 'react';
import { Stack } from 'expo-router';
import { StatusBar } from 'expo-status-bar';
import { useFrameworkReady } from '@/hooks/useFrameworkReady';
import { useFonts, Inter_400Regular, Inter_500Medium, Inter_600SemiBold, Inter_700Bold } from '@expo-google-fonts/inter';
import * as SplashScreen from 'expo-splash-screen';
import { ThemeProvider } from '@/contexts/ThemeContext';
import { AuthProvider } from '@/contexts/AuthContext';
import { DataProvider } from '@/contexts/DataContext';
import { NotifProvider } from '@/contexts/NotifContext';
import { LoadingProvider } from '@/contexts/LoadingContext';
import { GlobalLoadingSpinner } from '@/components/Common/GlobalLoadingSpinner';
import AsyncStorage from '@react-native-async-storage/async-storage';

SplashScreen.preventAutoHideAsync();

export default function RootLayout() {
  useFrameworkReady();

  const [fontsLoaded, fontError] = useFonts({
    'Inter-Regular': Inter_400Regular,
    'Inter-Medium': Inter_500Medium,
    'Inter-SemiBold': Inter_600SemiBold,
    'Inter-Bold': Inter_700Bold,
  });

  useEffect(() => {
    if (fontsLoaded || fontError) {
      SplashScreen.hideAsync();
      AsyncStorage.clear();
    }
  }, [fontsLoaded, fontError]);

  if (!fontsLoaded && !fontError) {
    return null;
  }  

  return (
    <ThemeProvider>
      <LoadingProvider>
        <AuthProvider>
          <NotifProvider>
            <DataProvider>
              <Stack screenOptions={{ headerShown: false }}>
                <Stack.Screen name="welcome" />
                <Stack.Screen name="(auth)" />
                <Stack.Screen name="(tabs)" />
                <Stack.Screen name="analytics" />
                <Stack.Screen name="+not-found" />
              </Stack>
              <StatusBar style="auto" />
              <GlobalLoadingSpinner />
            </DataProvider>
          </NotifProvider>
        </AuthProvider>
      </LoadingProvider>
    </ThemeProvider>
  );
}