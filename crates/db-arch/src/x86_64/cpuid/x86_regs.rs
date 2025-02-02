/* automatically generated by rust-bindgen */

// Copyright (C) 2019 Alibaba Cloud. All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(missing_docs)]
#![allow(non_upper_case_globals)]

pub const MSR_EFER: u32 = 3221225600;
pub const MSR_STAR: u32 = 3221225601;
pub const MSR_LSTAR: u32 = 3221225602;
pub const MSR_CSTAR: u32 = 3221225603;
pub const MSR_SYSCALL_MASK: u32 = 3221225604;
pub const MSR_FS_BASE: u32 = 3221225728;
pub const MSR_GS_BASE: u32 = 3221225729;
pub const MSR_KERNEL_GS_BASE: u32 = 3221225730;
pub const MSR_TSC_AUX: u32 = 3221225731;
pub const _EFER_SCE: u32 = 0;
pub const _EFER_LME: u32 = 8;
pub const _EFER_LMA: u32 = 10;
pub const _EFER_NX: u32 = 11;
pub const _EFER_SVME: u32 = 12;
pub const _EFER_LMSLE: u32 = 13;
pub const _EFER_FFXSR: u32 = 14;
pub const EFER_SCE: u32 = 1;
pub const EFER_LME: u32 = 256;
pub const EFER_LMA: u32 = 1024;
pub const EFER_NX: u32 = 2048;
pub const EFER_SVME: u32 = 4096;
pub const EFER_LMSLE: u32 = 8192;
pub const EFER_FFXSR: u32 = 16384;
pub const MSR_IA32_SPEC_CTRL: u32 = 72;
pub const SPEC_CTRL_IBRS: u32 = 1;
pub const SPEC_CTRL_STIBP: u32 = 2;
pub const SPEC_CTRL_SSBD_SHIFT: u32 = 2;
pub const SPEC_CTRL_SSBD: u32 = 4;
pub const MSR_IA32_PRED_CMD: u32 = 73;
pub const PRED_CMD_IBPB: u32 = 1;
pub const MSR_IA32_PERFCTR0: u32 = 193;
pub const MSR_IA32_PERFCTR1: u32 = 194;
pub const MSR_FSB_FREQ: u32 = 205;
pub const MSR_PLATFORM_INFO: u32 = 206;
pub const MSR_NHM_SNB_PKG_CST_CFG_CTL: u32 = 226;
pub const NHM_C3_AUTO_DEMOTE: u32 = 33554432;
pub const NHM_C1_AUTO_DEMOTE: u32 = 67108864;
pub const ATM_LNC_C6_AUTO_DEMOTE: u32 = 33554432;
pub const SNB_C1_AUTO_UNDEMOTE: u32 = 134217728;
pub const SNB_C3_AUTO_UNDEMOTE: u32 = 268435456;
pub const MSR_MTRRcap: u32 = 254;
pub const MSR_IA32_ARCH_CAPABILITIES: u32 = 266;
pub const ARCH_CAP_RDCL_NO: u32 = 1;
pub const ARCH_CAP_IBRS_ALL: u32 = 2;
pub const ARCH_CAP_SKIP_VMENTRY_L1DFLUSH: u32 = 8;
pub const ARCH_CAP_SSB_NO: u32 = 16;
pub const MSR_IA32_FLUSH_CMD: u32 = 267;
pub const L1D_FLUSH: u32 = 1;
pub const MSR_IA32_BBL_CR_CTL: u32 = 281;
pub const MSR_IA32_BBL_CR_CTL3: u32 = 286;
pub const MSR_IA32_SYSENTER_CS: u32 = 372;
pub const MSR_IA32_SYSENTER_ESP: u32 = 373;
pub const MSR_IA32_SYSENTER_EIP: u32 = 374;
pub const MSR_IA32_MCG_CAP: u32 = 377;
pub const MSR_IA32_MCG_STATUS: u32 = 378;
pub const MSR_IA32_MCG_CTL: u32 = 379;
pub const MSR_IA32_MCG_EXT_CTL: u32 = 1232;
pub const MSR_OFFCORE_RSP_0: u32 = 422;
pub const MSR_OFFCORE_RSP_1: u32 = 423;
pub const MSR_TURBO_RATIO_LIMIT: u32 = 429;
pub const MSR_TURBO_RATIO_LIMIT1: u32 = 430;
pub const MSR_TURBO_RATIO_LIMIT2: u32 = 431;
pub const MSR_LBR_SELECT: u32 = 456;
pub const MSR_LBR_TOS: u32 = 457;
pub const MSR_LBR_NHM_FROM: u32 = 1664;
pub const MSR_LBR_NHM_TO: u32 = 1728;
pub const MSR_LBR_CORE_FROM: u32 = 64;
pub const MSR_LBR_CORE_TO: u32 = 96;
pub const MSR_LBR_INFO_0: u32 = 3520;
pub const LBR_INFO_CYCLES: u32 = 65535;
pub const MSR_IA32_PEBS_ENABLE: u32 = 1009;
pub const MSR_IA32_DS_AREA: u32 = 1536;
pub const MSR_IA32_PERF_CAPABILITIES: u32 = 837;
pub const MSR_PEBS_LD_LAT_THRESHOLD: u32 = 1014;
pub const MSR_IA32_RTIT_CTL: u32 = 1392;
pub const MSR_IA32_RTIT_STATUS: u32 = 1393;
pub const MSR_IA32_RTIT_ADDR0_A: u32 = 1408;
pub const MSR_IA32_RTIT_ADDR0_B: u32 = 1409;
pub const MSR_IA32_RTIT_ADDR1_A: u32 = 1410;
pub const MSR_IA32_RTIT_ADDR1_B: u32 = 1411;
pub const MSR_IA32_RTIT_ADDR2_A: u32 = 1412;
pub const MSR_IA32_RTIT_ADDR2_B: u32 = 1413;
pub const MSR_IA32_RTIT_ADDR3_A: u32 = 1414;
pub const MSR_IA32_RTIT_ADDR3_B: u32 = 1415;
pub const MSR_IA32_RTIT_CR3_MATCH: u32 = 1394;
pub const MSR_IA32_RTIT_OUTPUT_BASE: u32 = 1376;
pub const MSR_IA32_RTIT_OUTPUT_MASK: u32 = 1377;
pub const MSR_MTRRfix64K_00000: u32 = 592;
pub const MSR_MTRRfix16K_80000: u32 = 600;
pub const MSR_MTRRfix16K_A0000: u32 = 601;
pub const MSR_MTRRfix4K_C0000: u32 = 616;
pub const MSR_MTRRfix4K_C8000: u32 = 617;
pub const MSR_MTRRfix4K_D0000: u32 = 618;
pub const MSR_MTRRfix4K_D8000: u32 = 619;
pub const MSR_MTRRfix4K_E0000: u32 = 620;
pub const MSR_MTRRfix4K_E8000: u32 = 621;
pub const MSR_MTRRfix4K_F0000: u32 = 622;
pub const MSR_MTRRfix4K_F8000: u32 = 623;
pub const MSR_MTRRdefType: u32 = 767;
pub const MSR_IA32_CR_PAT: u32 = 631;
pub const MSR_IA32_DEBUGCTLMSR: u32 = 473;
pub const MSR_IA32_LASTBRANCHFROMIP: u32 = 475;
pub const MSR_IA32_LASTBRANCHTOIP: u32 = 476;
pub const MSR_IA32_LASTINTFROMIP: u32 = 477;
pub const MSR_IA32_LASTINTTOIP: u32 = 478;
pub const DEBUGCTLMSR_LBR: u32 = 1;
pub const DEBUGCTLMSR_BTF_SHIFT: u32 = 1;
pub const DEBUGCTLMSR_BTF: u32 = 2;
pub const DEBUGCTLMSR_TR: u32 = 64;
pub const DEBUGCTLMSR_BTS: u32 = 128;
pub const DEBUGCTLMSR_BTINT: u32 = 256;
pub const DEBUGCTLMSR_BTS_OFF_OS: u32 = 512;
pub const DEBUGCTLMSR_BTS_OFF_USR: u32 = 1024;
pub const DEBUGCTLMSR_FREEZE_LBRS_ON_PMI: u32 = 2048;
pub const MSR_PEBS_FRONTEND: u32 = 1015;
pub const MSR_IA32_POWER_CTL: u32 = 508;
pub const MSR_IA32_MC0_CTL: u32 = 1024;
pub const MSR_IA32_MC0_STATUS: u32 = 1025;
pub const MSR_IA32_MC0_ADDR: u32 = 1026;
pub const MSR_IA32_MC0_MISC: u32 = 1027;
pub const MSR_PKG_C3_RESIDENCY: u32 = 1016;
pub const MSR_PKG_C6_RESIDENCY: u32 = 1017;
pub const MSR_PKG_C7_RESIDENCY: u32 = 1018;
pub const MSR_CORE_C3_RESIDENCY: u32 = 1020;
pub const MSR_CORE_C6_RESIDENCY: u32 = 1021;
pub const MSR_CORE_C7_RESIDENCY: u32 = 1022;
pub const MSR_KNL_CORE_C6_RESIDENCY: u32 = 1023;
pub const MSR_PKG_C2_RESIDENCY: u32 = 1549;
pub const MSR_PKG_C8_RESIDENCY: u32 = 1584;
pub const MSR_PKG_C9_RESIDENCY: u32 = 1585;
pub const MSR_PKG_C10_RESIDENCY: u32 = 1586;
pub const MSR_PKGC3_IRTL: u32 = 1546;
pub const MSR_PKGC6_IRTL: u32 = 1547;
pub const MSR_PKGC7_IRTL: u32 = 1548;
pub const MSR_PKGC8_IRTL: u32 = 1587;
pub const MSR_PKGC9_IRTL: u32 = 1588;
pub const MSR_PKGC10_IRTL: u32 = 1589;
pub const MSR_RAPL_POWER_UNIT: u32 = 1542;
pub const MSR_PKG_POWER_LIMIT: u32 = 1552;
pub const MSR_PKG_ENERGY_STATUS: u32 = 1553;
pub const MSR_PKG_PERF_STATUS: u32 = 1555;
pub const MSR_PKG_POWER_INFO: u32 = 1556;
pub const MSR_DRAM_POWER_LIMIT: u32 = 1560;
pub const MSR_DRAM_ENERGY_STATUS: u32 = 1561;
pub const MSR_DRAM_PERF_STATUS: u32 = 1563;
pub const MSR_DRAM_POWER_INFO: u32 = 1564;
pub const MSR_PP0_POWER_LIMIT: u32 = 1592;
pub const MSR_PP0_ENERGY_STATUS: u32 = 1593;
pub const MSR_PP0_POLICY: u32 = 1594;
pub const MSR_PP0_PERF_STATUS: u32 = 1595;
pub const MSR_PP1_POWER_LIMIT: u32 = 1600;
pub const MSR_PP1_ENERGY_STATUS: u32 = 1601;
pub const MSR_PP1_POLICY: u32 = 1602;
pub const MSR_CONFIG_TDP_NOMINAL: u32 = 1608;
pub const MSR_CONFIG_TDP_LEVEL_1: u32 = 1609;
pub const MSR_CONFIG_TDP_LEVEL_2: u32 = 1610;
pub const MSR_CONFIG_TDP_CONTROL: u32 = 1611;
pub const MSR_TURBO_ACTIVATION_RATIO: u32 = 1612;
pub const MSR_PLATFORM_ENERGY_STATUS: u32 = 1613;
pub const MSR_PKG_WEIGHTED_CORE_C0_RES: u32 = 1624;
pub const MSR_PKG_ANY_CORE_C0_RES: u32 = 1625;
pub const MSR_PKG_ANY_GFXE_C0_RES: u32 = 1626;
pub const MSR_PKG_BOTH_CORE_GFXE_C0_RES: u32 = 1627;
pub const MSR_CORE_C1_RES: u32 = 1632;
pub const MSR_CC6_DEMOTION_POLICY_CONFIG: u32 = 1640;
pub const MSR_MC6_DEMOTION_POLICY_CONFIG: u32 = 1641;
pub const MSR_CORE_PERF_LIMIT_REASONS: u32 = 1680;
pub const MSR_GFX_PERF_LIMIT_REASONS: u32 = 1712;
pub const MSR_RING_PERF_LIMIT_REASONS: u32 = 1713;
pub const MSR_PPERF: u32 = 1614;
pub const MSR_PERF_LIMIT_REASONS: u32 = 1615;
pub const MSR_PM_ENABLE: u32 = 1904;
pub const MSR_HWP_CAPABILITIES: u32 = 1905;
pub const MSR_HWP_REQUEST_PKG: u32 = 1906;
pub const MSR_HWP_INTERRUPT: u32 = 1907;
pub const MSR_HWP_REQUEST: u32 = 1908;
pub const MSR_HWP_STATUS: u32 = 1911;
pub const HWP_BASE_BIT: u32 = 128;
pub const HWP_NOTIFICATIONS_BIT: u32 = 256;
pub const HWP_ACTIVITY_WINDOW_BIT: u32 = 512;
pub const HWP_ENERGY_PERF_PREFERENCE_BIT: u32 = 1024;
pub const HWP_PACKAGE_LEVEL_REQUEST_BIT: u32 = 2048;
pub const MSR_AMD64_MC0_MASK: u32 = 3221291076;
pub const MSR_IA32_MC0_CTL2: u32 = 640;
pub const MSR_P6_PERFCTR0: u32 = 193;
pub const MSR_P6_PERFCTR1: u32 = 194;
pub const MSR_P6_EVNTSEL0: u32 = 390;
pub const MSR_P6_EVNTSEL1: u32 = 391;
pub const MSR_KNC_PERFCTR0: u32 = 32;
pub const MSR_KNC_PERFCTR1: u32 = 33;
pub const MSR_KNC_EVNTSEL0: u32 = 40;
pub const MSR_KNC_EVNTSEL1: u32 = 41;
pub const MSR_IA32_PMC0: u32 = 1217;
pub const MSR_AMD64_PATCH_LEVEL: u32 = 139;
pub const MSR_AMD64_TSC_RATIO: u32 = 3221225732;
pub const MSR_AMD64_NB_CFG: u32 = 3221291039;
pub const MSR_AMD64_PATCH_LOADER: u32 = 3221291040;
pub const MSR_AMD64_OSVW_ID_LENGTH: u32 = 3221291328;
pub const MSR_AMD64_OSVW_STATUS: u32 = 3221291329;
pub const MSR_AMD64_LS_CFG: u32 = 3221295136;
pub const MSR_AMD64_DC_CFG: u32 = 3221295138;
pub const MSR_AMD64_BU_CFG2: u32 = 3221295146;
pub const MSR_AMD64_IBSFETCHCTL: u32 = 3221295152;
pub const MSR_AMD64_IBSFETCHLINAD: u32 = 3221295153;
pub const MSR_AMD64_IBSFETCHPHYSAD: u32 = 3221295154;
pub const MSR_AMD64_IBSFETCH_REG_COUNT: u32 = 3;
pub const MSR_AMD64_IBSFETCH_REG_MASK: u32 = 7;
pub const MSR_AMD64_IBSOPCTL: u32 = 3221295155;
pub const MSR_AMD64_IBSOPRIP: u32 = 3221295156;
pub const MSR_AMD64_IBSOPDATA: u32 = 3221295157;
pub const MSR_AMD64_IBSOPDATA2: u32 = 3221295158;
pub const MSR_AMD64_IBSOPDATA3: u32 = 3221295159;
pub const MSR_AMD64_IBSDCLINAD: u32 = 3221295160;
pub const MSR_AMD64_IBSDCPHYSAD: u32 = 3221295161;
pub const MSR_AMD64_IBSOP_REG_COUNT: u32 = 7;
pub const MSR_AMD64_IBSOP_REG_MASK: u32 = 127;
pub const MSR_AMD64_IBSCTL: u32 = 3221295162;
pub const MSR_AMD64_IBSBRTARGET: u32 = 3221295163;
pub const MSR_AMD64_IBSOPDATA4: u32 = 3221295165;
pub const MSR_AMD64_IBS_REG_COUNT_MAX: u32 = 8;
pub const MSR_AMD64_VIRT_SPEC_CTRL: u32 = 3221291295;
pub const MSR_F17H_IRPERF: u32 = 3221225705;
pub const MSR_F16H_L2I_PERF_CTL: u32 = 3221291568;
pub const MSR_F16H_L2I_PERF_CTR: u32 = 3221291569;
pub const MSR_F16H_DR1_ADDR_MASK: u32 = 3221295129;
pub const MSR_F16H_DR2_ADDR_MASK: u32 = 3221295130;
pub const MSR_F16H_DR3_ADDR_MASK: u32 = 3221295131;
pub const MSR_F16H_DR0_ADDR_MASK: u32 = 3221295143;
pub const MSR_F15H_PERF_CTL: u32 = 3221291520;
pub const MSR_F15H_PERF_CTR: u32 = 3221291521;
pub const MSR_F15H_NB_PERF_CTL: u32 = 3221291584;
pub const MSR_F15H_NB_PERF_CTR: u32 = 3221291585;
pub const MSR_F15H_PTSC: u32 = 3221291648;
pub const MSR_F15H_IC_CFG: u32 = 3221295137;
pub const MSR_FAM10H_MMIO_CONF_BASE: u32 = 3221291096;
pub const FAM10H_MMIO_CONF_ENABLE: u32 = 1;
pub const FAM10H_MMIO_CONF_BUSRANGE_MASK: u32 = 15;
pub const FAM10H_MMIO_CONF_BUSRANGE_SHIFT: u32 = 2;
pub const FAM10H_MMIO_CONF_BASE_MASK: u32 = 268435455;
pub const FAM10H_MMIO_CONF_BASE_SHIFT: u32 = 20;
pub const MSR_FAM10H_NODE_ID: u32 = 3221295116;
pub const MSR_F10H_DECFG: u32 = 3221295145;
pub const MSR_F10H_DECFG_LFENCE_SERIALIZE_BIT: u32 = 1;
pub const MSR_K8_TOP_MEM1: u32 = 3221291034;
pub const MSR_K8_TOP_MEM2: u32 = 3221291037;
pub const MSR_K8_SYSCFG: u32 = 3221291024;
pub const MSR_K8_INT_PENDING_MSG: u32 = 3221291093;
pub const K8_INTP_C1E_ACTIVE_MASK: u32 = 402653184;
pub const MSR_K8_TSEG_ADDR: u32 = 3221291282;
pub const MSR_K8_TSEG_MASK: u32 = 3221291283;
pub const K8_MTRRFIXRANGE_DRAM_ENABLE: u32 = 262144;
pub const K8_MTRRFIXRANGE_DRAM_MODIFY: u32 = 524288;
pub const K8_MTRR_RDMEM_WRMEM_MASK: u32 = 404232216;
pub const MSR_K7_EVNTSEL0: u32 = 3221291008;
pub const MSR_K7_PERFCTR0: u32 = 3221291012;
pub const MSR_K7_EVNTSEL1: u32 = 3221291009;
pub const MSR_K7_PERFCTR1: u32 = 3221291013;
pub const MSR_K7_EVNTSEL2: u32 = 3221291010;
pub const MSR_K7_PERFCTR2: u32 = 3221291014;
pub const MSR_K7_EVNTSEL3: u32 = 3221291011;
pub const MSR_K7_PERFCTR3: u32 = 3221291015;
pub const MSR_K7_CLK_CTL: u32 = 3221291035;
pub const MSR_K7_HWCR: u32 = 3221291029;
pub const MSR_K7_FID_VID_CTL: u32 = 3221291073;
pub const MSR_K7_FID_VID_STATUS: u32 = 3221291074;
pub const MSR_K6_WHCR: u32 = 3221225602;
pub const MSR_K6_UWCCR: u32 = 3221225605;
pub const MSR_K6_EPMR: u32 = 3221225606;
pub const MSR_K6_PSOR: u32 = 3221225607;
pub const MSR_K6_PFIR: u32 = 3221225608;
pub const MSR_IDT_FCR1: u32 = 263;
pub const MSR_IDT_FCR2: u32 = 264;
pub const MSR_IDT_FCR3: u32 = 265;
pub const MSR_IDT_FCR4: u32 = 266;
pub const MSR_IDT_MCR0: u32 = 272;
pub const MSR_IDT_MCR1: u32 = 273;
pub const MSR_IDT_MCR2: u32 = 274;
pub const MSR_IDT_MCR3: u32 = 275;
pub const MSR_IDT_MCR4: u32 = 276;
pub const MSR_IDT_MCR5: u32 = 277;
pub const MSR_IDT_MCR6: u32 = 278;
pub const MSR_IDT_MCR7: u32 = 279;
pub const MSR_IDT_MCR_CTRL: u32 = 288;
pub const MSR_VIA_FCR: u32 = 4359;
pub const MSR_VIA_LONGHAUL: u32 = 4362;
pub const MSR_VIA_RNG: u32 = 4363;
pub const MSR_VIA_BCR2: u32 = 4423;
pub const MSR_TMTA_LONGRUN_CTRL: u32 = 2156298256;
pub const MSR_TMTA_LONGRUN_FLAGS: u32 = 2156298257;
pub const MSR_TMTA_LRTI_READOUT: u32 = 2156298264;
pub const MSR_TMTA_LRTI_VOLT_MHZ: u32 = 2156298266;
pub const MSR_IA32_P5_MC_ADDR: u32 = 0;
pub const MSR_IA32_P5_MC_TYPE: u32 = 1;
pub const MSR_IA32_TSC: u32 = 16;
pub const MSR_IA32_PLATFORM_ID: u32 = 23;
pub const MSR_IA32_EBL_CR_POWERON: u32 = 42;
pub const MSR_EBC_FREQUENCY_ID: u32 = 44;
pub const MSR_SMI_COUNT: u32 = 52;
pub const MSR_IA32_FEATURE_CONTROL: u32 = 58;
pub const MSR_IA32_TSC_ADJUST: u32 = 59;
pub const MSR_IA32_BNDCFGS: u32 = 3472;
pub const MSR_IA32_BNDCFGS_RSVD: u32 = 4092;
pub const MSR_IA32_XSS: u32 = 3488;
pub const FEATURE_CONTROL_LOCKED: u32 = 1;
pub const FEATURE_CONTROL_VMXON_ENABLED_INSIDE_SMX: u32 = 2;
pub const FEATURE_CONTROL_VMXON_ENABLED_OUTSIDE_SMX: u32 = 4;
pub const FEATURE_CONTROL_LMCE: u32 = 1048576;
pub const MSR_IA32_APICBASE: u32 = 27;
pub const MSR_IA32_APICBASE_BSP: u32 = 256;
pub const MSR_IA32_APICBASE_ENABLE: u32 = 2048;
pub const MSR_IA32_APICBASE_BASE: u32 = 4294963200;
pub const MSR_IA32_TSCDEADLINE: u32 = 1760;
pub const MSR_IA32_UCODE_WRITE: u32 = 121;
pub const MSR_IA32_UCODE_REV: u32 = 139;
pub const MSR_IA32_SMM_MONITOR_CTL: u32 = 155;
pub const MSR_IA32_SMBASE: u32 = 158;
pub const MSR_IA32_PERF_STATUS: u32 = 408;
pub const MSR_IA32_PERF_CTL: u32 = 409;
pub const INTEL_PERF_CTL_MASK: u32 = 65535;
pub const MSR_AMD_PSTATE_DEF_BASE: u32 = 3221291108;
pub const MSR_AMD_PERF_STATUS: u32 = 3221291107;
pub const MSR_AMD_PERF_CTL: u32 = 3221291106;
pub const MSR_IA32_MPERF: u32 = 231;
pub const MSR_IA32_APERF: u32 = 232;
pub const MSR_IA32_THERM_CONTROL: u32 = 410;
pub const MSR_IA32_THERM_INTERRUPT: u32 = 411;
pub const THERM_INT_HIGH_ENABLE: u32 = 1;
pub const THERM_INT_LOW_ENABLE: u32 = 2;
pub const THERM_INT_PLN_ENABLE: u32 = 16777216;
pub const MSR_IA32_THERM_STATUS: u32 = 412;
pub const THERM_STATUS_PROCHOT: u32 = 1;
pub const THERM_STATUS_POWER_LIMIT: u32 = 1024;
pub const MSR_THERM2_CTL: u32 = 413;
pub const MSR_THERM2_CTL_TM_SELECT: u32 = 65536;
pub const MSR_IA32_MISC_ENABLE: u32 = 416;
pub const MSR_IA32_TEMPERATURE_TARGET: u32 = 418;
pub const MSR_MISC_PWR_MGMT: u32 = 426;
pub const MSR_IA32_ENERGY_PERF_BIAS: u32 = 432;
pub const ENERGY_PERF_BIAS_PERFORMANCE: u32 = 0;
pub const ENERGY_PERF_BIAS_NORMAL: u32 = 6;
pub const ENERGY_PERF_BIAS_POWERSAVE: u32 = 15;
pub const MSR_IA32_PACKAGE_THERM_STATUS: u32 = 433;
pub const PACKAGE_THERM_STATUS_PROCHOT: u32 = 1;
pub const PACKAGE_THERM_STATUS_POWER_LIMIT: u32 = 1024;
pub const MSR_IA32_PACKAGE_THERM_INTERRUPT: u32 = 434;
pub const PACKAGE_THERM_INT_HIGH_ENABLE: u32 = 1;
pub const PACKAGE_THERM_INT_LOW_ENABLE: u32 = 2;
pub const PACKAGE_THERM_INT_PLN_ENABLE: u32 = 16777216;
pub const THERM_INT_THRESHOLD0_ENABLE: u32 = 32768;
pub const THERM_SHIFT_THRESHOLD0: u32 = 8;
pub const THERM_MASK_THRESHOLD0: u32 = 32512;
pub const THERM_INT_THRESHOLD1_ENABLE: u32 = 8388608;
pub const THERM_SHIFT_THRESHOLD1: u32 = 16;
pub const THERM_MASK_THRESHOLD1: u32 = 8323072;
pub const THERM_STATUS_THRESHOLD0: u32 = 64;
pub const THERM_LOG_THRESHOLD0: u32 = 128;
pub const THERM_STATUS_THRESHOLD1: u32 = 256;
pub const THERM_LOG_THRESHOLD1: u32 = 512;
pub const MSR_IA32_MISC_ENABLE_FAST_STRING_BIT: u32 = 0;
pub const MSR_IA32_MISC_ENABLE_FAST_STRING: u32 = 1;
pub const MSR_IA32_MISC_ENABLE_TCC_BIT: u32 = 1;
pub const MSR_IA32_MISC_ENABLE_TCC: u32 = 2;
pub const MSR_IA32_MISC_ENABLE_EMON_BIT: u32 = 7;
pub const MSR_IA32_MISC_ENABLE_EMON: u32 = 128;
pub const MSR_IA32_MISC_ENABLE_BTS_UNAVAIL_BIT: u32 = 11;
pub const MSR_IA32_MISC_ENABLE_BTS_UNAVAIL: u32 = 2048;
pub const MSR_IA32_MISC_ENABLE_PEBS_UNAVAIL_BIT: u32 = 12;
pub const MSR_IA32_MISC_ENABLE_PEBS_UNAVAIL: u32 = 4096;
pub const MSR_IA32_MISC_ENABLE_ENHANCED_SPEEDSTEP_BIT: u32 = 16;
pub const MSR_IA32_MISC_ENABLE_ENHANCED_SPEEDSTEP: u32 = 65536;
pub const MSR_IA32_MISC_ENABLE_MWAIT_BIT: u32 = 18;
pub const MSR_IA32_MISC_ENABLE_MWAIT: u32 = 262144;
pub const MSR_IA32_MISC_ENABLE_LIMIT_CPUID_BIT: u32 = 22;
pub const MSR_IA32_MISC_ENABLE_LIMIT_CPUID: u32 = 4194304;
pub const MSR_IA32_MISC_ENABLE_XTPR_DISABLE_BIT: u32 = 23;
pub const MSR_IA32_MISC_ENABLE_XTPR_DISABLE: u32 = 8388608;
pub const MSR_IA32_MISC_ENABLE_XD_DISABLE_BIT: u32 = 34;
pub const MSR_IA32_MISC_ENABLE_XD_DISABLE: u64 = 17179869184;
pub const MSR_IA32_MISC_ENABLE_X87_COMPAT_BIT: u32 = 2;
pub const MSR_IA32_MISC_ENABLE_X87_COMPAT: u32 = 4;
pub const MSR_IA32_MISC_ENABLE_TM1_BIT: u32 = 3;
pub const MSR_IA32_MISC_ENABLE_TM1: u32 = 8;
pub const MSR_IA32_MISC_ENABLE_SPLIT_LOCK_DISABLE_BIT: u32 = 4;
pub const MSR_IA32_MISC_ENABLE_SPLIT_LOCK_DISABLE: u32 = 16;
pub const MSR_IA32_MISC_ENABLE_L3CACHE_DISABLE_BIT: u32 = 6;
pub const MSR_IA32_MISC_ENABLE_L3CACHE_DISABLE: u32 = 64;
pub const MSR_IA32_MISC_ENABLE_SUPPRESS_LOCK_BIT: u32 = 8;
pub const MSR_IA32_MISC_ENABLE_SUPPRESS_LOCK: u32 = 256;
pub const MSR_IA32_MISC_ENABLE_PREFETCH_DISABLE_BIT: u32 = 9;
pub const MSR_IA32_MISC_ENABLE_PREFETCH_DISABLE: u32 = 512;
pub const MSR_IA32_MISC_ENABLE_FERR_BIT: u32 = 10;
pub const MSR_IA32_MISC_ENABLE_FERR: u32 = 1024;
pub const MSR_IA32_MISC_ENABLE_FERR_MULTIPLEX_BIT: u32 = 10;
pub const MSR_IA32_MISC_ENABLE_FERR_MULTIPLEX: u32 = 1024;
pub const MSR_IA32_MISC_ENABLE_TM2_BIT: u32 = 13;
pub const MSR_IA32_MISC_ENABLE_TM2: u32 = 8192;
pub const MSR_IA32_MISC_ENABLE_ADJ_PREF_DISABLE_BIT: u32 = 19;
pub const MSR_IA32_MISC_ENABLE_ADJ_PREF_DISABLE: u32 = 524288;
pub const MSR_IA32_MISC_ENABLE_SPEEDSTEP_LOCK_BIT: u32 = 20;
pub const MSR_IA32_MISC_ENABLE_SPEEDSTEP_LOCK: u32 = 1048576;
pub const MSR_IA32_MISC_ENABLE_L1D_CONTEXT_BIT: u32 = 24;
pub const MSR_IA32_MISC_ENABLE_L1D_CONTEXT: u32 = 16777216;
pub const MSR_IA32_MISC_ENABLE_DCU_PREF_DISABLE_BIT: u32 = 37;
pub const MSR_IA32_MISC_ENABLE_DCU_PREF_DISABLE: u64 = 137438953472;
pub const MSR_IA32_MISC_ENABLE_TURBO_DISABLE_BIT: u32 = 38;
pub const MSR_IA32_MISC_ENABLE_TURBO_DISABLE: u64 = 274877906944;
pub const MSR_IA32_MISC_ENABLE_IP_PREF_DISABLE_BIT: u32 = 39;
pub const MSR_IA32_MISC_ENABLE_IP_PREF_DISABLE: u64 = 549755813888;
pub const MSR_IA32_TSC_DEADLINE: u32 = 1760;
pub const MSR_TSX_FORCE_ABORT: u32 = 271;
pub const MSR_TFA_RTM_FORCE_ABORT_BIT: u32 = 0;
pub const MSR_IA32_MCG_EAX: u32 = 384;
pub const MSR_IA32_MCG_EBX: u32 = 385;
pub const MSR_IA32_MCG_ECX: u32 = 386;
pub const MSR_IA32_MCG_EDX: u32 = 387;
pub const MSR_IA32_MCG_ESI: u32 = 388;
pub const MSR_IA32_MCG_EDI: u32 = 389;
pub const MSR_IA32_MCG_EBP: u32 = 390;
pub const MSR_IA32_MCG_ESP: u32 = 391;
pub const MSR_IA32_MCG_EFLAGS: u32 = 392;
pub const MSR_IA32_MCG_EIP: u32 = 393;
pub const MSR_IA32_MCG_RESERVED: u32 = 394;
pub const MSR_P4_BPU_PERFCTR0: u32 = 768;
pub const MSR_P4_BPU_PERFCTR1: u32 = 769;
pub const MSR_P4_BPU_PERFCTR2: u32 = 770;
pub const MSR_P4_BPU_PERFCTR3: u32 = 771;
pub const MSR_P4_MS_PERFCTR0: u32 = 772;
pub const MSR_P4_MS_PERFCTR1: u32 = 773;
pub const MSR_P4_MS_PERFCTR2: u32 = 774;
pub const MSR_P4_MS_PERFCTR3: u32 = 775;
pub const MSR_P4_FLAME_PERFCTR0: u32 = 776;
pub const MSR_P4_FLAME_PERFCTR1: u32 = 777;
pub const MSR_P4_FLAME_PERFCTR2: u32 = 778;
pub const MSR_P4_FLAME_PERFCTR3: u32 = 779;
pub const MSR_P4_IQ_PERFCTR0: u32 = 780;
pub const MSR_P4_IQ_PERFCTR1: u32 = 781;
pub const MSR_P4_IQ_PERFCTR2: u32 = 782;
pub const MSR_P4_IQ_PERFCTR3: u32 = 783;
pub const MSR_P4_IQ_PERFCTR4: u32 = 784;
pub const MSR_P4_IQ_PERFCTR5: u32 = 785;
pub const MSR_P4_BPU_CCCR0: u32 = 864;
pub const MSR_P4_BPU_CCCR1: u32 = 865;
pub const MSR_P4_BPU_CCCR2: u32 = 866;
pub const MSR_P4_BPU_CCCR3: u32 = 867;
pub const MSR_P4_MS_CCCR0: u32 = 868;
pub const MSR_P4_MS_CCCR1: u32 = 869;
pub const MSR_P4_MS_CCCR2: u32 = 870;
pub const MSR_P4_MS_CCCR3: u32 = 871;
pub const MSR_P4_FLAME_CCCR0: u32 = 872;
pub const MSR_P4_FLAME_CCCR1: u32 = 873;
pub const MSR_P4_FLAME_CCCR2: u32 = 874;
pub const MSR_P4_FLAME_CCCR3: u32 = 875;
pub const MSR_P4_IQ_CCCR0: u32 = 876;
pub const MSR_P4_IQ_CCCR1: u32 = 877;
pub const MSR_P4_IQ_CCCR2: u32 = 878;
pub const MSR_P4_IQ_CCCR3: u32 = 879;
pub const MSR_P4_IQ_CCCR4: u32 = 880;
pub const MSR_P4_IQ_CCCR5: u32 = 881;
pub const MSR_P4_ALF_ESCR0: u32 = 970;
pub const MSR_P4_ALF_ESCR1: u32 = 971;
pub const MSR_P4_BPU_ESCR0: u32 = 946;
pub const MSR_P4_BPU_ESCR1: u32 = 947;
pub const MSR_P4_BSU_ESCR0: u32 = 928;
pub const MSR_P4_BSU_ESCR1: u32 = 929;
pub const MSR_P4_CRU_ESCR0: u32 = 952;
pub const MSR_P4_CRU_ESCR1: u32 = 953;
pub const MSR_P4_CRU_ESCR2: u32 = 972;
pub const MSR_P4_CRU_ESCR3: u32 = 973;
pub const MSR_P4_CRU_ESCR4: u32 = 992;
pub const MSR_P4_CRU_ESCR5: u32 = 993;
pub const MSR_P4_DAC_ESCR0: u32 = 936;
pub const MSR_P4_DAC_ESCR1: u32 = 937;
pub const MSR_P4_FIRM_ESCR0: u32 = 932;
pub const MSR_P4_FIRM_ESCR1: u32 = 933;
pub const MSR_P4_FLAME_ESCR0: u32 = 934;
pub const MSR_P4_FLAME_ESCR1: u32 = 935;
pub const MSR_P4_FSB_ESCR0: u32 = 930;
pub const MSR_P4_FSB_ESCR1: u32 = 931;
pub const MSR_P4_IQ_ESCR0: u32 = 954;
pub const MSR_P4_IQ_ESCR1: u32 = 955;
pub const MSR_P4_IS_ESCR0: u32 = 948;
pub const MSR_P4_IS_ESCR1: u32 = 949;
pub const MSR_P4_ITLB_ESCR0: u32 = 950;
pub const MSR_P4_ITLB_ESCR1: u32 = 951;
pub const MSR_P4_IX_ESCR0: u32 = 968;
pub const MSR_P4_IX_ESCR1: u32 = 969;
pub const MSR_P4_MOB_ESCR0: u32 = 938;
pub const MSR_P4_MOB_ESCR1: u32 = 939;
pub const MSR_P4_MS_ESCR0: u32 = 960;
pub const MSR_P4_MS_ESCR1: u32 = 961;
pub const MSR_P4_PMH_ESCR0: u32 = 940;
pub const MSR_P4_PMH_ESCR1: u32 = 941;
pub const MSR_P4_RAT_ESCR0: u32 = 956;
pub const MSR_P4_RAT_ESCR1: u32 = 957;
pub const MSR_P4_SAAT_ESCR0: u32 = 942;
pub const MSR_P4_SAAT_ESCR1: u32 = 943;
pub const MSR_P4_SSU_ESCR0: u32 = 958;
pub const MSR_P4_SSU_ESCR1: u32 = 959;
pub const MSR_P4_TBPU_ESCR0: u32 = 962;
pub const MSR_P4_TBPU_ESCR1: u32 = 963;
pub const MSR_P4_TC_ESCR0: u32 = 964;
pub const MSR_P4_TC_ESCR1: u32 = 965;
pub const MSR_P4_U2L_ESCR0: u32 = 944;
pub const MSR_P4_U2L_ESCR1: u32 = 945;
pub const MSR_P4_PEBS_MATRIX_VERT: u32 = 1010;
pub const MSR_CORE_PERF_FIXED_CTR0: u32 = 777;
pub const MSR_CORE_PERF_FIXED_CTR1: u32 = 778;
pub const MSR_CORE_PERF_FIXED_CTR2: u32 = 779;
pub const MSR_CORE_PERF_FIXED_CTR_CTRL: u32 = 909;
pub const MSR_CORE_PERF_GLOBAL_STATUS: u32 = 910;
pub const MSR_CORE_PERF_GLOBAL_CTRL: u32 = 911;
pub const MSR_CORE_PERF_GLOBAL_OVF_CTRL: u32 = 912;
pub const MSR_GEODE_BUSCONT_CONF0: u32 = 6400;
pub const MSR_IA32_VMX_BASIC: u32 = 1152;
pub const MSR_IA32_VMX_PINBASED_CTLS: u32 = 1153;
pub const MSR_IA32_VMX_PROCBASED_CTLS: u32 = 1154;
pub const MSR_IA32_VMX_EXIT_CTLS: u32 = 1155;
pub const MSR_IA32_VMX_ENTRY_CTLS: u32 = 1156;
pub const MSR_IA32_VMX_MISC: u32 = 1157;
pub const MSR_IA32_VMX_CR0_FIXED0: u32 = 1158;
pub const MSR_IA32_VMX_CR0_FIXED1: u32 = 1159;
pub const MSR_IA32_VMX_CR4_FIXED0: u32 = 1160;
pub const MSR_IA32_VMX_CR4_FIXED1: u32 = 1161;
pub const MSR_IA32_VMX_VMCS_ENUM: u32 = 1162;
pub const MSR_IA32_VMX_PROCBASED_CTLS2: u32 = 1163;
pub const MSR_IA32_VMX_EPT_VPID_CAP: u32 = 1164;
pub const MSR_IA32_VMX_TRUE_PINBASED_CTLS: u32 = 1165;
pub const MSR_IA32_VMX_TRUE_PROCBASED_CTLS: u32 = 1166;
pub const MSR_IA32_VMX_TRUE_EXIT_CTLS: u32 = 1167;
pub const MSR_IA32_VMX_TRUE_ENTRY_CTLS: u32 = 1168;
pub const MSR_IA32_VMX_VMFUNC: u32 = 1169;
pub const VMX_BASIC_VMCS_SIZE_SHIFT: u32 = 32;
pub const VMX_BASIC_TRUE_CTLS: u64 = 36028797018963968;
pub const VMX_BASIC_64: u64 = 281474976710656;
pub const VMX_BASIC_MEM_TYPE_SHIFT: u32 = 50;
pub const VMX_BASIC_MEM_TYPE_MASK: u64 = 16888498602639360;
pub const VMX_BASIC_MEM_TYPE_WB: u32 = 6;
pub const VMX_BASIC_INOUT: u64 = 18014398509481984;
pub const MSR_IA32_VMX_MISC_VMWRITE_SHADOW_RO_FIELDS: u32 = 536870912;
pub const MSR_IA32_VMX_MISC_PREEMPTION_TIMER_SCALE: u32 = 31;
pub const MSR_VM_CR: u32 = 3221291284;
pub const MSR_VM_IGNNE: u32 = 3221291285;
pub const MSR_VM_HSAVE_PA: u32 = 3221291287;
