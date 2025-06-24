#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Register `ISR` writer"]
pub type W = crate::W<IsrSpec>;
#[doc = "Field `EOAF` reader - End of acquisition flag"]
pub type EoafR = crate::BitReader;
#[doc = "Field `EOAF` writer - End of acquisition flag"]
pub type EoafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCEF` reader - Max count error flag"]
pub type McefR = crate::BitReader;
#[doc = "Field `MCEF` writer - Max count error flag"]
pub type McefW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - End of acquisition flag"]
    #[inline(always)]
    pub fn eoaf(&self) -> EoafR {
        EoafR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Max count error flag"]
    #[inline(always)]
    pub fn mcef(&self) -> McefR {
        McefR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End of acquisition flag"]
    #[inline(always)]
    pub fn eoaf(&mut self) -> EoafW<'_, IsrSpec> {
        EoafW::new(self, 0)
    }
    #[doc = "Bit 1 - Max count error flag"]
    #[inline(always)]
    pub fn mcef(&mut self) -> McefW<'_, IsrSpec> {
        McefW::new(self, 1)
    }
}
#[doc = "interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`write(|w| ..)` method takes [`isr::W`](W) writer structure"]
impl crate::Writable for IsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
