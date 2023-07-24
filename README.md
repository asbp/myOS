## myOS
This is a simple OS based on [Writing an OS in Rust](https://os.phil-opp.com/ "Home") by Philipp Oppermann, with some changes:

 - It uses VGA driver from https://github.com/rust-osdev/vga.
 - It uses buddy system allocator from https://github.com/rcore-os/buddy_system_allocator.
 - It incorporates Hashbrown from https://github.com/rust-lang/hashbrown for HashMap implementation.
 - It uses event emitter/listener model for interrupts, inspired by http://thispage.tech:9680/jclee1995/rust-eventemitter.
 - It features a simple command interpreter in which the command parsing is powered by https://github.com/comex/rust-shlex.

## How to run?
You must have QEMU installed on your machine, then run `cargo run`. If the compilation process succeed, then a QEMU window will appear.

## Todo

 - [ ] Filesystem feature (for rootfs I choose ext4)
 - [ ] Time and date
 - [ ] Concurrency (important for running external binaries)
 - [ ] Ability to run binary
 - [ ] Attachable/Detachable device drivers
 - [ ] Portability (currently only supports x86-64 BIOS system)

## Q&A

### Q: How mature is this OS?
This OS is just my hobby OS so the pace of the development is depending on my spare time. For now, it just simple imitation of DOS. Feel free to contribute or create an Issue if you want to request a new feature.

### Q: Why using event emitter/listener model?
For simple usage, I just want to avoid directly tapping to interrupt handler (or create another one).

### Q: What can I do with the command interpeter?
Right now it just three:

 1. `whoami`: it returns `github`.
 2. `greet <name>`: it displays `Hello, <name>!`. For example, command `greet Somebody` will outputs `Hello, Somebody!`.
 3. `add <...numbers>`: it will add numbers specified in `numbers` argument and then display the result. For example, `add 25 25 10` will display `60`.