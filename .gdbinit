# Allow reading/writing memory-mapped peripheral registers (0x4000_0000+).
# probe-rs sends GDB a memory map covering only flash+RAM, so GDB blocks
# reads outside it by default ("Cannot access memory at address ...").
set mem inaccessible-by-default off
