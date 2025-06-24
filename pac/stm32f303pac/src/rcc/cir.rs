#[doc = "Register `CIR` reader"]
pub type R = crate::R<CirSpec>;
#[doc = "Register `CIR` writer"]
pub type W = crate::W<CirSpec>;
#[doc = "Field `LSIRDYF` reader - LSI Ready Interrupt flag"]
pub type LsirdyfR = crate::BitReader;
#[doc = "Field `LSERDYF` reader - LSE Ready Interrupt flag"]
pub type LserdyfR = crate::BitReader;
#[doc = "Field `HSIRDYF` reader - HSI Ready Interrupt flag"]
pub type HsirdyfR = crate::BitReader;
#[doc = "Field `HSERDYF` reader - HSE Ready Interrupt flag"]
pub type HserdyfR = crate::BitReader;
#[doc = "Field `PLLRDYF` reader - PLL Ready Interrupt flag"]
pub type PllrdyfR = crate::BitReader;
#[doc = "Field `CSSF` reader - Clock Security System Interrupt flag"]
pub type CssfR = crate::BitReader;
#[doc = "Field `LSIRDYIE` reader - LSI Ready Interrupt Enable"]
pub type LsirdyieR = crate::BitReader;
#[doc = "Field `LSIRDYIE` writer - LSI Ready Interrupt Enable"]
pub type LsirdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDYIE` reader - LSE Ready Interrupt Enable"]
pub type LserdyieR = crate::BitReader;
#[doc = "Field `LSERDYIE` writer - LSE Ready Interrupt Enable"]
pub type LserdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDYIE` reader - HSI Ready Interrupt Enable"]
pub type HsirdyieR = crate::BitReader;
#[doc = "Field `HSIRDYIE` writer - HSI Ready Interrupt Enable"]
pub type HsirdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDYIE` reader - HSE Ready Interrupt Enable"]
pub type HserdyieR = crate::BitReader;
#[doc = "Field `HSERDYIE` writer - HSE Ready Interrupt Enable"]
pub type HserdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLRDYIE` reader - PLL Ready Interrupt Enable"]
pub type PllrdyieR = crate::BitReader;
#[doc = "Field `PLLRDYIE` writer - PLL Ready Interrupt Enable"]
pub type PllrdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSIRDYC` writer - LSI Ready Interrupt Clear"]
pub type LsirdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDYC` writer - LSE Ready Interrupt Clear"]
pub type LserdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDYC` writer - HSI Ready Interrupt Clear"]
pub type HsirdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDYC` writer - HSE Ready Interrupt Clear"]
pub type HserdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLRDYC` writer - PLL Ready Interrupt Clear"]
pub type PllrdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSSC` writer - Clock security system interrupt clear"]
pub type CsscW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LSI Ready Interrupt flag"]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LsirdyfR {
        LsirdyfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE Ready Interrupt flag"]
    #[inline(always)]
    pub fn lserdyf(&self) -> LserdyfR {
        LserdyfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSI Ready Interrupt flag"]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HsirdyfR {
        HsirdyfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSE Ready Interrupt flag"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HserdyfR {
        HserdyfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PLL Ready Interrupt flag"]
    #[inline(always)]
    pub fn pllrdyf(&self) -> PllrdyfR {
        PllrdyfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock Security System Interrupt flag"]
    #[inline(always)]
    pub fn cssf(&self) -> CssfR {
        CssfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LSI Ready Interrupt Enable"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LsirdyieR {
        LsirdyieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LSE Ready Interrupt Enable"]
    #[inline(always)]
    pub fn lserdyie(&self) -> LserdyieR {
        LserdyieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSI Ready Interrupt Enable"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HsirdyieR {
        HsirdyieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HSE Ready Interrupt Enable"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HserdyieR {
        HserdyieR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PLL Ready Interrupt Enable"]
    #[inline(always)]
    pub fn pllrdyie(&self) -> PllrdyieR {
        PllrdyieR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - LSI Ready Interrupt Enable"]
    #[inline(always)]
    pub fn lsirdyie(&mut self) -> LsirdyieW<'_, CirSpec> {
        LsirdyieW::new(self, 8)
    }
    #[doc = "Bit 9 - LSE Ready Interrupt Enable"]
    #[inline(always)]
    pub fn lserdyie(&mut self) -> LserdyieW<'_, CirSpec> {
        LserdyieW::new(self, 9)
    }
    #[doc = "Bit 10 - HSI Ready Interrupt Enable"]
    #[inline(always)]
    pub fn hsirdyie(&mut self) -> HsirdyieW<'_, CirSpec> {
        HsirdyieW::new(self, 10)
    }
    #[doc = "Bit 11 - HSE Ready Interrupt Enable"]
    #[inline(always)]
    pub fn hserdyie(&mut self) -> HserdyieW<'_, CirSpec> {
        HserdyieW::new(self, 11)
    }
    #[doc = "Bit 12 - PLL Ready Interrupt Enable"]
    #[inline(always)]
    pub fn pllrdyie(&mut self) -> PllrdyieW<'_, CirSpec> {
        PllrdyieW::new(self, 12)
    }
    #[doc = "Bit 16 - LSI Ready Interrupt Clear"]
    #[inline(always)]
    pub fn lsirdyc(&mut self) -> LsirdycW<'_, CirSpec> {
        LsirdycW::new(self, 16)
    }
    #[doc = "Bit 17 - LSE Ready Interrupt Clear"]
    #[inline(always)]
    pub fn lserdyc(&mut self) -> LserdycW<'_, CirSpec> {
        LserdycW::new(self, 17)
    }
    #[doc = "Bit 18 - HSI Ready Interrupt Clear"]
    #[inline(always)]
    pub fn hsirdyc(&mut self) -> HsirdycW<'_, CirSpec> {
        HsirdycW::new(self, 18)
    }
    #[doc = "Bit 19 - HSE Ready Interrupt Clear"]
    #[inline(always)]
    pub fn hserdyc(&mut self) -> HserdycW<'_, CirSpec> {
        HserdycW::new(self, 19)
    }
    #[doc = "Bit 20 - PLL Ready Interrupt Clear"]
    #[inline(always)]
    pub fn pllrdyc(&mut self) -> PllrdycW<'_, CirSpec> {
        PllrdycW::new(self, 20)
    }
    #[doc = "Bit 23 - Clock security system interrupt clear"]
    #[inline(always)]
    pub fn cssc(&mut self) -> CsscW<'_, CirSpec> {
        CsscW::new(self, 23)
    }
}
#[doc = "Clock interrupt register (RCC_CIR)\n\nYou can [`read`](crate::Reg::read) this register and get [`cir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CirSpec;
impl crate::RegisterSpec for CirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cir::R`](R) reader structure"]
impl crate::Readable for CirSpec {}
#[doc = "`write(|w| ..)` method takes [`cir::W`](W) writer structure"]
impl crate::Writable for CirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CIR to value 0"]
impl crate::Resettable for CirSpec {}
