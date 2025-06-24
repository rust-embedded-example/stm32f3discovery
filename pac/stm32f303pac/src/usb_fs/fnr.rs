#[doc = "Register `FNR` reader"]
pub type R = crate::R<FnrSpec>;
#[doc = "Field `FN` reader - Frame number"]
pub type FnR = crate::FieldReader<u16>;
#[doc = "Field `LSOF` reader - Lost SOF"]
pub type LsofR = crate::FieldReader;
#[doc = "Field `LCK` reader - Locked"]
pub type LckR = crate::BitReader;
#[doc = "Field `RXDM` reader - Receive data - line status"]
pub type RxdmR = crate::BitReader;
#[doc = "Field `RXDP` reader - Receive data + line status"]
pub type RxdpR = crate::BitReader;
impl R {
    #[doc = "Bits 0:10 - Frame number"]
    #[inline(always)]
    pub fn fn_(&self) -> FnR {
        FnR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:12 - Lost SOF"]
    #[inline(always)]
    pub fn lsof(&self) -> LsofR {
        LsofR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - Locked"]
    #[inline(always)]
    pub fn lck(&self) -> LckR {
        LckR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Receive data - line status"]
    #[inline(always)]
    pub fn rxdm(&self) -> RxdmR {
        RxdmR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Receive data + line status"]
    #[inline(always)]
    pub fn rxdp(&self) -> RxdpR {
        RxdpR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "frame number register\n\nYou can [`read`](crate::Reg::read) this register and get [`fnr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FnrSpec;
impl crate::RegisterSpec for FnrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fnr::R`](R) reader structure"]
impl crate::Readable for FnrSpec {}
#[doc = "`reset()` method sets FNR to value 0"]
impl crate::Resettable for FnrSpec {}
