export interface RecipeDisplay {
  id: string;
  name: string;
  inputs: { item: string; count: number }[];
}

export const CLIENT_RECIPES: RecipeDisplay[] = [
  { id: "WoodPickaxe", name: "Wood Pick", inputs: [{ item: "Wood", count: 10 }] },
  {
    id: "StonePickaxe",
    name: "Stone Pick",
    inputs: [
      { item: "Wood", count: 10 },
      { item: "Stone", count: 10 },
    ],
  },
  { id: "WoodWall", name: "Wood Wall", inputs: [{ item: "Wood", count: 20 }] },
  { id: "Door", name: "Door", inputs: [{ item: "Wood", count: 15 }] },
  { id: "Torch", name: "Torch", inputs: [{ item: "Wood", count: 2 }] },
  { id: "Workbench", name: "Workbench", inputs: [{ item: "Wood", count: 50 }] },
];
