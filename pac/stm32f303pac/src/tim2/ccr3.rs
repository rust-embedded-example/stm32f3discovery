#[doc = "Register `CCR3` reader"]
pub type R = crate::R<Ccr3Spec>;
#[doc = "Register `CCR3` writer"]
pub type W = crate::W<Ccr3Spec>;
#[doc = "Field `CCR3L` reader - Low Capture/Compare value"]
pub type Ccr3lR = crate::FieldReader<u16>;
#[doc = "Field `CCR3L` writer - Low Capture/Compare value"]
pub type Ccr3lW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CCR3H` reader - High Capture/Compare value (on TIM2)"]
pub type Ccr3hR = crate::FieldReader<u16>;
#[doc = "Field `CCR3H` writer - High Capture/Compare value (on TIM2)"]
pub type Ccr3hW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Low Capture/Compare value"]
    #[inline(always)]
    pub fn ccr3l(&self) -> Ccr3lR {
        Ccr3lR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High Capture/Compare value (on TIM2)"]
    #[inline(always)]
    pub fn ccr3h(&self) -> Ccr3hR {
        Ccr3hR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low Capture/Compare value"]
    #[inline(always)]
    pub fn ccr3l(&mut self) -> Ccr3lW<'_, Ccr3Spec> {
        Ccr3lW::new(self, 0)
    }
    #[doc = "Bits 16:31 - High Capture/Compare value (on TIM2)"]
    #[inline(always)]
    pub fn ccr3h(&mut self) -> Ccr3hW<'_, Ccr3Spec> {
        Ccr3hW::new(self, 16)
    }
}
#[doc = "capture/compare register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccr3Spec;
impl crate::RegisterSpec for Ccr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr3::R`](R) reader structure"]
impl crate::Readable for Ccr3Spec {}
#[doc = "`write(|w| ..)` method takes [`ccr3::W`](W) writer structure"]
impl crate::Writable for Ccr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCR3 to value 0"]
impl crate::Resettable for Ccr3Spec {}
