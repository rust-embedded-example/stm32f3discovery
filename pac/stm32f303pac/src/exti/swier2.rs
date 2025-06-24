#[doc = "Register `SWIER2` reader"]
pub type R = crate::R<Swier2Spec>;
#[doc = "Register `SWIER2` writer"]
pub type W = crate::W<Swier2Spec>;
#[doc = "Field `SWIER32` reader - Software interrupt on line 32"]
pub type Swier32R = crate::BitReader;
#[doc = "Field `SWIER32` writer - Software interrupt on line 32"]
pub type Swier32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIER33` reader - Software interrupt on line 33"]
pub type Swier33R = crate::BitReader;
#[doc = "Field `SWIER33` writer - Software interrupt on line 33"]
pub type Swier33W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software interrupt on line 32"]
    #[inline(always)]
    pub fn swier32(&self) -> Swier32R {
        Swier32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software interrupt on line 33"]
    #[inline(always)]
    pub fn swier33(&self) -> Swier33R {
        Swier33R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software interrupt on line 32"]
    #[inline(always)]
    pub fn swier32(&mut self) -> Swier32W<'_, Swier2Spec> {
        Swier32W::new(self, 0)
    }
    #[doc = "Bit 1 - Software interrupt on line 33"]
    #[inline(always)]
    pub fn swier33(&mut self) -> Swier33W<'_, Swier2Spec> {
        Swier33W::new(self, 1)
    }
}
#[doc = "Software interrupt event register\n\nYou can [`read`](crate::Reg::read) this register and get [`swier2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swier2Spec;
impl crate::RegisterSpec for Swier2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swier2::R`](R) reader structure"]
impl crate::Readable for Swier2Spec {}
#[doc = "`write(|w| ..)` method takes [`swier2::W`](W) writer structure"]
impl crate::Writable for Swier2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWIER2 to value 0"]
impl crate::Resettable for Swier2Spec {}
