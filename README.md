# Firmware

Docker Files, Docker Compose Files, Etc.
1. Dockerfile for a bunch of mock zenoh nodes (Generated form yaml files about for modules), lets use python for such things? Use Makefile/Cargo build.rs or a bash file?
2. Generate and orchestrate a zoo of mock zenoh modules: Docker Compose? Or Python + Docker SDK?

```sh
# Build all subprojects
cargo build

# Test all subprojects
cargo test
```
> Subprojects exclude firware crates. They currently require to be built and tested separately.

## Probable List of Modules
### Low Level Modules
- Slider
- Upwards and Downwards Stack
- Gantry
- Tip Alignment

### Higher Level Modules
- Gantry Controller
- Resource Managers
- Centrifuge Controller

# Architectural Hopes
- [ ] All modules in one repo
- [ ] All modules are tested
- [ ] Async drivers for Hardware
- [ ] Cloud based logging, monitoring (Sentry)
