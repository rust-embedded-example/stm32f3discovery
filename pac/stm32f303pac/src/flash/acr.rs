#[doc = "Register `ACR` reader"]
pub type R = crate::R<AcrSpec>;
#[doc = "Register `ACR` writer"]
pub type W = crate::W<AcrSpec>;
#[doc = "Field `LATENCY` reader - LATENCY"]
pub type LatencyR = crate::FieldReader;
#[doc = "Field `LATENCY` writer - LATENCY"]
pub type LatencyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRFTBE` reader - PRFTBE"]
pub type PrftbeR = crate::BitReader;
#[doc = "Field `PRFTBE` writer - PRFTBE"]
pub type PrftbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRFTBS` reader - PRFTBS"]
pub type PrftbsR = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - LATENCY"]
    #[inline(always)]
    pub fn latency(&self) -> LatencyR {
        LatencyR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - PRFTBE"]
    #[inline(always)]
    pub fn prftbe(&self) -> PrftbeR {
        PrftbeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PRFTBS"]
    #[inline(always)]
    pub fn prftbs(&self) -> PrftbsR {
        PrftbsR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - LATENCY"]
    #[inline(always)]
    pub fn latency(&mut self) -> LatencyW<'_, AcrSpec> {
        LatencyW::new(self, 0)
    }
    #[doc = "Bit 4 - PRFTBE"]
    #[inline(always)]
    pub fn prftbe(&mut self) -> PrftbeW<'_, AcrSpec> {
        PrftbeW::new(self, 4)
    }
}
#[doc = "Flash access control register\n\nYou can [`read`](crate::Reg::read) this register and get [`acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcrSpec;
impl crate::RegisterSpec for AcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acr::R`](R) reader structure"]
impl crate::Readable for AcrSpec {}
#[doc = "`write(|w| ..)` method takes [`acr::W`](W) writer structure"]
impl crate::Writable for AcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ACR to value 0x30"]
impl crate::Resettable for AcrSpec {
    const RESET_VALUE: u32 = 0x30;
}
