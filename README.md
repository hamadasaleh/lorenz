# Lorenz Attractor Simulation in Rust

This project implements the Lorenz system — a classic example of a chaotic dynamical system — in Rust. It simulates the evolution of three state variables (x, y, z) over time using the Euler method. The system is parameterized by:

σ (sigma): controls the response speed of x

ρ (rho): the driving force (temperature difference in convection)

β (beta): dissipation / geometric factor

The simulation outputs the trajectory of the system to a CSV file (lorenz.csv) and can be plotted to visualize the Lorenz attractor.