# HSM Library

## Decision Drivers
- **Support for HSMs**, it should make HSMs as easy to deal with as non-composite state machines
- **Should be actively maintained**, it should have a reasonable community to support in case we run into a problem
- **Support for visualization and simulation** would be highly useful
- **Highly Unit Testable**, this is important to ensure a minimum software quality

## Considered Options
- [Mathworks Stateflow](https://www.mathworks.com/products/stateflow.html)
- [[Boost::Ext].SML](https://boost-ext.github.io/sml/index.html)
- [HSMCPP](https://github.com/igor-krechetov/hsmcpp)
- [SMF](https://docs.zephyrproject.org/latest/guides/smf/index.html)

## Pros and Cons of the Options 

### Mathworks Stateflow
- Good, because it is allows for simulation of the state diagram
- Good, because it has an easy to use GUI
- Good, because it has an easy to learn using the inbuilt Bootcamp (in 1-3 days)
- Good, beacuse of comparatively large community of users and actively marketted product
- Bad, because it is a propretary software which requires a per node purchase
- Bad, because it does not generate a (good) human readable textual file

### [Boost::Ext].SML
- Good, because it supports Hierarchical State machines ([Example](https://boost-ext.github.io/sml/examples.html#composite))
- Good, because it maps nicely to PlantUML Syntax ([Talk by Kris Jusiak](https://www.youtube.com/watch?v=MskKTIdW45c&t=1273s))

### SFM
- Good, because it is built into the ZephyrRTOS

| Candidate | Heirachical | Maintained | Simulation | Testable |
| --- | --- | --- | --- | --- |
| Stateflow | ✔️ | ✔️ | ✔️ | 〰️ |
| [Boost.Ext].SML | ✔️ | ✔️ | ✖️ | ✔️ |
| HSMCPP | ✔️ | 〰️ | ✔️ | ✔️ |
| SMF | ✔️ | ✔️ | ✖️ | ✔️ |
