# WiringPi-rs
Rust bindings to the *wiringPi* library for interactions with the Raspberry Pi.


## Build and Deploy

To build for the Raspberry Pi ensure that you have the target:
**arm-unknown-linux-gnueabihf**
```
$ rustup target add arm-unknown-linux-gnueabihf
```

Along with the appropriate linker:
```
$ sudo apt-get install gcc-arm-linux-gnueabihf
```

And ensure Cargo knows to use this linker for the target by updating the `~/.cargo/config` to contain:
```
[target.arm-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
```

Then it is as simple as:
```
$ cargo build --target=arm-unknown-linux-gnueabihf
```

Finally you can put it onto the Raspberry Pi in whichever fashion you like, [`scp`] is quite simple to do for this.

[`scp`]: https://www.raspberrypi.org/documentation/remote-access/ssh/scp.md

## Examples

If you were wanting to know how to use the library the `examples` directory is
where you want to look. The goal is to align these to those provided on the
official site.
