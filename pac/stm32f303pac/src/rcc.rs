#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    cfgr: Cfgr,
    cir: Cir,
    apb2rstr: Apb2rstr,
    apb1rstr: Apb1rstr,
    ahbenr: Ahbenr,
    apb2enr: Apb2enr,
    apb1enr: Apb1enr,
    bdcr: Bdcr,
    csr: Csr,
    ahbrstr: Ahbrstr,
    cfgr2: Cfgr2,
    cfgr3: Cfgr3,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - Clock configuration register (RCC_CFGR)"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &Cfgr {
        &self.cfgr
    }
    #[doc = "0x08 - Clock interrupt register (RCC_CIR)"]
    #[inline(always)]
    pub const fn cir(&self) -> &Cir {
        &self.cir
    }
    #[doc = "0x0c - APB2 peripheral reset register (RCC_APB2RSTR)"]
    #[inline(always)]
    pub const fn apb2rstr(&self) -> &Apb2rstr {
        &self.apb2rstr
    }
    #[doc = "0x10 - APB1 peripheral reset register (RCC_APB1RSTR)"]
    #[inline(always)]
    pub const fn apb1rstr(&self) -> &Apb1rstr {
        &self.apb1rstr
    }
    #[doc = "0x14 - AHB Peripheral Clock enable register (RCC_AHBENR)"]
    #[inline(always)]
    pub const fn ahbenr(&self) -> &Ahbenr {
        &self.ahbenr
    }
    #[doc = "0x18 - APB2 peripheral clock enable register (RCC_APB2ENR)"]
    #[inline(always)]
    pub const fn apb2enr(&self) -> &Apb2enr {
        &self.apb2enr
    }
    #[doc = "0x1c - APB1 peripheral clock enable register (RCC_APB1ENR)"]
    #[inline(always)]
    pub const fn apb1enr(&self) -> &Apb1enr {
        &self.apb1enr
    }
    #[doc = "0x20 - Backup domain control register (RCC_BDCR)"]
    #[inline(always)]
    pub const fn bdcr(&self) -> &Bdcr {
        &self.bdcr
    }
    #[doc = "0x24 - Control/status register (RCC_CSR)"]
    #[inline(always)]
    pub const fn csr(&self) -> &Csr {
        &self.csr
    }
    #[doc = "0x28 - AHB peripheral reset register"]
    #[inline(always)]
    pub const fn ahbrstr(&self) -> &Ahbrstr {
        &self.ahbrstr
    }
    #[doc = "0x2c - Clock configuration register 2"]
    #[inline(always)]
    pub const fn cfgr2(&self) -> &Cfgr2 {
        &self.cfgr2
    }
    #[doc = "0x30 - Clock configuration register 3"]
    #[inline(always)]
    pub const fn cfgr3(&self) -> &Cfgr3 {
        &self.cfgr3
    }
}
#[doc = "CR (rw) register accessor: Clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Clock control register"]
pub mod cr;
#[doc = "CFGR (rw) register accessor: Clock configuration register (RCC_CFGR)\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`] module"]
#[doc(alias = "CFGR")]
pub type Cfgr = crate::Reg<cfgr::CfgrSpec>;
#[doc = "Clock configuration register (RCC_CFGR)"]
pub mod cfgr;
#[doc = "CIR (rw) register accessor: Clock interrupt register (RCC_CIR)\n\nYou can [`read`](crate::Reg::read) this register and get [`cir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cir`] module"]
#[doc(alias = "CIR")]
pub type Cir = crate::Reg<cir::CirSpec>;
#[doc = "Clock interrupt register (RCC_CIR)"]
pub mod cir;
#[doc = "APB2RSTR (rw) register accessor: APB2 peripheral reset register (RCC_APB2RSTR)\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2rstr`] module"]
#[doc(alias = "APB2RSTR")]
pub type Apb2rstr = crate::Reg<apb2rstr::Apb2rstrSpec>;
#[doc = "APB2 peripheral reset register (RCC_APB2RSTR)"]
pub mod apb2rstr;
#[doc = "APB1RSTR (rw) register accessor: APB1 peripheral reset register (RCC_APB1RSTR)\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1rstr`] module"]
#[doc(alias = "APB1RSTR")]
pub type Apb1rstr = crate::Reg<apb1rstr::Apb1rstrSpec>;
#[doc = "APB1 peripheral reset register (RCC_APB1RSTR)"]
pub mod apb1rstr;
#[doc = "AHBENR (rw) register accessor: AHB Peripheral Clock enable register (RCC_AHBENR)\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbenr`] module"]
#[doc(alias = "AHBENR")]
pub type Ahbenr = crate::Reg<ahbenr::AhbenrSpec>;
#[doc = "AHB Peripheral Clock enable register (RCC_AHBENR)"]
pub mod ahbenr;
#[doc = "APB2ENR (rw) register accessor: APB2 peripheral clock enable register (RCC_APB2ENR)\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2enr`] module"]
#[doc(alias = "APB2ENR")]
pub type Apb2enr = crate::Reg<apb2enr::Apb2enrSpec>;
#[doc = "APB2 peripheral clock enable register (RCC_APB2ENR)"]
pub mod apb2enr;
#[doc = "APB1ENR (rw) register accessor: APB1 peripheral clock enable register (RCC_APB1ENR)\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1enr`] module"]
#[doc(alias = "APB1ENR")]
pub type Apb1enr = crate::Reg<apb1enr::Apb1enrSpec>;
#[doc = "APB1 peripheral clock enable register (RCC_APB1ENR)"]
pub mod apb1enr;
#[doc = "BDCR (rw) register accessor: Backup domain control register (RCC_BDCR)\n\nYou can [`read`](crate::Reg::read) this register and get [`bdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdcr`] module"]
#[doc(alias = "BDCR")]
pub type Bdcr = crate::Reg<bdcr::BdcrSpec>;
#[doc = "Backup domain control register (RCC_BDCR)"]
pub mod bdcr;
#[doc = "CSR (rw) register accessor: Control/status register (RCC_CSR)\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`] module"]
#[doc(alias = "CSR")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = "Control/status register (RCC_CSR)"]
pub mod csr;
#[doc = "AHBRSTR (rw) register accessor: AHB peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbrstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbrstr`] module"]
#[doc(alias = "AHBRSTR")]
pub type Ahbrstr = crate::Reg<ahbrstr::AhbrstrSpec>;
#[doc = "AHB peripheral reset register"]
pub mod ahbrstr;
#[doc = "CFGR2 (rw) register accessor: Clock configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr2`] module"]
#[doc(alias = "CFGR2")]
pub type Cfgr2 = crate::Reg<cfgr2::Cfgr2Spec>;
#[doc = "Clock configuration register 2"]
pub mod cfgr2;
#[doc = "CFGR3 (rw) register accessor: Clock configuration register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr3`] module"]
#[doc(alias = "CFGR3")]
pub type Cfgr3 = crate::Reg<cfgr3::Cfgr3Spec>;
#[doc = "Clock configuration register 3"]
pub mod cfgr3;
