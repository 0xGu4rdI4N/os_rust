# GuardOS
    This is an operating system kernel written in Rust.
## Build:

You can build the project by running:
``` cargo build ```

To create a bootable disk image from the compiled kernel, you need to install the bootimage tool:
``` cargo install bootimage ```

After installing, you can create the bootable disk image by running:
``` cargo bootimage ```

## Run

You can run the disk image in QEMU through:
``` cargo run ```



