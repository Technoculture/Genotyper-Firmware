# This connects to the GDB server running locally.
#   - for openocd, use port :3333
#   - for JLinkGDBServer, use port :2331
target extended-remote :3333

# Due to https://github.com/rust-embedded/cortex-m-rt/issues/139,
#   we will get an infinite backtrace on a panic!(). Set a finite
#   limit to the backtrace to prevent the debugger falling into
#   an endless loop trying to read the backtrace
set backtrace limit 32

set print asm-demangle on
set print pretty on

# Load the specified firmware onto the device
load

# Reset the target device before running (using openocd)
monitor reset halt
# monitor arm semihosting enable

# Begin running the program
continue