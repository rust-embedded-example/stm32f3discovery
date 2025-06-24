#[doc = "Register `DIFSEL` reader"]
pub type R = crate::R<DifselSpec>;
#[doc = "Register `DIFSEL` writer"]
pub type W = crate::W<DifselSpec>;
#[doc = "Field `DIFSEL_1_15` reader - Differential mode for channels 15 to 1"]
pub type Difsel1_15R = crate::FieldReader<u16>;
#[doc = "Field `DIFSEL_1_15` writer - Differential mode for channels 15 to 1"]
pub type Difsel1_15W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `DIFSEL_16_18` reader - Differential mode for channels 18 to 16"]
pub type Difsel16_18R = crate::FieldReader;
impl R {
    #[doc = "Bits 1:15 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_1_15(&self) -> Difsel1_15R {
        Difsel1_15R::new(((self.bits >> 1) & 0x7fff) as u16)
    }
    #[doc = "Bits 16:18 - Differential mode for channels 18 to 16"]
    #[inline(always)]
    pub fn difsel_16_18(&self) -> Difsel16_18R {
        Difsel16_18R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 1:15 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_1_15(&mut self) -> Difsel1_15W<'_, DifselSpec> {
        Difsel1_15W::new(self, 1)
    }
}
#[doc = "Differential Mode Selection Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`difsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`difsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DifselSpec;
impl crate::RegisterSpec for DifselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`difsel::R`](R) reader structure"]
impl crate::Readable for DifselSpec {}
#[doc = "`write(|w| ..)` method takes [`difsel::W`](W) writer structure"]
impl crate::Writable for DifselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIFSEL to value 0"]
impl crate::Resettable for DifselSpec {}
