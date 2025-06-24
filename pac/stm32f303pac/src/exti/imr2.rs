#[doc = "Register `IMR2` reader"]
pub type R = crate::R<Imr2Spec>;
#[doc = "Register `IMR2` writer"]
pub type W = crate::W<Imr2Spec>;
#[doc = "Field `MR32` reader - Interrupt Mask on external/internal line 32"]
pub type Mr32R = crate::BitReader;
#[doc = "Field `MR32` writer - Interrupt Mask on external/internal line 32"]
pub type Mr32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR33` reader - Interrupt Mask on external/internal line 33"]
pub type Mr33R = crate::BitReader;
#[doc = "Field `MR33` writer - Interrupt Mask on external/internal line 33"]
pub type Mr33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR34` reader - Interrupt Mask on external/internal line 34"]
pub type Mr34R = crate::BitReader;
#[doc = "Field `MR34` writer - Interrupt Mask on external/internal line 34"]
pub type Mr34W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR35` reader - Interrupt Mask on external/internal line 35"]
pub type Mr35R = crate::BitReader;
#[doc = "Field `MR35` writer - Interrupt Mask on external/internal line 35"]
pub type Mr35W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt Mask on external/internal line 32"]
    #[inline(always)]
    pub fn mr32(&self) -> Mr32R {
        Mr32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Mask on external/internal line 33"]
    #[inline(always)]
    pub fn mr33(&self) -> Mr33R {
        Mr33R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Mask on external/internal line 34"]
    #[inline(always)]
    pub fn mr34(&self) -> Mr34R {
        Mr34R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Mask on external/internal line 35"]
    #[inline(always)]
    pub fn mr35(&self) -> Mr35R {
        Mr35R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Mask on external/internal line 32"]
    #[inline(always)]
    pub fn mr32(&mut self) -> Mr32W<'_, Imr2Spec> {
        Mr32W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt Mask on external/internal line 33"]
    #[inline(always)]
    pub fn mr33(&mut self) -> Mr33W<'_, Imr2Spec> {
        Mr33W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt Mask on external/internal line 34"]
    #[inline(always)]
    pub fn mr34(&mut self) -> Mr34W<'_, Imr2Spec> {
        Mr34W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt Mask on external/internal line 35"]
    #[inline(always)]
    pub fn mr35(&mut self) -> Mr35W<'_, Imr2Spec> {
        Mr35W::new(self, 3)
    }
}
#[doc = "Interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Imr2Spec;
impl crate::RegisterSpec for Imr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr2::R`](R) reader structure"]
impl crate::Readable for Imr2Spec {}
#[doc = "`write(|w| ..)` method takes [`imr2::W`](W) writer structure"]
impl crate::Writable for Imr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IMR2 to value 0xffff_fffc"]
impl crate::Resettable for Imr2Spec {
    const RESET_VALUE: u32 = 0xffff_fffc;
}
