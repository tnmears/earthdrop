/// Simple program to calculate the time taken to fall from 1 AU to the surface of the Sun.
/// Numerically integrates kinematic functions to arrive at a result.
/// Accuracy depends on size of time_step and precision of constants
/// Units in mks

fn main() {
    let grav_const: f64 = 0.0000000000667;  // Gravitational constant
    let height: f64 = 149597870700.0;       // Distance from earth to sun in meters (1 AU)
    let mass_sun: f64 = 1989550000000000000000000000000.0; // Mass of the sun in kg
    let radius_sun: f64 = 695700000.0;      // Radius of sun in meters
    let time_step: f64 = 0.01;              // time step amount for simulation
    let mut current_height: f64 = height;   // initializes the height. Needs to change
    let mut velocity: f64 = 0.0;            // starting velocity should be zero, is declared here because it changes throught the loop
    let mut time_total: f64 = 0.0;          // tracks the total time

    // loops until the sun's surface is reached
    while current_height > radius_sun {

        // acceleration is only needed once per loop
        // calculate acceleration
        let acceleration: f64 = - ( grav_const * mass_sun ) / current_height.powf(2.0);
        // calculate current velocity
        velocity = acceleration * time_step + velocity;
        // calculate current height
        current_height = current_height + velocity * time_step + ( 0.5 ) * acceleration * time_step.powf(2.0);
        // increment total time
        time_total = time_total + time_step;
    }
    println!("{}", time_total );            // prints total time
}
