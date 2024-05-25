#!/bin/bash

cargo build --target i686-unknown-uefi
rm -rf esp/
mkdir -p esp/efi/boot/

cp target/i686-unknown-uefi/debug/uefi-test.efi esp/efi/boot/bootia32.efi

# qemu-system-i386 \
#     -drive if=pflash,format=raw,readonly=on,file=OVMF32_CODE_4M.secboot.fd \
#     -drive if=pflash,format=raw,readonly=on,file=OVMF32_VARS_4M.fd \
#     -drive format=raw,file=fat:rw:esp,media=disk \
#     -m 1G

# See https://www.qemu.org/docs/master/system/bootindex.html for information about how to set the boot order

qemu-system-i386 -pflash '/home/andy/Downloads/ovmf/usr/share/edk2.git/ovmf-ia32/OVMF-pure-efi.fd' \
                 -drive file=fat:rw:esp,format=raw,if=none,id=disk1 \
                 -device ide-hd,drive=disk1,bootindex=1 #force the boot order to start up from our bootloader not the UEFI shell
