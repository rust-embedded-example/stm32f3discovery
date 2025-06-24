#[doc = "Register `SYSCFG_EXTICR2` reader"]
pub type R = crate::R<SyscfgExticr2Spec>;
#[doc = "Register `SYSCFG_EXTICR2` writer"]
pub type W = crate::W<SyscfgExticr2Spec>;
#[doc = "Field `EXTI4` reader - EXTI 4 configuration bits"]
pub type Exti4R = crate::FieldReader;
#[doc = "Field `EXTI4` writer - EXTI 4 configuration bits"]
pub type Exti4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI5` reader - EXTI 5 configuration bits"]
pub type Exti5R = crate::FieldReader;
#[doc = "Field `EXTI5` writer - EXTI 5 configuration bits"]
pub type Exti5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI6` reader - EXTI 6 configuration bits"]
pub type Exti6R = crate::FieldReader;
#[doc = "Field `EXTI6` writer - EXTI 6 configuration bits"]
pub type Exti6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI7` reader - EXTI 7 configuration bits"]
pub type Exti7R = crate::FieldReader;
#[doc = "Field `EXTI7` writer - EXTI 7 configuration bits"]
pub type Exti7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - EXTI 4 configuration bits"]
    #[inline(always)]
    pub fn exti4(&self) -> Exti4R {
        Exti4R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 5 configuration bits"]
    #[inline(always)]
    pub fn exti5(&self) -> Exti5R {
        Exti5R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 6 configuration bits"]
    #[inline(always)]
    pub fn exti6(&self) -> Exti6R {
        Exti6R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI 7 configuration bits"]
    #[inline(always)]
    pub fn exti7(&self) -> Exti7R {
        Exti7R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI 4 configuration bits"]
    #[inline(always)]
    pub fn exti4(&mut self) -> Exti4W<SyscfgExticr2Spec> {
        Exti4W::new(self, 0)
    }
    #[doc = "Bits 4:7 - EXTI 5 configuration bits"]
    #[inline(always)]
    pub fn exti5(&mut self) -> Exti5W<SyscfgExticr2Spec> {
        Exti5W::new(self, 4)
    }
    #[doc = "Bits 8:11 - EXTI 6 configuration bits"]
    #[inline(always)]
    pub fn exti6(&mut self) -> Exti6W<SyscfgExticr2Spec> {
        Exti6W::new(self, 8)
    }
    #[doc = "Bits 12:15 - EXTI 7 configuration bits"]
    #[inline(always)]
    pub fn exti7(&mut self) -> Exti7W<SyscfgExticr2Spec> {
        Exti7W::new(self, 12)
    }
}
#[doc = "external interrupt configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`syscfg_exticr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg_exticr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgExticr2Spec;
impl crate::RegisterSpec for SyscfgExticr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_exticr2::R`](R) reader structure"]
impl crate::Readable for SyscfgExticr2Spec {}
#[doc = "`write(|w| ..)` method takes [`syscfg_exticr2::W`](W) writer structure"]
impl crate::Writable for SyscfgExticr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCFG_EXTICR2 to value 0"]
impl crate::Resettable for SyscfgExticr2Spec {}
