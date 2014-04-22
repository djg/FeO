ARMPATH :=
RUSTPATH :=

-include ./config.mk

ARMGNU := $(ARMPATH)arm-none-eabi

# The intermediate directory for compiled object files.
BUILD = build/

KERNEL := $(BUILD)kernel.elf

IMAGE := $(BUILD)kernel.img

# The name of the assembler listing file to generate.
LIST := $(BUILD)kernel.list

# The name of the map file to generate.
MAP := $(BUILD)kernel.map

# The name of the linker script to use.
LINKER = src/kernel.ld

RUSTC := $(RUSTPATH)rustc
RUSTCFLAGS := -O --target arm-unknown-linux-gnueabihf

LD := $(ARMGNU)-ld
LDFLAGS := --no-undefined -Map $(MAP)

AS := $(ARMGNU)-as
ASFLAGS :=

OBJDIR := $(BUILD)obj/

RSRCS := src/main.rs
ASMSRCS := src/start.S

# The names of all object files that must be generated.
OBJECTS := $(patsubst %.rs,$(OBJDIR)%.o,$(RSRCS))
OBJECTS += $(patsubst %.S,$(OBJDIR)%.S.o,$(ASMSRCS))

.PHONY: clean all

all: $(KERNEL) $(IMAGE) $(LIST)

rebuild: all

# Rule to make the listing file
$(LIST): $(KERNEL)
	@echo "[LIST]" $@
	@$(ARMGNU)-objdump -d $(KERNEL) > $(LIST)

# Rule to make the image file.
$(IMAGE): $(KERNEL)
	@echo "[IMG ]" $@
	@$(ARMGNU)-objcopy $(KERNEL) -O binary $(IMAGE)

# Rule to make the elf file.
$(KERNEL): $(OBJECTS) $(LINKER)
	@echo "[LINK]" $@
	@$(LD) $(LDFLAGS) -o $@ -T $(LINKER) $(OBJECTS)

# Rule to make the object files
$(OBJDIR)%.o: %.rs
	@-mkdir -p `dirname $@`
	@echo "[RUST]" $@
	@$(RUSTC) $(RUSTCFLAGS) --emit obj --crate-type=lib -o $@ $<

$(OBJDIR)%.S.o: %.S
	@-mkdir -p `dirname $@`
	@echo "[AS  ]" $@
	@$(AS) $(ASFLAGS) -o $@ $<

# Rule to clean files.
clean:
	-rm -fr $(BUILD)
