# Podracer Arena

A physics-based vehicle game inspired by the podracers from Star Wars Episode I. Control a podracer using two independent engine thrusters in a top-down 2D arena.

## Architecture

```
Browser (TypeScript + Canvas)          Rust Server (Tokio + Rapier2D)
+-----------------------+              +----------------------------+
| Keyboard Input ---------- WebSocket ---> Receive Input            |
|                       |              | Apply Forces (Rapier2D)    |
| Canvas Renderer <-------- WebSocket <--- Broadcast State          |
+-----------------------+              +----------------------------+
```

**Server-authoritative**: all physics runs on the Rust server. The browser is a thin client that sends key presses and renders the state it receives at 60 Hz.

### Communication

WebSocket with JSON messages on port 8080:

```
Client -> Server: { "type": "input", "left": true, "right": false }
Server -> Client: { "type": "joined", "id": 1 }
Server -> Client: { "type": "state", "tick": 42, "vehicles": [...] }
```

## Vehicle Design

The podracer is a compound rigid body made of three circle colliders attached at fixed offsets:

```
        [L Engine]---o---[R Engine]      (front, engines pull the craft)
                     |
                     |  energy binders
                     |
                  [Cockpit]              (rear, trails behind)
```

### Layout (local coordinates, Y+ = forward)

| Part         | Position             | Radius | Density |
|--------------|----------------------|--------|---------|
| Left Engine  | (-1.5, +0.8)        | 0.5    | 2.0     |
| Right Engine | (+1.5, +0.8)        | 0.5    | 2.0     |
| Cockpit      | (0.0, -0.8)         | 0.4    | 1.5     |

Circle colliders are used instead of rectangles for smoother wall collisions and bouncing.

## Physics Model

### Engine: Rapier2D

Top-down 2D physics with zero gravity. The server runs a fixed 60 Hz timestep using Rapier2D.

### Controls

Two buttons: **Left Engine** (A / Left Arrow) and **Right Engine** (D / Right Arrow).

| Input              | Effect                                    |
|--------------------|-------------------------------------------|
| Both engines       | Strong forward thrust (42 units total)    |
| Single engine      | Weak forward thrust (5 units) + rotation  |
| No engines         | Vehicle decelerates via damping           |

### Force Application

The key design decision is using **separate `add_force` and `add_torque`** calls rather than `add_force_at_point`. This decouples forward motion from rotation, allowing independent tuning of each.

```
Both engines pressed:
  -> add_force(forward * BOTH_FORWARD_FORCE * 2)

Single engine pressed:
  -> add_force(forward * SINGLE_FORWARD_FORCE)
  -> add_torque(+/- TURN_TORQUE)
```

The **asymmetric thrust** model is critical: a single engine contributes very little forward force (5) compared to both engines (21 each, 42 total). This makes single-button presses primarily rotate the vehicle with minimal forward drift.

### Damping

Rapier2D's built-in exponential damping handles deceleration naturally:

- **Linear damping = 26**: vehicle stops within ~0.3s of releasing thrust
- **Angular damping = 10**: rotation stops quickly after releasing a single engine

No manual friction or velocity clamping is used. Rapier's damping model (`v *= 1 / (1 + dt * damping)`) provides smooth, natural deceleration.

### Tuning Parameters (V38)

```
BOTH_FORWARD_FORCE  = 21.0    # forward force per engine when both fire
SINGLE_FORWARD_FORCE = 5.0    # forward force when only one engine fires
TURN_TORQUE          = 10.0   # rotational torque from single engine
LINEAR_DAMPING       = 26.0   # how fast linear velocity decays
ANGULAR_DAMPING      = 10.0   # how fast angular velocity decays
ENGINE_DENSITY       = 2.0    # mass contribution from engines
COCKPIT_DENSITY      = 1.5    # mass contribution from cockpit
RESTITUTION          = 0.6    # bounciness of vehicle colliders
```

These values were found through ~40 iterations of evolutionary tuning with human-in-the-loop scoring (1-10 scale), reaching a score of 9.6/10.

## Arena

Rectangular arena: 160m x 120m (half-extents: 80 x 60). Bounded by static wall colliders with restitution 0.8 (bouncy walls). No obstacles currently.

## Project Structure

```
project3/
+-- Cargo.toml              # workspace root
+-- server/
|   +-- src/
|       +-- main.rs          # tokio runtime, WebSocket, 60 Hz game loop
|       +-- game.rs          # Rapier2D world, physics stepping
|       +-- vehicle.rs       # vehicle creation, thrust/torque application
|       +-- map.rs           # arena walls
|       +-- network.rs       # WebSocket message handling
|       +-- recording.rs     # input recording (disabled, for future use)
+-- shared/
|   +-- src/
|       +-- lib.rs
|       +-- protocol.rs      # ClientMessage / ServerMessage types
|       +-- types.rs         # VehicleState, Vec2
+-- web/
    +-- src/
        +-- main.ts          # entry point, game loop
        +-- renderer.ts      # Canvas 2D rendering
        +-- network.ts       # WebSocket client
        +-- input.ts         # keyboard capture
```

## Running

```bash
# Terminal 1: start the server
cd project3
cargo run

# Terminal 2: start the frontend
cd project3/web
npm install
npm run dev

# Open http://localhost:3000 in browser
```

## Future Plans

- Ghost replay system (record inputs, replay as ghost vehicles)
- Map editor
- Replay viewer
