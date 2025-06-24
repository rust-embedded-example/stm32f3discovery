#[doc = "Register `RTSR2` reader"]
pub type R = crate::R<Rtsr2Spec>;
#[doc = "Register `RTSR2` writer"]
pub type W = crate::W<Rtsr2Spec>;
#[doc = "Field `TR32` reader - Rising trigger event configuration bit of line 32"]
pub type Tr32R = crate::BitReader;
#[doc = "Field `TR32` writer - Rising trigger event configuration bit of line 32"]
pub type Tr32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR33` reader - Rising trigger event configuration bit of line 33"]
pub type Tr33R = crate::BitReader;
#[doc = "Field `TR33` writer - Rising trigger event configuration bit of line 33"]
pub type Tr33W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rising trigger event configuration bit of line 32"]
    #[inline(always)]
    pub fn tr32(&self) -> Tr32R {
        Tr32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration bit of line 33"]
    #[inline(always)]
    pub fn tr33(&self) -> Tr33R {
        Tr33R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rising trigger event configuration bit of line 32"]
    #[inline(always)]
    pub fn tr32(&mut self) -> Tr32W<'_, Rtsr2Spec> {
        Tr32W::new(self, 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration bit of line 33"]
    #[inline(always)]
    pub fn tr33(&mut self) -> Tr33W<'_, Rtsr2Spec> {
        Tr33W::new(self, 1)
    }
}
#[doc = "Rising Trigger selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtsr2Spec;
impl crate::RegisterSpec for Rtsr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtsr2::R`](R) reader structure"]
impl crate::Readable for Rtsr2Spec {}
#[doc = "`write(|w| ..)` method takes [`rtsr2::W`](W) writer structure"]
impl crate::Writable for Rtsr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTSR2 to value 0"]
impl crate::Resettable for Rtsr2Spec {}
