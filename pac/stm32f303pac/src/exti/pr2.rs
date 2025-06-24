#[doc = "Register `PR2` reader"]
pub type R = crate::R<Pr2Spec>;
#[doc = "Register `PR2` writer"]
pub type W = crate::W<Pr2Spec>;
#[doc = "Field `PR32` reader - Pending bit on line 32"]
pub type Pr32R = crate::BitReader;
#[doc = "Field `PR32` writer - Pending bit on line 32"]
pub type Pr32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR33` reader - Pending bit on line 33"]
pub type Pr33R = crate::BitReader;
#[doc = "Field `PR33` writer - Pending bit on line 33"]
pub type Pr33W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pending bit on line 32"]
    #[inline(always)]
    pub fn pr32(&self) -> Pr32R {
        Pr32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pending bit on line 33"]
    #[inline(always)]
    pub fn pr33(&self) -> Pr33R {
        Pr33R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pending bit on line 32"]
    #[inline(always)]
    pub fn pr32(&mut self) -> Pr32W<'_, Pr2Spec> {
        Pr32W::new(self, 0)
    }
    #[doc = "Bit 1 - Pending bit on line 33"]
    #[inline(always)]
    pub fn pr33(&mut self) -> Pr33W<'_, Pr2Spec> {
        Pr33W::new(self, 1)
    }
}
#[doc = "Pending register\n\nYou can [`read`](crate::Reg::read) this register and get [`pr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr2Spec;
impl crate::RegisterSpec for Pr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr2::R`](R) reader structure"]
impl crate::Readable for Pr2Spec {}
#[doc = "`write(|w| ..)` method takes [`pr2::W`](W) writer structure"]
impl crate::Writable for Pr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PR2 to value 0"]
impl crate::Resettable for Pr2Spec {}
