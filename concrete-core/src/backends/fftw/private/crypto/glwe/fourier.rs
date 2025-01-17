use crate::backends::fftw::private::crypto::bootstrap::FourierBuffers;
use crate::backends::fftw::private::math::fft::{Complex64, FourierPolynomial};
use crate::commons::crypto::glwe::GlweCiphertext;
use crate::commons::math::tensor::{
    ck_dim_div, AsMutSlice, AsMutTensor, AsRefSlice, AsRefTensor, IntoTensor, Tensor,
};
use crate::commons::math::torus::UnsignedTorus;
use concrete_commons::parameters::{GlweSize, PolynomialSize};
use concrete_fftw::array::AlignedVec;
#[cfg(feature = "backend_fftw_serialization")]
use serde::{Deserialize, Serialize};

/// A GLWE ciphertext in the Fourier Domain.
#[cfg_attr(feature = "backend_fftw_serialization", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FourierGlweCiphertext<Cont, Scalar> {
    tensor: Tensor<Cont>,
    pub poly_size: PolynomialSize,
    pub glwe_size: GlweSize,
    _scalar: std::marker::PhantomData<Scalar>,
}

impl<Scalar> FourierGlweCiphertext<AlignedVec<Complex64>, Scalar> {
    /// Allocates a new GLWE ciphertext in the Fourier domain whose coefficients are all `value`.
    ///
    /// # Example
    ///
    /// ```
    /// use concrete_commons::parameters::{GlweSize, PolynomialSize};
    /// use concrete_core::backends::fftw::private::crypto::glwe::FourierGlweCiphertext;
    /// use concrete_core::backends::fftw::private::math::fft::Complex64;
    /// let glwe: FourierGlweCiphertext<_, u32> =
    ///     FourierGlweCiphertext::allocate(Complex64::new(0., 0.), PolynomialSize(10), GlweSize(7));
    /// assert_eq!(glwe.glwe_size(), GlweSize(7));
    /// assert_eq!(glwe.polynomial_size(), PolynomialSize(10));
    /// ```
    pub fn allocate(value: Complex64, poly_size: PolynomialSize, glwe_size: GlweSize) -> Self
    where
        Scalar: Copy,
    {
        let mut tensor = Tensor::from_container(AlignedVec::new(glwe_size.0 * poly_size.0));
        tensor.as_mut_tensor().fill_with_element(value);
        FourierGlweCiphertext {
            tensor,
            poly_size,
            glwe_size,
            _scalar: Default::default(),
        }
    }
}

impl<Cont, Scalar: UnsignedTorus> FourierGlweCiphertext<Cont, Scalar> {
    /// Creates a GLWE ciphertext in the Fourier domain from an existing container.
    ///
    /// # Example
    ///
    /// ```
    /// use concrete_commons::parameters::{GlweSize, PolynomialSize};
    /// use concrete_core::backends::fftw::private::crypto::glwe::FourierGlweCiphertext;
    /// use concrete_core::backends::fftw::private::math::fft::Complex64;
    ///
    /// let glwe: FourierGlweCiphertext<_, u32> = FourierGlweCiphertext::from_container(
    ///     vec![Complex64::new(0., 0.); 7 * 10],
    ///     GlweSize(7),
    ///     PolynomialSize(10),
    /// );
    /// assert_eq!(glwe.glwe_size(), GlweSize(7));
    /// assert_eq!(glwe.polynomial_size(), PolynomialSize(10));
    /// ```
    pub fn from_container(cont: Cont, glwe_size: GlweSize, poly_size: PolynomialSize) -> Self
    where
        Cont: AsRefSlice,
    {
        let tensor = Tensor::from_container(cont);
        ck_dim_div!(tensor.len() => glwe_size.0, poly_size.0);
        FourierGlweCiphertext {
            tensor,
            poly_size,
            glwe_size,
            _scalar: Default::default(),
        }
    }

    /// Returns the size of the GLWE ciphertext
    ///
    /// # Example
    ///
    /// ```
    /// use concrete_commons::parameters::{GlweSize, PolynomialSize};
    /// use concrete_core::backends::fftw::private::crypto::glwe::FourierGlweCiphertext;
    /// use concrete_core::backends::fftw::private::math::fft::Complex64;
    ///
    /// let glwe: FourierGlweCiphertext<_, u32> =
    ///     FourierGlweCiphertext::allocate(Complex64::new(0., 0.), PolynomialSize(10), GlweSize(7));
    /// assert_eq!(glwe.glwe_size(), GlweSize(7));
    /// ```
    pub fn glwe_size(&self) -> GlweSize {
        self.glwe_size
    }

    /// Returns the size of the polynomials used in the ciphertext.
    ///
    /// # Example
    ///
    /// ```
    /// use concrete_commons::parameters::{GlweSize, PolynomialSize};
    /// use concrete_core::backends::fftw::private::crypto::glwe::FourierGlweCiphertext;
    /// use concrete_core::backends::fftw::private::math::fft::Complex64;
    ///
    /// let glwe: FourierGlweCiphertext<_, u32> =
    ///     FourierGlweCiphertext::allocate(Complex64::new(0., 0.), PolynomialSize(10), GlweSize(7));
    /// assert_eq!(glwe.polynomial_size(), PolynomialSize(10));
    /// ```
    pub fn polynomial_size(&self) -> PolynomialSize {
        self.poly_size
    }

    /// Fills a Fourier GLWE ciphertext with the Fourier transform of a GLWE ciphertext in
    /// coefficient domain.
    ///
    /// ```
    /// use concrete_commons::parameters::{GlweSize, PolynomialSize};
    /// use concrete_core::backends::fftw::private::crypto::bootstrap::FourierBuffers;
    /// use concrete_core::backends::fftw::private::crypto::glwe::FourierGlweCiphertext;
    /// use concrete_core::backends::fftw::private::math::fft::Complex64;
    /// use concrete_core::commons::crypto::glwe::GlweCiphertext;
    /// let mut fourier_glwe: FourierGlweCiphertext<_, u32> =
    ///     FourierGlweCiphertext::allocate(Complex64::new(0., 0.), PolynomialSize(128), GlweSize(7));
    ///
    /// let mut buffers = FourierBuffers::new(fourier_glwe.poly_size, fourier_glwe.glwe_size);
    ///
    /// let glwe = GlweCiphertext::allocate(0 as u32, PolynomialSize(128), GlweSize(7));
    ///
    /// fourier_glwe.fill_with_forward_fourier(&glwe, &mut buffers)
    /// ```
    pub fn fill_with_forward_fourier<InputCont>(
        &mut self,
        glwe: &GlweCiphertext<InputCont>,
        buffers: &mut FourierBuffers<Scalar>,
    ) where
        Cont: AsMutSlice<Element = Complex64>,
        GlweCiphertext<InputCont>: AsRefTensor<Element = Scalar>,
        Scalar: UnsignedTorus,
    {
        // We retrieve a buffer for the fft.
        let fft_buffer = &mut buffers.fft_buffers.first_buffer;
        let fft = &mut buffers.fft_buffers.fft;

        // We move every polynomial to the fourier domain.
        let poly_list = glwe.as_polynomial_list();
        let iterator = self.polynomial_iter_mut().zip(poly_list.polynomial_iter());
        for (mut fourier_poly, coef_poly) in iterator {
            fft.forward_as_torus(fft_buffer, &coef_poly);
            fourier_poly
                .as_mut_tensor()
                .fill_with_one((fft_buffer).as_tensor(), |a| *a);
        }
    }

    /// Fills a GLWE ciphertext with the inverse fourier transform of a Fourier GLWE ciphertext
    /// ```
    /// use concrete_commons::parameters::{GlweSize, PolynomialSize};
    /// use concrete_core::backends::fftw::private::crypto::bootstrap::FourierBuffers;
    /// use concrete_core::backends::fftw::private::crypto::glwe::FourierGlweCiphertext;
    /// use concrete_core::backends::fftw::private::math::fft::Complex64;
    /// use concrete_core::commons::crypto::glwe::GlweCiphertext;
    ///
    /// let fourier_glwe: FourierGlweCiphertext<_, u32> =
    ///     FourierGlweCiphertext::allocate(Complex64::new(0., 0.), PolynomialSize(128), GlweSize(7));
    ///
    /// let mut buffers = FourierBuffers::new(fourier_glwe.poly_size, fourier_glwe.glwe_size);
    /// let mut buffers_out = FourierBuffers::new(fourier_glwe.poly_size, fourier_glwe.glwe_size);
    ///
    /// let mut glwe = GlweCiphertext::allocate(0 as u32, PolynomialSize(128), GlweSize(7));
    ///
    /// fourier_glwe.fill_glwe_with_backward_fourier(&mut glwe, &mut buffers);
    ///
    /// let mut glwe_out = GlweCiphertext::allocate(0 as u32, PolynomialSize(128), GlweSize(7));
    ///
    /// fourier_glwe.fill_glwe_with_backward_fourier(&mut glwe_out, &mut buffers_out);
    /// ```
    pub fn fill_glwe_with_backward_fourier<InputCont>(
        &self,
        glwe: &mut GlweCiphertext<InputCont>,
        buffers: &mut FourierBuffers<Scalar>,
    ) where
        Cont: AsMutSlice<Element = Complex64>,
        GlweCiphertext<InputCont>: AsMutTensor<Element = Scalar>,
    {
        // We get the fft to use from the passed buffers
        let fft = &mut buffers.fft_buffers.fft;

        // Output buffer is large enough to hold self which is a FourierGlweCiphertext
        let input_fourier_polynomials_buffer = &mut buffers.fft_buffers.output_buffer;

        input_fourier_polynomials_buffer
            .as_mut_tensor()
            .fill_with_copy(self.as_tensor());

        // Create an iterator that takes chunk of the input buffer and map these to polynomials
        let input_fourier_polynomials_list = input_fourier_polynomials_buffer
            .subtensor_iter_mut(self.poly_size.0)
            .map(FourierPolynomial::from_tensor);

        // Prepare the output polynomials list
        let mut output_std_glwe_polynomials = glwe.as_mut_polynomial_list();

        // Prepare the iterator for the backward calls
        let iterator = output_std_glwe_polynomials
            .polynomial_iter_mut()
            .zip(input_fourier_polynomials_list);

        for (mut coef_poly, mut fourier_poly) in iterator {
            fft.backward_as_torus(&mut coef_poly, &mut fourier_poly);
        }
    }

    /// Returns an iterator over references to the polynomials contained in the GLWE.
    ///
    /// # Example
    ///
    /// ```
    /// use concrete_commons::parameters::PolynomialSize;
    /// use concrete_core::commons::math::polynomial::PolynomialList;
    /// let mut list =
    ///     PolynomialList::from_container(vec![1u8, 2, 3, 4, 5, 6, 7, 8], PolynomialSize(2));
    /// for polynomial in list.polynomial_iter() {
    ///     assert_eq!(polynomial.polynomial_size(), PolynomialSize(2));
    /// }
    /// assert_eq!(list.polynomial_iter().count(), 4);
    /// ```
    pub fn polynomial_iter(
        &self,
    ) -> impl Iterator<Item = FourierPolynomial<&[<Self as AsRefTensor>::Element]>>
    where
        Self: AsRefTensor,
    {
        self.as_tensor()
            .subtensor_iter(self.poly_size.0)
            .map(FourierPolynomial::from_tensor)
    }

    /// Returns an iterator over mutable references to the polynomials contained in the list.
    ///
    /// # Example
    ///
    /// ```
    /// use concrete_commons::parameters::{MonomialDegree, PolynomialSize};
    /// use concrete_core::commons::math::polynomial::PolynomialList;
    /// let mut list =
    ///     PolynomialList::from_container(vec![1u8, 2, 3, 4, 5, 6, 7, 8], PolynomialSize(2));
    /// for mut polynomial in list.polynomial_iter_mut() {
    ///     polynomial
    ///         .get_mut_monomial(MonomialDegree(0))
    ///         .set_coefficient(10u8);
    ///     assert_eq!(polynomial.polynomial_size(), PolynomialSize(2));
    /// }
    /// for polynomial in list.polynomial_iter() {
    ///     assert_eq!(
    ///         *polynomial.get_monomial(MonomialDegree(0)).get_coefficient(),
    ///         10u8
    ///     );
    /// }
    /// assert_eq!(list.polynomial_iter_mut().count(), 4);
    /// ```
    pub fn polynomial_iter_mut(
        &mut self,
    ) -> impl Iterator<Item = FourierPolynomial<&mut [<Self as AsMutTensor>::Element]>>
    where
        Self: AsMutTensor,
    {
        let chunks_size = self.poly_size.0;
        self.as_mut_tensor()
            .subtensor_iter_mut(chunks_size)
            .map(FourierPolynomial::from_tensor)
    }
}

impl<Element, Cont, Scalar> AsRefTensor for FourierGlweCiphertext<Cont, Scalar>
where
    Cont: AsRefSlice<Element = Element>,
    Scalar: UnsignedTorus,
{
    type Element = Element;
    type Container = Cont;
    fn as_tensor(&self) -> &Tensor<Self::Container> {
        &self.tensor
    }
}

impl<Element, Cont, Scalar> AsMutTensor for FourierGlweCiphertext<Cont, Scalar>
where
    Cont: AsMutSlice<Element = Element>,
    Scalar: UnsignedTorus,
{
    type Element = Element;
    type Container = Cont;
    fn as_mut_tensor(&mut self) -> &mut Tensor<<Self as AsMutTensor>::Container> {
        &mut self.tensor
    }
}

impl<Cont, Scalar> IntoTensor for FourierGlweCiphertext<Cont, Scalar>
where
    Cont: AsRefSlice,
    Scalar: UnsignedTorus,
{
    type Element = <Cont as AsRefSlice>::Element;
    type Container = Cont;
    fn into_tensor(self) -> Tensor<Self::Container> {
        self.tensor
    }
}
