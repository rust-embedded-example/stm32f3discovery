#[doc = "Register `CCR1` reader"]
pub type R = crate::R<Ccr1Spec>;
#[doc = "Register `CCR1` writer"]
pub type W = crate::W<Ccr1Spec>;
#[doc = "Field `CCR1L` reader - Low Capture/Compare 1 value"]
pub type Ccr1lR = crate::FieldReader<u16>;
#[doc = "Field `CCR1L` writer - Low Capture/Compare 1 value"]
pub type Ccr1lW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CCR1H` reader - High Capture/Compare 1 value (on TIM2)"]
pub type Ccr1hR = crate::FieldReader<u16>;
#[doc = "Field `CCR1H` writer - High Capture/Compare 1 value (on TIM2)"]
pub type Ccr1hW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Low Capture/Compare 1 value"]
    #[inline(always)]
    pub fn ccr1l(&self) -> Ccr1lR {
        Ccr1lR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High Capture/Compare 1 value (on TIM2)"]
    #[inline(always)]
    pub fn ccr1h(&self) -> Ccr1hR {
        Ccr1hR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low Capture/Compare 1 value"]
    #[inline(always)]
    pub fn ccr1l(&mut self) -> Ccr1lW<'_, Ccr1Spec> {
        Ccr1lW::new(self, 0)
    }
    #[doc = "Bits 16:31 - High Capture/Compare 1 value (on TIM2)"]
    #[inline(always)]
    pub fn ccr1h(&mut self) -> Ccr1hW<'_, Ccr1Spec> {
        Ccr1hW::new(self, 16)
    }
}
#[doc = "capture/compare register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccr1Spec;
impl crate::RegisterSpec for Ccr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr1::R`](R) reader structure"]
impl crate::Readable for Ccr1Spec {}
#[doc = "`write(|w| ..)` method takes [`ccr1::W`](W) writer structure"]
impl crate::Writable for Ccr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCR1 to value 0"]
impl crate::Resettable for Ccr1Spec {}
