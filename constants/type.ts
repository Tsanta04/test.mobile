import { ColorValue } from "react-native";

export  type ColorType = {                       
    background: string;
    surface: string;
    primary: string;
    secondary: string;
    text: string;
    textSecondary: string;
    border: string;
    error: string;
    success: string;
    warning: string;
}

export type ProductType = {
    name: string;
    description: string;
    price: string;
    stock: string;
    category: string;
    seller: string;
    image: string;
}

export type ProductFormProps = {
    handleSave: () => void, 
    handleAddCategory: (newCategoryName: string) => void, 
    handleAddSeller: (newSellerName: string) => void, 
    formData: ProductType, 
    setFormData: (formData: ProductType) => void,
    title: string
}

export type ChartDataPoint = {
    label: string;
    value: number;
    color?: ColorValue;
}

export type StatProductType = {
    totalProducts:number;
    totalValue:number;
    totalStock:number;
    averagePrice:number;
    highestPriced:number;
    lowestPriced:number;
    averageStock:number;
    categoryData: ChartDataPoint[];
    sellerData: ChartDataPoint[];    
    priceRangeData:ChartDataPoint[];
    stockRangeData :ChartDataPoint[]
}

