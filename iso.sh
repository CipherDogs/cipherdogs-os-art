# compiling kernel
cargo bootimage

# Move kernel
mv target/x86_64-os/debug/bootimage-cipherdogs_os_art.bin boot/kernel.bin

# check multiboot
if grub-file --is-x86-multiboot boot/kernel.bin; then
  echo multiboot confirmed
else
  echo the file is not multiboot
fi

# create iso
mkdir -p isodir/boot/grub
cp boot/kernel.bin isodir/boot/kernel.bin
cp boot/grub.cfg isodir/boot/grub/grub.cfg
grub-mkrescue -o cipherdogs.iso isodir