use crate::generation::prototyping::PrototypesLweCiphertextVector;
use crate::generation::{IntegerPrecision, KeyDistributionMarker};
use concrete_core::prelude::LweCiphertextVectorEntity;

/// A trait allowing to synthesize an actual lwe ciphertext vector entity from a prototype.
pub trait SynthesizesLweCiphertextVector<
    Precision: IntegerPrecision,
    KeyDistribution: KeyDistributionMarker,
    LweCiphertextVector,
>: PrototypesLweCiphertextVector<Precision, KeyDistribution> where
    LweCiphertextVector: LweCiphertextVectorEntity,
{
    fn synthesize_lwe_ciphertext_vector(
        &mut self,
        prototype: &Self::LweCiphertextVectorProto,
    ) -> LweCiphertextVector;
    fn unsynthesize_lwe_ciphertext_vector(
        &mut self,
        entity: LweCiphertextVector,
    ) -> Self::LweCiphertextVectorProto;
    fn destroy_lwe_ciphertext_vector(&mut self, entity: LweCiphertextVector);
}

mod backend_default {
    use crate::generation::prototypes::{
        ProtoBinaryLweCiphertextVector32, ProtoBinaryLweCiphertextVector64,
    };
    use crate::generation::synthesizing::SynthesizesLweCiphertextVector;
    use crate::generation::{BinaryKeyDistribution, Maker, Precision32, Precision64};
    use concrete_core::prelude::{LweCiphertextVector32, LweCiphertextVector64};

    impl SynthesizesLweCiphertextVector<Precision32, BinaryKeyDistribution, LweCiphertextVector32>
        for Maker
    {
        fn synthesize_lwe_ciphertext_vector(
            &mut self,
            prototype: &Self::LweCiphertextVectorProto,
        ) -> LweCiphertextVector32 {
            prototype.0.to_owned()
        }

        fn unsynthesize_lwe_ciphertext_vector(
            &mut self,
            entity: LweCiphertextVector32,
        ) -> Self::LweCiphertextVectorProto {
            ProtoBinaryLweCiphertextVector32(entity)
        }

        fn destroy_lwe_ciphertext_vector(&mut self, _entity: LweCiphertextVector32) {}
    }

    impl SynthesizesLweCiphertextVector<Precision64, BinaryKeyDistribution, LweCiphertextVector64>
        for Maker
    {
        fn synthesize_lwe_ciphertext_vector(
            &mut self,
            prototype: &Self::LweCiphertextVectorProto,
        ) -> LweCiphertextVector64 {
            prototype.0.to_owned()
        }

        fn unsynthesize_lwe_ciphertext_vector(
            &mut self,
            entity: LweCiphertextVector64,
        ) -> Self::LweCiphertextVectorProto {
            ProtoBinaryLweCiphertextVector64(entity)
        }

        fn destroy_lwe_ciphertext_vector(&mut self, _entity: LweCiphertextVector64) {}
    }
}
#[cfg(all(feature = "backend_cuda", not(feature = "_ci_do_not_compile")))]
mod backend_cuda {
    use crate::generation::prototypes::{
        ProtoBinaryLweCiphertextVector32, ProtoBinaryLweCiphertextVector64,
    };
    use crate::generation::synthesizing::SynthesizesLweCiphertextVector;
    use crate::generation::{BinaryKeyDistribution, Maker, Precision32, Precision64};
    use concrete_core::prelude::{
        CudaLweCiphertextVector32, CudaLweCiphertextVector64, LweCiphertextVectorConversionEngine,
    };

    impl
        SynthesizesLweCiphertextVector<
            Precision32,
            BinaryKeyDistribution,
            CudaLweCiphertextVector32,
        > for Maker
    {
        fn synthesize_lwe_ciphertext_vector(
            &mut self,
            prototype: &Self::LweCiphertextVectorProto,
        ) -> CudaLweCiphertextVector32 {
            self.cuda_engine
                .convert_lwe_ciphertext_vector(&prototype.0)
                .unwrap()
        }
        fn unsynthesize_lwe_ciphertext_vector(
            &mut self,
            entity: CudaLweCiphertextVector32,
        ) -> Self::LweCiphertextVectorProto {
            let proto = self
                .cuda_engine
                .convert_lwe_ciphertext_vector(&entity)
                .unwrap();
            ProtoBinaryLweCiphertextVector32(proto)
        }
        fn destroy_lwe_ciphertext_vector(&mut self, _entity: CudaLweCiphertextVector32) {}
    }

    impl
        SynthesizesLweCiphertextVector<
            Precision64,
            BinaryKeyDistribution,
            CudaLweCiphertextVector64,
        > for Maker
    {
        fn synthesize_lwe_ciphertext_vector(
            &mut self,
            prototype: &Self::LweCiphertextVectorProto,
        ) -> CudaLweCiphertextVector64 {
            self.cuda_engine
                .convert_lwe_ciphertext_vector(&prototype.0)
                .unwrap()
        }
        fn unsynthesize_lwe_ciphertext_vector(
            &mut self,
            entity: CudaLweCiphertextVector64,
        ) -> Self::LweCiphertextVectorProto {
            let proto = self
                .cuda_engine
                .convert_lwe_ciphertext_vector(&entity)
                .unwrap();
            ProtoBinaryLweCiphertextVector64(proto)
        }
        fn destroy_lwe_ciphertext_vector(&mut self, _entity: CudaLweCiphertextVector64) {}
    }
}
