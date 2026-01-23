export class MathUtils {
    public static clamp(value: number, min: number, max: number): number {
        return Math.min(Math.max(value, min), max);
    }

    public static lerp(start: number, end: number, t: number): number {
        return start + (end - start) * t;
    }

    public static randomRange(min: number, max: number): number {
        return Math.random() * (max - min) + min;
    }

    public static randomInt(min: number, max: number): number {
        return Math.floor(Math.random() * (max - min + 1)) + min;
    }

    public static degreeToRadians(degrees: number): number {
        return degrees * (Math.PI / 180);
    }

    public static radiansToDegrees(radians: number): number {
        return radians * (180 / Math.PI);
    }
}