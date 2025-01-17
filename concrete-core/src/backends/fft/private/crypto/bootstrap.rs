use super::super::math::fft::FftView;
use super::super::{c64, izip, Container, IntoChunks};
use super::ggsw::{cmux, *};
use super::glwe::{GlweCiphertextMutView, GlweCiphertextView};
use crate::commons::math::torus::UnsignedTorus;
use aligned_vec::CACHELINE_ALIGN;
use concrete_commons::numeric::CastInto;
use concrete_commons::parameters::{
    DecompositionBaseLog, DecompositionLevelCount, GlweSize, LutCountLog, LweDimension,
    ModulusSwitchOffset, PolynomialSize,
};
use dyn_stack::{DynStack, ReborrowMut, SizeOverflow, StackReq};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "backend_fft_serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct StandardLweBootstrapKey<C> {
    data: C,
    key_size: LweDimension,
    polynomial_size: PolynomialSize,
    glwe_size: GlweSize,
    decomposition_base_log: DecompositionBaseLog,
    decomposition_level_count: DecompositionLevelCount,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FourierLweBootstrapKey<C> {
    data: C,
    key_size: LweDimension,
    polynomial_size: PolynomialSize,
    glwe_size: GlweSize,
    decomposition_base_log: DecompositionBaseLog,
    decomposition_level_count: DecompositionLevelCount,
}

pub type StandardLweBootstrapKeyView<'a, Scalar> = StandardLweBootstrapKey<&'a [Scalar]>;
pub type StandardLweBootstrapKeyMutView<'a, Scalar> = StandardLweBootstrapKey<&'a mut [Scalar]>;
pub type FourierLweBootstrapKeyView<'a> = FourierLweBootstrapKey<&'a [c64]>;
pub type FourierLweBootstrapKeyMutView<'a> = FourierLweBootstrapKey<&'a mut [c64]>;

impl<C> StandardLweBootstrapKey<C> {
    pub fn new(
        data: C,
        key_size: LweDimension,
        polynomial_size: PolynomialSize,
        glwe_size: GlweSize,
        decomposition_base_log: DecompositionBaseLog,
        decomposition_level_count: DecompositionLevelCount,
    ) -> Self
    where
        C: Container,
    {
        assert_eq!(
            data.container_len(),
            key_size.0
                * polynomial_size.0
                * decomposition_level_count.0
                * glwe_size.0
                * glwe_size.0
        );
        Self {
            data,
            key_size,
            polynomial_size,
            glwe_size,
            decomposition_base_log,
            decomposition_level_count,
        }
    }

    /// Returns an iterator over the GGSW ciphertexts composing the key.
    pub fn into_ggsw_iter(self) -> impl DoubleEndedIterator<Item = StandardGgswCiphertext<C>>
    where
        C: IntoChunks + Container,
    {
        self.data.split_into(self.key_size.0).map(move |slice| {
            StandardGgswCiphertext::new(
                slice,
                self.polynomial_size,
                self.glwe_size,
                self.decomposition_base_log,
                self.decomposition_level_count,
            )
        })
    }

    pub fn key_size(&self) -> LweDimension {
        self.key_size
    }

    pub fn polynomial_size(&self) -> PolynomialSize {
        self.polynomial_size
    }

    pub fn glwe_size(&self) -> GlweSize {
        self.glwe_size
    }

    pub fn decomposition_base_log(&self) -> DecompositionBaseLog {
        self.decomposition_base_log
    }

    pub fn decomposition_level_count(&self) -> DecompositionLevelCount {
        self.decomposition_level_count
    }

    pub fn data(self) -> C {
        self.data
    }

    pub fn as_view<Scalar>(&self) -> StandardLweBootstrapKeyView<'_, Scalar>
    where
        C: AsRef<[Scalar]>,
    {
        StandardLweBootstrapKeyView {
            data: self.data.as_ref(),
            key_size: self.key_size,
            polynomial_size: self.polynomial_size,
            glwe_size: self.glwe_size,
            decomposition_base_log: self.decomposition_base_log,
            decomposition_level_count: self.decomposition_level_count,
        }
    }

    pub fn as_mut_view<Scalar>(&mut self) -> StandardLweBootstrapKeyMutView<'_, Scalar>
    where
        C: AsMut<[Scalar]>,
    {
        StandardLweBootstrapKeyMutView {
            data: self.data.as_mut(),
            key_size: self.key_size,
            polynomial_size: self.polynomial_size,
            glwe_size: self.glwe_size,
            decomposition_base_log: self.decomposition_base_log,
            decomposition_level_count: self.decomposition_level_count,
        }
    }
}

impl<C> FourierLweBootstrapKey<C> {
    pub fn new(
        data: C,
        key_size: LweDimension,
        polynomial_size: PolynomialSize,
        glwe_size: GlweSize,
        decomposition_base_log: DecompositionBaseLog,
        decomposition_level_count: DecompositionLevelCount,
    ) -> Self
    where
        C: Container,
    {
        assert_eq!(polynomial_size.0 % 2, 0);
        assert_eq!(
            data.container_len(),
            key_size.0 * polynomial_size.0 / 2
                * decomposition_level_count.0
                * glwe_size.0
                * glwe_size.0
        );
        Self {
            data,
            key_size,
            polynomial_size,
            glwe_size,
            decomposition_base_log,
            decomposition_level_count,
        }
    }

    /// Returns an iterator over the GGSW ciphertexts composing the key.
    pub fn into_ggsw_iter(self) -> impl DoubleEndedIterator<Item = FourierGgswCiphertext<C>>
    where
        C: IntoChunks + Container,
    {
        self.data.split_into(self.key_size.0).map(move |slice| {
            FourierGgswCiphertext::new(
                slice,
                self.polynomial_size,
                self.glwe_size,
                self.decomposition_base_log,
                self.decomposition_level_count,
            )
        })
    }

    pub fn key_size(&self) -> LweDimension {
        self.key_size
    }

    pub fn polynomial_size(&self) -> PolynomialSize {
        self.polynomial_size
    }

    pub fn glwe_size(&self) -> GlweSize {
        self.glwe_size
    }

    pub fn decomposition_base_log(&self) -> DecompositionBaseLog {
        self.decomposition_base_log
    }

    pub fn decomposition_level_count(&self) -> DecompositionLevelCount {
        self.decomposition_level_count
    }

    pub fn data(self) -> C {
        self.data
    }

    pub fn as_view(&self) -> FourierLweBootstrapKeyView<'_>
    where
        C: AsRef<[c64]>,
    {
        FourierLweBootstrapKeyView {
            data: self.data.as_ref(),
            key_size: self.key_size,
            polynomial_size: self.polynomial_size,
            glwe_size: self.glwe_size,
            decomposition_base_log: self.decomposition_base_log,
            decomposition_level_count: self.decomposition_level_count,
        }
    }

    pub fn as_mut_view(&mut self) -> FourierLweBootstrapKeyMutView<'_>
    where
        C: AsMut<[c64]>,
    {
        FourierLweBootstrapKeyMutView {
            data: self.data.as_mut(),
            key_size: self.key_size,
            polynomial_size: self.polynomial_size,
            glwe_size: self.glwe_size,
            decomposition_base_log: self.decomposition_base_log,
            decomposition_level_count: self.decomposition_level_count,
        }
    }
}

/// Returns the required memory for [`FourierLweBootstrapKeyMutView::fill_with_forward_fourier`].
pub fn fill_with_forward_fourier_scratch(fft: FftView<'_>) -> Result<StackReq, SizeOverflow> {
    fft.forward_scratch()
}

impl<'a> FourierLweBootstrapKeyMutView<'a> {
    /// Fills a bootstrapping key with the Fourier transform of a bootstrapping key in the standard
    /// domain.
    pub fn fill_with_forward_fourier<Scalar: UnsignedTorus + CastInto<usize>>(
        mut self,
        coef_bsk: StandardLweBootstrapKeyView<Scalar>,
        fft: FftView<'_>,
        mut stack: DynStack<'_>,
    ) {
        for (fourier_ggsw, standard_ggsw) in izip!(
            self.as_mut_view().into_ggsw_iter(),
            coef_bsk.into_ggsw_iter()
        ) {
            fourier_ggsw.fill_with_forward_fourier(standard_ggsw, fft, stack.rb_mut());
        }
    }
}

/// Returns the required memory for [`FourierLweBootstrapKeyView::blind_rotate`].
pub fn blind_rotate_scratch<Scalar>(
    glwe_size: GlweSize,
    polynomial_size: PolynomialSize,
    fft: FftView<'_>,
) -> Result<StackReq, SizeOverflow> {
    StackReq::try_new_aligned::<Scalar>(glwe_size.0 * polynomial_size.0, CACHELINE_ALIGN)?
        .try_and(cmux_scratch::<Scalar>(glwe_size, polynomial_size, fft)?)
}

/// Returns the required memory for [`FourierLweBootstrapKeyView::bootstrap`].
pub fn bootstrap_scratch<Scalar>(
    glwe_size: GlweSize,
    polynomial_size: PolynomialSize,
    fft: FftView<'_>,
) -> Result<StackReq, SizeOverflow> {
    blind_rotate_scratch::<Scalar>(glwe_size, polynomial_size, fft)?.try_and(
        StackReq::try_new_aligned::<Scalar>(glwe_size.0 * polynomial_size.0, CACHELINE_ALIGN)?,
    )
}

impl<'a> FourierLweBootstrapKeyView<'a> {
    pub fn blind_rotate<Scalar: UnsignedTorus + CastInto<usize>>(
        self,
        mut lut: GlweCiphertextMutView<'_, Scalar>,
        lwe: &[Scalar],
        fft: FftView<'_>,
        mut stack: DynStack<'_>,
    ) {
        let (lwe_body, lwe_mask) = lwe.split_last().unwrap();

        let lut_poly_size = lut.polynomial_size();
        let monomial_degree = pbs_modulus_switch(
            *lwe_body,
            lut_poly_size,
            ModulusSwitchOffset(0),
            LutCountLog(0),
        );
        lut.as_mut_view().into_polynomials().for_each(|poly| {
            poly.update_with_wrapping_unit_monomial_div(monomial_degree);
        });

        // We initialize the ct_0 used for the successive cmuxes
        let mut ct0 = lut;

        for (lwe_mask_element, bootstrap_key_ggsw) in izip!(lwe_mask.iter(), self.into_ggsw_iter())
        {
            if *lwe_mask_element != Scalar::ZERO {
                let stack = stack.rb_mut();
                // We copy ct_0 to ct_1
                let (mut ct1, stack) =
                    stack.collect_aligned(CACHELINE_ALIGN, ct0.as_view().data().iter().copied());

                let mut ct1 =
                    GlweCiphertextMutView::new(&mut ct1, ct0.polynomial_size(), ct0.glwe_size());

                // We rotate ct_1 by performing ct_1 <- ct_1 * X^{a_hat}
                for poly in ct1.as_mut_view().into_polynomials() {
                    poly.update_with_wrapping_unit_monomial_mul(pbs_modulus_switch(
                        *lwe_mask_element,
                        lut_poly_size,
                        ModulusSwitchOffset(0),
                        LutCountLog(0),
                    ));
                }

                cmux(ct0.as_mut_view(), ct1, bootstrap_key_ggsw, fft, stack);
            }
        }
    }

    pub fn bootstrap<'out, Scalar: UnsignedTorus + CastInto<usize>>(
        self,
        lwe_out: &'out mut [Scalar],
        lwe_in: &[Scalar],
        accumulator: GlweCiphertextView<'_, Scalar>,
        fft: FftView<'_>,
        stack: DynStack<'_>,
    ) {
        let (mut local_accumulator_data, stack) =
            stack.collect_aligned(CACHELINE_ALIGN, accumulator.data().iter().copied());
        let mut local_accumulator = GlweCiphertextMutView::new(
            &mut local_accumulator_data,
            accumulator.polynomial_size(),
            accumulator.glwe_size(),
        );
        self.blind_rotate(local_accumulator.as_mut_view(), lwe_in, fft, stack);
        local_accumulator
            .as_view()
            .fill_lwe_with_sample_extraction(lwe_out, 0);
    }
}

/// This function switches modulus for a single coefficient of a ciphertext,
/// only in the context of a PBS
///
/// offset: the number of msb discarded
/// lut_count_log: the right padding
pub fn pbs_modulus_switch<Scalar: UnsignedTorus + CastInto<usize>>(
    input: Scalar,
    poly_size: PolynomialSize,
    offset: ModulusSwitchOffset,
    lut_count_log: LutCountLog,
) -> usize {
    // First, do the left shift (we discard the offset msb)
    let mut output = input << offset.0;
    // Start doing the right shift
    output >>= Scalar::BITS - poly_size.log2().0 - 2 + lut_count_log.0;
    // Do the rounding
    output += output & Scalar::ONE;
    // Finish the right shift
    output >>= 1;
    // Apply the lsb padding
    output <<= lut_count_log.0;
    <Scalar as CastInto<usize>>::cast_into(output)
}
