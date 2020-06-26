# Drogue Boot

This library provides (at this point, a very basic) mechanism for
a 2-stage booting of an embedded ARM Cortex-M system. 

Using whatever method you desire, create an embedded application
to be flashed at the default initial boot location, and use Drogue Boot
to boot the primary application located (linked against) the location
passed as the boot partition.

For instance, using `cortex-m-rt`, and RTT-based logging, a simple bootloader
application for an STM Bluepill might look like:

If the `rtt` feature is enabled on the dependency, an `RTTLogger` is available
for use.

```rust
use drogue_boot::{ 
    Boot,
    Logger, 
    rtt_logger::RTTLogger, 
}

#[entry]
fn entry() -> ! {
    let logger = RttLogger::new();

    let mut boot = Boot::new(0x08008000);
    boot.with_logger(&logger);
    logger.log_message("Booting MyApp now");

    boot.boot();
}
```

The bootloader's own `memory.x` places this application at `0x08000000`,
and boots the primary application located at `0x08008000`.

## Currently unsupported

Basically everything. The vector table is not yet adjusted, nor are interrupts
correctly disabled.

## Future features

* Support for on-device upgrades of the bootable image from secondary storage
(or elsewhere in flash)
* Image verification (signed hashes, etc).

