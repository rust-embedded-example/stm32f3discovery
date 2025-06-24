#[doc = "Register `DR` reader"]
pub type R = crate::R<DrSpec>;
#[doc = "Field `regularDATA` reader - regularDATA"]
pub type RegularDataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - regularDATA"]
    #[inline(always)]
    pub fn regular_data(&self) -> RegularDataR {
        RegularDataR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "regular Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DrSpec;
impl crate::RegisterSpec for DrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DrSpec {}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::Resettable for DrSpec {}
