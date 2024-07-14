# 100 lines of code to play with Entropy

This project simulates a heat source with algorithms for temperature, density generation, heat generation, and entropy extraction using Rust and the Tokio crate. This is part of my learning journey to understand the concepts of entropy and heat generation in a simplified model, as part of a course on thermodynamics. The formulas that I used are partially subjective and simplified for educational purposes.

I think it is just fun to understand the chaos behind the scenes.


## Explanation of Formulas

### Heat Generation Formula

Heat Generated
=
Specific Heat Capacity
×
Mass
×
Temperature
Heat Generated= 
Specific Heat Capacity×Mass×Temperature
​	
 
Specific Heat Capacity is the amount of heat required to raise the temperature of a unit mass of a substance by one degree Celsius. For water, it is 4.18 J/(g·°C). Mass is calculated as density (kg/m³) times volume (m³). In this code, we assume a unit volume, so mass is directly derived from density. Temperature is the temperature of the hot source in degrees Celsius.

## Explanation of Formulas

### Heat Generation Formula

\[
\text{Heat Generated} = \sqrt{\text{Specific Heat Capacity} \times \text{Mass} \times \text{Temperature}}
\]

Specific Heat Capacity is the amount of heat required to raise the temperature of a unit mass of a substance by one degree Celsius. For water, it is 4.18 J/(g·°C). Mass is calculated as density (kg/m³) times volume (m³). In this code, we assume a unit volume, so mass is directly derived from density. Temperature is the temperature of the hot source in degrees Celsius.

### Entropy Extraction Formula

\[
\text{Entropy} = k \times \text{Volume} \times (\ln(\text{Internal Energy}) + 1.0)
\]

Boltzmann Constant (k) is a physical constant relating the average kinetic energy of particles in a gas with the temperature of the gas, approximately \(1.380649 \times 10^{-23}\) J/K. Volume is assumed to be 1.0 m³ in this simplified model. Internal Energy is calculated as temperature times density.
