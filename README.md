# CSE2425 Embedded Software project: Robot Lab
The Embedded Software course lab project. Author: Arnas Misevicius

## Table of contents
* Project structure
* Libraries used
* Assignment solutions
    - Assignment 1: Variable speed
    - Assignment 2: Obstacle detection
    - Assignment 3: Line follower
* Building and running the assignments.

## Project structure

The project is written in rust and structured as a rust binary crate that is then flashed onto the rp2040.
The entry point of the binary is the function `fn main() -> !` in the main.rs file, annotated with the ``#![entry]`` macro.
The assignment logic is defined in separate modules and launched through the mission control defined in the main function.
Additional utility modules are defined in the ``utils/`` folder and re-exported through the ``utils.rs`` file.

The main.rs file defines the following:
* Panic handler
    - The panic handler sets the on-board rp2040 led, indicating that the program has paniced.
* Peripheral resets.
    - Brings the peripherals out of reset and busy-waits for them to be ready.
* Mission control
    - Through the `SELECTED_MISSION` static and `fn mission_launch(...)` selects which assignment mission to launch. Matches the selected mission from 3 in the MISSION enum. 
* Interrupt definition
    - Defines the interrupt used for the obstacle detection algorithm.
    
The utils file defines the following:
* Global ``OBSTACLE`` state
    - Used for obstacle detection. Gets modified by and interrupt and read by assignment 2 logic
* Re-exports utility modules
    - ``motors.rs`` provides an easy way to get a struct of a PWM motor, giving the ability to set the DC and direction of the wheel.
    - ``clocks.rs`` provides the method used to initialise and set up all the system clocks. Default values for these clocks are all taken from the rp2040 datasheet.

## Libraries used
All code is written using: 
* [rp2040_pac](https://docs.rs/rp2040-pac/latest/rp2040_pac/index.html) peripheral access crate used for direct access to peripheral registers.
* [cortex_m](https://docs.rs/cortex-m/latest/cortex_m/) low-level processor access crate.

All other used libraries/modules are defined within the project itself.

## Assignment solutions

### Assignment 1: Variable speed.
The solution to this assignment constructs the two motor structs from the utility module described above.
The routine to the assignment:
1. Set the dc of both motors to 75% and hold for 120 million cpu cycles.
2. Set the dc of both motors to 100% and hold for 60 million cpu cycles.
3. Loop while progressively decreasing the dc by 5 every 30 million cycles.
4. Set the dc to 0 at the end for good measure.

### Assignment 2: Obstacle detection
The solution to this assignment uses the HC-SR04 ultrasonic sound sensor.
The assignment launch initialises the GPIOs to their respective functions and registers the echo pin edge interrupts to be handled by the interrupt handler defined in the ``main.rs`` file.
```rust
let timeout = p.TIMER.timelr().read().bits();
while p.TIMER.timelr().read().bits().wrapping_sub(timeout) <= 6000 {}
```
Reads the current system time and waits for 6000 cycles so that the measurements are not done too fast.

```rust
let begin = p.TIMER.timelr().read().bits();
p.SIO.gpio_out_set().write(|w| w.gpio_out_set().variant(1 << TRIG_PIN));
while p.TIMER.timelr().read().bits().wrapping_sub(begin) < 10 {}
p.SIO.gpio_out_clr().write(|w| w.gpio_out_clr().variant(1 << TRIG_PIN));
```
Sends a 10us signal through the trigger pin of the ultrasonic sensor and records the beginning time of the measurement.
\

The interrupt defined for the rising edge of the echo pin saves the system time into a global `ECHO_BEGIN` value.\
The falling edge interrupt calculates the difference between `ECHO_BEGIN` and the curent time. The result is the elapsed time in microseconds.\
Dividing this value by 58 gives the distance from detected obstacle in centimeters. If this distance is less than 10, the interrupt sets the `OBSTACLE` static to true.\

```rust
if !unsafe { crate::OBSTACLE } {
    mot1.set_dc(100);
    mot2.set_dc(100);
} else {
    mot1.set_dc(0);
    mot2.set_dc(0);
}
```

Finally, a simple boolean check every loop sets the motor dc to either full or none depending on the obstacle state.

### Assignment 3: Line follower
This assignment consists of a basic Moore finite state machine that has three states: STRAIGHT, LEFT and RIGHT.

The assignment launch once again clears all the relevant reset bits and busy waits for them to be done.

The state machine is a simple loop that takes in the readings of the 2 IR inputs and determines the next state based on that.

The logic of the state matching from readings is as follows:
1. Set the two IR sensors to have one on the side(outside sensor) and one in the middle(inside sensor).
2. The inside sensor is positioned on the line and the outside sensor - to the side.
3. The FSM reads the input from both the IR sensors in the loop and gets a tuple of the boolean values determining whether they are on the line or not.
    - The sensor is determined to be above the line if the ADC value read from it is over 750. This value was reached through physical experimentation.
4. Based on the reading the next state is determined as follows:
```rust
match (inner, outer) {
    (true, true) => {
        // Both sensors on the line -> Turn right
        current_state = STATE::RIGHT;
    },
    (true, false) => {
        // Only the inner sensor is on the line -> Go straight
        current_state = STATE::STRAIGHT;
    },
    (false, true) => {
        // Only the outer sensor is on the line -> Turn left
        current_state = STATE::LEFT;
    },
    (false, false) => {
        // None of the sensors are on the line -> Turn right
        current_state = STATE::RIGHT;
    },
}
```
This tactic of having one sensor on the line and one off the line reduces the wiggling that occurs when sensors are trying to find the line.

## Building and running the solutions
The assignment mission selection is hardcoded and the program has to be rebuilt and flashed onto the rp2040
every time a new mission is selected.\
To select a different mission simply change the `SELECTED_MISSION` static variable to one of 3 assignment missions defined in `enum MISSION`

To build and run the project, the rp2040 has to be connected to the computer in `BOOTSEL` mode. Once that is done, running 
```bash
cargo build && cargo run
```

will build and flash the solution onto the pico.
