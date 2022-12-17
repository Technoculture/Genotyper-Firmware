# Architecturally Significant Concerns

## Desirable Architectural Characteristics
- Availability
    The device should be able to accept new samples from users and have its services avilable thoughout its daily operational period without obstruction due to its internal workings.
- **Recoverability**
    If there is a power failure of the system as a whole, the system should be able to recover its operational state from exactly where it left off. This is essential in order to not destroy all the various samples held by the device. This should allow the device to make an informed decision on the extent to which the state of the the system is fully or partially recoverable. This is a fallback and should be viewed as a worst case scenario.
- **Reliability**
    The device should operate in a manner true to its operational specification without a proliferation of edge case bugs.
- Performance
- Robustness
- Scalability

## Structural Architectural Characteristics
- Configurability
- Extensibility
- Localization
- Maintainability
- Supportability
- Upgradeability

## Cross-cutting Architectural Characteristics
- Accessibility
- Security
- Privacy
