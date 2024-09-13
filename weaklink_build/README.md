# How to use
You will need to create a list of symbols that need to be stubbed. This may be done manually, however in many cases it
will be easier to extract APIs of the target dylib programmatically, using [`exports::dylib_exports`]
(this is a platform-independent wrapper around the [Goblin](https://crates.io/crates/goblin) library).

In order to effectivaly filter this list, `weaklink_build` also provides [`imports::archive_imports`]
function, which allows extracting the list of used symbols from your program's object files.

Having done that, you will need to create an instance of [`Config`] and add the symbols that need to be stubbed.
