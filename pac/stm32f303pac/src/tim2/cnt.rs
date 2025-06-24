#[doc = "Register `CNT` reader"]
pub type R = crate::R<CntSpec>;
#[doc = "Register `CNT` writer"]
pub type W = crate::W<CntSpec>;
#[doc = "Field `CNTL` reader - Low counter value"]
pub type CntlR = crate::FieldReader<u16>;
#[doc = "Field `CNTL` writer - Low counter value"]
pub type CntlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CNTH` reader - High counter value"]
pub type CnthR = crate::FieldReader<u16>;
#[doc = "Field `CNTH` writer - High counter value"]
pub type CnthW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `CNT_or_UIFCPY` reader - if IUFREMAP=0 than CNT with read write access else UIFCPY with read only access"]
pub type CntOrUifcpyR = crate::BitReader;
#[doc = "Field `CNT_or_UIFCPY` writer - if IUFREMAP=0 than CNT with read write access else UIFCPY with read only access"]
pub type CntOrUifcpyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Low counter value"]
    #[inline(always)]
    pub fn cntl(&self) -> CntlR {
        CntlR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:30 - High counter value"]
    #[inline(always)]
    pub fn cnth(&self) -> CnthR {
        CnthR::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - if IUFREMAP=0 than CNT with read write access else UIFCPY with read only access"]
    #[inline(always)]
    pub fn cnt_or_uifcpy(&self) -> CntOrUifcpyR {
        CntOrUifcpyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low counter value"]
    #[inline(always)]
    pub fn cntl(&mut self) -> CntlW<'_, CntSpec> {
        CntlW::new(self, 0)
    }
    #[doc = "Bits 16:30 - High counter value"]
    #[inline(always)]
    pub fn cnth(&mut self) -> CnthW<'_, CntSpec> {
        CnthW::new(self, 16)
    }
    #[doc = "Bit 31 - if IUFREMAP=0 than CNT with read write access else UIFCPY with read only access"]
    #[inline(always)]
    pub fn cnt_or_uifcpy(&mut self) -> CntOrUifcpyW<'_, CntSpec> {
        CntOrUifcpyW::new(self, 31)
    }
}
#[doc = "counter\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntSpec;
impl crate::RegisterSpec for CntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnt::R`](R) reader structure"]
impl crate::Readable for CntSpec {}
#[doc = "`write(|w| ..)` method takes [`cnt::W`](W) writer structure"]
impl crate::Writable for CntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNT to value 0"]
impl crate::Resettable for CntSpec {}
