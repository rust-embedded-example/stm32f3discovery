#[doc = "Register `COMP7_CSR` reader"]
pub type R = crate::R<Comp7CsrSpec>;
#[doc = "Register `COMP7_CSR` writer"]
pub type W = crate::W<Comp7CsrSpec>;
#[doc = "Field `COMP7EN` reader - Comparator 7 enable"]
pub type Comp7enR = crate::BitReader;
#[doc = "Field `COMP7EN` writer - Comparator 7 enable"]
pub type Comp7enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP7MODE` reader - Comparator 7 mode"]
pub type Comp7modeR = crate::FieldReader;
#[doc = "Field `COMP7MODE` writer - Comparator 7 mode"]
pub type Comp7modeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP7INSEL` reader - Comparator 7 inverting input selection"]
pub type Comp7inselR = crate::FieldReader;
#[doc = "Field `COMP7INSEL` writer - Comparator 7 inverting input selection"]
pub type Comp7inselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMP7INPSEL` reader - Comparator 7 non inverted input selection"]
pub type Comp7inpselR = crate::BitReader;
#[doc = "Field `COMP7INPSEL` writer - Comparator 7 non inverted input selection"]
pub type Comp7inpselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP7_OUT_SEL` reader - Comparator 7 output selection"]
pub type Comp7OutSelR = crate::FieldReader;
#[doc = "Field `COMP7_OUT_SEL` writer - Comparator 7 output selection"]
pub type Comp7OutSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `COMP7POL` reader - Comparator 7 output polarity"]
pub type Comp7polR = crate::BitReader;
#[doc = "Field `COMP7POL` writer - Comparator 7 output polarity"]
pub type Comp7polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP7HYST` reader - Comparator 7 hysteresis"]
pub type Comp7hystR = crate::FieldReader;
#[doc = "Field `COMP7HYST` writer - Comparator 7 hysteresis"]
pub type Comp7hystW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP7_BLANKING` reader - Comparator 7 blanking source"]
pub type Comp7BlankingR = crate::FieldReader;
#[doc = "Field `COMP7_BLANKING` writer - Comparator 7 blanking source"]
pub type Comp7BlankingW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMP7OUT` reader - Comparator 7 output"]
pub type Comp7outR = crate::BitReader;
#[doc = "Field `COMP7LOCK` reader - Comparator 7 lock"]
pub type Comp7lockR = crate::BitReader;
#[doc = "Field `COMP7LOCK` writer - Comparator 7 lock"]
pub type Comp7lockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparator 7 enable"]
    #[inline(always)]
    pub fn comp7en(&self) -> Comp7enR {
        Comp7enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Comparator 7 mode"]
    #[inline(always)]
    pub fn comp7mode(&self) -> Comp7modeR {
        Comp7modeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 7 inverting input selection"]
    #[inline(always)]
    pub fn comp7insel(&self) -> Comp7inselR {
        Comp7inselR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Comparator 7 non inverted input selection"]
    #[inline(always)]
    pub fn comp7inpsel(&self) -> Comp7inpselR {
        Comp7inpselR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 10:13 - Comparator 7 output selection"]
    #[inline(always)]
    pub fn comp7_out_sel(&self) -> Comp7OutSelR {
        Comp7OutSelR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comparator 7 output polarity"]
    #[inline(always)]
    pub fn comp7pol(&self) -> Comp7polR {
        Comp7polR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 7 hysteresis"]
    #[inline(always)]
    pub fn comp7hyst(&self) -> Comp7hystR {
        Comp7hystR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Comparator 7 blanking source"]
    #[inline(always)]
    pub fn comp7_blanking(&self) -> Comp7BlankingR {
        Comp7BlankingR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 30 - Comparator 7 output"]
    #[inline(always)]
    pub fn comp7out(&self) -> Comp7outR {
        Comp7outR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Comparator 7 lock"]
    #[inline(always)]
    pub fn comp7lock(&self) -> Comp7lockR {
        Comp7lockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 7 enable"]
    #[inline(always)]
    pub fn comp7en(&mut self) -> Comp7enW<'_, Comp7CsrSpec> {
        Comp7enW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Comparator 7 mode"]
    #[inline(always)]
    pub fn comp7mode(&mut self) -> Comp7modeW<'_, Comp7CsrSpec> {
        Comp7modeW::new(self, 2)
    }
    #[doc = "Bits 4:6 - Comparator 7 inverting input selection"]
    #[inline(always)]
    pub fn comp7insel(&mut self) -> Comp7inselW<'_, Comp7CsrSpec> {
        Comp7inselW::new(self, 4)
    }
    #[doc = "Bit 7 - Comparator 7 non inverted input selection"]
    #[inline(always)]
    pub fn comp7inpsel(&mut self) -> Comp7inpselW<'_, Comp7CsrSpec> {
        Comp7inpselW::new(self, 7)
    }
    #[doc = "Bits 10:13 - Comparator 7 output selection"]
    #[inline(always)]
    pub fn comp7_out_sel(&mut self) -> Comp7OutSelW<'_, Comp7CsrSpec> {
        Comp7OutSelW::new(self, 10)
    }
    #[doc = "Bit 15 - Comparator 7 output polarity"]
    #[inline(always)]
    pub fn comp7pol(&mut self) -> Comp7polW<'_, Comp7CsrSpec> {
        Comp7polW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Comparator 7 hysteresis"]
    #[inline(always)]
    pub fn comp7hyst(&mut self) -> Comp7hystW<'_, Comp7CsrSpec> {
        Comp7hystW::new(self, 16)
    }
    #[doc = "Bits 18:20 - Comparator 7 blanking source"]
    #[inline(always)]
    pub fn comp7_blanking(&mut self) -> Comp7BlankingW<'_, Comp7CsrSpec> {
        Comp7BlankingW::new(self, 18)
    }
    #[doc = "Bit 31 - Comparator 7 lock"]
    #[inline(always)]
    pub fn comp7lock(&mut self) -> Comp7lockW<'_, Comp7CsrSpec> {
        Comp7lockW::new(self, 31)
    }
}
#[doc = "control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp7_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp7_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp7CsrSpec;
impl crate::RegisterSpec for Comp7CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp7_csr::R`](R) reader structure"]
impl crate::Readable for Comp7CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`comp7_csr::W`](W) writer structure"]
impl crate::Writable for Comp7CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP7_CSR to value 0"]
impl crate::Resettable for Comp7CsrSpec {}
