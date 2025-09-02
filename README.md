# Road Intersection Traffic Simulation

This project aims to solve the traffic congestion problem in a capital city by creating a traffic control strategy and visualizing it with a simulation. The simulation is built using Rust, leveraging the `sdl2` library for rendering and user interaction.

## Objectives

The primary objective is to create a dynamic traffic simulation that manages vehicle flow through a road intersection, preventing congestion and collisions.

## Environment and Rules

### Roads

The simulation features two roads crossing each other, forming an intersection. Each road has one lane in each direction. Vehicles entering the intersection can choose to turn left, turn right, or continue straight.

```
                        North
                    |  ↓  |  ↑  |
                    |  ↓  |  ↑  |
                    |     |     |
                    |     |     |
                    |     |     |
                    |     |     |
     _______________|     |     |_______________
     ← ←                                     ← ←
East ---------------             --------------- West
     → →                                     → →
     _______________             _______________
                    |     |     |
                    |     |     |
                    |     |     |
                    |     |     |
                    |     |     |
                    |  ↓  |  ↑  |
                    |  ↓  |  ↑  |
                        South
```

### Traffic Lights

Traffic lights are positioned at each lane's entry point to the intersection. They operate with only two states: red and green. The traffic light system implements a dynamic algorithm to prevent traffic congestion.

**Dynamic Congestion Rule:**
The maximum allowed queue length for each lane is calculated as:
`capacity = floor(lane_length / (vehicle_length + safety_gap))`

If a lane's vehicle count reaches its capacity, the traffic light logic will adjust (e.g., extend green time) to prevent overflow. The system's core function is to avoid collisions while adapting to congestion.

### Vehicles

Vehicles in the simulation adhere to the following rules:

-   **Route Visualization:** Vehicles are colored to indicate their chosen route (e.g., yellow for right turn, blue for straight, red for left turn).
-   **Fixed Route:** Once a route is selected, it cannot be changed.
-   **Fixed Velocity:** Each vehicle maintains a constant speed.
-   **Safety Distance:** Vehicles maintain a safe distance from each other, stopping if the vehicle ahead stops.
-   **Traffic Light Obedience:** Vehicles stop at red lights and proceed on green.
-   **No Special Privileges:** All vehicles follow the same rules; there are no emergency vehicles.

## Commands

Vehicles are spawned using keyboard input:

-   **`↑` (Up Arrow):** Spawns a vehicle from the South.
-   **`↓` (Down Arrow):** Spawns a vehicle from the North.
-   **`→` (Right Arrow):** Spawns a vehicle from the West.
-   **`←` (Left Arrow):** Spawns a vehicle from the East.
-   **`r`:** Spawns a vehicle from a random direction.
-   **`Esc` (Escape):** Ends the simulation.

To prevent "spamming," vehicles are spawned with a safe distance between them.

## Technologies

-   **Rust:** The primary programming language.
-   **SDL2:** Used for rendering the simulation graphics and handling user input.

## How to Run

To run this simulation, ensure you have Rust and `sdl2` development libraries installed. Then, navigate to the project root and execute:

```bash
cargo run
```