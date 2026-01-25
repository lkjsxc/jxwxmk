# Touch Input Implementation

## Event Listeners
- `touchstart`
- `touchmove`
- `touchend`

## Virtual Joystick Class
Calculates the delta vector from the initial touch point.

```typescript
class Joystick {
    origin: {x, y} | null;
    current: {x, y};
    
    update(touch) {
        // Calculate vector
        // Normalize to -1..1
    }
}
```

Multi-touch support is critical (Move + Attack simultaneously). `changedTouches` list must be iterated to track IDs.
