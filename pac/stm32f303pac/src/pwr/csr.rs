#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "Field `WUF` reader - Wakeup flag"]
pub type WufR = crate::BitReader;
#[doc = "Field `SBF` reader - Standby flag"]
pub type SbfR = crate::BitReader;
#[doc = "Field `PVDO` reader - PVD output"]
pub type PvdoR = crate::BitReader;
#[doc = "Field `EWUP1` reader - Enable WKUP1 pin"]
pub type Ewup1R = crate::BitReader;
#[doc = "Field `EWUP1` writer - Enable WKUP1 pin"]
pub type Ewup1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP2` reader - Enable WKUP2 pin"]
pub type Ewup2R = crate::BitReader;
#[doc = "Field `EWUP2` writer - Enable WKUP2 pin"]
pub type Ewup2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Wakeup flag"]
    #[inline(always)]
    pub fn wuf(&self) -> WufR {
        WufR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Standby flag"]
    #[inline(always)]
    pub fn sbf(&self) -> SbfR {
        SbfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PVD output"]
    #[inline(always)]
    pub fn pvdo(&self) -> PvdoR {
        PvdoR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable WKUP1 pin"]
    #[inline(always)]
    pub fn ewup1(&self) -> Ewup1R {
        Ewup1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable WKUP2 pin"]
    #[inline(always)]
    pub fn ewup2(&self) -> Ewup2R {
        Ewup2R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Enable WKUP1 pin"]
    #[inline(always)]
    pub fn ewup1(&mut self) -> Ewup1W<CsrSpec> {
        Ewup1W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable WKUP2 pin"]
    #[inline(always)]
    pub fn ewup2(&mut self) -> Ewup2W<CsrSpec> {
        Ewup2W::new(self, 9)
    }
}
#[doc = "power control/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CsrSpec {}
