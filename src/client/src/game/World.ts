export class World {
    public width: number;
    public height: number;
    public resources: Resource[];
    public entities: any[];
    
    constructor(width: number, height: number) {
        this.width = width;
        this.height = height;
        this.resources = [];
        this.entities = [];
    }
    
    public update(deltaTime: number): void {
        // Update world state
        // In a real implementation, this would handle:
        // - Resource respawns
        // - Entity movement
        // - Day/night cycle
        // - Weather effects
    }
    
    public addResource(resource: Resource): void {
        this.resources.push(resource);
    }
    
    public removeResource(resourceId: string): boolean {
        const index = this.resources.findIndex(r => r.id === resourceId);
        if (index >= 0) {
            this.resources.splice(index, 1);
            return true;
        }
        return false;
    }
    
    public getResourcesInArea(x: number, y: number, radius: number): Resource[] {
        return this.resources.filter(resource => {
            const dx = resource.position.x - x;
            const dy = resource.position.y - y;
            return Math.sqrt(dx * dx + dy * dy) <= radius;
        });
    }
}

export interface Resource {
    id: string;
    type: ResourceType;
    position: { x: number; y: number };
    quantity: number;
    maxQuantity: number;
    biome: BiomeType;
}

export enum ResourceType {
    Tree = 'tree',
    Rock = 'rock',
    Bush = 'bush',
    Ore = 'ore',
    Water = 'water'
}

export enum BiomeType {
    Forest = 'forest',
    Desert = 'desert',
    Mountain = 'mountain',
    Ocean = 'ocean',
    Grassland = 'grassland'
}