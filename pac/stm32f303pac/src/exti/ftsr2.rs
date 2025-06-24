#[doc = "Register `FTSR2` reader"]
pub type R = crate::R<Ftsr2Spec>;
#[doc = "Register `FTSR2` writer"]
pub type W = crate::W<Ftsr2Spec>;
#[doc = "Field `TR32` reader - Falling trigger event configuration bit of line 32"]
pub type Tr32R = crate::BitReader;
#[doc = "Field `TR32` writer - Falling trigger event configuration bit of line 32"]
pub type Tr32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR33` reader - Falling trigger event configuration bit of line 33"]
pub type Tr33R = crate::BitReader;
#[doc = "Field `TR33` writer - Falling trigger event configuration bit of line 33"]
pub type Tr33W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Falling trigger event configuration bit of line 32"]
    #[inline(always)]
    pub fn tr32(&self) -> Tr32R {
        Tr32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Falling trigger event configuration bit of line 33"]
    #[inline(always)]
    pub fn tr33(&self) -> Tr33R {
        Tr33R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Falling trigger event configuration bit of line 32"]
    #[inline(always)]
    pub fn tr32(&mut self) -> Tr32W<'_, Ftsr2Spec> {
        Tr32W::new(self, 0)
    }
    #[doc = "Bit 1 - Falling trigger event configuration bit of line 33"]
    #[inline(always)]
    pub fn tr33(&mut self) -> Tr33W<'_, Ftsr2Spec> {
        Tr33W::new(self, 1)
    }
}
#[doc = "Falling Trigger selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`ftsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ftsr2Spec;
impl crate::RegisterSpec for Ftsr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ftsr2::R`](R) reader structure"]
impl crate::Readable for Ftsr2Spec {}
#[doc = "`write(|w| ..)` method takes [`ftsr2::W`](W) writer structure"]
impl crate::Writable for Ftsr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FTSR2 to value 0"]
impl crate::Resettable for Ftsr2Spec {}
