#[doc = "Register `ECCR3` reader"]
pub type R = crate::R<Eccr3Spec>;
#[doc = "Field `ECCx` reader - ECCx"]
pub type EccxR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ECCx"]
    #[inline(always)]
    pub fn eccx(&self) -> EccxR {
        EccxR::new(self.bits)
    }
}
#[doc = "ECC result register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`eccr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Eccr3Spec;
impl crate::RegisterSpec for Eccr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccr3::R`](R) reader structure"]
impl crate::Readable for Eccr3Spec {}
#[doc = "`reset()` method sets ECCR3 to value 0"]
impl crate::Resettable for Eccr3Spec {}
