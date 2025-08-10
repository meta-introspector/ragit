# Simulated Annealing for Chunk Comparison

This document describes the simulated annealing approach used to identify similar file paths in the `ragit` project.

## Overview

The `run_chunk_comparison.rs` program uses a simulated annealing algorithm to find file paths that are semantically similar. This approach treats file paths as nodes in a graph and attempts to find a low-energy configuration of the graph where similar nodes are placed close to each other.

## Algorithm

The algorithm works as follows:

1.  **Initialization:**
    *   File paths are read from `index.rs.txt`.
    *   Each file path is converted into a numerical vector (embedding) using a pre-trained term embedding model (`term_embeddings.json`).
    *   Each file path is represented as a `Node` with a path, an embedding, a random initial position, and a velocity.

2.  **Simulated Annealing:**
    *   The system starts at a high temperature.
    *   In each iteration, a random node is moved to a new position.
    *   The change in the system's energy is calculated. The energy of the system is a function of the distance between nodes and the similarity of their embeddings.
    *   If the new energy is lower than the current energy, the move is accepted.
    *   If the new energy is higher, the move is accepted with a certain probability that depends on the current temperature.
    *   The temperature is gradually decreased according to a cooling schedule.
    *   This process is repeated for a fixed number of iterations or until a timeout is reached.

3.  **Result:**
    *   After the simulation is finished, the distances between all pairs of nodes are calculated.
    *   Pairs of nodes with a distance below a certain threshold are reported as similar.

## Parameters

The following parameters can be tuned to control the behavior of the algorithm:

*   `initial_temp`: The initial temperature of the system.
*   `cooling_rate`: The rate at which the temperature is decreased.
*   `iterations`: The number of iterations to run the simulation.
*   `viscosity`: A factor that slows down the movement of nodes.
*   `timeout`: The maximum time to run the simulation.
