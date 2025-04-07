# Tower Defense Game - Networking Techniques

A Clash Royale-inspired multiplayer tower defense game built in Rust. Client uses Macroquad, server uses Tokio with UDP. Below are the techniques to implement for optimal performance:

## Server Techniques
- [ ] Server Reconciliation
- [ ] Lag Compensation
- [ ] Delta Compression
- [ ] Interest Management / Area of Interest
- [ ] Fixed Timestep
- [ ] Dead Reckoning
- [ ] Packet Batching and Coalescing
- [ ] Reliable UDP (Custom Protocol)

## Client Techniques
- [ ] Client Prediction
- [ ] Entity Interpolation
- [ ] Fixed Timestep with Rendering Interpolation
- [ ] Input Buffering
- [ ] Dead Reckoning
- [ ] Optimistic Rendering

## Shared
- [ ] Shared Game Logic (in `game_core` crate)
- [ ] Serialization with `serde` + `bincode`
- [ ] Clock Synchronization


## Status
- **Phase 1: Setup and Core Structure** - [ ] In Progress / [ ] Done
- **Phase 2: Core Gameplay Mechanics** - [ ] In Progress / [ ] Done
- **Phase 3: Networking Foundation** - [ ] In Progress / [ ] Done
- **Phase 4: Multiplayer Sync Techniques** - [ ] In Progress / [ ] Done
- **Phase 5: Optimization and Polish** - [ ] In Progress / [ ] Done
- **Phase 6: Testing and Deployment** - [ ] In Progress / [ ] Done

## Features
### Implemented
- [ ] Project workspace with `client`, `server`, `shared`
- [ ] Basic card and unit rendering (single-player)
- [ ] Card dragging and unit spawning
- [ ] Unit movement and tower combat
- [ ] UDP client-server communication
- [ ] Client prediction
- [ ] Server reconciliation
- [ ] Lag compensation

### To Do
- [ ] Delta compression
- [ ] Entity interpolation
- [ ] Card selection UI
- [ ] Real assets (sprites)
- [ ] Lag simulation testing
- [ ] Multiplayer stress test
- [ ] WebAssembly build

## Setup
1. Clone the repo: `git clone <repo-url>`
2. Build: `cargo build --release`
3. Run server: `cargo run --release -p server`
4. Run client: `cargo run --release -p client`

## Tech Stack
- **Client**: Macroquad (Rust 2D framework)
- **Server**: Tokio (async runtime) with UDP
- **Shared**: Rust logic with serde/bincode
- **Networking**: Client prediction, server reconciliation, lag compensation

## Development Plan


Phase 1: Setup and Core Structure

Goal: Establish the project foundation and a single-player prototype.

    Create Workspace: Set up a Cargo workspace with client, server, and shared crates as outlined earlier.
        Tasks:
            Initialize tower-defense-game/Cargo.toml with [workspace] and members.
            Create empty client, server, and shared crates with their Cargo.toml files.
    Define Shared Logic: Build basic game entities and logic in shared.
        Tasks:
            Add serde and bincode dependencies to shared.
            Define structs: Card, Unit, Tower, Player, GameState.
            Implement simple logic (e.g., spawn_unit, deal_damage).
    Single-Player Client: Get a basic Macroquad client running.
        Tasks:
            Add macroquad to client/Cargo.toml.
            Load a background texture and render a static “battlefield.”
            Display a placeholder card and unit sprite.

Milestone: A window opens with a battlefield and a static card/unit.


Phase 2: Core Gameplay Mechanics

Goal: Implement card placement and basic tower defense mechanics in single-player mode.

    Input Handling: Allow card dragging and placement.
        Tasks:
            In client/src/input.rs, track mouse input for dragging cards.
            Snap cards to valid battlefield positions (e.g., grid-based).
    Unit Movement: Make spawned units move across the battlefield.
        Tasks:
            In shared, add a move_unit system with position/velocity.
            In client, update and render unit positions each frame.
    Combat: Add basic tower-unit interactions.
        Tasks:
            In shared, implement Tower::attack to damage nearby units.
            In client, visualize attacks (e.g., draw lines or projectiles).

Milestone: You can drag a card to spawn a unit, it moves, and a tower damages it.


Phase 3: Networking Foundation

Goal: Connect client and server with basic UDP communication.

    Server Setup: Create a Tokio/UDP server.
        Tasks:
            Add tokio to server/Cargo.toml.
            Bind a UdpSocket and echo received packets for testing.
    Client Networking: Send inputs to the server.
        Tasks:
            In client/src/net.rs, send card placement inputs via UDP.
            Run Tokio in a separate thread (e.g., tokio::runtime::Runtime).
    Server-to-Client Updates: Broadcast game state.
        Tasks:
            In server, serialize and send GameState to connected clients.
            In client, receive and log server updates.

Milestone: Client sends card placements to the server, and server echoes back the updated state.


Phase 4: Multiplayer Sync Techniques

Goal: Implement client prediction, server reconciliation, and lag compensation.

    Client Prediction: Predict unit spawns locally.
        Tasks:
            In client/src/prediction.rs, apply inputs immediately to GameState.
            Store a history of inputs with timestamps.
    Server Reconciliation: Sync client with server state.
        Tasks:
            Server sends authoritative GameState with timestamps.
            Client reconciles by re-applying unprocessed inputs.
    Lag Compensation: Rewind server state for fairness.
        Tasks:
            In server, store a short history of states (e.g., VecDeque).
            Process inputs against the state at the client’s timestamp.

Milestone: Two clients can play, with smooth prediction and minimal desync.


Phase 5: Optimization and Polish

Goal: Enhance performance and add visual/gameplay polish.

    Delta Compression: Reduce bandwidth usage.
        Tasks:
            In shared/net.rs, implement delta encoding for GameState.
            Server sends only changes since the last acknowledged state.
    Entity Interpolation: Smooth unit movement.
        Tasks:
            In client, buffer server updates and interpolate positions.
    UI Polish: Add a card selection HUD.
        Tasks:
            Integrate egui-macroquad (optional) or build a custom UI.
            Display a hand of cards with click/drag support.
    Assets: Replace placeholders with real sprites.
        Tasks:
            Add card, unit, and tower sprites to assets/.

Milestone: Game feels responsive, looks decent, and handles multiple players efficiently.


Phase 6: Testing and Deployment

Goal: Ensure stability and prepare for release.

    Lag Simulation: Test under poor network conditions.
        Tasks:
            Use tc (Linux) or Clumsy (Windows) to add latency/jitter.
            Verify prediction and reconciliation hold up.
    Multiplayer Stress Test: Run multiple clients.
        Tasks:
            Spin up 4-8 clients locally or on separate machines.
            Check server performance and sync accuracy.
    Web Build: Test a WebAssembly version.
        Tasks:
            Compile client to WASM with macroquad’s instructions.
            Host server separately and connect via IP.

Goal: Achieve low-latency, responsive gameplay with smooth visuals and fair multiplayer interactions.
