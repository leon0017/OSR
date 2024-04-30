.PHONY: all clean

X86_UEFI_APP_DIR := arch/x86/boot/uefi-app

all:
	cd $(X86_UEFI_APP_DIR) && cargo build

	dd if=/dev/zero of=OROS.img bs=1M count=64
	mkfs.fat -F 32 OROS.img
	sudo mkdir -p SYS/
	sudo mount -o loop OROS.img SYS/
	sudo mkdir -p SYS/EFI/BOOT/
	sudo cp $(X86_UEFI_APP_DIR)/target/x86_64-unknown-uefi/debug/uefi-app.efi SYS/EFI/BOOT/BOOTX64.EFI
	sudo umount SYS
	sudo rm -rf SYS/

	sudo qemu-system-x86_64 -enable-kvm -cpu host -smp 2 -m 2G -bios /usr/share/ovmf/OVMF.fd -cdrom OROS.img -boot d

clean:
	cd $(X86_UEFI_APP_DIR) && cargo clean