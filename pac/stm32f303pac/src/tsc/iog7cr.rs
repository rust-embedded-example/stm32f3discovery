#[doc = "Register `IOG7CR` reader"]
pub type R = crate::R<Iog7crSpec>;
#[doc = "Field `CNT` reader - Counter value"]
pub type CntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - Counter value"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "I/O group x counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`iog7cr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iog7crSpec;
impl crate::RegisterSpec for Iog7crSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iog7cr::R`](R) reader structure"]
impl crate::Readable for Iog7crSpec {}
#[doc = "`reset()` method sets IOG7CR to value 0"]
impl crate::Resettable for Iog7crSpec {}
