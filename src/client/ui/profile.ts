import { PlayerSession } from "../state/player";

export interface ProfileLayout {
  copyButton: { x: number; y: number; w: number; h: number };
  nameField: { x: number; y: number; w: number; h: number };
  nameButton: { x: number; y: number; w: number; h: number };
  loginField: { x: number; y: number; w: number; h: number };
  loginButton: { x: number; y: number; w: number; h: number };
}

export function drawProfile(
  ctx: CanvasRenderingContext2D,
  session: PlayerSession,
  nameBuffer: string,
  loginBuffer: string,
): ProfileLayout {
  const panelX = 80;
  const panelY = 120;
  const panelW = ctx.canvas.width - 160;
  const panelH = ctx.canvas.height - 200;

  ctx.save();
  ctx.fillStyle = "rgba(15,23,42,0.9)";
  ctx.fillRect(panelX, panelY, panelW, panelH);
  ctx.fillStyle = "#e2e8f0";
  ctx.font = "16px Space Grotesk";
  ctx.fillText("Profile", panelX + 20, panelY + 28);

  ctx.font = "13px Space Grotesk";
  ctx.fillText(`Player ID: ${session.id ?? ""}`, panelX + 20, panelY + 60);
  const copyButton = { x: panelX + 360, y: panelY + 44, w: 60, h: 22 };
  ctx.fillStyle = "#22c55e";
  ctx.fillRect(copyButton.x, copyButton.y, copyButton.w, copyButton.h);
  ctx.fillStyle = "#0f172a";
  ctx.fillText("Copy", copyButton.x + 14, copyButton.y + 16);
  ctx.fillText(`Session: ${session.sessionRevoked ? "revoked" : "connected"}`, panelX + 20, panelY + 80);

  const nameY = panelY + 120;
  ctx.fillText("Name:", panelX + 20, nameY);
  ctx.fillStyle = "#0f172a";
  const nameField = { x: panelX + 80, y: nameY - 14, w: 220, h: 24 };
  ctx.fillRect(nameField.x, nameField.y, nameField.w, nameField.h);
  ctx.fillStyle = "#e2e8f0";
  ctx.fillText(nameBuffer, panelX + 86, nameY + 2);

  const nameButton = { x: panelX + 320, y: nameY - 18, w: 120, h: 28 };
  ctx.fillStyle = "#38bdf8";
  ctx.fillRect(nameButton.x, nameButton.y, nameButton.w, nameButton.h);
  ctx.fillStyle = "#0f172a";
  ctx.fillText("Update Name", nameButton.x + 12, nameButton.y + 18);

  const loginY = nameY + 60;
  ctx.fillStyle = "#e2e8f0";
  ctx.fillText("Device Login:", panelX + 20, loginY);
  ctx.fillStyle = "#0f172a";
  const loginField = { x: panelX + 140, y: loginY - 14, w: 260, h: 24 };
  ctx.fillRect(loginField.x, loginField.y, loginField.w, loginField.h);
  ctx.fillStyle = "#e2e8f0";
  ctx.fillText(loginBuffer, panelX + 146, loginY + 2);

  const loginButton = { x: panelX + 20, y: loginY + 16, w: 220, h: 28 };
  ctx.fillStyle = "#f97316";
  ctx.fillRect(loginButton.x, loginButton.y, loginButton.w, loginButton.h);
  ctx.fillStyle = "#0f172a";
  ctx.fillText("Login on this device", loginButton.x + 10, loginButton.y + 18);

  ctx.restore();
  return { copyButton, nameField, nameButton, loginField, loginButton };
}
