/*
 * ARM Cortex-M 自定义链接脚本
 *
 * 开发者注意事项:
 * - 双下划线开头的符号 (__) 被视为"私有"符号
 * - 单下划线开头的符号 (_) 被视为"半公开"符号，可在用户链接脚本中覆盖
 * - EXTERN 强制链接器保留符号在最终二进制文件中
 * - PROVIDE 用于提供可被用户链接脚本覆盖的默认值
 * - 对齐要求：.bss 和 .data 的 VMA 边界以及 .data 的 LMA 必须 4 字节对齐
 */

/* 包含设备内存布局信息 */
/* 由用户提供 (参见 `memory.x`) 或由板级支持包提供 */
INCLUDE memory.x

/* 程序入口点 = 复位向量 */
EXTERN(__RESET_VECTOR);
EXTERN(Reset);
ENTRY(Reset);

/* 异常向量表 */
/* 这实际上是链接器级别的弱别名 */
/* 用户可以通过定义相应符号来覆盖这些别名 (参见 `exception!` 宏) */
EXTERN(__EXCEPTIONS); /* 依赖于所有这些 PROVIDED 符号 */

EXTERN(DefaultHandler);

/* 异常处理程序默认值 */
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

/* 中断向量表 */
EXTERN(__INTERRUPTS); /* 类似于 `__EXCEPTIONS` 的静态变量 */

/* 预初始化函数 */
/* 如果用户使用 `pre_init!` 宏或创建 `__pre_init` 函数覆盖此函数，
   则该函数将在 RAM 初始化之前被调用 */
PROVIDE(__pre_init = DefaultPreInit);

/* 内存段定义 */
SECTIONS
{
  /* RAM 区域定义 */
  PROVIDE(_ram_start = ORIGIN(RAM));
  PROVIDE(_ram_end = ORIGIN(RAM) + LENGTH(RAM));
  PROVIDE(_stack_start = _ram_end);

  /* FLASH 中的段 */
  /* 中断向量表 */
  .vector_table ORIGIN(FLASH) :
  {
    __vector_table = .;

    /* 初始栈指针 (SP) 值
     * 屏蔽低 3 位以强制 8 字节对齐
     * 尽管后面有断言检查，但单独的链接脚本可能在断言检查后覆盖 _stack_start
     */
    LONG(_stack_start & 0xFFFFFFF8);

    /* 复位向量 */
    KEEP(*(.vector_table.reset_vector)); /* 这是 `__RESET_VECTOR` 符号 */

    /* 异常向量 */
    __exceptions = .; /* 异常开始位置 */
    KEEP(*(.vector_table.exceptions)); /* 这是 `__EXCEPTIONS` 符号 */
    __eexceptions = .; /* 异常结束位置 */

    /* 设备特定中断 */
    KEEP(*(.vector_table.interrupts)); /* 这是 `__INTERRUPTS` 符号 */
  } > FLASH

  PROVIDE(_stext = ADDR(.vector_table) + SIZEOF(.vector_table));

  /* 代码段 */
  .text _stext :
  {
    __stext = .;
    *(.Reset);

    *(.text .text.*);

    /* HardFaultTrampoline 使用 `b` 指令进入 `HardFault`，
       因此必须放置在其附近 */
    *(.HardFaultTrampoline);
    *(.HardFault.*);

    . = ALIGN(4); /* 填充 .text 到对齐边界，解决旧版 lld 的重叠加载段 bug */
    __etext = .;
  } > FLASH

  /* 只读数据段 */
  .rodata : ALIGN(4)
  {
    . = ALIGN(4);
    __srodata = .;
    *(.rodata .rodata.*);

    /* 4 字节对齐段末尾 (VMA)
       LLD 要求这样做以确保后续 .data 段的 LMA 具有正确的对齐 */
    . = ALIGN(4);
    __erodata = .;
  } > FLASH

  /* RAM 中的段 */
  /* 初始化数据段 */
  .data : ALIGN(4)
  {
    . = ALIGN(4);
    __sdata = .;
    *(.data .data.*);
    . = ALIGN(4); /* 4 字节对齐段末尾 (VMA) */
  } > RAM AT>FLASH
  /* 允许用户 `memory.x` 中使用 `INSERT AFTER .data` 注入的段
   * 通过推送 __edata 来使用 .data 加载机制。注意：不要在这些用户段中
   * 更改输出区域或加载区域！ */
  . = ALIGN(4);
  __edata = .;

  /* .data 段的加载地址 (LMA) */
  __sidata = LOADADDR(.data);

  /* ### .gnu.sgstubs
     This section contains the TrustZone-M veneers put there by the Arm GNU linker. */
  /* Security Attribution Unit blocks must be 32 bytes aligned. */
  /* Note that this pads the FLASH usage to 32 byte alignment. */
  .gnu.sgstubs : ALIGN(32)
  {
    . = ALIGN(32);
    __veneer_base = .;
    *(.gnu.sgstubs*)
    . = ALIGN(32);
  } > FLASH
  /* Place `__veneer_limit` outside the `.gnu.sgstubs` section because veneers are
   * always inserted last in the section, which would otherwise be _after_ the `__veneer_limit` symbol.
   */
  . = ALIGN(32);
  __veneer_limit = .;

  /* ### .bss */
  .bss (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    __sbss = .;
    *(.bss .bss.*);
    *(COMMON); /* Uninitialized C statics */
    . = ALIGN(4); /* 4-byte align the end (VMA) of this section */
  } > RAM
  /* Allow sections from user `memory.x` injected using `INSERT AFTER .bss` to
   * use the .bss zeroing mechanism by pushing __ebss. Note: do not change
   * output region or load region in those user sections! */
  . = ALIGN(4);
  __ebss = .;

  /* ### .uninit */
  .uninit (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    __suninit = .;
    *(.uninit .uninit.*);
    . = ALIGN(4);
    __euninit = .;
  } > RAM

  /* Place the heap right after `.uninit` in RAM */
  PROVIDE(__sheap = __euninit);

  /* Place stack end at the end of allocated RAM */
  PROVIDE(_stack_end = __euninit);

  /* ## .got */
  /* Dynamic relocations are unsupported. This section is only used to detect relocatable code in
     the input files and raise an error if relocatable code is found */
  .got (NOLOAD) :
  {
    KEEP(*(.got .got.*));
  }

  /* ## Discarded sections */
  /DISCARD/ :
  {
    /* Unused exception related info that only wastes space */
    *(.ARM.exidx);
    *(.ARM.exidx.*);
    *(.ARM.extab.*);
  }
}

/* Do not exceed this mark in the error messages below                                    | */
/* # Alignment checks */
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

/* # Position checks */

/* ## .vector_table
 *
 * If the *start* of exception vectors is not 8 bytes past the start of the
 * vector table, then we somehow did not place the reset vector, which should
 * live 4 bytes past the start of the vector table.
 */
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

/* ## .text */
ASSERT(ADDR(.vector_table) + SIZEOF(.vector_table) <= _stext, "
ERROR(cortex-m-rt): The .text section can't be placed inside the .vector_table section
Set _stext to an address greater than the end of .vector_table (See output of `nm`)");

ASSERT(_stext > ORIGIN(FLASH) && _stext < ORIGIN(FLASH) + LENGTH(FLASH), "
ERROR(cortex-m-rt): The .text section must be placed inside the FLASH memory.
Set _stext to an address within the FLASH region.");

/* # Other checks */
ASSERT(SIZEOF(.got) == 0, "
ERROR(cortex-m-rt): .got section detected in the input object files
Dynamic relocations are not supported. If you are linking to C code compiled using
the 'cc' crate then modify your build script to compile the C code _without_
the -fPIC flag. See the documentation of the `cc::Build.pic` method for details.");
/* Do not exceed this mark in the error messages above                                    | */

ASSERT(SIZEOF(.vector_table) <= 0x400, "
There can't be more than 240 interrupt handlers. This may be a bug in
your device crate, or you may have registered more than 240 interrupt
handlers.");
