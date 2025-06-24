#[doc = "Register `COMP1_CSR` reader"]
pub type R = crate::R<Comp1CsrSpec>;
#[doc = "Register `COMP1_CSR` writer"]
pub type W = crate::W<Comp1CsrSpec>;
#[doc = "Field `COMP1EN` reader - Comparator 1 enable"]
pub type Comp1enR = crate::BitReader;
#[doc = "Field `COMP1EN` writer - Comparator 1 enable"]
pub type Comp1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1_INP_DAC` reader - COMP1_INP_DAC"]
pub type Comp1InpDacR = crate::BitReader;
#[doc = "Field `COMP1_INP_DAC` writer - COMP1_INP_DAC"]
pub type Comp1InpDacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1MODE` reader - Comparator 1 mode"]
pub type Comp1modeR = crate::FieldReader;
#[doc = "Field `COMP1MODE` writer - Comparator 1 mode"]
pub type Comp1modeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP1INSEL` reader - Comparator 1 inverting input selection"]
pub type Comp1inselR = crate::FieldReader;
#[doc = "Field `COMP1INSEL` writer - Comparator 1 inverting input selection"]
pub type Comp1inselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMP1_OUT_SEL` reader - Comparator 1 output selection"]
pub type Comp1OutSelR = crate::FieldReader;
#[doc = "Field `COMP1_OUT_SEL` writer - Comparator 1 output selection"]
pub type Comp1OutSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `COMP1POL` reader - Comparator 1 output polarity"]
pub type Comp1polR = crate::BitReader;
#[doc = "Field `COMP1POL` writer - Comparator 1 output polarity"]
pub type Comp1polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1HYST` reader - Comparator 1 hysteresis"]
pub type Comp1hystR = crate::FieldReader;
#[doc = "Field `COMP1HYST` writer - Comparator 1 hysteresis"]
pub type Comp1hystW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP1_BLANKING` reader - Comparator 1 blanking source"]
pub type Comp1BlankingR = crate::FieldReader;
#[doc = "Field `COMP1_BLANKING` writer - Comparator 1 blanking source"]
pub type Comp1BlankingW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMP1OUT` reader - Comparator 1 output"]
pub type Comp1outR = crate::BitReader;
#[doc = "Field `COMP1LOCK` reader - Comparator 1 lock"]
pub type Comp1lockR = crate::BitReader;
#[doc = "Field `COMP1LOCK` writer - Comparator 1 lock"]
pub type Comp1lockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparator 1 enable"]
    #[inline(always)]
    pub fn comp1en(&self) -> Comp1enR {
        Comp1enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - COMP1_INP_DAC"]
    #[inline(always)]
    pub fn comp1_inp_dac(&self) -> Comp1InpDacR {
        Comp1InpDacR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Comparator 1 mode"]
    #[inline(always)]
    pub fn comp1mode(&self) -> Comp1modeR {
        Comp1modeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 1 inverting input selection"]
    #[inline(always)]
    pub fn comp1insel(&self) -> Comp1inselR {
        Comp1inselR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 10:13 - Comparator 1 output selection"]
    #[inline(always)]
    pub fn comp1_out_sel(&self) -> Comp1OutSelR {
        Comp1OutSelR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comparator 1 output polarity"]
    #[inline(always)]
    pub fn comp1pol(&self) -> Comp1polR {
        Comp1polR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 1 hysteresis"]
    #[inline(always)]
    pub fn comp1hyst(&self) -> Comp1hystR {
        Comp1hystR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Comparator 1 blanking source"]
    #[inline(always)]
    pub fn comp1_blanking(&self) -> Comp1BlankingR {
        Comp1BlankingR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 30 - Comparator 1 output"]
    #[inline(always)]
    pub fn comp1out(&self) -> Comp1outR {
        Comp1outR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Comparator 1 lock"]
    #[inline(always)]
    pub fn comp1lock(&self) -> Comp1lockR {
        Comp1lockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 1 enable"]
    #[inline(always)]
    pub fn comp1en(&mut self) -> Comp1enW<'_, Comp1CsrSpec> {
        Comp1enW::new(self, 0)
    }
    #[doc = "Bit 1 - COMP1_INP_DAC"]
    #[inline(always)]
    pub fn comp1_inp_dac(&mut self) -> Comp1InpDacW<'_, Comp1CsrSpec> {
        Comp1InpDacW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Comparator 1 mode"]
    #[inline(always)]
    pub fn comp1mode(&mut self) -> Comp1modeW<'_, Comp1CsrSpec> {
        Comp1modeW::new(self, 2)
    }
    #[doc = "Bits 4:6 - Comparator 1 inverting input selection"]
    #[inline(always)]
    pub fn comp1insel(&mut self) -> Comp1inselW<'_, Comp1CsrSpec> {
        Comp1inselW::new(self, 4)
    }
    #[doc = "Bits 10:13 - Comparator 1 output selection"]
    #[inline(always)]
    pub fn comp1_out_sel(&mut self) -> Comp1OutSelW<'_, Comp1CsrSpec> {
        Comp1OutSelW::new(self, 10)
    }
    #[doc = "Bit 15 - Comparator 1 output polarity"]
    #[inline(always)]
    pub fn comp1pol(&mut self) -> Comp1polW<'_, Comp1CsrSpec> {
        Comp1polW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Comparator 1 hysteresis"]
    #[inline(always)]
    pub fn comp1hyst(&mut self) -> Comp1hystW<'_, Comp1CsrSpec> {
        Comp1hystW::new(self, 16)
    }
    #[doc = "Bits 18:20 - Comparator 1 blanking source"]
    #[inline(always)]
    pub fn comp1_blanking(&mut self) -> Comp1BlankingW<'_, Comp1CsrSpec> {
        Comp1BlankingW::new(self, 18)
    }
    #[doc = "Bit 31 - Comparator 1 lock"]
    #[inline(always)]
    pub fn comp1lock(&mut self) -> Comp1lockW<'_, Comp1CsrSpec> {
        Comp1lockW::new(self, 31)
    }
}
#[doc = "control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp1_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp1_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp1CsrSpec;
impl crate::RegisterSpec for Comp1CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp1_csr::R`](R) reader structure"]
impl crate::Readable for Comp1CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`comp1_csr::W`](W) writer structure"]
impl crate::Writable for Comp1CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP1_CSR to value 0"]
impl crate::Resettable for Comp1CsrSpec {}
