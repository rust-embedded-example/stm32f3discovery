#[doc = "Register `EMR2` reader"]
pub type R = crate::R<Emr2Spec>;
#[doc = "Register `EMR2` writer"]
pub type W = crate::W<Emr2Spec>;
#[doc = "Field `MR32` reader - Event mask on external/internal line 32"]
pub type Mr32R = crate::BitReader;
#[doc = "Field `MR32` writer - Event mask on external/internal line 32"]
pub type Mr32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR33` reader - Event mask on external/internal line 33"]
pub type Mr33R = crate::BitReader;
#[doc = "Field `MR33` writer - Event mask on external/internal line 33"]
pub type Mr33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR34` reader - Event mask on external/internal line 34"]
pub type Mr34R = crate::BitReader;
#[doc = "Field `MR34` writer - Event mask on external/internal line 34"]
pub type Mr34W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR35` reader - Event mask on external/internal line 35"]
pub type Mr35R = crate::BitReader;
#[doc = "Field `MR35` writer - Event mask on external/internal line 35"]
pub type Mr35W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Event mask on external/internal line 32"]
    #[inline(always)]
    pub fn mr32(&self) -> Mr32R {
        Mr32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Event mask on external/internal line 33"]
    #[inline(always)]
    pub fn mr33(&self) -> Mr33R {
        Mr33R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Event mask on external/internal line 34"]
    #[inline(always)]
    pub fn mr34(&self) -> Mr34R {
        Mr34R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Event mask on external/internal line 35"]
    #[inline(always)]
    pub fn mr35(&self) -> Mr35R {
        Mr35R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event mask on external/internal line 32"]
    #[inline(always)]
    pub fn mr32(&mut self) -> Mr32W<'_, Emr2Spec> {
        Mr32W::new(self, 0)
    }
    #[doc = "Bit 1 - Event mask on external/internal line 33"]
    #[inline(always)]
    pub fn mr33(&mut self) -> Mr33W<'_, Emr2Spec> {
        Mr33W::new(self, 1)
    }
    #[doc = "Bit 2 - Event mask on external/internal line 34"]
    #[inline(always)]
    pub fn mr34(&mut self) -> Mr34W<'_, Emr2Spec> {
        Mr34W::new(self, 2)
    }
    #[doc = "Bit 3 - Event mask on external/internal line 35"]
    #[inline(always)]
    pub fn mr35(&mut self) -> Mr35W<'_, Emr2Spec> {
        Mr35W::new(self, 3)
    }
}
#[doc = "Event mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`emr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Emr2Spec;
impl crate::RegisterSpec for Emr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emr2::R`](R) reader structure"]
impl crate::Readable for Emr2Spec {}
#[doc = "`write(|w| ..)` method takes [`emr2::W`](W) writer structure"]
impl crate::Writable for Emr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EMR2 to value 0"]
impl crate::Resettable for Emr2Spec {}
