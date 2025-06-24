#[doc = "Register `IOG4CR` reader"]
pub type R = crate::R<Iog4crSpec>;
#[doc = "Field `CNT` reader - Counter value"]
pub type CntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - Counter value"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "I/O group x counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`iog4cr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iog4crSpec;
impl crate::RegisterSpec for Iog4crSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iog4cr::R`](R) reader structure"]
impl crate::Readable for Iog4crSpec {}
#[doc = "`reset()` method sets IOG4CR to value 0"]
impl crate::Resettable for Iog4crSpec {}
