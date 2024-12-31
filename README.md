# Praxis Forge

<p align="center">
  <em>Transform intentions into actions, actions into habits, habits into mastery.</em>
</p>

## Vision

Praxis Forge is a sophisticated task and habit management system that embodies the philosophical concept of "praxis" - the process of putting theoretical knowledge into practice. Built with Rust, Elixir, and modern web technologies, it's more than just a task management system—it's a comprehensive platform designed to transform how we approach personal development and productivity. By integrating task management, habit formation, and skill development, it creates a holistic environment for continuous growth and mastery and transforms how we approach personal development and productivity.

### Core Philosophy

The name "Praxis Forge" combines two powerful concepts:

- **Praxis**: The practice of applying knowledge, emphasizing the transformation of theory into action
- **Forge**: A place of creation and transformation, where raw materials are shaped into refined forms

This synthesis reflects our core principles:

- **Intentional Practice**: Every action is an opportunity for improvement
- **Systematic Progress**: Break down complex goals into manageable steps
- **Measured Growth**: Track and visualize your journey to mastery
- **Adaptive Learning**: Adjust and refine your approach based on data
- **Sustainable Development**: Build lasting habits through consistent practice

## Architecture

### Forge Core (Backend)

- Built with Elixir/Phoenix
- Handles data processing and persistence
- Real-time communication via Phoenix Channels
- Native Implemented Functions (NIFs) in Rust for performance-critical operations

### Praxis Workshop (Frontend)

- Built with Rust/Yew and WebAssembly
- Responsive and performant user interface
- Offline-first architecture
- Real-time updates and synchronization

## Setup

### Backend ([forge-core](forge-core))

1. Navigate to the backend directory:

   ```bash
   cd forge-core
   ```

2. Setup the database:

   ```bash
   mix ecto.create
   mix ecto.migrate
   ```

3. Start Phoenix server:
   ```bash
   mix phx.server
   ```

### Frontend ([praxis-workshop](praxis-workshop))

1. Navigate to the frontend directory:

   ```bash
   cd praxis-workshop
   ```

2. Install dependencies and start:
   ```bash
   cargo tauri dev
   ```

## Core Features

### Forge Operations (Tasks)

- ✅ Task Creation
- ✅ Task Organization
- ✅ Task Status Updates
- ✅ Task Deletion
- ✅ Task Editing

## Stretch Features

### Forge Practices (Habit System)

- [ ] Habit formation and tracking
- [ ] Streak monitoring
- [ ] Progress visualization
- [ ] Behavioral pattern analysis

### Praxis Paths (Goal Tracking)

- [ ] Goal setting and decomposition
- [ ] Progress tracking
- [ ] Milestone management
- [ ] Achievement visualization

### Forge Rituals (Daily Management)

- [ ] Daily task organization
- [ ] Routine management
- [ ] Time blocking
- [ ] Priority alignment

### Praxis Matrix (Progress Tracking)

- [ ] Progress analytics
- [ ] Performance insights
- [ ] Growth visualization
- [ ] Pattern recognition

### Forge Mastery (Skill Development)

- [ ] Skill progression tracking
- [ ] Learning path management
- [ ] Competency mapping
- [ ] Mastery milestones

## Technical Features

- **Real-time Synchronization**: Seamless updates across devices
- **Offline Support**: Full functionality without internet connection
- **Native Performance**: Rust NIFs for computation-intensive operations
- **Data Security**: End-to-end encryption for sensitive information
- **Cross-platform**: Desktop and web support through Tauri

## Technical Stack

- Backend: Elixir/Phoenix
- Frontend: Rust/Tauri
- Database: PostgreSQL

## License

This project is licensed under the [MIT License](LICENSE).

---

<p align="center">
  <em>"Through practice, we forge our future."</em>
</p>
