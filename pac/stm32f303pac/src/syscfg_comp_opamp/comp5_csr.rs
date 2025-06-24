#[doc = "Register `COMP5_CSR` reader"]
pub type R = crate::R<Comp5CsrSpec>;
#[doc = "Register `COMP5_CSR` writer"]
pub type W = crate::W<Comp5CsrSpec>;
#[doc = "Field `COMP5EN` reader - Comparator 5 enable"]
pub type Comp5enR = crate::BitReader;
#[doc = "Field `COMP5EN` writer - Comparator 5 enable"]
pub type Comp5enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP5MODE` reader - Comparator 5 mode"]
pub type Comp5modeR = crate::FieldReader;
#[doc = "Field `COMP5MODE` writer - Comparator 5 mode"]
pub type Comp5modeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP5INSEL` reader - Comparator 5 inverting input selection"]
pub type Comp5inselR = crate::FieldReader;
#[doc = "Field `COMP5INSEL` writer - Comparator 5 inverting input selection"]
pub type Comp5inselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMP5INPSEL` reader - Comparator 5 non inverted input selection"]
pub type Comp5inpselR = crate::BitReader;
#[doc = "Field `COMP5INPSEL` writer - Comparator 5 non inverted input selection"]
pub type Comp5inpselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP5_OUT_SEL` reader - Comparator 5 output selection"]
pub type Comp5OutSelR = crate::FieldReader;
#[doc = "Field `COMP5_OUT_SEL` writer - Comparator 5 output selection"]
pub type Comp5OutSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `COMP5POL` reader - Comparator 5 output polarity"]
pub type Comp5polR = crate::BitReader;
#[doc = "Field `COMP5POL` writer - Comparator 5 output polarity"]
pub type Comp5polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP5HYST` reader - Comparator 5 hysteresis"]
pub type Comp5hystR = crate::FieldReader;
#[doc = "Field `COMP5HYST` writer - Comparator 5 hysteresis"]
pub type Comp5hystW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP5_BLANKING` reader - Comparator 5 blanking source"]
pub type Comp5BlankingR = crate::FieldReader;
#[doc = "Field `COMP5_BLANKING` writer - Comparator 5 blanking source"]
pub type Comp5BlankingW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMP5OUT` reader - Comparator51 output"]
pub type Comp5outR = crate::BitReader;
#[doc = "Field `COMP5LOCK` reader - Comparator 5 lock"]
pub type Comp5lockR = crate::BitReader;
#[doc = "Field `COMP5LOCK` writer - Comparator 5 lock"]
pub type Comp5lockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparator 5 enable"]
    #[inline(always)]
    pub fn comp5en(&self) -> Comp5enR {
        Comp5enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Comparator 5 mode"]
    #[inline(always)]
    pub fn comp5mode(&self) -> Comp5modeR {
        Comp5modeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 5 inverting input selection"]
    #[inline(always)]
    pub fn comp5insel(&self) -> Comp5inselR {
        Comp5inselR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Comparator 5 non inverted input selection"]
    #[inline(always)]
    pub fn comp5inpsel(&self) -> Comp5inpselR {
        Comp5inpselR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 10:13 - Comparator 5 output selection"]
    #[inline(always)]
    pub fn comp5_out_sel(&self) -> Comp5OutSelR {
        Comp5OutSelR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comparator 5 output polarity"]
    #[inline(always)]
    pub fn comp5pol(&self) -> Comp5polR {
        Comp5polR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 5 hysteresis"]
    #[inline(always)]
    pub fn comp5hyst(&self) -> Comp5hystR {
        Comp5hystR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Comparator 5 blanking source"]
    #[inline(always)]
    pub fn comp5_blanking(&self) -> Comp5BlankingR {
        Comp5BlankingR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 30 - Comparator51 output"]
    #[inline(always)]
    pub fn comp5out(&self) -> Comp5outR {
        Comp5outR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Comparator 5 lock"]
    #[inline(always)]
    pub fn comp5lock(&self) -> Comp5lockR {
        Comp5lockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 5 enable"]
    #[inline(always)]
    pub fn comp5en(&mut self) -> Comp5enW<'_, Comp5CsrSpec> {
        Comp5enW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Comparator 5 mode"]
    #[inline(always)]
    pub fn comp5mode(&mut self) -> Comp5modeW<'_, Comp5CsrSpec> {
        Comp5modeW::new(self, 2)
    }
    #[doc = "Bits 4:6 - Comparator 5 inverting input selection"]
    #[inline(always)]
    pub fn comp5insel(&mut self) -> Comp5inselW<'_, Comp5CsrSpec> {
        Comp5inselW::new(self, 4)
    }
    #[doc = "Bit 7 - Comparator 5 non inverted input selection"]
    #[inline(always)]
    pub fn comp5inpsel(&mut self) -> Comp5inpselW<'_, Comp5CsrSpec> {
        Comp5inpselW::new(self, 7)
    }
    #[doc = "Bits 10:13 - Comparator 5 output selection"]
    #[inline(always)]
    pub fn comp5_out_sel(&mut self) -> Comp5OutSelW<'_, Comp5CsrSpec> {
        Comp5OutSelW::new(self, 10)
    }
    #[doc = "Bit 15 - Comparator 5 output polarity"]
    #[inline(always)]
    pub fn comp5pol(&mut self) -> Comp5polW<'_, Comp5CsrSpec> {
        Comp5polW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Comparator 5 hysteresis"]
    #[inline(always)]
    pub fn comp5hyst(&mut self) -> Comp5hystW<'_, Comp5CsrSpec> {
        Comp5hystW::new(self, 16)
    }
    #[doc = "Bits 18:20 - Comparator 5 blanking source"]
    #[inline(always)]
    pub fn comp5_blanking(&mut self) -> Comp5BlankingW<'_, Comp5CsrSpec> {
        Comp5BlankingW::new(self, 18)
    }
    #[doc = "Bit 31 - Comparator 5 lock"]
    #[inline(always)]
    pub fn comp5lock(&mut self) -> Comp5lockW<'_, Comp5CsrSpec> {
        Comp5lockW::new(self, 31)
    }
}
#[doc = "control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp5_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp5_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp5CsrSpec;
impl crate::RegisterSpec for Comp5CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp5_csr::R`](R) reader structure"]
impl crate::Readable for Comp5CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`comp5_csr::W`](W) writer structure"]
impl crate::Writable for Comp5CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP5_CSR to value 0"]
impl crate::Resettable for Comp5CsrSpec {}
