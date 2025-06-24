#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    syscfg_cfgr1: SyscfgCfgr1,
    syscfg_rcr: SyscfgRcr,
    syscfg_exticr1: SyscfgExticr1,
    syscfg_exticr2: SyscfgExticr2,
    syscfg_exticr3: SyscfgExticr3,
    syscfg_exticr4: SyscfgExticr4,
    syscfg_cfgr2: SyscfgCfgr2,
    comp1_csr: Comp1Csr,
    comp2_csr: Comp2Csr,
    comp3_csr: Comp3Csr,
    comp4_csr: Comp4Csr,
    comp5_csr: Comp5Csr,
    comp6_csr: Comp6Csr,
    comp7_csr: Comp7Csr,
    opamp1_csr: Opamp1Csr,
    opamp2_csr: Opamp2Csr,
    opamp3_csr: Opamp3Csr,
    opamp4_csr: Opamp4Csr,
    syscfg_cfgr4: SyscfgCfgr4,
    _reserved19: [u8; 0x04],
    syscfg_cfgr3: SyscfgCfgr3,
}
impl RegisterBlock {
    #[doc = "0x00 - configuration register 1"]
    #[inline(always)]
    pub const fn syscfg_cfgr1(&self) -> &SyscfgCfgr1 {
        &self.syscfg_cfgr1
    }
    #[doc = "0x04 - CCM SRAM protection register"]
    #[inline(always)]
    pub const fn syscfg_rcr(&self) -> &SyscfgRcr {
        &self.syscfg_rcr
    }
    #[doc = "0x08 - external interrupt configuration register 1"]
    #[inline(always)]
    pub const fn syscfg_exticr1(&self) -> &SyscfgExticr1 {
        &self.syscfg_exticr1
    }
    #[doc = "0x0c - external interrupt configuration register 2"]
    #[inline(always)]
    pub const fn syscfg_exticr2(&self) -> &SyscfgExticr2 {
        &self.syscfg_exticr2
    }
    #[doc = "0x10 - external interrupt configuration register 3"]
    #[inline(always)]
    pub const fn syscfg_exticr3(&self) -> &SyscfgExticr3 {
        &self.syscfg_exticr3
    }
    #[doc = "0x14 - external interrupt configuration register 4"]
    #[inline(always)]
    pub const fn syscfg_exticr4(&self) -> &SyscfgExticr4 {
        &self.syscfg_exticr4
    }
    #[doc = "0x18 - configuration register 2"]
    #[inline(always)]
    pub const fn syscfg_cfgr2(&self) -> &SyscfgCfgr2 {
        &self.syscfg_cfgr2
    }
    #[doc = "0x1c - control and status register"]
    #[inline(always)]
    pub const fn comp1_csr(&self) -> &Comp1Csr {
        &self.comp1_csr
    }
    #[doc = "0x20 - control and status register"]
    #[inline(always)]
    pub const fn comp2_csr(&self) -> &Comp2Csr {
        &self.comp2_csr
    }
    #[doc = "0x24 - control and status register"]
    #[inline(always)]
    pub const fn comp3_csr(&self) -> &Comp3Csr {
        &self.comp3_csr
    }
    #[doc = "0x28 - control and status register"]
    #[inline(always)]
    pub const fn comp4_csr(&self) -> &Comp4Csr {
        &self.comp4_csr
    }
    #[doc = "0x2c - control and status register"]
    #[inline(always)]
    pub const fn comp5_csr(&self) -> &Comp5Csr {
        &self.comp5_csr
    }
    #[doc = "0x30 - control and status register"]
    #[inline(always)]
    pub const fn comp6_csr(&self) -> &Comp6Csr {
        &self.comp6_csr
    }
    #[doc = "0x34 - control and status register"]
    #[inline(always)]
    pub const fn comp7_csr(&self) -> &Comp7Csr {
        &self.comp7_csr
    }
    #[doc = "0x38 - control register"]
    #[inline(always)]
    pub const fn opamp1_csr(&self) -> &Opamp1Csr {
        &self.opamp1_csr
    }
    #[doc = "0x3c - control register"]
    #[inline(always)]
    pub const fn opamp2_csr(&self) -> &Opamp2Csr {
        &self.opamp2_csr
    }
    #[doc = "0x40 - control register"]
    #[inline(always)]
    pub const fn opamp3_csr(&self) -> &Opamp3Csr {
        &self.opamp3_csr
    }
    #[doc = "0x44 - control register"]
    #[inline(always)]
    pub const fn opamp4_csr(&self) -> &Opamp4Csr {
        &self.opamp4_csr
    }
    #[doc = "0x48 - SYSCFG configuration register 4"]
    #[inline(always)]
    pub const fn syscfg_cfgr4(&self) -> &SyscfgCfgr4 {
        &self.syscfg_cfgr4
    }
    #[doc = "0x50 - SYSCFG configuration register 3"]
    #[inline(always)]
    pub const fn syscfg_cfgr3(&self) -> &SyscfgCfgr3 {
        &self.syscfg_cfgr3
    }
}
#[doc = "SYSCFG_CFGR1 (rw) register accessor: configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`syscfg_cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg_cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_cfgr1`] module"]
#[doc(alias = "SYSCFG_CFGR1")]
pub type SyscfgCfgr1 = crate::Reg<syscfg_cfgr1::SyscfgCfgr1Spec>;
#[doc = "configuration register 1"]
pub mod syscfg_cfgr1;
#[doc = "SYSCFG_EXTICR1 (rw) register accessor: external interrupt configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`syscfg_exticr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg_exticr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_exticr1`] module"]
#[doc(alias = "SYSCFG_EXTICR1")]
pub type SyscfgExticr1 = crate::Reg<syscfg_exticr1::SyscfgExticr1Spec>;
#[doc = "external interrupt configuration register 1"]
pub mod syscfg_exticr1;
#[doc = "SYSCFG_EXTICR2 (rw) register accessor: external interrupt configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`syscfg_exticr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg_exticr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_exticr2`] module"]
#[doc(alias = "SYSCFG_EXTICR2")]
pub type SyscfgExticr2 = crate::Reg<syscfg_exticr2::SyscfgExticr2Spec>;
#[doc = "external interrupt configuration register 2"]
pub mod syscfg_exticr2;
#[doc = "SYSCFG_EXTICR3 (rw) register accessor: external interrupt configuration register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`syscfg_exticr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg_exticr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_exticr3`] module"]
#[doc(alias = "SYSCFG_EXTICR3")]
pub type SyscfgExticr3 = crate::Reg<syscfg_exticr3::SyscfgExticr3Spec>;
#[doc = "external interrupt configuration register 3"]
pub mod syscfg_exticr3;
#[doc = "SYSCFG_EXTICR4 (rw) register accessor: external interrupt configuration register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`syscfg_exticr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg_exticr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_exticr4`] module"]
#[doc(alias = "SYSCFG_EXTICR4")]
pub type SyscfgExticr4 = crate::Reg<syscfg_exticr4::SyscfgExticr4Spec>;
#[doc = "external interrupt configuration register 4"]
pub mod syscfg_exticr4;
#[doc = "SYSCFG_CFGR2 (rw) register accessor: configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`syscfg_cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg_cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_cfgr2`] module"]
#[doc(alias = "SYSCFG_CFGR2")]
pub type SyscfgCfgr2 = crate::Reg<syscfg_cfgr2::SyscfgCfgr2Spec>;
#[doc = "configuration register 2"]
pub mod syscfg_cfgr2;
#[doc = "SYSCFG_RCR (rw) register accessor: CCM SRAM protection register\n\nYou can [`read`](crate::Reg::read) this register and get [`syscfg_rcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg_rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_rcr`] module"]
#[doc(alias = "SYSCFG_RCR")]
pub type SyscfgRcr = crate::Reg<syscfg_rcr::SyscfgRcrSpec>;
#[doc = "CCM SRAM protection register"]
pub mod syscfg_rcr;
#[doc = "COMP1_CSR (rw) register accessor: control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp1_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp1_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp1_csr`] module"]
#[doc(alias = "COMP1_CSR")]
pub type Comp1Csr = crate::Reg<comp1_csr::Comp1CsrSpec>;
#[doc = "control and status register"]
pub mod comp1_csr;
#[doc = "COMP2_CSR (rw) register accessor: control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp2_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp2_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp2_csr`] module"]
#[doc(alias = "COMP2_CSR")]
pub type Comp2Csr = crate::Reg<comp2_csr::Comp2CsrSpec>;
#[doc = "control and status register"]
pub mod comp2_csr;
#[doc = "COMP3_CSR (rw) register accessor: control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp3_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp3_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp3_csr`] module"]
#[doc(alias = "COMP3_CSR")]
pub type Comp3Csr = crate::Reg<comp3_csr::Comp3CsrSpec>;
#[doc = "control and status register"]
pub mod comp3_csr;
#[doc = "COMP4_CSR (rw) register accessor: control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp4_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp4_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp4_csr`] module"]
#[doc(alias = "COMP4_CSR")]
pub type Comp4Csr = crate::Reg<comp4_csr::Comp4CsrSpec>;
#[doc = "control and status register"]
pub mod comp4_csr;
#[doc = "COMP5_CSR (rw) register accessor: control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp5_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp5_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp5_csr`] module"]
#[doc(alias = "COMP5_CSR")]
pub type Comp5Csr = crate::Reg<comp5_csr::Comp5CsrSpec>;
#[doc = "control and status register"]
pub mod comp5_csr;
#[doc = "COMP6_CSR (rw) register accessor: control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp6_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp6_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp6_csr`] module"]
#[doc(alias = "COMP6_CSR")]
pub type Comp6Csr = crate::Reg<comp6_csr::Comp6CsrSpec>;
#[doc = "control and status register"]
pub mod comp6_csr;
#[doc = "COMP7_CSR (rw) register accessor: control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp7_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp7_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp7_csr`] module"]
#[doc(alias = "COMP7_CSR")]
pub type Comp7Csr = crate::Reg<comp7_csr::Comp7CsrSpec>;
#[doc = "control and status register"]
pub mod comp7_csr;
#[doc = "OPAMP1_CSR (rw) register accessor: control register\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp1_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp1_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp1_csr`] module"]
#[doc(alias = "OPAMP1_CSR")]
pub type Opamp1Csr = crate::Reg<opamp1_csr::Opamp1CsrSpec>;
#[doc = "control register"]
pub mod opamp1_csr;
#[doc = "OPAMP2_CSR (rw) register accessor: control register\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp2_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp2_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp2_csr`] module"]
#[doc(alias = "OPAMP2_CSR")]
pub type Opamp2Csr = crate::Reg<opamp2_csr::Opamp2CsrSpec>;
#[doc = "control register"]
pub mod opamp2_csr;
#[doc = "OPAMP3_CSR (rw) register accessor: control register\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp3_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp3_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp3_csr`] module"]
#[doc(alias = "OPAMP3_CSR")]
pub type Opamp3Csr = crate::Reg<opamp3_csr::Opamp3CsrSpec>;
#[doc = "control register"]
pub mod opamp3_csr;
#[doc = "OPAMP4_CSR (rw) register accessor: control register\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp4_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp4_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp4_csr`] module"]
#[doc(alias = "OPAMP4_CSR")]
pub type Opamp4Csr = crate::Reg<opamp4_csr::Opamp4CsrSpec>;
#[doc = "control register"]
pub mod opamp4_csr;
#[doc = "SYSCFG_CFGR3 (rw) register accessor: SYSCFG configuration register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`syscfg_cfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg_cfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_cfgr3`] module"]
#[doc(alias = "SYSCFG_CFGR3")]
pub type SyscfgCfgr3 = crate::Reg<syscfg_cfgr3::SyscfgCfgr3Spec>;
#[doc = "SYSCFG configuration register 3"]
pub mod syscfg_cfgr3;
#[doc = "SYSCFG_CFGR4 (rw) register accessor: SYSCFG configuration register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`syscfg_cfgr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg_cfgr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_cfgr4`] module"]
#[doc(alias = "SYSCFG_CFGR4")]
pub type SyscfgCfgr4 = crate::Reg<syscfg_cfgr4::SyscfgCfgr4Spec>;
#[doc = "SYSCFG configuration register 4"]
pub mod syscfg_cfgr4;
