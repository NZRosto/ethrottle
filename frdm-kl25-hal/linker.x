
MEMORY
{
    FLASH : ORIGIN = 0x00000000, LENGTH = 128K
    RAM : ORIGIN = 0x1FFFF000, LENGTH = 16K
}

INCLUDE device.x

EXTERN(__RESET_VECTOR);
EXTERN(Reset);
ENTRY(Reset);

EXTERN(__EXCEPTIONS);

EXTERN(DefaultHandler);

PROVIDE(NonMaskableInt = DefaultHandler);
EXTERN(HardFaultTrampoline);
PROVIDE(MemoryManagement = DefaultHandler);
PROVIDE(BusFault = DefaultHandler);
PROVIDE(UsageFault = DefaultHandler);
PROVIDE(SecureFault = DefaultHandler);
PROVIDE(SVCall = DefaultHandler);
PROVIDE(DebugMonitor = DefaultHandler);
PROVIDE(PendSV = DefaultHandler);
PROVIDE(SysTick = DefaultHandler);

PROVIDE(DefaultHandler = DefaultHandler_);
PROVIDE(HardFault = HardFault_);

EXTERN(__INTERRUPTS);

PROVIDE(__pre_init = DefaultPreInit);

SECTIONS
{
  PROVIDE(_ram_start = ORIGIN(RAM));
  PROVIDE(_ram_end = ORIGIN(RAM) + LENGTH(RAM));
  PROVIDE(_stack_start = _ram_end);

  .vector_table ORIGIN(FLASH) :
  {
    __vector_table = .;

    LONG(_stack_start & 0xFFFFFFF8);

    KEEP(*(.vector_table.reset_vector));

    __exceptions = .;
    KEEP(*(.vector_table.exceptions));
    __eexceptions = .;

    KEEP(*(.vector_table.interrupts));
  } > FLASH

  PROVIDE(__flash_protect = 0x00000400);

  .flash_protect __flash_protect :
  {
    . = 0x00000400;
    LONG(0xFFFFFFFF);
    LONG(0xFFFFFFFF);
    LONG(0xFFFFFFFF);
    LONG(0xFFFFFFFE);
  } > FLASH

  PROVIDE(_stext = ADDR(.flash_protect) + SIZEOF(.flash_protect));

  .text _stext :
  {
    __stext = .;
    *(.Reset);

    *(.text .text.*);

    *(.HardFaultTrampoline);
    *(.HardFault.*);

    . = ALIGN(4);
    __etext = .;
  } > FLASH

  .rodata : ALIGN(4)
  {
    . = ALIGN(4);
    __srodata = .;
    *(.rodata .rodata.*);

    . = ALIGN(4);
    __erodata = .;
  } > FLASH

  .data : ALIGN(4)
  {
    . = ALIGN(4);
    __sdata = .;
    *(.data .data.*);
    . = ALIGN(4);
  } > RAM AT>FLASH
  
  . = ALIGN(4);
  __edata = .;

  __sidata = LOADADDR(.data);

  .gnu.sgstubs : ALIGN(32)
  {
    . = ALIGN(32);
    __veneer_base = .;
    *(.gnu.sgstubs*)
    . = ALIGN(32);
  } > FLASH
 
  . = ALIGN(32);
  __veneer_limit = .;

  .bss (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    __sbss = .;
    *(.bss .bss.*);
    *(COMMON);
    . = ALIGN(4);
  } > RAM
 
  . = ALIGN(4);
  __ebss = .;

  .uninit (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    __suninit = .;
    *(.uninit .uninit.*);
    . = ALIGN(4);
    __euninit = .;
  } > RAM

  PROVIDE(__sheap = __euninit);

  PROVIDE(_stack_end = __euninit);

  .got (NOLOAD) :
  {
    KEEP(*(.got .got.*));
  }

  /DISCARD/ :
  {
    *(.ARM.exidx);
    *(.ARM.exidx.*);
    *(.ARM.extab.*);
  }
}

ASSERT(ORIGIN(FLASH) % 4 == 0, "
ERROR(cortex-m-rt): the start of the FLASH region must be 4-byte aligned");

ASSERT(ORIGIN(RAM) % 4 == 0, "
ERROR(cortex-m-rt): the start of the RAM region must be 4-byte aligned");

ASSERT(__sdata % 4 == 0 && __edata % 4 == 0, "
BUG(cortex-m-rt): .data is not 4-byte aligned");

ASSERT(__sidata % 4 == 0, "
BUG(cortex-m-rt): the LMA of .data is not 4-byte aligned");

ASSERT(__sbss % 4 == 0 && __ebss % 4 == 0, "
BUG(cortex-m-rt): .bss is not 4-byte aligned");

ASSERT(__sheap % 4 == 0, "
BUG(cortex-m-rt): start of .heap is not 4-byte aligned");

ASSERT(_stack_start % 8 == 0, "
ERROR(cortex-m-rt): stack start address is not 8-byte aligned.
If you have set _stack_start, check it's set to an address which is a multiple of 8 bytes.
If you haven't, stack starts at the end of RAM by default. Check that both RAM
origin and length are set to multiples of 8 in the `memory.x` file.");

ASSERT(_stack_end % 4 == 0, "
ERROR(cortex-m-rt): end of stack is not 4-byte aligned");

ASSERT(_stack_start >= _stack_end, "
ERROR(cortex-m-rt): stack end address is not below stack start.");


ASSERT(__exceptions == ADDR(.vector_table) + 0x8, "
BUG(cortex-m-rt): the reset vector is missing");

ASSERT(__eexceptions == ADDR(.vector_table) + 0x40, "
BUG(cortex-m-rt): the exception vectors are missing");

ASSERT(SIZEOF(.vector_table) > 0x40, "
ERROR(cortex-m-rt): The interrupt vectors are missing.
Possible solutions, from most likely to less likely:
- Link to a svd2rust generated device crate
- Check that you actually use the device/hal/bsp crate in your code
- Disable the 'device' feature of cortex-m-rt to build a generic application (a dependency
may be enabling it)
- Supply the interrupt handlers yourself. Check the documentation for details.");

ASSERT(ADDR(.vector_table) + SIZEOF(.vector_table) <= _stext, "
ERROR(cortex-m-rt): The .text section can't be placed inside the .vector_table section
Set _stext to an address greater than the end of .vector_table (See output of `nm`)");

ASSERT(_stext > ORIGIN(FLASH) && _stext < ORIGIN(FLASH) + LENGTH(FLASH), "
ERROR(cortex-m-rt): The .text section must be placed inside the FLASH memory.
Set _stext to an address within the FLASH region.");

ASSERT(SIZEOF(.got) == 0, "
ERROR(cortex-m-rt): .got section detected in the input object files
Dynamic relocations are not supported. If you are linking to C code compiled using
the 'cc' crate then modify your build script to compile the C code _without_
the -fPIC flag. See the documentation of the `cc::Build.pic` method for details.");

