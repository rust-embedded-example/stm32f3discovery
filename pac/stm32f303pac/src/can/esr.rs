#[doc = "Register `ESR` reader"]
pub type R = crate::R<EsrSpec>;
#[doc = "Register `ESR` writer"]
pub type W = crate::W<EsrSpec>;
#[doc = "Field `EWGF` reader - EWGF"]
pub type EwgfR = crate::BitReader;
#[doc = "Field `EPVF` reader - EPVF"]
pub type EpvfR = crate::BitReader;
#[doc = "Field `BOFF` reader - BOFF"]
pub type BoffR = crate::BitReader;
#[doc = "Field `LEC` reader - LEC"]
pub type LecR = crate::FieldReader;
#[doc = "Field `LEC` writer - LEC"]
pub type LecW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TEC` reader - TEC"]
pub type TecR = crate::FieldReader;
#[doc = "Field `REC` reader - REC"]
pub type RecR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - EWGF"]
    #[inline(always)]
    pub fn ewgf(&self) -> EwgfR {
        EwgfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EPVF"]
    #[inline(always)]
    pub fn epvf(&self) -> EpvfR {
        EpvfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BOFF"]
    #[inline(always)]
    pub fn boff(&self) -> BoffR {
        BoffR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - LEC"]
    #[inline(always)]
    pub fn lec(&self) -> LecR {
        LecR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 16:23 - TEC"]
    #[inline(always)]
    pub fn tec(&self) -> TecR {
        TecR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - REC"]
    #[inline(always)]
    pub fn rec(&self) -> RecR {
        RecR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - LEC"]
    #[inline(always)]
    pub fn lec(&mut self) -> LecW<'_, EsrSpec> {
        LecW::new(self, 4)
    }
}
#[doc = "error status register\n\nYou can [`read`](crate::Reg::read) this register and get [`esr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EsrSpec;
impl crate::RegisterSpec for EsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esr::R`](R) reader structure"]
impl crate::Readable for EsrSpec {}
#[doc = "`write(|w| ..)` method takes [`esr::W`](W) writer structure"]
impl crate::Writable for EsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ESR to value 0"]
impl crate::Resettable for EsrSpec {}
