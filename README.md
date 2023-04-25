# fdlf-rs-learning-first
A repo for learning how to implement fast decoupled load flow in rust from scracth. A tutorial project.


## IEEE 14-bus
![IEEE 14-bus system](data/14bus.jpg)

## ChatGPT suggested Step-by-step
Sure, here is a step-by-step guide on how to implement the fast decoupled load flow algorithm from scratch in Rust:

- [x] Define the data structures for the power system, including buses, branches, and loads. You can use Rust structs to represent these elements and their properties.
- [ ] Implement a function to read the power system data from a file or database and populate the data structures.
- [ ] Create a function to calculate the initial voltage and phase angle for each bus in the power system. You can use the flat start method to initialize the voltage and phase angle.
- [ ] Write a function to calculate the admittance matrix of the power system, which includes the series and shunt admittances of the branches.
- [ ] Implement the fast decoupled load flow algorithm. This involves dividing the power system into two subsystems (P and Q), and iterating between the P and Q subsystems until convergence is achieved. Here are the basic steps:
    - [ ] a. Calculate the P and Q mismatches for each bus, using the initial voltage and phase angle values. The P mismatch is the difference between the total real power injections and the total real power consumption at the bus. The Q mismatch is the difference between the total reactive power injections and the total reactive power consumption at the bus.
    - [ ] b. Update the voltage and phase angle values for the P subsystem, using the P mismatches and the series admittance matrix. This can be done using the Newton-Raphson method.
    - [ ] c. Update the voltage values for the Q subsystem, using the Q mismatches and the shunt admittance matrix.
    - [ ] d. Repeat steps a-c until convergence is achieved. You can use a convergence criterion based on the maximum P and Q mismatches.
- [ ] Once the load flow solution has converged, calculate the real and reactive power flows on each branch in the power system.
- [ ] Write a function to output the load flow results, including the bus voltages and phase angles, branch power flows, and other relevant data.

These are the basic steps for implementing the fast decoupled load flow algorithm in Rust. Of course, there may be additional details and optimizations you can add, depending on the specific requirements of your project.

## Data descriptions
### 14bus_buses.csv
| Header   | Description                                                           |
|----------|-----------------------------------------------------------------------|
| id       | Bus ID                                                                |
| name     | Bus name                                                              |
| bus_type | Type of bus (`1`: PQ, `2`: PV, `3`: reference)                        |
| p_load   | Active power demand at the bus (in MW)                                |
| q_load   | Reactive power demand at the bus (in MVAr)                            |
| shunt_conductance       | Shunt conductance (in MW)                                             |
| shunt_susceptance       | Shunt susceptance (in MVAr)                                           |
| area_number     | Area number                                                           |
| voltage  | Voltage magnitude setpoint (in per unit)                              |
| angle    | Voltage angle setpoint (in degrees)                                    |
| base_voltage   | Base voltage (in kV)                                                  |
| loss_zone     | Loss zone number                                                       |
| max_voltage_magnitude     | Maximum voltage magnitude limit (in per unit)                         |
| min_voltage_magnitude     | Minimum voltage magnitude limit (in per unit)                         |

### 14bus_generators.csv
| Header | Description                                              |
|--------|----------------------------------------------------------|
| bus_number    | Bus number to which the generator is attached            |
| real_power     | Real power output of the generator (in MW)                |
| reactive_power     | Reactive power output of the generator (in MVAr)          |
| max_reactive_power   | Maximum reactive power output of the generator (in MVAr) |
| min_reactive_power   | Minimum reactive power output of the generator (in MVAr) |
| voltage_magnitude     | Voltage magnitude setpoint for the generator (in per unit)|
| rated_power  | Rated power of the generator (in MVA)                     |
| status | Generator status (`0`: offline, `1`: online)             |
| max_real_power   | Maximum real power output of the generator (in MW)        |
| min_real_power   | Minimum real power output of the generator (in MW)        |

### 14bus_generatorcost.csv
| Header | Description                                                     |
|--------|-----------------------------------------------------------------|
| 2      | Type of cost function (`2`: quadratic)                          |
| startup| Startup cost for the generator (in USD)                         |
| shutdown| Shutdown cost for the generator (in USD)                         |
| n      | Number of cost coefficients                                      |
| c(n-1) | Coefficient for `Pg^2` term in the cost function (in USD/MW^2)   |
| ...    | Additional coefficients for the cost function (in descending order) |
| c0     | Coefficient for constant term in the cost function (in USD)     |

### 14bus_branches.csv
| Header | Description                                                         |
|--------|---------------------------------------------------------------------|
| fbus   | Bus number for the "from" end of the branch                          |
| tbus   | Bus number for the "to" end of the branch                            |
| r      | Resistance of the branch (in p.u.)                                   |
| x      | Reactance of the branch (in p.u.)                                    |
| b      | Charging susceptance of the branch (in p.u.)                         |
| rateA  | MVA rating A of the branch                                           |
| rateB  | MVA rating B of the branch                                           |
| rateC  | MVA rating C of the branch                                           |
| ratio  | Transformer off-nominal turns ratio or tap setting (if applicable)   |
| angle  | Transformer phase shift angle (if applicable)                       
