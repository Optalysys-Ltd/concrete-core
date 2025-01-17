use crate::generation::{BinaryKeyDistribution, KeyDistributionMarker};
use concrete_commons::dispersion::Variance;
use concrete_commons::parameters::{LweCiphertextCount, LweDimension};
use concrete_core::prelude::{
    LweCiphertextVectorDecryptionEngine, LweCiphertextVectorEncryptionEngine,
    LweCiphertextVectorTrivialDecryptionEngine, LweCiphertextVectorTrivialEncryptionEngine,
    PlaintextVectorCreationEngine,
};

use crate::generation::prototypes::{
    LweCiphertextVectorPrototype, ProtoBinaryLweCiphertextVector32,
    ProtoBinaryLweCiphertextVector64, ProtoPlaintextVector32, ProtoPlaintextVector64,
};
use crate::generation::prototyping::lwe_secret_key::PrototypesLweSecretKey;
use crate::generation::prototyping::plaintext_vector::PrototypesPlaintextVector;
use crate::generation::{IntegerPrecision, Maker, Precision32, Precision64};

/// A trait allowing to manipulate lwe ciphertext vector prototypes.
pub trait PrototypesLweCiphertextVector<
    Precision: IntegerPrecision,
    KeyDistribution: KeyDistributionMarker,
>:
    PrototypesPlaintextVector<Precision> + PrototypesLweSecretKey<Precision, KeyDistribution>
{
    type LweCiphertextVectorProto: LweCiphertextVectorPrototype<
        Precision = Precision,
        KeyDistribution = KeyDistribution,
    >;
    fn trivially_encrypt_zeros_to_lwe_ciphertext_vector(
        &mut self,
        lwe_dimension: LweDimension,
        count: LweCiphertextCount,
    ) -> Self::LweCiphertextVectorProto;

    fn trivially_encrypt_plaintext_vector_to_lwe_ciphertext_vector(
        &mut self,
        lwe_dimension: LweDimension,
        plaintext_vector: &Self::PlaintextVectorProto,
    ) -> Self::LweCiphertextVectorProto;

    fn encrypt_plaintext_vector_to_lwe_ciphertext_vector(
        &mut self,
        secret_key: &Self::LweSecretKeyProto,
        plaintext_vector: &Self::PlaintextVectorProto,
        noise: Variance,
    ) -> Self::LweCiphertextVectorProto;

    fn decrypt_lwe_ciphertext_vector_to_plaintext_vector(
        &mut self,
        secret_key: &Self::LweSecretKeyProto,
        ciphertext_vector: &Self::LweCiphertextVectorProto,
    ) -> Self::PlaintextVectorProto;

    fn trivially_decrypt_lwe_ciphertext_vector_to_plaintext_vector(
        &mut self,
        ciphertext: &Self::LweCiphertextVectorProto,
    ) -> Self::PlaintextVectorProto;
}

impl PrototypesLweCiphertextVector<Precision32, BinaryKeyDistribution> for Maker {
    type LweCiphertextVectorProto = ProtoBinaryLweCiphertextVector32;

    fn trivially_encrypt_zeros_to_lwe_ciphertext_vector(
        &mut self,
        lwe_dimension: LweDimension,
        count: LweCiphertextCount,
    ) -> Self::LweCiphertextVectorProto {
        let plaintext_vector = self
            .default_engine
            .create_plaintext_vector_from(&vec![0u32; count.0])
            .unwrap();
        ProtoBinaryLweCiphertextVector32(
            self.default_engine
                .trivially_encrypt_lwe_ciphertext_vector(
                    lwe_dimension.to_lwe_size(),
                    &plaintext_vector,
                )
                .unwrap(),
        )
    }

    fn trivially_encrypt_plaintext_vector_to_lwe_ciphertext_vector(
        &mut self,
        lwe_dimension: LweDimension,
        plaintext_vector: &Self::PlaintextVectorProto,
    ) -> Self::LweCiphertextVectorProto {
        ProtoBinaryLweCiphertextVector32(
            self.default_engine
                .trivially_encrypt_lwe_ciphertext_vector(
                    lwe_dimension.to_lwe_size(),
                    &plaintext_vector.0,
                )
                .unwrap(),
        )
    }

    fn encrypt_plaintext_vector_to_lwe_ciphertext_vector(
        &mut self,
        secret_key: &Self::LweSecretKeyProto,
        plaintext_vector: &Self::PlaintextVectorProto,
        noise: Variance,
    ) -> Self::LweCiphertextVectorProto {
        ProtoBinaryLweCiphertextVector32(
            self.default_engine
                .encrypt_lwe_ciphertext_vector(&secret_key.0, &plaintext_vector.0, noise)
                .unwrap(),
        )
    }

    fn decrypt_lwe_ciphertext_vector_to_plaintext_vector(
        &mut self,
        secret_key: &Self::LweSecretKeyProto,
        ciphertext_vector: &Self::LweCiphertextVectorProto,
    ) -> Self::PlaintextVectorProto {
        ProtoPlaintextVector32(
            self.default_engine
                .decrypt_lwe_ciphertext_vector(&secret_key.0, &ciphertext_vector.0)
                .unwrap(),
        )
    }

    fn trivially_decrypt_lwe_ciphertext_vector_to_plaintext_vector(
        &mut self,
        ciphertext_vector: &Self::LweCiphertextVectorProto,
    ) -> Self::PlaintextVectorProto {
        ProtoPlaintextVector32(
            self.default_engine
                .trivially_decrypt_lwe_ciphertext_vector(&ciphertext_vector.0)
                .unwrap(),
        )
    }
}

impl PrototypesLweCiphertextVector<Precision64, BinaryKeyDistribution> for Maker {
    type LweCiphertextVectorProto = ProtoBinaryLweCiphertextVector64;

    fn trivially_encrypt_zeros_to_lwe_ciphertext_vector(
        &mut self,
        lwe_dimension: LweDimension,
        count: LweCiphertextCount,
    ) -> Self::LweCiphertextVectorProto {
        let plaintext_vector = self
            .default_engine
            .create_plaintext_vector_from(&vec![0u64; count.0])
            .unwrap();
        ProtoBinaryLweCiphertextVector64(
            self.default_engine
                .trivially_encrypt_lwe_ciphertext_vector(
                    lwe_dimension.to_lwe_size(),
                    &plaintext_vector,
                )
                .unwrap(),
        )
    }

    fn trivially_encrypt_plaintext_vector_to_lwe_ciphertext_vector(
        &mut self,
        lwe_dimension: LweDimension,
        plaintext_vector: &Self::PlaintextVectorProto,
    ) -> Self::LweCiphertextVectorProto {
        ProtoBinaryLweCiphertextVector64(
            self.default_engine
                .trivially_encrypt_lwe_ciphertext_vector(
                    lwe_dimension.to_lwe_size(),
                    &plaintext_vector.0,
                )
                .unwrap(),
        )
    }

    fn encrypt_plaintext_vector_to_lwe_ciphertext_vector(
        &mut self,
        secret_key: &Self::LweSecretKeyProto,
        plaintext_vector: &Self::PlaintextVectorProto,
        noise: Variance,
    ) -> Self::LweCiphertextVectorProto {
        ProtoBinaryLweCiphertextVector64(
            self.default_engine
                .encrypt_lwe_ciphertext_vector(&secret_key.0, &plaintext_vector.0, noise)
                .unwrap(),
        )
    }

    fn decrypt_lwe_ciphertext_vector_to_plaintext_vector(
        &mut self,
        secret_key: &Self::LweSecretKeyProto,
        ciphertext_vector: &Self::LweCiphertextVectorProto,
    ) -> Self::PlaintextVectorProto {
        ProtoPlaintextVector64(
            self.default_engine
                .decrypt_lwe_ciphertext_vector(&secret_key.0, &ciphertext_vector.0)
                .unwrap(),
        )
    }

    fn trivially_decrypt_lwe_ciphertext_vector_to_plaintext_vector(
        &mut self,
        ciphertext_vector: &Self::LweCiphertextVectorProto,
    ) -> Self::PlaintextVectorProto {
        ProtoPlaintextVector64(
            self.default_engine
                .trivially_decrypt_lwe_ciphertext_vector(&ciphertext_vector.0)
                .unwrap(),
        )
    }
}
