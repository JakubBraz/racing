import type { VehicleState } from "./network";

const PIXELS_PER_METER = 25;

// Must match server layout constants
const ENGINE_RADIUS = 0.5;
const COCKPIT_RADIUS = 0.4;
const ENGINE_OFFSET_X = 1.5;
const ENGINE_OFFSET_Y = 0.8;
const COCKPIT_OFFSET_Y = -0.8;

const ARENA_HALF_WIDTH = 80.0;
const ARENA_HALF_HEIGHT = 60.0;

/** Transform a local offset to world position given body pos + angle */
function localToWorld(
  bx: number,
  by: number,
  angle: number,
  lx: number,
  ly: number
): [number, number] {
  const cos = Math.cos(angle);
  const sin = Math.sin(angle);
  return [bx + lx * cos - ly * sin, by + lx * sin + ly * cos];
}

export class Renderer {
  private canvas: HTMLCanvasElement;
  private ctx: CanvasRenderingContext2D;
  private cameraX = 0;
  private cameraY = 0;

  constructor(canvas: HTMLCanvasElement) {
    this.canvas = canvas;
    this.ctx = canvas.getContext("2d")!;
    this.resize();
    window.addEventListener("resize", () => this.resize());
  }

  private resize() {
    this.canvas.width = window.innerWidth;
    this.canvas.height = window.innerHeight;
  }

  render(vehicles: VehicleState[], playerId: number | null, tick: number = 0) {
    const ctx = this.ctx;
    const w = this.canvas.width;
    const h = this.canvas.height;

    const player = vehicles.find((v) => v.id === playerId);
    if (player) {
      this.cameraX = player.x;
      this.cameraY = player.y;
    }

    ctx.clearRect(0, 0, w, h);
    ctx.fillStyle = "#111";
    ctx.fillRect(0, 0, w, h);

    ctx.save();
    ctx.translate(w / 2, h / 2);
    ctx.scale(PIXELS_PER_METER, -PIXELS_PER_METER);
    ctx.translate(-this.cameraX, -this.cameraY);

    this.drawArena(ctx);

    for (const v of vehicles) {
      this.drawPodracer(ctx, v, v.id === playerId);
    }

    ctx.restore();
    this.drawHUD(ctx, vehicles, playerId, tick);
  }

  private drawArena(ctx: CanvasRenderingContext2D) {
    ctx.strokeStyle = "#444";
    ctx.lineWidth = 0.2;
    ctx.strokeRect(
      -ARENA_HALF_WIDTH,
      -ARENA_HALF_HEIGHT,
      ARENA_HALF_WIDTH * 2,
      ARENA_HALF_HEIGHT * 2
    );

    ctx.strokeStyle = "#333";
    ctx.lineWidth = 0.05;
    for (let x = -ARENA_HALF_WIDTH; x <= ARENA_HALF_WIDTH; x += 5) {
      ctx.beginPath();
      ctx.moveTo(x, -ARENA_HALF_HEIGHT);
      ctx.lineTo(x, ARENA_HALF_HEIGHT);
      ctx.stroke();
    }
    for (let y = -ARENA_HALF_HEIGHT; y <= ARENA_HALF_HEIGHT; y += 5) {
      ctx.beginPath();
      ctx.moveTo(-ARENA_HALF_WIDTH, y);
      ctx.lineTo(ARENA_HALF_WIDTH, y);
      ctx.stroke();
    }
  }

  private drawPodracer(
    ctx: CanvasRenderingContext2D,
    v: VehicleState,
    isPlayer: boolean
  ) {
    const alpha = v.is_ghost ? 0.4 : 1.0;

    const [leX, leY] = localToWorld(v.x, v.y, v.angle, -ENGINE_OFFSET_X, ENGINE_OFFSET_Y);
    const [reX, reY] = localToWorld(v.x, v.y, v.angle, ENGINE_OFFSET_X, ENGINE_OFFSET_Y);
    const [cpX, cpY] = localToWorld(v.x, v.y, v.angle, 0, COCKPIT_OFFSET_Y);

    // Energy binders
    this.drawEnergyBinder(ctx, cpX, cpY, leX, leY, isPlayer, alpha);
    this.drawEnergyBinder(ctx, cpX, cpY, reX, reY, isPlayer, alpha);

    // Engines
    this.drawEngine(ctx, leX, leY, v.angle, v.left_thrust, isPlayer, alpha);
    this.drawEngine(ctx, reX, reY, v.angle, v.right_thrust, isPlayer, alpha);

    // Cockpit
    this.drawCockpit(ctx, cpX, cpY, isPlayer, alpha);
  }

  private drawEnergyBinder(
    ctx: CanvasRenderingContext2D,
    x1: number, y1: number,
    x2: number, y2: number,
    isPlayer: boolean,
    alpha: number
  ) {
    ctx.strokeStyle = isPlayer
      ? `rgba(100, 180, 255, ${0.3 * alpha})`
      : `rgba(255, 100, 100, ${0.3 * alpha})`;
    ctx.lineWidth = 0.2;
    ctx.beginPath();
    ctx.moveTo(x1, y1);
    ctx.lineTo(x2, y2);
    ctx.stroke();

    ctx.strokeStyle = isPlayer
      ? `rgba(150, 220, 255, ${0.7 * alpha})`
      : `rgba(255, 150, 150, ${0.7 * alpha})`;
    ctx.lineWidth = 0.07;
    ctx.beginPath();
    ctx.moveTo(x1, y1);
    ctx.lineTo(x2, y2);
    ctx.stroke();
  }

  private drawEngine(
    ctx: CanvasRenderingContext2D,
    x: number, y: number, angle: number,
    thrusting: boolean,
    isPlayer: boolean,
    alpha: number
  ) {
    // Engine circle
    if (isPlayer) {
      ctx.fillStyle = `rgba(60, 80, 120, ${alpha})`;
      ctx.strokeStyle = `rgba(100, 140, 200, ${alpha})`;
    } else {
      ctx.fillStyle = `rgba(120, 60, 60, ${alpha})`;
      ctx.strokeStyle = `rgba(200, 100, 100, ${alpha})`;
    }
    ctx.lineWidth = 0.06;
    ctx.beginPath();
    ctx.arc(x, y, ENGINE_RADIUS, 0, Math.PI * 2);
    ctx.fill();
    ctx.stroke();

    // Thrust flame behind the engine
    if (thrusting) {
      const cos = Math.cos(angle);
      const sin = Math.sin(angle);
      // Flame position: behind the engine in the body's backward direction
      const flameX = x + sin * (ENGINE_RADIUS + 0.3);
      const flameY = y - cos * (ENGINE_RADIUS + 0.3);

      ctx.fillStyle = "rgba(255, 120, 0, 0.7)";
      ctx.beginPath();
      ctx.arc(flameX, flameY, 0.2, 0, Math.PI * 2);
      ctx.fill();
      ctx.fillStyle = "rgba(255, 255, 100, 0.9)";
      ctx.beginPath();
      ctx.arc(flameX, flameY, 0.1, 0, Math.PI * 2);
      ctx.fill();
    }
  }

  private drawCockpit(
    ctx: CanvasRenderingContext2D,
    x: number, y: number,
    isPlayer: boolean,
    alpha: number
  ) {
    if (isPlayer) {
      ctx.fillStyle = `rgba(50, 100, 60, ${alpha})`;
      ctx.strokeStyle = `rgba(80, 160, 100, ${alpha})`;
    } else {
      ctx.fillStyle = `rgba(100, 50, 60, ${alpha})`;
      ctx.strokeStyle = `rgba(160, 80, 100, ${alpha})`;
    }
    ctx.lineWidth = 0.06;
    ctx.beginPath();
    ctx.arc(x, y, COCKPIT_RADIUS, 0, Math.PI * 2);
    ctx.fill();
    ctx.stroke();

    // Windshield highlight
    if (isPlayer) {
      ctx.fillStyle = `rgba(120, 200, 255, ${0.4 * alpha})`;
    } else {
      ctx.fillStyle = `rgba(255, 180, 120, ${0.4 * alpha})`;
    }
    ctx.beginPath();
    ctx.arc(x, y, COCKPIT_RADIUS * 0.5, 0, Math.PI * 2);
    ctx.fill();
  }

  private drawHUD(
    ctx: CanvasRenderingContext2D,
    vehicles: VehicleState[],
    playerId: number | null,
    tick: number
  ) {
    const player = vehicles.find((v) => v.id === playerId);

    ctx.fillStyle = "#ff0";
    ctx.font = "bold 28px monospace";
    ctx.fillText("V38", this.canvas.width - 80, 35);

    ctx.fillStyle = "#aaa";
    ctx.font = "14px monospace";
    if (player) {
      const speed = Math.sqrt(player.vx ** 2 + player.vy ** 2).toFixed(1);
      ctx.fillText(`Speed: ${speed}  Tick: ${tick}`, 10, 20);
    }

    ctx.fillStyle = "#666";
    ctx.font = "12px monospace";
    ctx.fillText("A/D or Left/Right to thrust engines", 10, this.canvas.height - 10);
  }
}
