#[doc = "Register `CCR4` reader"]
pub type R = crate::R<Ccr4Spec>;
#[doc = "Register `CCR4` writer"]
pub type W = crate::W<Ccr4Spec>;
#[doc = "Field `CCR4L` reader - Low Capture/Compare value"]
pub type Ccr4lR = crate::FieldReader<u16>;
#[doc = "Field `CCR4L` writer - Low Capture/Compare value"]
pub type Ccr4lW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CCR4H` reader - High Capture/Compare value (on TIM2)"]
pub type Ccr4hR = crate::FieldReader<u16>;
#[doc = "Field `CCR4H` writer - High Capture/Compare value (on TIM2)"]
pub type Ccr4hW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Low Capture/Compare value"]
    #[inline(always)]
    pub fn ccr4l(&self) -> Ccr4lR {
        Ccr4lR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High Capture/Compare value (on TIM2)"]
    #[inline(always)]
    pub fn ccr4h(&self) -> Ccr4hR {
        Ccr4hR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low Capture/Compare value"]
    #[inline(always)]
    pub fn ccr4l(&mut self) -> Ccr4lW<'_, Ccr4Spec> {
        Ccr4lW::new(self, 0)
    }
    #[doc = "Bits 16:31 - High Capture/Compare value (on TIM2)"]
    #[inline(always)]
    pub fn ccr4h(&mut self) -> Ccr4hW<'_, Ccr4Spec> {
        Ccr4hW::new(self, 16)
    }
}
#[doc = "capture/compare register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccr4Spec;
impl crate::RegisterSpec for Ccr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr4::R`](R) reader structure"]
impl crate::Readable for Ccr4Spec {}
#[doc = "`write(|w| ..)` method takes [`ccr4::W`](W) writer structure"]
impl crate::Writable for Ccr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCR4 to value 0"]
impl crate::Resettable for Ccr4Spec {}
