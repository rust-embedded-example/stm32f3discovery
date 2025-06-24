#[doc = "Register `ICR` reader"]
pub type R = crate::R<IcrSpec>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `EOAIC` reader - End of acquisition interrupt clear"]
pub type EoaicR = crate::BitReader;
#[doc = "Field `EOAIC` writer - End of acquisition interrupt clear"]
pub type EoaicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCEIC` reader - Max count error interrupt clear"]
pub type MceicR = crate::BitReader;
#[doc = "Field `MCEIC` writer - Max count error interrupt clear"]
pub type MceicW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - End of acquisition interrupt clear"]
    #[inline(always)]
    pub fn eoaic(&self) -> EoaicR {
        EoaicR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Max count error interrupt clear"]
    #[inline(always)]
    pub fn mceic(&self) -> MceicR {
        MceicR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End of acquisition interrupt clear"]
    #[inline(always)]
    pub fn eoaic(&mut self) -> EoaicW<'_, IcrSpec> {
        EoaicW::new(self, 0)
    }
    #[doc = "Bit 1 - Max count error interrupt clear"]
    #[inline(always)]
    pub fn mceic(&mut self) -> MceicW<'_, IcrSpec> {
        MceicW::new(self, 1)
    }
}
#[doc = "interrupt clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for IcrSpec {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {}
