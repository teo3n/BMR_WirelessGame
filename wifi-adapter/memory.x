/* This linker script generated from xt-genldscripts.tpp for LSP . */
/* Linker Script for ld -N */
MEMORY
{
  dport0_0_seg :                        org = 0x3FF00000, len = 0x10
  dram0_0_seg :                         org = 0x3FFE8000, len = 0x14000
  iram1_0_seg :                         org = 0x40100000, len = 0xf000
  irom0_0_seg :                         org = 0x40210000, len = 0x5C000
}

PHDRS
{
  dport0_0_phdr PT_LOAD;
  dram0_0_phdr PT_LOAD;
  dram0_0_bss_phdr PT_LOAD;
  iram1_0_phdr PT_LOAD;
  irom0_0_phdr PT_LOAD;
}


/*  Default entry point:  */
ENTRY(call_user_start)
EXTERN(_DebugExceptionVector)
EXTERN(_DoubleExceptionVector)
EXTERN(_KernelExceptionVector)
EXTERN(_NMIExceptionVector)
EXTERN(_UserExceptionVector)
PROVIDE(_memmap_vecbase_reset = 0x40000000);
/* Various memory-map dependent cache attribute settings: */
_memmap_cacheattr_wb_base = 0x00000110;
_memmap_cacheattr_wt_base = 0x00000110;
_memmap_cacheattr_bp_base = 0x00000220;
_memmap_cacheattr_unused_mask = 0xFFFFF00F;
_memmap_cacheattr_wb_trapnull = 0x2222211F;
_memmap_cacheattr_wba_trapnull = 0x2222211F;
_memmap_cacheattr_wbna_trapnull = 0x2222211F;
_memmap_cacheattr_wt_trapnull = 0x2222211F;
_memmap_cacheattr_bp_trapnull = 0x2222222F;
_memmap_cacheattr_wb_strict = 0xFFFFF11F;
_memmap_cacheattr_wt_strict = 0xFFFFF11F;
_memmap_cacheattr_bp_strict = 0xFFFFF22F;
_memmap_cacheattr_wb_allvalid = 0x22222112;
_memmap_cacheattr_wt_allvalid = 0x22222112;
_memmap_cacheattr_bp_allvalid = 0x22222222;
PROVIDE(_memmap_cacheattr_reset = _memmap_cacheattr_wb_trapnull);

SECTIONS
{

  .dport0.rodata : ALIGN(4)
  {
    _dport0_rodata_start = ABSOLUTE(.);
    *(.dport0.rodata)
    *(.dport.rodata)
    _dport0_rodata_end = ABSOLUTE(.);
  } >dport0_0_seg :dport0_0_phdr

  .dport0.literal : ALIGN(4)
  {
    _dport0_literal_start = ABSOLUTE(.);
    *(.dport0.literal)
    *(.dport.literal)
    _dport0_literal_end = ABSOLUTE(.);
  } >dport0_0_seg :dport0_0_phdr

  .dport0.data : ALIGN(4)
  {
    _dport0_data_start = ABSOLUTE(.);
    *(.dport0.data)
    *(.dport.data)
    _dport0_data_end = ABSOLUTE(.);
  } >dport0_0_seg :dport0_0_phdr

  .data : ALIGN(4)
  {
    _data_start = ABSOLUTE(.);
    *(.data)
    *(.data.*)
    *(.gnu.linkonce.d.*)
    *(.data1)
    *(.sdata)
    *(.sdata.*)
    *(.gnu.linkonce.s.*)
    *(.sdata2)
    *(.sdata2.*)
    *(.gnu.linkonce.s2.*)
    *(.jcr)
    _data_end = ABSOLUTE(.);
  } >dram0_0_seg :dram0_0_phdr

  .rodata : ALIGN(4)
  {
    _rodata_start = ABSOLUTE(.);
    *(.sdk.version)
    *(.rodata)
    *(.rodata.*)
    *(.gnu.linkonce.r.*)
    *(.rodata1)
    __XT_EXCEPTION_TABLE__ = ABSOLUTE(.);
    *(.xt_except_table)
    *(.gcc_except_table)
    *(.gnu.linkonce.e.*)
    *(.gnu.version_r)
    *(.eh_frame)
    /*  C++ constructor and destructor tables, properly ordered:  */
    KEEP (*crtbegin.o(.ctors))
    KEEP (*(EXCLUDE_FILE (*crtend.o) .ctors))
    KEEP (*(SORT(.ctors.*)))
    KEEP (*(.ctors))
    KEEP (*crtbegin.o(.dtors))
    KEEP (*(EXCLUDE_FILE (*crtend.o) .dtors))
    KEEP (*(SORT(.dtors.*)))
    KEEP (*(.dtors))
    /*  C++ exception handlers table:  */
    __XT_EXCEPTION_DESCS__ = ABSOLUTE(.);
    *(.xt_except_desc)
    *(.gnu.linkonce.h.*)
    __XT_EXCEPTION_DESCS_END__ = ABSOLUTE(.);
    *(.xt_except_desc_end)
    *(.dynamic)
    *(.gnu.version_d)
    . = ALIGN(4);		/* this table MUST be 4-byte aligned */
    _bss_table_start = ABSOLUTE(.);
    LONG(_bss_start)
    LONG(_bss_end)
    _bss_table_end = ABSOLUTE(.);
    _rodata_end = ABSOLUTE(.);
  } >dram0_0_seg :dram0_0_phdr

  .bss ALIGN(8) (NOLOAD) : ALIGN(4)
  {
    . = ALIGN (8);
    _bss_start = ABSOLUTE(.);
    *(.dynsbss)
    *(.sbss)
    *(.sbss.*)
    *(.gnu.linkonce.sb.*)
    *(.scommon)
    *(.sbss2)
    *(.sbss2.*)
    *(.gnu.linkonce.sb2.*)
    *(.dynbss)
    *(.bss)
    *(.bss.*)
    *(.gnu.linkonce.b.*)
    *(COMMON)
    . = ALIGN (8);
    _bss_end = ABSOLUTE(.);
    _heap_start = ABSOLUTE(.);
/*    _stack_sentry = ALIGN(0x8); */
  } >dram0_0_seg :dram0_0_bss_phdr
/* __stack = 0x3ffc8000; */

  .irom0.text : ALIGN(4)
  {
    _irom0_text_start = ABSOLUTE(.);

    *libat.a:(.literal.* .text.*)
    *libcrypto.a:(.literal.* .text.*)
    *libespnow.a:(.literal.* .text.*)
    *libjson.a:(.literal.* .text.*)
    *liblwip.a:(.literal.* .text.*)
    *libnet80211.a:(.literal.* .text.*)
    *libsmartconfig.a:(.literal.* .text.*)
    *libssl.a:(.literal.* .text.*)
    *libupgrade.a:(.literal.* .text.*)
    *libwpa.a:(.literal.* .text.*)
    *libwpa2.a:(.literal.* .text.*)
    *libwps.a:(.literal.* .text.*)

    *libmbedtls.a:(.literal.* .text.*)

    *libm.a:(.literal .text .literal.* .text.*)

    *(.irom0.literal .irom.literal .irom.text.literal .irom0.text .irom.text)
    _irom0_text_end = ABSOLUTE(.);
  } >irom0_0_seg :irom0_0_phdr

  .text : ALIGN(4)
  {
    _stext = .;
    _text_start = ABSOLUTE(.);
    *(.UserEnter.text)
    . = ALIGN(16);
    *(.DebugExceptionVector.text)
    . = ALIGN(16);
    *(.NMIExceptionVector.text)
    . = ALIGN(16);
    *(.KernelExceptionVector.text)
    LONG(0)
    LONG(0)
    LONG(0)
    LONG(0)
    . = ALIGN(16);
    *(.UserExceptionVector.text)
    LONG(0)
    LONG(0)
    LONG(0)
    LONG(0)
    . = ALIGN(16);
    *(.DoubleExceptionVector.text)
    LONG(0)
    LONG(0)
    LONG(0)
    LONG(0)
    . = ALIGN (16);
    *(.entry.text)
    *(.init.literal)
    *(.init)
    *(.literal .text .literal.* .text.* .stub .gnu.warning .gnu.linkonce.literal.* .gnu.linkonce.t.*.literal .gnu.linkonce.t.*)
    *(.fini.literal)
    *(.fini)
    *(.gnu.version)
    _text_end = ABSOLUTE(.);
    _etext = .;
  } >iram1_0_seg :iram1_0_phdr

  .lit4 : ALIGN(4)
  {
    _lit4_start = ABSOLUTE(.);
    *(*.lit4)
    *(.lit4.*)
    *(.gnu.linkonce.lit4.*)
    _lit4_end = ABSOLUTE(.);
  } >iram1_0_seg :iram1_0_phdr
}

PROVIDE ( Cache_Read_Disable = 0x400047f0 );
PROVIDE ( Cache_Read_Enable = 0x40004678 );
PROVIDE ( FilePacketSendReqMsgProc = 0x400035a0 );
PROVIDE ( FlashDwnLdParamCfgMsgProc = 0x4000368c );
PROVIDE ( FlashDwnLdStartMsgProc = 0x40003538 );
PROVIDE ( FlashDwnLdStopReqMsgProc = 0x40003658 );
PROVIDE ( GetUartDevice = 0x40003f4c );
PROVIDE ( MD5Final = 0x40009900 );
PROVIDE ( MD5Init = 0x40009818 );
PROVIDE ( MD5Update = 0x40009834 );
PROVIDE ( MemDwnLdStartMsgProc = 0x400036c4 );
PROVIDE ( MemDwnLdStopReqMsgProc = 0x4000377c );
PROVIDE ( MemPacketSendReqMsgProc = 0x400036f0 );
PROVIDE ( RcvMsg = 0x40003eac );
PROVIDE ( SHA1Final = 0x4000b648 );
PROVIDE ( SHA1Init = 0x4000b584 );
PROVIDE ( SHA1Transform = 0x4000a364 );
PROVIDE ( SHA1Update = 0x4000b5a8 );
PROVIDE ( SPI_read_status = 0x400043c8 );
PROVIDE ( SPI_write_status = 0x40004400 );
PROVIDE ( SPI_write_enable = 0x4000443c );
PROVIDE ( Wait_SPI_Idle = 0x4000448c );
PROVIDE ( Enable_QMode = 0x400044c0 );
PROVIDE ( SPIEraseArea = 0x40004b44 );
PROVIDE ( SPIEraseBlock = 0x400049b4 );
PROVIDE ( SPIEraseChip = 0x40004984 );
PROVIDE ( SPIEraseSector = 0x40004a00 );
PROVIDE ( SPILock = 0x400048a8 );
PROVIDE ( SPIParamCfg = 0x40004c2c );
PROVIDE ( SPIRead = 0x40004b1c );
PROVIDE ( SPIReadModeCnfig = 0x400048ec );
PROVIDE ( SPIUnlock = 0x40004878 );
PROVIDE ( SPIWrite = 0x40004a4c );
PROVIDE ( SelectSpiFunction = 0x40003f58 );
PROVIDE ( SendMsg = 0x40003cf4 );
PROVIDE ( UartConnCheck = 0x40003230 );
PROVIDE ( UartConnectProc = 0x400037a0 );
PROVIDE ( UartDwnLdProc = 0x40003368 );
PROVIDE ( UartGetCmdLn = 0x40003ef4 );
PROVIDE ( UartRegReadProc = 0x4000381c );
PROVIDE ( UartRegWriteProc = 0x400037ac );
PROVIDE ( UartRxString = 0x40003c30 );
PROVIDE ( Uart_Init = 0x40003a14 );
PROVIDE ( _DebugExceptionVector = 0x40000010 );
PROVIDE ( _DoubleExceptionVector = 0x40000070 );
PROVIDE ( _KernelExceptionVector = 0x40000030 );
PROVIDE ( _NMIExceptionVector = 0x40000020 );
PROVIDE ( _ResetHandler = 0x400000a4 );
PROVIDE ( _ResetVector = 0x40000080 );
PROVIDE ( _UserExceptionVector = 0x40000050 );
PROVIDE ( __adddf3 = 0x4000c538 );
PROVIDE ( __addsf3 = 0x4000c180 );
PROVIDE ( __divdf3 = 0x4000cb94 );
PROVIDE ( __divdi3 = 0x4000ce60 );
PROVIDE ( __divsi3 = 0x4000dc88 );
PROVIDE ( __extendsfdf2 = 0x4000cdfc );
PROVIDE ( __fixdfsi = 0x4000ccb8 );
PROVIDE ( __fixunsdfsi = 0x4000cd00 );
PROVIDE ( __fixunssfsi = 0x4000c4c4 );
PROVIDE ( __floatsidf = 0x4000e2f0 );
PROVIDE ( __floatsisf = 0x4000e2ac );
PROVIDE ( __floatunsidf = 0x4000e2e8 );
PROVIDE ( __floatunsisf = 0x4000e2a4 );
PROVIDE ( __muldf3 = 0x4000c8f0 );
PROVIDE ( __muldi3 = 0x40000650 );
PROVIDE ( __mulsf3 = 0x4000c3dc );
PROVIDE ( __subdf3 = 0x4000c688 );
PROVIDE ( __subsf3 = 0x4000c268 );
PROVIDE ( __truncdfsf2 = 0x4000cd5c );
PROVIDE ( __udivdi3 = 0x4000d310 );
PROVIDE ( __udivsi3 = 0x4000e21c );
PROVIDE ( __umoddi3 = 0x4000d770 );
PROVIDE ( __umodsi3 = 0x4000e268 );
PROVIDE ( __umulsidi3 = 0x4000dcf0 );
PROVIDE ( _rom_store = 0x4000e388 );
PROVIDE ( _rom_store_table = 0x4000e328 );
PROVIDE ( _start = 0x4000042c );
PROVIDE ( _xtos_alloca_handler = 0x4000dbe0 );
PROVIDE ( _xtos_c_wrapper_handler = 0x40000598 );
PROVIDE ( _xtos_cause3_handler = 0x40000590 );
PROVIDE ( _xtos_ints_off = 0x4000bda4 );
PROVIDE ( _xtos_ints_on = 0x4000bd84 );
PROVIDE ( _xtos_l1int_handler = 0x4000048c );
PROVIDE ( _xtos_p_none = 0x4000dbf8 );
PROVIDE ( _xtos_restore_intlevel = 0x4000056c );
PROVIDE ( _xtos_return_from_exc = 0x4000dc54 );
PROVIDE ( _xtos_set_exception_handler = 0x40000454 );
PROVIDE ( _xtos_set_interrupt_handler = 0x4000bd70 );
PROVIDE ( _xtos_set_interrupt_handler_arg = 0x4000bd28 );
PROVIDE ( _xtos_set_intlevel = 0x4000dbfc );
PROVIDE ( _xtos_set_min_intlevel = 0x4000dc18 );
PROVIDE ( _xtos_set_vpri = 0x40000574 );
PROVIDE ( _xtos_syscall_handler = 0x4000dbe4 );
PROVIDE ( _xtos_unhandled_exception = 0x4000dc44 );
PROVIDE ( _xtos_unhandled_interrupt = 0x4000dc3c );
PROVIDE ( aes_decrypt = 0x400092d4 );
PROVIDE ( aes_decrypt_deinit = 0x400092e4 );
PROVIDE ( aes_decrypt_init = 0x40008ea4 );
PROVIDE ( aes_unwrap = 0x40009410 );
PROVIDE ( base64_decode = 0x40009648 );
PROVIDE ( base64_encode = 0x400094fc );
PROVIDE ( bzero = 0x4000de84 );
PROVIDE ( cmd_parse = 0x40000814 );
PROVIDE ( conv_str_decimal = 0x40000b24 );
PROVIDE ( conv_str_hex = 0x40000cb8 );
PROVIDE ( convert_para_str = 0x40000a60 );
PROVIDE ( dtm_get_intr_mask = 0x400026d0 );
PROVIDE ( dtm_params_init = 0x4000269c );
PROVIDE ( dtm_set_intr_mask = 0x400026c8 );
PROVIDE ( dtm_set_params = 0x400026dc );
PROVIDE ( eprintf = 0x40001d14 );
PROVIDE ( eprintf_init_buf = 0x40001cb8 );
PROVIDE ( eprintf_to_host = 0x40001d48 );
PROVIDE ( est_get_printf_buf_remain_len = 0x40002494 );
PROVIDE ( est_reset_printf_buf_len = 0x4000249c );
PROVIDE ( ets_bzero = 0x40002ae8 );
PROVIDE ( ets_char2xdigit = 0x40002b74 );
PROVIDE ( ets_delay_us = 0x40002ecc );
PROVIDE ( ets_enter_sleep = 0x400027b8 );
PROVIDE ( ets_external_printf = 0x40002578 );
PROVIDE ( ets_get_cpu_frequency = 0x40002f0c );
PROVIDE ( ets_getc = 0x40002bcc );
PROVIDE ( ets_install_external_printf = 0x40002450 );
PROVIDE ( ets_install_putc1 = 0x4000242c );
PROVIDE ( ets_install_putc2 = 0x4000248c );
PROVIDE ( ets_install_uart_printf = 0x40002438 );
PROVIDE ( ets_intr_lock = 0x40000f74 );
PROVIDE ( ets_intr_unlock = 0x40000f80 );
PROVIDE ( ets_isr_attach = 0x40000f88 );
PROVIDE ( ets_isr_mask = 0x40000f98 );
PROVIDE ( ets_isr_unmask = 0x40000fa8 );
PROVIDE ( ets_memcmp = 0x400018d4 );
PROVIDE ( ets_memcpy = 0x400018b4 );
PROVIDE ( ets_memmove = 0x400018c4 );
PROVIDE ( ets_memset = 0x400018a4 );
PROVIDE ( ets_post = 0x40000e24 );
PROVIDE ( ets_printf = 0x400024cc );
PROVIDE ( ets_putc = 0x40002be8 );
PROVIDE ( ets_rtc_int_register = 0x40002a40 );
PROVIDE ( ets_run = 0x40000e04 );
PROVIDE ( ets_set_idle_cb = 0x40000dc0 );
PROVIDE ( ets_set_user_start = 0x40000fbc );
PROVIDE ( ets_str2macaddr = 0x40002af8 );
PROVIDE ( ets_strcmp = 0x40002aa8 );
PROVIDE ( ets_strcpy = 0x40002a88 );
PROVIDE ( ets_strlen = 0x40002ac8 );
PROVIDE ( ets_strncmp = 0x40002ab8 );
PROVIDE ( ets_strncpy = 0x40002a98 );
PROVIDE ( ets_strstr = 0x40002ad8 );
PROVIDE ( ets_task = 0x40000dd0 );
PROVIDE ( ets_timer_arm = 0x40002cc4 );
PROVIDE ( ets_timer_disarm = 0x40002d40 );
PROVIDE ( ets_timer_done = 0x40002d80 );
PROVIDE ( ets_timer_handler_isr = 0x40002da8 );
PROVIDE ( ets_timer_init = 0x40002e68 );
PROVIDE ( ets_timer_setfn = 0x40002c48 );
PROVIDE ( ets_uart_printf = 0x40002544 );
PROVIDE ( ets_update_cpu_frequency = 0x40002f04 );
PROVIDE ( ets_vprintf = 0x40001f00 );
PROVIDE ( ets_wdt_disable = 0x400030f0 );
PROVIDE ( ets_wdt_enable = 0x40002fa0 );
PROVIDE ( ets_wdt_get_mode = 0x40002f34 );
PROVIDE ( ets_wdt_init = 0x40003170 );
PROVIDE ( ets_wdt_restore = 0x40003158 );
PROVIDE ( ets_write_char = 0x40001da0 );
PROVIDE ( get_first_seg = 0x4000091c );
PROVIDE ( gpio_init = 0x40004c50 );
PROVIDE ( gpio_input_get = 0x40004cf0 );
PROVIDE ( gpio_intr_ack = 0x40004dcc );
PROVIDE ( gpio_intr_handler_register = 0x40004e28 );
PROVIDE ( gpio_intr_pending = 0x40004d88 );
PROVIDE ( gpio_intr_test = 0x40004efc );
PROVIDE ( gpio_output_set = 0x40004cd0 );
PROVIDE ( gpio_pin_intr_state_set = 0x40004d90 );
PROVIDE ( gpio_pin_wakeup_disable = 0x40004ed4 );
PROVIDE ( gpio_pin_wakeup_enable = 0x40004e90 );
PROVIDE ( gpio_register_get = 0x40004d5c );
PROVIDE ( gpio_register_set = 0x40004d04 );
PROVIDE ( hmac_md5 = 0x4000a2cc );
PROVIDE ( hmac_md5_vector = 0x4000a160 );
PROVIDE ( hmac_sha1 = 0x4000ba28 );
PROVIDE ( hmac_sha1_vector = 0x4000b8b4 );
PROVIDE ( lldesc_build_chain = 0x40004f40 );
PROVIDE ( lldesc_num2link = 0x40005050 );
PROVIDE ( lldesc_set_owner = 0x4000507c );
PROVIDE ( main = 0x40000fec );
PROVIDE ( md5_vector = 0x400097ac );
PROVIDE ( mem_calloc = 0x40001c2c );
PROVIDE ( mem_free = 0x400019e0 );
PROVIDE ( mem_init = 0x40001998 );
PROVIDE ( mem_malloc = 0x40001b40 );
PROVIDE ( mem_realloc = 0x40001c6c );
PROVIDE ( mem_trim = 0x40001a14 );
PROVIDE ( mem_zalloc = 0x40001c58 );
PROVIDE ( memcmp = 0x4000dea8 );
PROVIDE ( memcpy = 0x4000df48 );
PROVIDE ( memmove = 0x4000e04c );
PROVIDE ( memset = 0x4000e190 );
PROVIDE ( multofup = 0x400031c0 );
PROVIDE ( pbkdf2_sha1 = 0x4000b840 );
PROVIDE ( phy_get_romfuncs = 0x40006b08 );
PROVIDE ( rand = 0x40000600 );
PROVIDE ( rc4_skip = 0x4000dd68 );
PROVIDE ( recv_packet = 0x40003d08 );
PROVIDE ( remove_head_space = 0x40000a04 );
PROVIDE ( rijndaelKeySetupDec = 0x40008dd0 );
PROVIDE ( rijndaelKeySetupEnc = 0x40009300 );
PROVIDE ( rom_abs_temp = 0x400060c0 );
PROVIDE ( rom_ana_inf_gating_en = 0x40006b10 );
PROVIDE ( rom_cal_tos_v50 = 0x40007a28 );
PROVIDE ( rom_chip_50_set_channel = 0x40006f84 );
PROVIDE ( rom_chip_v5_disable_cca = 0x400060d0 );
PROVIDE ( rom_chip_v5_enable_cca = 0x400060ec );
PROVIDE ( rom_chip_v5_rx_init = 0x4000711c );
PROVIDE ( rom_chip_v5_sense_backoff = 0x4000610c );
PROVIDE ( rom_chip_v5_tx_init = 0x4000718c );
PROVIDE ( rom_dc_iq_est = 0x4000615c );
PROVIDE ( rom_en_pwdet = 0x400061b8 );
PROVIDE ( rom_get_bb_atten = 0x40006238 );
PROVIDE ( rom_get_corr_power = 0x40006260 );
PROVIDE ( rom_get_fm_sar_dout = 0x400062dc );
PROVIDE ( rom_get_noisefloor = 0x40006394 );
PROVIDE ( rom_get_power_db = 0x400063b0 );
PROVIDE ( rom_i2c_readReg = 0x40007268 );
PROVIDE ( rom_i2c_readReg_Mask = 0x4000729c );
PROVIDE ( rom_i2c_writeReg = 0x400072d8 );
PROVIDE ( rom_i2c_writeReg_Mask = 0x4000730c );
PROVIDE ( rom_iq_est_disable = 0x40006400 );
PROVIDE ( rom_iq_est_enable = 0x40006430 );
PROVIDE ( rom_linear_to_db = 0x40006484 );
PROVIDE ( rom_mhz2ieee = 0x400065a4 );
PROVIDE ( rom_pbus_dco___SA2 = 0x40007bf0 );
PROVIDE ( rom_pbus_debugmode = 0x4000737c );
PROVIDE ( rom_pbus_enter_debugmode = 0x40007410 );
PROVIDE ( rom_pbus_exit_debugmode = 0x40007448 );
PROVIDE ( rom_pbus_force_test = 0x4000747c );
PROVIDE ( rom_pbus_rd = 0x400074d8 );
PROVIDE ( rom_pbus_set_rxgain = 0x4000754c );
PROVIDE ( rom_pbus_set_txgain = 0x40007610 );
PROVIDE ( rom_pbus_workmode = 0x40007648 );
PROVIDE ( rom_pbus_xpd_rx_off = 0x40007688 );
PROVIDE ( rom_pbus_xpd_rx_on = 0x400076cc );
PROVIDE ( rom_pbus_xpd_tx_off = 0x400076fc );
PROVIDE ( rom_pbus_xpd_tx_on = 0x40007740 );
PROVIDE ( rom_pbus_xpd_tx_on__low_gain = 0x400077a0 );
PROVIDE ( rom_phy_reset_req = 0x40007804 );
PROVIDE ( rom_restart_cal = 0x4000781c );
PROVIDE ( rom_rfcal_pwrctrl = 0x40007eb4 );
PROVIDE ( rom_rfcal_rxiq = 0x4000804c );
PROVIDE ( rom_rfcal_rxiq_set_reg = 0x40008264 );
PROVIDE ( rom_rfcal_txcap = 0x40008388 );
PROVIDE ( rom_rfcal_txiq = 0x40008610 );
PROVIDE ( rom_rfcal_txiq_cover = 0x400088b8 );
PROVIDE ( rom_rfcal_txiq_set_reg = 0x40008a70 );
PROVIDE ( rom_rfpll_reset = 0x40007868 );
PROVIDE ( rom_rfpll_set_freq = 0x40007968 );
PROVIDE ( rom_rxiq_cover_mg_mp = 0x40008b6c );
PROVIDE ( rom_rxiq_get_mis = 0x40006628 );
PROVIDE ( rom_sar_init = 0x40006738 );
PROVIDE ( rom_set_ana_inf_tx_scale = 0x4000678c );
PROVIDE ( rom_set_channel_freq = 0x40006c50 );
PROVIDE ( rom_set_loopback_gain = 0x400067c8 );
PROVIDE ( rom_set_noise_floor = 0x40006830 );
PROVIDE ( rom_set_rxclk_en = 0x40006550 );
PROVIDE ( rom_set_txbb_atten = 0x40008c6c );
PROVIDE ( rom_set_txclk_en = 0x4000650c );
PROVIDE ( rom_set_txiq_cal = 0x40008d34 );
PROVIDE ( rom_start_noisefloor = 0x40006874 );
PROVIDE ( rom_start_tx_tone = 0x400068b4 );
PROVIDE ( rom_stop_tx_tone = 0x4000698c );
PROVIDE ( rom_tx_mac_disable = 0x40006a98 );
PROVIDE ( rom_tx_mac_enable = 0x40006ad4 );
PROVIDE ( rom_txtone_linear_pwr = 0x40006a1c );
PROVIDE ( rom_write_rfpll_sdm = 0x400078dc );
PROVIDE ( roundup2 = 0x400031b4 );
PROVIDE ( rtc_enter_sleep = 0x40002870 );
PROVIDE ( rtc_get_reset_reason = 0x400025e0 );
PROVIDE ( rtc_intr_handler = 0x400029ec );
PROVIDE ( rtc_set_sleep_mode = 0x40002668 );
PROVIDE ( save_rxbcn_mactime = 0x400027a4 );
PROVIDE ( save_tsf_us = 0x400027ac );
PROVIDE ( send_packet = 0x40003c80 );
PROVIDE ( sha1_prf = 0x4000ba48 );
PROVIDE ( sha1_vector = 0x4000a2ec );
PROVIDE ( sip_alloc_to_host_evt = 0x40005180 );
PROVIDE ( sip_get_ptr = 0x400058a8 );
PROVIDE ( sip_get_state = 0x40005668 );
PROVIDE ( sip_init_attach = 0x4000567c );
PROVIDE ( sip_install_rx_ctrl_cb = 0x4000544c );
PROVIDE ( sip_install_rx_data_cb = 0x4000545c );
PROVIDE ( sip_post = 0x400050fc );
PROVIDE ( sip_post_init = 0x400056c4 );
PROVIDE ( sip_reclaim_from_host_cmd = 0x4000534c );
PROVIDE ( sip_reclaim_tx_data_pkt = 0x400052c0 );
PROVIDE ( sip_send = 0x40005808 );
PROVIDE ( sip_to_host_chain_append = 0x40005864 );
PROVIDE ( sip_to_host_evt_send_done = 0x40005234 );
PROVIDE ( slc_add_credits = 0x400060ac );
PROVIDE ( slc_enable = 0x40005d90 );
PROVIDE ( slc_from_host_chain_fetch = 0x40005f24 );
PROVIDE ( slc_from_host_chain_recycle = 0x40005e94 );
PROVIDE ( slc_init_attach = 0x40005c50 );
PROVIDE ( slc_init_credit = 0x4000608c );
PROVIDE ( slc_pause_from_host = 0x40006014 );
PROVIDE ( slc_reattach = 0x40005c1c );
PROVIDE ( slc_resume_from_host = 0x4000603c );
PROVIDE ( slc_select_tohost_gpio = 0x40005dc0 );
PROVIDE ( slc_select_tohost_gpio_mode = 0x40005db8 );
PROVIDE ( slc_send_to_host_chain = 0x40005de4 );
PROVIDE ( slc_set_host_io_max_window = 0x40006068 );
PROVIDE ( slc_to_host_chain_recycle = 0x40005f10 );
PROVIDE ( software_reset = 0x4000264c );
PROVIDE ( spi_flash_attach = 0x40004644 );
PROVIDE ( srand = 0x400005f0 );
PROVIDE ( strcmp = 0x4000bdc8 );
PROVIDE ( strcpy = 0x4000bec8 );
PROVIDE ( strlen = 0x4000bf4c );
PROVIDE ( strncmp = 0x4000bfa8 );
PROVIDE ( strncpy = 0x4000c0a0 );
PROVIDE ( strstr = 0x4000e1e0 );
PROVIDE ( timer_insert = 0x40002c64 );
PROVIDE ( uartAttach = 0x4000383c );
PROVIDE ( uart_baudrate_detect = 0x40003924 );
PROVIDE ( uart_buff_switch = 0x400038a4 );
PROVIDE ( uart_rx_intr_handler = 0x40003bbc );
PROVIDE ( uart_rx_one_char = 0x40003b8c );
PROVIDE ( uart_rx_one_char_block = 0x40003b64 );
PROVIDE ( uart_rx_readbuff = 0x40003ec8 );
PROVIDE ( uart_tx_one_char = 0x40003b30 );
PROVIDE ( wepkey_128 = 0x4000bc40 );
PROVIDE ( wepkey_64 = 0x4000bb3c );
PROVIDE ( xthal_bcopy = 0x40000688 );
PROVIDE ( xthal_copy123 = 0x4000074c );
PROVIDE ( xthal_get_ccompare = 0x4000dd4c );
PROVIDE ( xthal_get_ccount = 0x4000dd38 );
PROVIDE ( xthal_get_interrupt = 0x4000dd58 );
PROVIDE ( xthal_get_intread = 0x4000dd58 );
PROVIDE ( xthal_memcpy = 0x400006c4 );
PROVIDE ( xthal_set_ccompare = 0x4000dd40 );
PROVIDE ( xthal_set_intclear = 0x4000dd60 );
PROVIDE ( xthal_spill_registers_into_stack_nw = 0x4000e320 );
PROVIDE ( xthal_window_spill = 0x4000e324 );
PROVIDE ( xthal_window_spill_nw = 0x4000e320 );

PROVIDE ( Te0 = 0x3fffccf0 );
PROVIDE ( Td0 = 0x3fffd100 );
PROVIDE ( Td4s = 0x3fffd500);
PROVIDE ( rcons = 0x3fffd0f0);
PROVIDE ( UartDev = 0x3fffde10 );
PROVIDE ( flashchip = 0x3fffc714);
