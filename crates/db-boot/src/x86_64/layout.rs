// Copyright 2021-2022 Alibaba Cloud. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
//
// Portions Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the THIRD-PARTY file.

/// Magic addresses externally used to lay out x86_64 VMs.

/// Address of Global Descriptor Table (GDT)
pub const BOOT_GDT_ADDRESS: u64 = 0x500;
/// Number of initial GDT entries.
pub const BOOT_GDT_MAX: usize = 4;

/// Address of Interrupt Descriptor Table (IDT)
pub const BOOT_IDT_ADDRESS: u64 = 0x520;

/// The 'zero page', a.k.a linux kernel bootparams.
pub const ZERO_PAGE_START: u64 = 0x7000;

/// Initial stack for the boot CPU.
pub const BOOT_STACK_POINTER: u64 = 0x8ff0;

/// Address of page table level 4 page
pub const PML4_START: u64 = 0x9000;
/// Address of page table level 3 page
pub const PDPTE_START: u64 = 0xa000;
/// Address of page table level 2 page
pub const PDE_START: u64 = 0xb000;

/// Kernel command line start address.
pub const CMDLINE_START: u64 = 0x20000;
/// Kernel command line start address maximum size.
pub const CMDLINE_MAX_SIZE: usize = 0x10000;

/// Kernel dragonball boot parameters start address.
pub const DB_BOOT_PARAM_START: u64 = 0x30000;
/// Kernel dragonball boot parameters length maximum size.
pub const DB_BOOT_PARAM_MAX_SIZE: u32 = 0x10000;

/// Start of the high memory.
pub const HIMEM_START: u64 = 0x0010_0000; //1 MB.

// Typically, on x86 systems 16 IRQs are used (0-15).
/// First usable IRQ ID for virtio device interrupts on x86_64.
pub const IRQ_BASE: u32 = 5;
/// Last usable IRQ ID for virtio device interrupts on x86_64.
pub const IRQ_MAX: u32 = 15;

/// Address for the TSS setup.
pub const KVM_TSS_ADDRESS: u64 = 0xfffb_d000;
