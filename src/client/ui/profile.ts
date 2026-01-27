import { PlayerProfile } from "../state/player";

export function drawProfile(
  ctx: CanvasRenderingContext2D,
  x: number,
  y: number,
  width: number,
  height: number,
  profile: PlayerProfile,
  nameBuffer: string,
  loginBuffer: string
): {
  copyBox: { x: number; y: number; w: number; h: number };
  nameBox: { x: number; y: number; w: number; h: number };
  updateBox: { x: number; y: number; w: number; h: number };
  loginBox: { x: number; y: number; w: number; h: number };
  loginButton: { x: number; y: number; w: number; h: number };
} {
  ctx.fillStyle = "rgba(0,0,0,0.6)";
  ctx.fillRect(x, y, width, height);
  ctx.fillStyle = "white";
  ctx.font = "14px sans-serif";
  ctx.fillText(`Player ID: ${profile.id ?? ""}`, x + 12, y + 24);

  const copyBox = { x: x + width - 100, y: y + 10, w: 80, h: 22 };
  ctx.fillStyle = "rgba(255,255,255,0.2)";
  ctx.fillRect(copyBox.x, copyBox.y, copyBox.w, copyBox.h);
  ctx.fillStyle = "white";
  ctx.fillText("Copy", copyBox.x + 20, copyBox.y + 16);

  ctx.font = "12px sans-serif";
  ctx.fillText(`Level: ${profile.level}  XP: ${profile.xp}`, x + 12, y + 50);
  ctx.fillText(`Kills: ${profile.stats.kills}  Deaths: ${profile.stats.deaths}`, x + 12, y + 68);

  ctx.fillText("Name", x + 12, y + 96);
  const nameBox = { x: x + 60, y: y + 82, w: 200, h: 22 };
  ctx.fillStyle = "rgba(255,255,255,0.1)";
  ctx.fillRect(nameBox.x, nameBox.y, nameBox.w, nameBox.h);
  ctx.fillStyle = "white";
  ctx.fillText(nameBuffer, nameBox.x + 6, nameBox.y + 15);

  const updateBox = { x: nameBox.x + 210, y: nameBox.y, w: 80, h: 22 };
  ctx.fillStyle = "rgba(255,255,255,0.2)";
  ctx.fillRect(updateBox.x, updateBox.y, updateBox.w, updateBox.h);
  ctx.fillStyle = "white";
  ctx.fillText("Update", updateBox.x + 12, updateBox.y + 15);

  ctx.fillText("Device Login", x + 12, y + 130);
  const loginBox = { x: x + 12, y: y + 140, w: 260, h: 22 };
  ctx.fillStyle = "rgba(255,255,255,0.1)";
  ctx.fillRect(loginBox.x, loginBox.y, loginBox.w, loginBox.h);
  ctx.fillStyle = "white";
  ctx.fillText(loginBuffer, loginBox.x + 6, loginBox.y + 15);

  const loginButton = { x: loginBox.x + 270, y: loginBox.y, w: 120, h: 22 };
  ctx.fillStyle = "rgba(255,255,255,0.2)";
  ctx.fillRect(loginButton.x, loginButton.y, loginButton.w, loginButton.h);
  ctx.fillStyle = "white";
  ctx.fillText("Login", loginButton.x + 32, loginButton.y + 15);

  return { copyBox, nameBox, updateBox, loginBox, loginButton };
}
