#[doc = "Register `CCR2` reader"]
pub type R = crate::R<Ccr2Spec>;
#[doc = "Register `CCR2` writer"]
pub type W = crate::W<Ccr2Spec>;
#[doc = "Field `CCR2L` reader - Low Capture/Compare 2 value"]
pub type Ccr2lR = crate::FieldReader<u16>;
#[doc = "Field `CCR2L` writer - Low Capture/Compare 2 value"]
pub type Ccr2lW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CCR2H` reader - High Capture/Compare 2 value (on TIM2)"]
pub type Ccr2hR = crate::FieldReader<u16>;
#[doc = "Field `CCR2H` writer - High Capture/Compare 2 value (on TIM2)"]
pub type Ccr2hW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Low Capture/Compare 2 value"]
    #[inline(always)]
    pub fn ccr2l(&self) -> Ccr2lR {
        Ccr2lR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High Capture/Compare 2 value (on TIM2)"]
    #[inline(always)]
    pub fn ccr2h(&self) -> Ccr2hR {
        Ccr2hR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low Capture/Compare 2 value"]
    #[inline(always)]
    pub fn ccr2l(&mut self) -> Ccr2lW<'_, Ccr2Spec> {
        Ccr2lW::new(self, 0)
    }
    #[doc = "Bits 16:31 - High Capture/Compare 2 value (on TIM2)"]
    #[inline(always)]
    pub fn ccr2h(&mut self) -> Ccr2hW<'_, Ccr2Spec> {
        Ccr2hW::new(self, 16)
    }
}
#[doc = "capture/compare register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccr2Spec;
impl crate::RegisterSpec for Ccr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr2::R`](R) reader structure"]
impl crate::Readable for Ccr2Spec {}
#[doc = "`write(|w| ..)` method takes [`ccr2::W`](W) writer structure"]
impl crate::Writable for Ccr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCR2 to value 0"]
impl crate::Resettable for Ccr2Spec {}
