#[doc = "Register `OFR1` reader"]
pub type R = crate::R<Ofr1Spec>;
#[doc = "Register `OFR1` writer"]
pub type W = crate::W<Ofr1Spec>;
#[doc = "Field `OFFSET1` reader - OFFSET1"]
pub type Offset1R = crate::FieldReader<u16>;
#[doc = "Field `OFFSET1` writer - OFFSET1"]
pub type Offset1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `OFFSET1_CH` reader - OFFSET1_CH"]
pub type Offset1ChR = crate::FieldReader;
#[doc = "Field `OFFSET1_CH` writer - OFFSET1_CH"]
pub type Offset1ChW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OFFSET1_EN` reader - OFFSET1_EN"]
pub type Offset1EnR = crate::BitReader;
#[doc = "Field `OFFSET1_EN` writer - OFFSET1_EN"]
pub type Offset1EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - OFFSET1"]
    #[inline(always)]
    pub fn offset1(&self) -> Offset1R {
        Offset1R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 26:30 - OFFSET1_CH"]
    #[inline(always)]
    pub fn offset1_ch(&self) -> Offset1ChR {
        Offset1ChR::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - OFFSET1_EN"]
    #[inline(always)]
    pub fn offset1_en(&self) -> Offset1EnR {
        Offset1EnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - OFFSET1"]
    #[inline(always)]
    pub fn offset1(&mut self) -> Offset1W<'_, Ofr1Spec> {
        Offset1W::new(self, 0)
    }
    #[doc = "Bits 26:30 - OFFSET1_CH"]
    #[inline(always)]
    pub fn offset1_ch(&mut self) -> Offset1ChW<'_, Ofr1Spec> {
        Offset1ChW::new(self, 26)
    }
    #[doc = "Bit 31 - OFFSET1_EN"]
    #[inline(always)]
    pub fn offset1_en(&mut self) -> Offset1EnW<'_, Ofr1Spec> {
        Offset1EnW::new(self, 31)
    }
}
#[doc = "offset register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ofr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ofr1Spec;
impl crate::RegisterSpec for Ofr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ofr1::R`](R) reader structure"]
impl crate::Readable for Ofr1Spec {}
#[doc = "`write(|w| ..)` method takes [`ofr1::W`](W) writer structure"]
impl crate::Writable for Ofr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OFR1 to value 0"]
impl crate::Resettable for Ofr1Spec {}
