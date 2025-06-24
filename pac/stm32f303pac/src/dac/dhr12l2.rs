#[doc = "Register `DHR12L2` reader"]
pub type R = crate::R<Dhr12l2Spec>;
#[doc = "Register `DHR12L2` writer"]
pub type W = crate::W<Dhr12l2Spec>;
#[doc = "Field `DACC2DHR` reader - DAC channel2 12-bit left-aligned data"]
pub type Dacc2dhrR = crate::FieldReader<u16>;
#[doc = "Field `DACC2DHR` writer - DAC channel2 12-bit left-aligned data"]
pub type Dacc2dhrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 4:15 - DAC channel2 12-bit left-aligned data"]
    #[inline(always)]
    pub fn dacc2dhr(&self) -> Dacc2dhrR {
        Dacc2dhrR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 4:15 - DAC channel2 12-bit left-aligned data"]
    #[inline(always)]
    pub fn dacc2dhr(&mut self) -> Dacc2dhrW<Dhr12l2Spec> {
        Dacc2dhrW::new(self, 4)
    }
}
#[doc = "channel2 12-bit left aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr12l2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12l2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dhr12l2Spec;
impl crate::RegisterSpec for Dhr12l2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dhr12l2::R`](R) reader structure"]
impl crate::Readable for Dhr12l2Spec {}
#[doc = "`write(|w| ..)` method takes [`dhr12l2::W`](W) writer structure"]
impl crate::Writable for Dhr12l2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DHR12L2 to value 0"]
impl crate::Resettable for Dhr12l2Spec {}
