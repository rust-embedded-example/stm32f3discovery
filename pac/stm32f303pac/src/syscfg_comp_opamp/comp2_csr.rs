#[doc = "Register `COMP2_CSR` reader"]
pub type R = crate::R<Comp2CsrSpec>;
#[doc = "Register `COMP2_CSR` writer"]
pub type W = crate::W<Comp2CsrSpec>;
#[doc = "Field `COMP2EN` reader - Comparator 2 enable"]
pub type Comp2enR = crate::BitReader;
#[doc = "Field `COMP2EN` writer - Comparator 2 enable"]
pub type Comp2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP2MODE` reader - Comparator 2 mode"]
pub type Comp2modeR = crate::FieldReader;
#[doc = "Field `COMP2MODE` writer - Comparator 2 mode"]
pub type Comp2modeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP2INSEL` reader - Comparator 2 inverting input selection"]
pub type Comp2inselR = crate::FieldReader;
#[doc = "Field `COMP2INSEL` writer - Comparator 2 inverting input selection"]
pub type Comp2inselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMP2INPSEL` reader - Comparator 2 non inverted input selection"]
pub type Comp2inpselR = crate::BitReader;
#[doc = "Field `COMP2INPSEL` writer - Comparator 2 non inverted input selection"]
pub type Comp2inpselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP2INMSEL` reader - Comparator 1inverting input selection"]
pub type Comp2inmselR = crate::BitReader;
#[doc = "Field `COMP2INMSEL` writer - Comparator 1inverting input selection"]
pub type Comp2inmselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP2_OUT_SEL` reader - Comparator 2 output selection"]
pub type Comp2OutSelR = crate::FieldReader;
#[doc = "Field `COMP2_OUT_SEL` writer - Comparator 2 output selection"]
pub type Comp2OutSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `COMP2POL` reader - Comparator 2 output polarity"]
pub type Comp2polR = crate::BitReader;
#[doc = "Field `COMP2POL` writer - Comparator 2 output polarity"]
pub type Comp2polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP2HYST` reader - Comparator 2 hysteresis"]
pub type Comp2hystR = crate::FieldReader;
#[doc = "Field `COMP2HYST` writer - Comparator 2 hysteresis"]
pub type Comp2hystW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP2_BLANKING` reader - Comparator 2 blanking source"]
pub type Comp2BlankingR = crate::FieldReader;
#[doc = "Field `COMP2_BLANKING` writer - Comparator 2 blanking source"]
pub type Comp2BlankingW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMP2OUT` reader - Comparator 2 output"]
pub type Comp2outR = crate::BitReader;
#[doc = "Field `COMP2OUT` writer - Comparator 2 output"]
pub type Comp2outW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP2LOCK` reader - Comparator 2 lock"]
pub type Comp2lockR = crate::BitReader;
#[doc = "Field `COMP2LOCK` writer - Comparator 2 lock"]
pub type Comp2lockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparator 2 enable"]
    #[inline(always)]
    pub fn comp2en(&self) -> Comp2enR {
        Comp2enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Comparator 2 mode"]
    #[inline(always)]
    pub fn comp2mode(&self) -> Comp2modeR {
        Comp2modeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 2 inverting input selection"]
    #[inline(always)]
    pub fn comp2insel(&self) -> Comp2inselR {
        Comp2inselR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Comparator 2 non inverted input selection"]
    #[inline(always)]
    pub fn comp2inpsel(&self) -> Comp2inpselR {
        Comp2inpselR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Comparator 1inverting input selection"]
    #[inline(always)]
    pub fn comp2inmsel(&self) -> Comp2inmselR {
        Comp2inmselR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - Comparator 2 output selection"]
    #[inline(always)]
    pub fn comp2_out_sel(&self) -> Comp2OutSelR {
        Comp2OutSelR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comparator 2 output polarity"]
    #[inline(always)]
    pub fn comp2pol(&self) -> Comp2polR {
        Comp2polR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 2 hysteresis"]
    #[inline(always)]
    pub fn comp2hyst(&self) -> Comp2hystR {
        Comp2hystR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Comparator 2 blanking source"]
    #[inline(always)]
    pub fn comp2_blanking(&self) -> Comp2BlankingR {
        Comp2BlankingR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 30 - Comparator 2 output"]
    #[inline(always)]
    pub fn comp2out(&self) -> Comp2outR {
        Comp2outR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Comparator 2 lock"]
    #[inline(always)]
    pub fn comp2lock(&self) -> Comp2lockR {
        Comp2lockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 2 enable"]
    #[inline(always)]
    pub fn comp2en(&mut self) -> Comp2enW<'_, Comp2CsrSpec> {
        Comp2enW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Comparator 2 mode"]
    #[inline(always)]
    pub fn comp2mode(&mut self) -> Comp2modeW<'_, Comp2CsrSpec> {
        Comp2modeW::new(self, 2)
    }
    #[doc = "Bits 4:6 - Comparator 2 inverting input selection"]
    #[inline(always)]
    pub fn comp2insel(&mut self) -> Comp2inselW<'_, Comp2CsrSpec> {
        Comp2inselW::new(self, 4)
    }
    #[doc = "Bit 7 - Comparator 2 non inverted input selection"]
    #[inline(always)]
    pub fn comp2inpsel(&mut self) -> Comp2inpselW<'_, Comp2CsrSpec> {
        Comp2inpselW::new(self, 7)
    }
    #[doc = "Bit 9 - Comparator 1inverting input selection"]
    #[inline(always)]
    pub fn comp2inmsel(&mut self) -> Comp2inmselW<'_, Comp2CsrSpec> {
        Comp2inmselW::new(self, 9)
    }
    #[doc = "Bits 10:13 - Comparator 2 output selection"]
    #[inline(always)]
    pub fn comp2_out_sel(&mut self) -> Comp2OutSelW<'_, Comp2CsrSpec> {
        Comp2OutSelW::new(self, 10)
    }
    #[doc = "Bit 15 - Comparator 2 output polarity"]
    #[inline(always)]
    pub fn comp2pol(&mut self) -> Comp2polW<'_, Comp2CsrSpec> {
        Comp2polW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Comparator 2 hysteresis"]
    #[inline(always)]
    pub fn comp2hyst(&mut self) -> Comp2hystW<'_, Comp2CsrSpec> {
        Comp2hystW::new(self, 16)
    }
    #[doc = "Bits 18:20 - Comparator 2 blanking source"]
    #[inline(always)]
    pub fn comp2_blanking(&mut self) -> Comp2BlankingW<'_, Comp2CsrSpec> {
        Comp2BlankingW::new(self, 18)
    }
    #[doc = "Bit 30 - Comparator 2 output"]
    #[inline(always)]
    pub fn comp2out(&mut self) -> Comp2outW<'_, Comp2CsrSpec> {
        Comp2outW::new(self, 30)
    }
    #[doc = "Bit 31 - Comparator 2 lock"]
    #[inline(always)]
    pub fn comp2lock(&mut self) -> Comp2lockW<'_, Comp2CsrSpec> {
        Comp2lockW::new(self, 31)
    }
}
#[doc = "control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp2_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp2_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp2CsrSpec;
impl crate::RegisterSpec for Comp2CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp2_csr::R`](R) reader structure"]
impl crate::Readable for Comp2CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`comp2_csr::W`](W) writer structure"]
impl crate::Writable for Comp2CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP2_CSR to value 0"]
impl crate::Resettable for Comp2CsrSpec {}
