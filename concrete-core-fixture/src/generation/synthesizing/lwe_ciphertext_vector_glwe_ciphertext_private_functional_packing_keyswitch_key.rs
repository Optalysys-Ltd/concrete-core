use crate::generation::prototyping::PrototypesLwePrivateFunctionalPackingKeyswitchKey;
use crate::generation::{IntegerPrecision, KeyDistributionMarker};
use concrete_core::prelude::LwePrivateFunctionalPackingKeyswitchKeyEntity;

pub trait SynthesizesPrivateFunctionalPackingKeyswitchKey<
    Precision: IntegerPrecision,
    InputKeyDistribution: KeyDistributionMarker,
    OutputKeyDistribution: KeyDistributionMarker,
    LwePrivateFunctionalPackingKeyswitchKey,
>:
    PrototypesLwePrivateFunctionalPackingKeyswitchKey<
    Precision,
    InputKeyDistribution,
    OutputKeyDistribution,
> where
    LwePrivateFunctionalPackingKeyswitchKey: LwePrivateFunctionalPackingKeyswitchKeyEntity,
{
    fn synthesize_private_functional_packing_keyswitch_key(
        &mut self,
        prototype: &Self::LwePrivateFunctionalPackingKeyswitchKeyProto,
    ) -> LwePrivateFunctionalPackingKeyswitchKey;
    fn unsynthesize_private_functional_packing_keyswitch_key(
        &mut self,
        entity: LwePrivateFunctionalPackingKeyswitchKey,
    ) -> Self::LwePrivateFunctionalPackingKeyswitchKeyProto;
    fn destroy_private_functional_packing_keyswitch_key(
        &mut self,
        entity: LwePrivateFunctionalPackingKeyswitchKey,
    );
}

mod backend_default {
    use crate::generation::prototypes::{
        ProtoBinaryBinaryLwePrivateFunctionalPackingKeyswitchKey32,
        ProtoBinaryBinaryLwePrivateFunctionalPackingKeyswitchKey64,
    };
    use crate::generation::synthesizing::SynthesizesPrivateFunctionalPackingKeyswitchKey;
    use crate::generation::{BinaryKeyDistribution, Maker, Precision32, Precision64};
    use concrete_core::prelude::{
        LwePrivateFunctionalPackingKeyswitchKey32, LwePrivateFunctionalPackingKeyswitchKey64,
    };

    impl
        SynthesizesPrivateFunctionalPackingKeyswitchKey<
            Precision32,
            BinaryKeyDistribution,
            BinaryKeyDistribution,
            LwePrivateFunctionalPackingKeyswitchKey32,
        > for Maker
    {
        fn synthesize_private_functional_packing_keyswitch_key(
            &mut self,
            prototype: &Self::LwePrivateFunctionalPackingKeyswitchKeyProto,
        ) -> LwePrivateFunctionalPackingKeyswitchKey32 {
            prototype.0.to_owned()
        }

        fn unsynthesize_private_functional_packing_keyswitch_key(
            &mut self,
            entity: LwePrivateFunctionalPackingKeyswitchKey32,
        ) -> Self::LwePrivateFunctionalPackingKeyswitchKeyProto {
            ProtoBinaryBinaryLwePrivateFunctionalPackingKeyswitchKey32(entity)
        }

        fn destroy_private_functional_packing_keyswitch_key(
            &mut self,
            _entity: LwePrivateFunctionalPackingKeyswitchKey32,
        ) {
        }
    }

    impl
        SynthesizesPrivateFunctionalPackingKeyswitchKey<
            Precision64,
            BinaryKeyDistribution,
            BinaryKeyDistribution,
            LwePrivateFunctionalPackingKeyswitchKey64,
        > for Maker
    {
        fn synthesize_private_functional_packing_keyswitch_key(
            &mut self,
            prototype: &Self::LwePrivateFunctionalPackingKeyswitchKeyProto,
        ) -> LwePrivateFunctionalPackingKeyswitchKey64 {
            prototype.0.to_owned()
        }

        fn unsynthesize_private_functional_packing_keyswitch_key(
            &mut self,
            entity: LwePrivateFunctionalPackingKeyswitchKey64,
        ) -> Self::LwePrivateFunctionalPackingKeyswitchKeyProto {
            ProtoBinaryBinaryLwePrivateFunctionalPackingKeyswitchKey64(entity)
        }

        fn destroy_private_functional_packing_keyswitch_key(
            &mut self,
            _entity: LwePrivateFunctionalPackingKeyswitchKey64,
        ) {
        }
    }
}
