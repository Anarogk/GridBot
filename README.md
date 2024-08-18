
# GridBot

GridBot is a basic 2D robot simulator built using the Piston game engine in Rust. In this simulation, you control a robot that moves around a grid while avoiding obstacles. The project demonstrates fundamental concepts in game development with Rust, such as rendering, grid-based movement, collision detection, and user input handling.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Running the Simulation](#running-the-simulation)
- [Usage](#usage)
- [Customization](#customization)

## Overview

GridBot is a simple 2D simulator designed to introduce game development in Rust. The robot (represented as a blue square) can be controlled by the user via the arrow keys to navigate a grid. The grid contains obstacles (red squares) that the robot must avoid. You can expand the project to add more complex behaviors such as autonomous navigation, additional sensors, or even 3D graphics.

## Features

- Simple 2D grid-based movement
- User-controlled robot using arrow keys
- Randomly generated obstacles on the grid
- Collision detection to prevent the robot from moving through obstacles
- Easily extendable for more complex behaviors (e.g., pathfinding algorithms)

## Getting Started

### Prerequisites

To run this project, you need to have the following installed:

- [Rust](https://www.rust-lang.org/tools/install) (Ensure you have `cargo` installed)
- A modern C++ compiler and development environment (required for compiling certain dependencies)

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/GridBot.git
   cd GridBot
   ```

2. Install dependencies and build the project:

   ```bash
   cargo build
   ```

### Running the Simulation

To run the simulation, use the following command:

```bash
cargo run
```

This will launch a window where you can control the robot using the arrow keys.

## Usage

- **Movement**: Use the arrow keys to move the robot (blue square) around the grid.
- **Objective**: Navigate the robot around the obstacles (red squares) without colliding with them.

The grid has a fixed size, and the robot cannot move beyond the grid boundaries or through obstacles.

### File Descriptions

- **Cargo.toml**: Manages the project dependencies, including Piston, graphics libraries, and random number generation.
- **src/main.rs**: Contains the core logic of the robot simulator, including:
  - The `Robot` struct for handling robot position and movement
  - The `World` struct for managing the grid and obstacles
  - The `App` struct for rendering and user input handling

## Customization

Here are some ways to customize and expand GridBot:

1. **Change Grid Size**: Modify the `GRID_SIZE` constant in `src/main.rs` to alter the size of the grid.
2. **Add More Obstacles**: Adjust the number of obstacles by changing the `generate_obstacles` function in `World`.
3. **Obstacle Avoidance**: Implement a pathfinding algorithm like A* to automatically navigate the grid while avoiding obstacles.
4. **3D Graphics**: Use a more advanced engine like Amethyst to render the simulation in 3D.

Feel free to fork this project and add your own features!

## Contributing

Contributions are welcome! If you want to contribute to this project, please fork the repository and submit a pull request. You can also open issues for feature requests or bug reports.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

