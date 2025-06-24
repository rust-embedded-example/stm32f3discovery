#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    idcode: Idcode,
    cr: Cr,
    apb1fz: Apb1fz,
    apb2fz: Apb2fz,
}
impl RegisterBlock {
    #[doc = "0x00 - MCU Device ID Code Register"]
    #[inline(always)]
    pub const fn idcode(&self) -> &Idcode {
        &self.idcode
    }
    #[doc = "0x04 - Debug MCU Configuration Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x08 - APB Low Freeze Register"]
    #[inline(always)]
    pub const fn apb1fz(&self) -> &Apb1fz {
        &self.apb1fz
    }
    #[doc = "0x0c - APB High Freeze Register"]
    #[inline(always)]
    pub const fn apb2fz(&self) -> &Apb2fz {
        &self.apb2fz
    }
}
#[doc = "IDCODE (r) register accessor: MCU Device ID Code Register\n\nYou can [`read`](crate::Reg::read) this register and get [`idcode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idcode`] module"]
#[doc(alias = "IDCODE")]
pub type Idcode = crate::Reg<idcode::IdcodeSpec>;
#[doc = "MCU Device ID Code Register"]
pub mod idcode;
#[doc = "CR (rw) register accessor: Debug MCU Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Debug MCU Configuration Register"]
pub mod cr;
#[doc = "APB1FZ (rw) register accessor: APB Low Freeze Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1fz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1fz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1fz`] module"]
#[doc(alias = "APB1FZ")]
pub type Apb1fz = crate::Reg<apb1fz::Apb1fzSpec>;
#[doc = "APB Low Freeze Register"]
pub mod apb1fz;
#[doc = "APB2FZ (rw) register accessor: APB High Freeze Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2fz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2fz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2fz`] module"]
#[doc(alias = "APB2FZ")]
pub type Apb2fz = crate::Reg<apb2fz::Apb2fzSpec>;
#[doc = "APB High Freeze Register"]
pub mod apb2fz;
