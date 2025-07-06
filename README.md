# Food Truck Mobile App

Une application mobile moderne développée avec React Native et Expo pour la gestion de produits.

## Fonctionnalités

- **Authentification complète** : Inscription, connexion, changement de mot de passe
- **Gestion de produits** : Ajout, modification, suppression et visualisation de produits (Seul l'utilisateur ayant créé le produit peut le modifier ou le supprimer.)
- **Système de notifications** : Notifications en temps réel
- **Analytics** : Tableaux de bord avec statistiques et graphiques
- **Interface moderne** : Design responsive avec thème sombre/clair
- **Navigation intuitive** : Navigation par onglets avec Expo Router

## Prérequis

Avant de commencer, assurez-vous d'avoir installé :

- **Node.js** (version 18 ou supérieure)
- **npm** ou **yarn**
- **Expo CLI** : `npm install -g @expo/cli`
- **Git**

### Pour le développement mobile :
- **Expo Go** (application mobile pour tester)
- **Android Studio** (pour émulateur Android)
- **Xcode** (pour émulateur iOS - macOS uniquement)

## Installation

1. **Cloner le repository**
   ```bash
   git clone https://github.com/Tsanta04/test.mobile
   cd test.mobile
   ```

2. **Installer les dépendances**
   ```bash
   npm install
   # ou
   yarn install
   ```

## Lancement

### Développement local

1. **Démarrer le serveur de développement**
   ```bash
   npm start
   # ou
   yarn start
   ```

2. **Choisir la plateforme de test**
   - Appuyez sur `a` pour Android
   - Appuyez sur `i` pour iOS
   - Appuyez sur `w` pour Web
   - Scannez le QR code avec Expo Go sur votre téléphone

### Scripts disponibles

```bash
# Démarrer le serveur de développement
npm start

# Lancer sur Android
npm run android

# Lancer sur iOS
npm run ios

# Lancer sur Web
npm run web

# Lancer les tests
npm test
```

## 📱 Test sur appareil physique

1. **Installer Expo Go** sur votre téléphone
   - [Android](https://play.google.com/store/apps/details?id=host.exp.exponent)
   - [iOS](https://apps.apple.com/app/expo-go/id982107779)

2. **Scanner le QR code** affiché dans le terminal ou le navigateur

3. **Assurez-vous que votre téléphone et votre ordinateur sont sur le même réseau WiFi**

## Structure du projet

```
test.mobile/
├── app/                    # Pages et navigation (Expo Router)
│   ├── (auth)/            # Pages d'authentification
│   ├── (tabs)/            # Navigation par onglets
│   └── product/           # Pages de gestion des produits
├── components/            # Composants réutilisables
│   ├── Analytic/         # Composants d'analytics
│   ├── Common/           # Composants communs
│   ├── Form/             # Composants de formulaire
│   └── ProductList/      # Composants de liste de produits
├── contexts/             # Contextes React (état global)
├── data/                 # Données JSON statiques
└── constants/            # Constantes et types
```


## Dépannage

### Problèmes courants

1. **Erreur de dépendances**
   ```bash
   rm -rf node_modules
   npm install
   ```

2. **Cache Expo corrompu**
   ```bash
   expo r -c
   ```

3. **Problèmes de métro bundler**
   ```bash
   npx expo start --clear
   ```

### Logs et débogage

- Utilisez `console.log()` pour le débogage
- Les logs apparaissent dans le terminal de développement
- Pour les erreurs, vérifiez la console du navigateur (web) ou les logs Expo


## Support

Pour toute question ou problème :
- N'hésitez pas de à me contacter
- Consulter la documentation Expo : https://docs.expo.dev/
- Consulter la documentation React Native : https://reactnative.dev/docs/getting-started


##  Mises à jour

Pour mettre à jour les dépendances :

```bash
# Mettre à jour Expo CLI
npm install -g @expo/cli@latest

# Mettre à jour les dépendances
npm update

# Vérifier les vulnérabilités
npm audit
```
