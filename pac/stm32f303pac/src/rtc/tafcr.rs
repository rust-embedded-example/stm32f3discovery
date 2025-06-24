#[doc = "Register `TAFCR` reader"]
pub type R = crate::R<TafcrSpec>;
#[doc = "Register `TAFCR` writer"]
pub type W = crate::W<TafcrSpec>;
#[doc = "Field `TAMP1E` reader - Tamper 1 detection enable"]
pub type Tamp1eR = crate::BitReader;
#[doc = "Field `TAMP1E` writer - Tamper 1 detection enable"]
pub type Tamp1eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP1TRG` reader - Active level for tamper 1"]
pub type Tamp1trgR = crate::BitReader;
#[doc = "Field `TAMP1TRG` writer - Active level for tamper 1"]
pub type Tamp1trgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPIE` reader - Tamper interrupt enable"]
pub type TampieR = crate::BitReader;
#[doc = "Field `TAMPIE` writer - Tamper interrupt enable"]
pub type TampieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP2E` reader - Tamper 2 detection enable"]
pub type Tamp2eR = crate::BitReader;
#[doc = "Field `TAMP2E` writer - Tamper 2 detection enable"]
pub type Tamp2eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP2TRG` reader - Active level for tamper 2"]
pub type Tamp2trgR = crate::BitReader;
#[doc = "Field `TAMP2TRG` writer - Active level for tamper 2"]
pub type Tamp2trgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP3E` reader - Tamper 3 detection enable"]
pub type Tamp3eR = crate::BitReader;
#[doc = "Field `TAMP3E` writer - Tamper 3 detection enable"]
pub type Tamp3eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP3TRG` reader - Active level for tamper 3"]
pub type Tamp3trgR = crate::BitReader;
#[doc = "Field `TAMP3TRG` writer - Active level for tamper 3"]
pub type Tamp3trgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPTS` reader - Activate timestamp on tamper detection event"]
pub type TamptsR = crate::BitReader;
#[doc = "Field `TAMPTS` writer - Activate timestamp on tamper detection event"]
pub type TamptsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPFREQ` reader - Tamper sampling frequency"]
pub type TampfreqR = crate::FieldReader;
#[doc = "Field `TAMPFREQ` writer - Tamper sampling frequency"]
pub type TampfreqW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TAMPFLT` reader - Tamper filter count"]
pub type TampfltR = crate::FieldReader;
#[doc = "Field `TAMPFLT` writer - Tamper filter count"]
pub type TampfltW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TAMPPRCH` reader - Tamper precharge duration"]
pub type TampprchR = crate::FieldReader;
#[doc = "Field `TAMPPRCH` writer - Tamper precharge duration"]
pub type TampprchW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TAMPPUDIS` reader - TAMPER pull-up disable"]
pub type TamppudisR = crate::BitReader;
#[doc = "Field `TAMPPUDIS` writer - TAMPER pull-up disable"]
pub type TamppudisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC13VALUE` reader - PC13 value"]
pub type Pc13valueR = crate::BitReader;
#[doc = "Field `PC13VALUE` writer - PC13 value"]
pub type Pc13valueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC13MODE` reader - PC13 mode"]
pub type Pc13modeR = crate::BitReader;
#[doc = "Field `PC13MODE` writer - PC13 mode"]
pub type Pc13modeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC14VALUE` reader - PC14 value"]
pub type Pc14valueR = crate::BitReader;
#[doc = "Field `PC14VALUE` writer - PC14 value"]
pub type Pc14valueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC14MODE` reader - PC 14 mode"]
pub type Pc14modeR = crate::BitReader;
#[doc = "Field `PC14MODE` writer - PC 14 mode"]
pub type Pc14modeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC15VALUE` reader - PC15 value"]
pub type Pc15valueR = crate::BitReader;
#[doc = "Field `PC15VALUE` writer - PC15 value"]
pub type Pc15valueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC15MODE` reader - PC15 mode"]
pub type Pc15modeR = crate::BitReader;
#[doc = "Field `PC15MODE` writer - PC15 mode"]
pub type Pc15modeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Tamper 1 detection enable"]
    #[inline(always)]
    pub fn tamp1e(&self) -> Tamp1eR {
        Tamp1eR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Active level for tamper 1"]
    #[inline(always)]
    pub fn tamp1trg(&self) -> Tamp1trgR {
        Tamp1trgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper interrupt enable"]
    #[inline(always)]
    pub fn tampie(&self) -> TampieR {
        TampieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Tamper 2 detection enable"]
    #[inline(always)]
    pub fn tamp2e(&self) -> Tamp2eR {
        Tamp2eR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Active level for tamper 2"]
    #[inline(always)]
    pub fn tamp2trg(&self) -> Tamp2trgR {
        Tamp2trgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Tamper 3 detection enable"]
    #[inline(always)]
    pub fn tamp3e(&self) -> Tamp3eR {
        Tamp3eR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Active level for tamper 3"]
    #[inline(always)]
    pub fn tamp3trg(&self) -> Tamp3trgR {
        Tamp3trgR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Activate timestamp on tamper detection event"]
    #[inline(always)]
    pub fn tampts(&self) -> TamptsR {
        TamptsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Tamper sampling frequency"]
    #[inline(always)]
    pub fn tampfreq(&self) -> TampfreqR {
        TampfreqR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12 - Tamper filter count"]
    #[inline(always)]
    pub fn tampflt(&self) -> TampfltR {
        TampfltR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - Tamper precharge duration"]
    #[inline(always)]
    pub fn tampprch(&self) -> TampprchR {
        TampprchR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - TAMPER pull-up disable"]
    #[inline(always)]
    pub fn tamppudis(&self) -> TamppudisR {
        TamppudisR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - PC13 value"]
    #[inline(always)]
    pub fn pc13value(&self) -> Pc13valueR {
        Pc13valueR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PC13 mode"]
    #[inline(always)]
    pub fn pc13mode(&self) -> Pc13modeR {
        Pc13modeR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PC14 value"]
    #[inline(always)]
    pub fn pc14value(&self) -> Pc14valueR {
        Pc14valueR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PC 14 mode"]
    #[inline(always)]
    pub fn pc14mode(&self) -> Pc14modeR {
        Pc14modeR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PC15 value"]
    #[inline(always)]
    pub fn pc15value(&self) -> Pc15valueR {
        Pc15valueR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PC15 mode"]
    #[inline(always)]
    pub fn pc15mode(&self) -> Pc15modeR {
        Pc15modeR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper 1 detection enable"]
    #[inline(always)]
    pub fn tamp1e(&mut self) -> Tamp1eW<TafcrSpec> {
        Tamp1eW::new(self, 0)
    }
    #[doc = "Bit 1 - Active level for tamper 1"]
    #[inline(always)]
    pub fn tamp1trg(&mut self) -> Tamp1trgW<TafcrSpec> {
        Tamp1trgW::new(self, 1)
    }
    #[doc = "Bit 2 - Tamper interrupt enable"]
    #[inline(always)]
    pub fn tampie(&mut self) -> TampieW<TafcrSpec> {
        TampieW::new(self, 2)
    }
    #[doc = "Bit 3 - Tamper 2 detection enable"]
    #[inline(always)]
    pub fn tamp2e(&mut self) -> Tamp2eW<TafcrSpec> {
        Tamp2eW::new(self, 3)
    }
    #[doc = "Bit 4 - Active level for tamper 2"]
    #[inline(always)]
    pub fn tamp2trg(&mut self) -> Tamp2trgW<TafcrSpec> {
        Tamp2trgW::new(self, 4)
    }
    #[doc = "Bit 5 - Tamper 3 detection enable"]
    #[inline(always)]
    pub fn tamp3e(&mut self) -> Tamp3eW<TafcrSpec> {
        Tamp3eW::new(self, 5)
    }
    #[doc = "Bit 6 - Active level for tamper 3"]
    #[inline(always)]
    pub fn tamp3trg(&mut self) -> Tamp3trgW<TafcrSpec> {
        Tamp3trgW::new(self, 6)
    }
    #[doc = "Bit 7 - Activate timestamp on tamper detection event"]
    #[inline(always)]
    pub fn tampts(&mut self) -> TamptsW<TafcrSpec> {
        TamptsW::new(self, 7)
    }
    #[doc = "Bits 8:10 - Tamper sampling frequency"]
    #[inline(always)]
    pub fn tampfreq(&mut self) -> TampfreqW<TafcrSpec> {
        TampfreqW::new(self, 8)
    }
    #[doc = "Bits 11:12 - Tamper filter count"]
    #[inline(always)]
    pub fn tampflt(&mut self) -> TampfltW<TafcrSpec> {
        TampfltW::new(self, 11)
    }
    #[doc = "Bits 13:14 - Tamper precharge duration"]
    #[inline(always)]
    pub fn tampprch(&mut self) -> TampprchW<TafcrSpec> {
        TampprchW::new(self, 13)
    }
    #[doc = "Bit 15 - TAMPER pull-up disable"]
    #[inline(always)]
    pub fn tamppudis(&mut self) -> TamppudisW<TafcrSpec> {
        TamppudisW::new(self, 15)
    }
    #[doc = "Bit 18 - PC13 value"]
    #[inline(always)]
    pub fn pc13value(&mut self) -> Pc13valueW<TafcrSpec> {
        Pc13valueW::new(self, 18)
    }
    #[doc = "Bit 19 - PC13 mode"]
    #[inline(always)]
    pub fn pc13mode(&mut self) -> Pc13modeW<TafcrSpec> {
        Pc13modeW::new(self, 19)
    }
    #[doc = "Bit 20 - PC14 value"]
    #[inline(always)]
    pub fn pc14value(&mut self) -> Pc14valueW<TafcrSpec> {
        Pc14valueW::new(self, 20)
    }
    #[doc = "Bit 21 - PC 14 mode"]
    #[inline(always)]
    pub fn pc14mode(&mut self) -> Pc14modeW<TafcrSpec> {
        Pc14modeW::new(self, 21)
    }
    #[doc = "Bit 22 - PC15 value"]
    #[inline(always)]
    pub fn pc15value(&mut self) -> Pc15valueW<TafcrSpec> {
        Pc15valueW::new(self, 22)
    }
    #[doc = "Bit 23 - PC15 mode"]
    #[inline(always)]
    pub fn pc15mode(&mut self) -> Pc15modeW<TafcrSpec> {
        Pc15modeW::new(self, 23)
    }
}
#[doc = "tamper and alternate function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tafcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tafcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TafcrSpec;
impl crate::RegisterSpec for TafcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tafcr::R`](R) reader structure"]
impl crate::Readable for TafcrSpec {}
#[doc = "`write(|w| ..)` method takes [`tafcr::W`](W) writer structure"]
impl crate::Writable for TafcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TAFCR to value 0"]
impl crate::Resettable for TafcrSpec {}
