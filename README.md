## SFLASHER

A tool for flashing custom qmk firmware for sn32f2xx based keyboards.
The flashing logic is based on the [sonix-flasher](https://github.com/SonixQMK/sonix-flasher)

How to flash

```sh
sflasher flash filename.bin
```
It will automatically detect if only one device is connected in bootloader mode and select that.

If multiple devices are connected in bootloader mode

```sh
sflasher list
```

and then

```sh
sflasher flash filename.bin -k vid:pid
```

Not sure on how to flash if multiple devices with same vid:pid is connected.
I don't have enough keyboards to test as well.
