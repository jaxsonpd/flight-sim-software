# flight-sim-software

This repository contains the software that is used to run on the flight simulator hardware developed and stored in jaxsonpd/flight-sim-hardware. The setup for this project is detailed in [setup](SETUP.md)

## Device Types

### Aviation Com, Nav and Transponder Radio

Editing of a range of frequencies availible on the aircraft. This includes:

- Com
- Nav
- ADF
- DME
- XPDR (Transponder)

Changes are made using dual quadrature encoders and data is transmitted using a custom packet format over RS232. A custom PCB is used to house the quadrature encoders, rotary switch and 12 7-segment displays this can be found in [Flight-Sim-Hardware](https://github.com/jaxsonpd/flight-sim-hardware).

Here is an image of the input section of the project showing an active frequency of 118.00 MHz and a standby frequency of 108.05 MHz.

![Input Device for Radio](docs/photos/IMG_6265.jpeg)

## Architecture

This project is comprised of two programs. One to run on the embedded hardware located in device/target/src and the second which handles communication with msfs2020 located in driver-rust/src. These two programs comunicate over RS232 using a custom packet format detailed in device/target/libs/custom-can-protocol or in [Custom Can Protocol](https://github.com/jaxsonpd/custom-can-protocol). The driver comunicates with the host using the simconnect api which is compatible with various leading flight simulators.

### Testing

This project uses the [unity](https://github.com/ThrowTheSwitch/Unity) test framework to allow unity testing. This is supplemented by the [FFF](https://github.com/meekrosoft/fff) Fake Function Framework used for faking where needed. The tests are located in `./tests/` and use cmake for execution. It can be manually executed from the host directory using:

```bash
cmake -B build tests
cmake --build build --parallel
ctest --test-dir build --output-on-error
```

Or using the provided bash script as follows:

```bash
source run_tests.sh
```
