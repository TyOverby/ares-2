# ares

The main ares crate.  This crate contains the three large pieces of functionality
that consumers of the embedded require.

1. The **compiler** takes program text and turns it into bytecode that the vm can execute.
2. The **vm** runs bytecode and provides fairly rudimentary inspection of the machine state.
3. The **host** is a wrapper around the compiler and vm.  "Host" is what defines
   the various Context types which form the main Ares programatic interface.

More information on all of these systems is available in the readme.md in their respective
directories.
