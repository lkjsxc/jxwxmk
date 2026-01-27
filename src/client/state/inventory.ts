export interface ItemStack {
  item: string;
  count: number;
}

export interface InventorySlot {
  item: ItemStack | null;
}

export interface InventoryState {
  slots: InventorySlot[];
  activeSlot: number;
}

export function createEmptyInventory(size: number): InventoryState {
  return {
    slots: Array.from({ length: size }, () => ({ item: null })),
    activeSlot: 0,
  };
}
