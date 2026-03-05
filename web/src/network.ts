export interface VehicleState {
  id: number;
  x: number;
  y: number;
  angle: number;
  vx: number;
  vy: number;
  angular_vel: number;
  left_thrust: boolean;
  right_thrust: boolean;
  is_ghost: boolean;
}

export interface JoinedMessage {
  type: "joined";
  id: number;
}

export interface StateMessage {
  type: "state";
  tick: number;
  vehicles: VehicleState[];
}

export interface PlayerLeftMessage {
  type: "player_left";
  id: number;
}

export type ServerMessage = JoinedMessage | StateMessage | PlayerLeftMessage;

export class NetworkClient {
  private ws: WebSocket | null = null;
  private onJoined: ((id: number) => void) | null = null;
  private onState: ((tick: number, vehicles: VehicleState[]) => void) | null =
    null;
  private onDisconnect: (() => void) | null = null;

  connect(url: string) {
    this.ws = new WebSocket(url);

    this.ws.onmessage = (event) => {
      const msg: ServerMessage = JSON.parse(event.data);
      switch (msg.type) {
        case "joined":
          this.onJoined?.(msg.id);
          break;
        case "state":
          this.onState?.(msg.tick, msg.vehicles);
          break;
      }
    };

    this.ws.onclose = () => {
      this.onDisconnect?.();
    };

    this.ws.onerror = () => {
      this.onDisconnect?.();
    };
  }

  setOnJoined(cb: (id: number) => void) {
    this.onJoined = cb;
  }
  setOnState(cb: (tick: number, vehicles: VehicleState[]) => void) {
    this.onState = cb;
  }
  setOnDisconnect(cb: () => void) {
    this.onDisconnect = cb;
  }

  sendInput(left: boolean, right: boolean) {
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify({ type: "input", left, right }));
    }
  }
}
