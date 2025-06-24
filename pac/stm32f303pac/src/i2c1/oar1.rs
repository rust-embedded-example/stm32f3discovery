#[doc = "Register `OAR1` reader"]
pub type R = crate::R<Oar1Spec>;
#[doc = "Register `OAR1` writer"]
pub type W = crate::W<Oar1Spec>;
#[doc = "Field `OA1_0` reader - Interface address"]
pub type Oa1_0R = crate::BitReader;
#[doc = "Field `OA1_0` writer - Interface address"]
pub type Oa1_0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OA1_1` reader - Interface address"]
pub type Oa1_1R = crate::FieldReader;
#[doc = "Field `OA1_1` writer - Interface address"]
pub type Oa1_1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `OA1_8` reader - Interface address"]
pub type Oa1_8R = crate::FieldReader;
#[doc = "Field `OA1_8` writer - Interface address"]
pub type Oa1_8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OA1MODE` reader - Own Address 1 10-bit mode"]
pub type Oa1modeR = crate::BitReader;
#[doc = "Field `OA1MODE` writer - Own Address 1 10-bit mode"]
pub type Oa1modeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OA1EN` reader - Own Address 1 enable"]
pub type Oa1enR = crate::BitReader;
#[doc = "Field `OA1EN` writer - Own Address 1 enable"]
pub type Oa1enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interface address"]
    #[inline(always)]
    pub fn oa1_0(&self) -> Oa1_0R {
        Oa1_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn oa1_1(&self) -> Oa1_1R {
        Oa1_1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - Interface address"]
    #[inline(always)]
    pub fn oa1_8(&self) -> Oa1_8R {
        Oa1_8R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Own Address 1 10-bit mode"]
    #[inline(always)]
    pub fn oa1mode(&self) -> Oa1modeR {
        Oa1modeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Own Address 1 enable"]
    #[inline(always)]
    pub fn oa1en(&self) -> Oa1enR {
        Oa1enR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interface address"]
    #[inline(always)]
    pub fn oa1_0(&mut self) -> Oa1_0W<'_, Oar1Spec> {
        Oa1_0W::new(self, 0)
    }
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn oa1_1(&mut self) -> Oa1_1W<'_, Oar1Spec> {
        Oa1_1W::new(self, 1)
    }
    #[doc = "Bits 8:9 - Interface address"]
    #[inline(always)]
    pub fn oa1_8(&mut self) -> Oa1_8W<'_, Oar1Spec> {
        Oa1_8W::new(self, 8)
    }
    #[doc = "Bit 10 - Own Address 1 10-bit mode"]
    #[inline(always)]
    pub fn oa1mode(&mut self) -> Oa1modeW<'_, Oar1Spec> {
        Oa1modeW::new(self, 10)
    }
    #[doc = "Bit 15 - Own Address 1 enable"]
    #[inline(always)]
    pub fn oa1en(&mut self) -> Oa1enW<'_, Oar1Spec> {
        Oa1enW::new(self, 15)
    }
}
#[doc = "Own address register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`oar1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oar1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Oar1Spec;
impl crate::RegisterSpec for Oar1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oar1::R`](R) reader structure"]
impl crate::Readable for Oar1Spec {}
#[doc = "`write(|w| ..)` method takes [`oar1::W`](W) writer structure"]
impl crate::Writable for Oar1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OAR1 to value 0"]
impl crate::Resettable for Oar1Spec {}
