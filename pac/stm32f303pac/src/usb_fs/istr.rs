#[doc = "Register `ISTR` reader"]
pub type R = crate::R<IstrSpec>;
#[doc = "Register `ISTR` writer"]
pub type W = crate::W<IstrSpec>;
#[doc = "Field `EP_ID` reader - Endpoint Identifier"]
pub type EpIdR = crate::FieldReader;
#[doc = "Field `DIR` reader - Direction of transaction"]
pub type DirR = crate::BitReader;
#[doc = "Field `ESOF` reader - Expected start frame"]
pub type EsofR = crate::BitReader;
#[doc = "Field `ESOF` writer - Expected start frame"]
pub type EsofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOF` reader - start of frame"]
pub type SofR = crate::BitReader;
#[doc = "Field `SOF` writer - start of frame"]
pub type SofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` reader - reset request"]
pub type ResetR = crate::BitReader;
#[doc = "Field `RESET` writer - reset request"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSP` reader - Suspend mode request"]
pub type SuspR = crate::BitReader;
#[doc = "Field `SUSP` writer - Suspend mode request"]
pub type SuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUP` reader - Wakeup"]
pub type WkupR = crate::BitReader;
#[doc = "Field `WKUP` writer - Wakeup"]
pub type WkupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR` reader - Error"]
pub type ErrR = crate::BitReader;
#[doc = "Field `ERR` writer - Error"]
pub type ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMAOVR` reader - Packet memory area over / underrun"]
pub type PmaovrR = crate::BitReader;
#[doc = "Field `PMAOVR` writer - Packet memory area over / underrun"]
pub type PmaovrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTR` reader - Correct transfer"]
pub type CtrR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Endpoint Identifier"]
    #[inline(always)]
    pub fn ep_id(&self) -> EpIdR {
        EpIdR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Direction of transaction"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Expected start frame"]
    #[inline(always)]
    pub fn esof(&self) -> EsofR {
        EsofR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - start of frame"]
    #[inline(always)]
    pub fn sof(&self) -> SofR {
        SofR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - reset request"]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Suspend mode request"]
    #[inline(always)]
    pub fn susp(&self) -> SuspR {
        SuspR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wakeup"]
    #[inline(always)]
    pub fn wkup(&self) -> WkupR {
        WkupR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Error"]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Packet memory area over / underrun"]
    #[inline(always)]
    pub fn pmaovr(&self) -> PmaovrR {
        PmaovrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Correct transfer"]
    #[inline(always)]
    pub fn ctr(&self) -> CtrR {
        CtrR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Expected start frame"]
    #[inline(always)]
    pub fn esof(&mut self) -> EsofW<'_, IstrSpec> {
        EsofW::new(self, 8)
    }
    #[doc = "Bit 9 - start of frame"]
    #[inline(always)]
    pub fn sof(&mut self) -> SofW<'_, IstrSpec> {
        SofW::new(self, 9)
    }
    #[doc = "Bit 10 - reset request"]
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<'_, IstrSpec> {
        ResetW::new(self, 10)
    }
    #[doc = "Bit 11 - Suspend mode request"]
    #[inline(always)]
    pub fn susp(&mut self) -> SuspW<'_, IstrSpec> {
        SuspW::new(self, 11)
    }
    #[doc = "Bit 12 - Wakeup"]
    #[inline(always)]
    pub fn wkup(&mut self) -> WkupW<'_, IstrSpec> {
        WkupW::new(self, 12)
    }
    #[doc = "Bit 13 - Error"]
    #[inline(always)]
    pub fn err(&mut self) -> ErrW<'_, IstrSpec> {
        ErrW::new(self, 13)
    }
    #[doc = "Bit 14 - Packet memory area over / underrun"]
    #[inline(always)]
    pub fn pmaovr(&mut self) -> PmaovrW<'_, IstrSpec> {
        PmaovrW::new(self, 14)
    }
}
#[doc = "interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`istr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`istr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IstrSpec;
impl crate::RegisterSpec for IstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`istr::R`](R) reader structure"]
impl crate::Readable for IstrSpec {}
#[doc = "`write(|w| ..)` method takes [`istr::W`](W) writer structure"]
impl crate::Writable for IstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ISTR to value 0"]
impl crate::Resettable for IstrSpec {}
