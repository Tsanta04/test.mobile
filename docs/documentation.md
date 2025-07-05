# Documentation de l'Architecture - Food Truck Mobile App

## Vue d'ensemble

Cet application est une solution mobile moderne développée avec **React Native** et **Expo**, utilisant une architecture basée sur les **Contextes React** pour la gestion d'état global et **Expo Router** pour la navigation.

## Architecture générale

### Stack technologique

- **Frontend** : React Native 0.79.4
- **Framework** : Expo SDK 53
- **Navigation** : Expo Router 5.1.2
- **Langage** : TypeScript 5.8.3
- **Gestion d'état** : React Context API
- **Stockage local** : AsyncStorage
- **UI Components** : React Native Elements + Lucide React Native
- **Animations** : React Native Reanimated

### Architecture en couches

```
┌─────────────────────────────────────┐
│           UI Layer                  │
│  (Components, Screens, Navigation)  │
├─────────────────────────────────────┤
│         Business Logic              │
│    (Contexts, Services, ...)        │
├─────────────────────────────────────┤
│         Data Layer                  │
│    (AsyncStorage, JSON Data)        │
└─────────────────────────────────────┘
```

## Structure des dossiers

### `/app` - Pages et Navigation (Expo Router)

L'application utilise le système de routage basé sur les fichiers d'Expo Router :

```
app/
├── _layout.tsx              # Layout racine avec providers
├── welcome.tsx              # Page d'accueil / page de bienvenue
├── (auth)/                  # Groupe de routes d'authentification
│   ├── _layout.tsx         # Layout pour les pages auth
│   ├── login.tsx           # Page de connexion
│   ├── register.tsx        # Page d'inscription
│   └── changePassword.tsx  # Page de changement de mot de passe
├── (tabs)/                  # Groupe de routes principales
│   ├── _layout.tsx         # Layout avec navigation par onglets
│   ├── index.tsx           # Page principale (liste des produits)
│   ├── notifications.tsx   # Page des notifications
│   └── profile.tsx         # Page de profil
├── product/                 # Routes de gestion des produits
│   ├── [id].tsx            # Détail d'un produit
│   ├── add.tsx             # Ajout de produit
│   └── edit/
│       └── [id].tsx        # Modification de produit
├── analytics.tsx            # Page de statistiques des produits
└── +not-found.tsx          # Page 404
```

### `/components` - Composants réutilisables

Organisation modulaire des composants par domaine fonctionnel :

```
components/
├── Analytic/               # Composants de statistique
│   ├── DistributionChart.tsx       # Composant pour les stats par distibution (categorie, vendeur, stock...)
│   ├── OverviewStatistics.tsx      # Composant pour vue generale des produits
│   ├── SimpleChart.tsx             # Composant utilisé par DistributionChart
│   └── StatCard.tsx                # Composant utilisé par OverviewStatistics
├── Common/                 # Composants communs
│   ├── EmptyDataMessage.tsx        # Composant pour afficher le message si les données a afficher sont vides
│   ├── GlobalLoadingSpinner.tsx    # Composant de loading (avec le message)
│   ├── Header.tsx                  # Composant de en-tete des pages
│   ├── LoadingSpinner.tsx          # Composant utilisé par GlobalLoadingSpinner
│   ├── SelectionModal.tsx          # Composant pour afficher des options en modal (ex: Categorie, Vendeur...)
├── Form/                   # Composants de formulaire
│   ├── ButtonForm.tsx              # Composant de bouton utilisé dans tous les pages
│   ├── ImageUrlModal.tsx           # Composant pour entrer l' "url option" dans le formulaire
│   ├── InputForm.tsx               # Composant de input utilisé dans tous les pages
│   ├── ProductForm.tsx             # Composant pour produits (ajout et modification)
│   └── SelectForm.tsx              # Composant de select utilisé dans tous les pages
├── ProductList/            # Composants de liste
│   ├── FilterPanels.tsx            # Composant de filtre dans la page liste des produits
│   ├── Pagination.tsx              # Composant Pagination
│   └── ProductCard.tsx             # Composant pour afficher un de produit
├── Notification/           # Composants de notification
│   └── NotificationItem.tsx        # Composant pour une notification
├── Profil/                 # Composants de profil
│   └── EditProfil.tsx              # Composant pour un petit formulaire de modification de profil
```

### `/contexts` - Gestion d'état global

L'application utilise le pattern Context API de React pour la gestion d'état global :

```
contexts/
├── AuthContext.tsx         # Gestion de l'authentification
├── DataContext.tsx         # Gestion des données produits
├── LoadingContext.tsx      # Gestion des états de chargement
├── NotifContext.tsx        # Gestion des notifications
└── ThemeContext.tsx        # Gestion du thème (clair/sombre)
```

### `/data` - Données statiques

Données JSON pour le développement et les tests :

```
data/
├── categories.json         # Catégories de produits
├── products.json          # Produits de démonstration
├── sellers.json           # Vendeurs
└── users.json             # Utilisateurs de test
```

## 🔄 Flux de données

### Architecture de gestion d'état

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Components    │    │    Contexts     │    │   AsyncStorage  │
│                 │◄──►│                 │◄──►│                 │
│  (UI Layer)     │    │ (State Layer)   │    │ (Persistence)   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   User Input    │    │  State Updates  │    │  Data Storage   │
│                 │    │                 │    │                 │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Flux d'authentification

1. **Connexion** : `AuthContext.login()` → Validation → AsyncStorage → Mise à jour état
2. **Inscription** : `AuthContext.register()` → Hashage mot de passe → Ajout utilisateur
3. **Déconnexion** : `AuthContext.logout()` → Nettoyage AsyncStorage → Reset état

### Flux de gestion des produits

1. **Lecture** : `DataContext` → Chargement depuis JSON → Mise à jour état
2. **Création** : Formulaire → Validation → Ajout au contexte → Persistance
3. **Modification** : Formulaire → Validation → Mise à jour contexte → Persistance
4. **Suppression** : Confirmation → Suppression du contexte → Persistance

## Système d'authentification

### Architecture de sécurité

- **Hashage des mots de passe** : bcryptjs avec salt rounds = 10
- **Stockage sécurisé** : AsyncStorage pour la persistance locale
- **Validation** : Vérification email unique, formatage des données
- **Session** : Gestion automatique de la session utilisateur

### Interface AuthContext

```typescript
interface AuthContextType {
  user: User | null;
  isLoading: boolean;
  login: (email: string, password: string) => Promise<boolean>;
  register: (name: string, email: string, password: string) => Promise<boolean>;
  logout: () => Promise<void>;
  changePassword: (email: string, newPassword: string) => Promise<boolean>;
  updateProfile: (name: string, email: string) => Promise<boolean>;
}
```

## Système de données

### Modèle de données

#### Produit
```typescript
interface Product {
  id: string;
  name: string;
  description: string;
  price: number;
  stock: number;
  category: string;
  seller: string;
  image: string;
  isActive: boolean;
  createdBy: string;
  createdAt: string;
}
```

#### Utilisateur
```typescript
interface User {
  id: string;
  email: string;
  name: string;
  password: string; // Hashé en production
}
```

### Gestion des données

- **Source de données** : Fichiers JSON statiques pour le développement
- **Persistance** : AsyncStorage pour les données utilisateur
- **Cache** : État local dans les contextes React
- **Synchronisation** : Mise à jour en temps réel via les contextes

## Système de thème

### Architecture du thème

- **Provider** : `ThemeContext` gère l'état du thème
- **Persistance** : AsyncStorage pour sauvegarder la préférence
- **Application** : Couleurs et styles dynamiques
- **Composants** : Adaptation automatique au thème actuel

### Couleurs et styles

```typescript
// constants/Colors.ts
export default {
  light: {
    text: '#000',
    background: '#fff',
    tint: tintColorLight,
    tabIconDefault: '#ccc',
    tabIconSelected: tintColorLight,
  },
  dark: {
    text: '#fff',
    background: '#000',
    tint: tintColorDark,
    tabIconDefault: '#ccc',
    tabIconSelected: tintColorDark,
  },
};
```

## Système de notifications

### Architecture des notifications

- **Context** : `NotifContext` gère l'état des notifications
- **Types** : Succès, erreur, information, avertissement
- **Persistance** : Stockage local des notifications
- **Affichage** : Composants dédiés avec animations

### Interface de notification

```typescript
interface Notification {
  id: string;
  type: 'success' | 'error' | 'info' | 'warning';
  title: string;
  message: string;
  timestamp: Date;
  read: boolean;
}
```

## Navigation et routage

### Expo Router

L'application utilise Expo Router pour une navigation basée sur les fichiers :

- **Routage automatique** : Basé sur la structure des dossiers
- **Navigation imbriquée** : Groupes de routes pour l'organisation
- **Paramètres dynamiques** : Routes avec `[id].tsx`
- **Layouts** : Layouts spécifiques par groupe de routes

### Structure de navigation

```
Root Layout (_layout.tsx)
├── Welcome
├── Auth Group ((auth)/_layout.tsx)
│   ├── Login
│   ├── Register
│   └── Change Password
├── Tabs Group ((tabs)/_layout.tsx)
│   ├── Home (index)
│   ├── Notifications
│   └── Profile
├── Product Routes
│   ├── Product Detail ([id])
│   ├── Add Product
│   └── Edit Product ([id])
└── Analytics
```


## Performance et optimisation

### Stratégies d'optimisation

- **Lazy loading** : Chargement à la demande des composants
- **Bundle splitting** : Séparation du code par fonctionnalité

### Monitoring

- **Performance** : Métriques de rendu et navigation
- **Erreurs** : Capture et reporting des erreurs
- **Analytics** : Suivi des interactions utilisateur
- **Crash reporting** : Rapports de crash automatiques

## Évolutions futures

### Améliorations prévues

1. **Push notifications** : Notifications push avec Expo Notifications
2. **Offline mode** : Synchronisation hors ligne
3. **Responsive et design** : Plus moderne et mieux responsive
5. **Internationalisation** : Support multi-langues
6. **Accessibilité** : Amélioration de l'accessibilité

### Architecture évolutive

L'architecture actuelle est conçue pour faciliter l'évolution :

- **Séparation des couches** : Facilite l'ajout de nouvelles fonctionnalités
- **Contextes modulaires** : Permet l'ajout de nouveaux contextes
- **Composants réutilisables** : Réduction de la duplication de code
- **TypeScript** : Sécurité de type pour l'évolution du code
