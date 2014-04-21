RUSTC := rustc --target arm-unknown-linux-gnueabihf
ARMPATH := /Volumes/xtools/arm-none-eabi/bin
ARMGNU := $(ARMPATH)/arm-none-eabi

# The intermediate directory for compiled object files.
BUILD = build/

# The name of the output file to generate.
TARGET = kernel.img

# The name of the assembler listing file to generate.
LIST = kernel.list

# The name of the map file to generate.
MAP = kernel.map

# The name of the linker script to use.
LINKER = kernel.ld

# The names of all object files that must be generated.
OBJECTS := $(patsubst asm/%.s,$(BUILD)%.o,$(wildcard asm/*.s))
OBJECTS += $(patsubst kernel/%.rs,$(BUILD)%.o,$(wildcard kernel/*.rs))

all: $(TARGET) $(LIST)

rebuild: all

# Rule to make the listing file
$(LIST): $(BUILD)output.elf
	$(ARMGNU)-objdump -d $(BUILD)output.elf > $(LIST)

# Rule to make the image file.
$(TARGET): $(BUILD)output.elf
	$(ARMGNU)-objcopy $(BUILD)output.elf -O binary $(TARGET)

# Rule to make the elf file.
$(BUILD)output.elf: $(OBJECTS) $(LINKER)
	$(ARMGNU)-ld --no-undefined $(OBJECTS) -Map $(MAP) -o $(BUILD)output.elf -T $(LINKER)

# Rule to make the object files
$(BUILD)%.o: kernel/%.rs
	$(RUSTC) -O --out-dir $(BUILD) --emit obj --crate-type=lib -o $@ $<

$(BUILD)%.o: asm/%.s
	-mkdir $(BUILD)
	$(ARMGNU)-as -o $@ $<

# Rule to clean files.
clean:
	-rm -fr $(BUILD)
	-rm -f $(TARGET)
	-rm -f $(LIST)
	-rm -f $(MAP)
