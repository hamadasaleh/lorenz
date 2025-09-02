use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // -----------------------------
    // Physical parameters of Lorenz system
    // -----------------------------
    let sigma: f64 = 10.0;        // σ: rate of response of x to difference with y
    let rho: f64 = 28.0;          // ρ: driving force (temperature difference in convection)
    let beta: f64 = 8.0 / 3.0;    // β: geometric/dissipation factor

    // Initial conditions
    let x0: f64 = 1.0;
    let y0: f64 = 1.0;
    let z0: f64 = 1.0;

    let dt: f64 = 0.01;           // timestep
    let t: i32 = 1000;            // total number of steps

    // Storage for the trajectory
    let mut states: Vec<(f64, f64, f64)> = Vec::new();

    // Current state
    let mut x = x0;
    let mut y = y0;
    let mut z = z0;

    // -----------------------------
    // Simulation loop (Euler integration)
    // -----------------------------
    for _step in 1..t {
        // Compute derivatives for current state
        let [dx, dy, dz] = lorenz_derivatives(x, y, z, sigma, rho, beta);

        // Update state using Euler method
        x += dx * dt;   // x moves toward y at speed proportional to sigma
        y += dy * dt;   // y grows with x*(rho - z) and decays with -y
        z += dz * dt;   // z grows with x*y and decays with -beta*z

        // Save the new state
        states.push((x, y, z));
    }

    // -----------------------------
    // Save trajectory to CSV
    // -----------------------------
    let mut wtr = csv::Writer::from_path("lorenz.csv")?;
    wtr.write_record(&["x", "y", "z"])?; // header
    for (x, y, z) in &states {
        wtr.write_record(&[x.to_string(), y.to_string(), z.to_string()])?;
    }
    wtr.flush()?;
    println!("Saved lorenz.csv successfully!");

    Ok(())
}

// -----------------------------
// Lorenz derivatives function
// -----------------------------
fn lorenz_derivatives(x: f64, y: f64, z: f64, sigma: f64, rho: f64, beta: f64) -> [f64; 3] {
    // dx/dt = σ * (y - x)
    // - x moves toward y at a speed controlled by sigma
    let dx = sigma * (y - x);

    // dy/dt = x * (ρ - z) - y
    // - y grows if x*(rho - z) is positive, decays with -y damping
    // - rho controls the driving force; higher rho → stronger growth
    let dy = x * (rho - z) - y;

    // dz/dt = x * y - β * z
    // - z grows if x*y is positive, decays with -beta*z damping
    // - beta controls dissipation / geometric factor
    let dz = x * y - beta * z;

    [dx, dy, dz]
}
