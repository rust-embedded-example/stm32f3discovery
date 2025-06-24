#[doc = "Register `SWTRIGR` writer"]
pub type W = crate::W<SwtrigrSpec>;
#[doc = "Field `SWTRIG1` writer - DAC channel1 software trigger"]
pub type Swtrig1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWTRIG2` writer - DAC channel2 software trigger"]
pub type Swtrig2W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - DAC channel1 software trigger"]
    #[inline(always)]
    pub fn swtrig1(&mut self) -> Swtrig1W<'_, SwtrigrSpec> {
        Swtrig1W::new(self, 0)
    }
    #[doc = "Bit 1 - DAC channel2 software trigger"]
    #[inline(always)]
    pub fn swtrig2(&mut self) -> Swtrig2W<'_, SwtrigrSpec> {
        Swtrig2W::new(self, 1)
    }
}
#[doc = "software trigger register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrigr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwtrigrSpec;
impl crate::RegisterSpec for SwtrigrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swtrigr::W`](W) writer structure"]
impl crate::Writable for SwtrigrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWTRIGR to value 0"]
impl crate::Resettable for SwtrigrSpec {}
