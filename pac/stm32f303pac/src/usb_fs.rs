#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    usb_ep0r: UsbEp0r,
    usb_ep1r: UsbEp1r,
    usb_ep2r: UsbEp2r,
    usb_ep3r: UsbEp3r,
    usb_ep4r: UsbEp4r,
    usb_ep5r: UsbEp5r,
    usb_ep6r: UsbEp6r,
    usb_ep7r: UsbEp7r,
    _reserved8: [u8; 0x20],
    usb_cntr: UsbCntr,
    istr: Istr,
    fnr: Fnr,
    daddr: Daddr,
    btable: Btable,
}
impl RegisterBlock {
    #[doc = "0x00 - endpoint 0 register"]
    #[inline(always)]
    pub const fn usb_ep0r(&self) -> &UsbEp0r {
        &self.usb_ep0r
    }
    #[doc = "0x04 - endpoint 1 register"]
    #[inline(always)]
    pub const fn usb_ep1r(&self) -> &UsbEp1r {
        &self.usb_ep1r
    }
    #[doc = "0x08 - endpoint 2 register"]
    #[inline(always)]
    pub const fn usb_ep2r(&self) -> &UsbEp2r {
        &self.usb_ep2r
    }
    #[doc = "0x0c - endpoint 3 register"]
    #[inline(always)]
    pub const fn usb_ep3r(&self) -> &UsbEp3r {
        &self.usb_ep3r
    }
    #[doc = "0x10 - endpoint 4 register"]
    #[inline(always)]
    pub const fn usb_ep4r(&self) -> &UsbEp4r {
        &self.usb_ep4r
    }
    #[doc = "0x14 - endpoint 5 register"]
    #[inline(always)]
    pub const fn usb_ep5r(&self) -> &UsbEp5r {
        &self.usb_ep5r
    }
    #[doc = "0x18 - endpoint 6 register"]
    #[inline(always)]
    pub const fn usb_ep6r(&self) -> &UsbEp6r {
        &self.usb_ep6r
    }
    #[doc = "0x1c - endpoint 7 register"]
    #[inline(always)]
    pub const fn usb_ep7r(&self) -> &UsbEp7r {
        &self.usb_ep7r
    }
    #[doc = "0x40 - control register"]
    #[inline(always)]
    pub const fn usb_cntr(&self) -> &UsbCntr {
        &self.usb_cntr
    }
    #[doc = "0x44 - interrupt status register"]
    #[inline(always)]
    pub const fn istr(&self) -> &Istr {
        &self.istr
    }
    #[doc = "0x48 - frame number register"]
    #[inline(always)]
    pub const fn fnr(&self) -> &Fnr {
        &self.fnr
    }
    #[doc = "0x4c - device address"]
    #[inline(always)]
    pub const fn daddr(&self) -> &Daddr {
        &self.daddr
    }
    #[doc = "0x50 - Buffer table address"]
    #[inline(always)]
    pub const fn btable(&self) -> &Btable {
        &self.btable
    }
}
#[doc = "USB_EP0R (rw) register accessor: endpoint 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_ep0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_ep0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_ep0r`] module"]
#[doc(alias = "USB_EP0R")]
pub type UsbEp0r = crate::Reg<usb_ep0r::UsbEp0rSpec>;
#[doc = "endpoint 0 register"]
pub mod usb_ep0r;
#[doc = "USB_EP1R (rw) register accessor: endpoint 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_ep1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_ep1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_ep1r`] module"]
#[doc(alias = "USB_EP1R")]
pub type UsbEp1r = crate::Reg<usb_ep1r::UsbEp1rSpec>;
#[doc = "endpoint 1 register"]
pub mod usb_ep1r;
#[doc = "USB_EP2R (rw) register accessor: endpoint 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_ep2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_ep2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_ep2r`] module"]
#[doc(alias = "USB_EP2R")]
pub type UsbEp2r = crate::Reg<usb_ep2r::UsbEp2rSpec>;
#[doc = "endpoint 2 register"]
pub mod usb_ep2r;
#[doc = "USB_EP3R (rw) register accessor: endpoint 3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_ep3r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_ep3r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_ep3r`] module"]
#[doc(alias = "USB_EP3R")]
pub type UsbEp3r = crate::Reg<usb_ep3r::UsbEp3rSpec>;
#[doc = "endpoint 3 register"]
pub mod usb_ep3r;
#[doc = "USB_EP4R (rw) register accessor: endpoint 4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_ep4r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_ep4r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_ep4r`] module"]
#[doc(alias = "USB_EP4R")]
pub type UsbEp4r = crate::Reg<usb_ep4r::UsbEp4rSpec>;
#[doc = "endpoint 4 register"]
pub mod usb_ep4r;
#[doc = "USB_EP5R (rw) register accessor: endpoint 5 register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_ep5r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_ep5r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_ep5r`] module"]
#[doc(alias = "USB_EP5R")]
pub type UsbEp5r = crate::Reg<usb_ep5r::UsbEp5rSpec>;
#[doc = "endpoint 5 register"]
pub mod usb_ep5r;
#[doc = "USB_EP6R (rw) register accessor: endpoint 6 register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_ep6r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_ep6r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_ep6r`] module"]
#[doc(alias = "USB_EP6R")]
pub type UsbEp6r = crate::Reg<usb_ep6r::UsbEp6rSpec>;
#[doc = "endpoint 6 register"]
pub mod usb_ep6r;
#[doc = "USB_EP7R (rw) register accessor: endpoint 7 register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_ep7r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_ep7r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_ep7r`] module"]
#[doc(alias = "USB_EP7R")]
pub type UsbEp7r = crate::Reg<usb_ep7r::UsbEp7rSpec>;
#[doc = "endpoint 7 register"]
pub mod usb_ep7r;
#[doc = "USB_CNTR (rw) register accessor: control register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_cntr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_cntr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_cntr`] module"]
#[doc(alias = "USB_CNTR")]
pub type UsbCntr = crate::Reg<usb_cntr::UsbCntrSpec>;
#[doc = "control register"]
pub mod usb_cntr;
#[doc = "ISTR (rw) register accessor: interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`istr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`istr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@istr`] module"]
#[doc(alias = "ISTR")]
pub type Istr = crate::Reg<istr::IstrSpec>;
#[doc = "interrupt status register"]
pub mod istr;
#[doc = "FNR (r) register accessor: frame number register\n\nYou can [`read`](crate::Reg::read) this register and get [`fnr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fnr`] module"]
#[doc(alias = "FNR")]
pub type Fnr = crate::Reg<fnr::FnrSpec>;
#[doc = "frame number register"]
pub mod fnr;
#[doc = "DADDR (rw) register accessor: device address\n\nYou can [`read`](crate::Reg::read) this register and get [`daddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daddr`] module"]
#[doc(alias = "DADDR")]
pub type Daddr = crate::Reg<daddr::DaddrSpec>;
#[doc = "device address"]
pub mod daddr;
#[doc = "BTABLE (rw) register accessor: Buffer table address\n\nYou can [`read`](crate::Reg::read) this register and get [`btable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btable`] module"]
#[doc(alias = "BTABLE")]
pub type Btable = crate::Reg<btable::BtableSpec>;
#[doc = "Buffer table address"]
pub mod btable;
