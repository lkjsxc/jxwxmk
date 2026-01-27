export interface AchievementInfo {
  id: string;
  name: string;
  description: string;
}

export const ALL_ACHIEVEMENTS: AchievementInfo[] = [
  {
    id: "first_steps",
    name: "First Steps",
    description: "Walk 100 steps.",
  },
];
