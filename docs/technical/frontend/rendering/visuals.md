# Visual Effects & Animations

Advanced visual feedback for player actions and game events.

## Damage Feedback
- **Animation**: When an entity (Resource, Mob, Structure) takes damage, it scales up by 20% and then returns to its original size over 0.25 seconds.
- **Implementation**: Entities track a `lastHitAt` timestamp and calculate scale using a quadratic or linear curve.

## Cooldown Visuals (A/B Buttons)
- **Pie Chart Overlay**: A semi-transparent dark circle that shrinks (clock-like) to represent the remaining cooldown time.
- **Countdown**: The remaining seconds (e.g., "0.4") are displayed in the center of the button instead of the button label ("A" or "B") during the cooldown phase.
- **Transparency**: Buttons use a base alpha of 0.4, increasing to 0.8 when pressed.