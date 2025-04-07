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

Goal: Achieve low-latency, responsive gameplay with smooth visuals and fair multiplayer interactions.
