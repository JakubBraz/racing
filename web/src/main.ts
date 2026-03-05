import { InputManager } from "./input";
import { NetworkClient, VehicleState } from "./network";
import { Renderer } from "./renderer";

const canvas = document.getElementById("game") as HTMLCanvasElement;
const renderer = new Renderer(canvas);
const input = new InputManager();
const network = new NetworkClient();

let playerId: number | null = null;
let latestVehicles: VehicleState[] = [];
let latestTick = 0;

network.setOnJoined((id) => {
  playerId = id;
  console.log(`Joined as player ${id}`);
});

network.setOnState((tick, vehicles) => {
  latestTick = tick;
  latestVehicles = vehicles;
});

network.setOnDisconnect(() => {
  playerId = null;
  latestVehicles = [];
  console.log("Disconnected from server");
});

input.setOnChange((left, right) => {
  network.sendInput(left, right);
});

network.connect("ws://localhost:8080");

function gameLoop() {
  renderer.render(latestVehicles, playerId, latestTick);
  requestAnimationFrame(gameLoop);
}

requestAnimationFrame(gameLoop);
