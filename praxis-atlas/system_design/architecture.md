# Architecture Overview

## High-Level Architecture

### Frontend (Primary Focus)

- Rust/WASM with Yew
- Local state management
- WebSocket integration
- IndexedDB storage

### Backend

- Phoenix Framework
- Real-time channels
- PostgreSQL database
- RESTful API

### Integration Layer

- Type-safe contracts
- Binary serialization
- WebSocket protocols
- State synchronization

## Data Flow

### 1. Local First

- User action → Local state
- Optimistic update → UI
- Background sync → Server

### 2. Real-Time Updates

- Server event → WebSocket
- State update → Local store
- UI refresh → Component
