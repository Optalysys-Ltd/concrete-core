#include "bootstrap_low_latency.cuh"

/* Perform bootstrapping on a batch of input LWE ciphertexts
 *
 *  - lwe_out: output batch of num_samples bootstrapped ciphertexts c =
 * (a0,..an-1,b) where n is the LWE dimension
 *  - lut_vector: should hold as many test vectors of size polynomial_size
 * as there are input ciphertexts, but actually holds
 * num_lut_vectors vectors to reduce memory usage
 *  - lut_vector_indexes: stores the index corresponding to
 * which test vector to use for each sample in
 * lut_vector
 *  - lwe_in: input batch of num_samples LWE ciphertexts, containing n
 * mask values + 1 body value
 *  - bootstrapping_key: RGSW encryption of the LWE secret key sk1
 * under secret key sk2
 * bsk = Z + sk1 H
 * where H is the gadget matrix and Z is a matrix (k+1).l
 * containing GLWE encryptions of 0 under sk2.
 * bsk is thus a tensor of size (k+1)^2.l.N.n
 * where l is the number of decomposition levels and
 * k is the GLWE dimension, N is the polynomial size for
 * GLWE. The polynomial size for GLWE and the test vector
 * are the same because they have to be in the same ring
 * to be multiplied.
 * Note: it is necessary to generate (k+1).k.l.N.n
 * uniformly random coefficients for the zero encryptions
 * - lwe_dimension: size of the Torus vector used to encrypt the input
 * LWE ciphertexts - referred to as n above (~ 600)
 * - polynomial_size: size of the test polynomial (test vector) and size of the
 * GLWE polynomial (~1024)
 * - base_log: log base used for the gadget matrix - B = 2^base_log (~8)
 * - l_gadget: number of decomposition levels in the gadget matrix (~4)
 * - num_samples: number of encrypted input messages
 * - num_lut_vectors: parameter to set the actual number of test vectors to be
 * used
 * - q: number of bytes in the integer representation (32 or 64)
 *
 * This function calls a wrapper to a device kernel that performs the
 * bootstrapping:
 * 	- the kernel is templatized based on integer discretization and
 * polynomial degree
 * 	- num_samples blocks of threads are launched, where each thread is going
 * to handle one or more polynomial coefficients at each stage:
 * 		- perform the blind rotation
 * 		- round the result
 * 		- decompose into l_gadget levels, then for each level:
 * 		  - switch to the FFT domain
 * 		  - multiply with the bootstrapping key
 * 		  - come back to the coefficients representation
 * 	- between each stage a synchronization of the threads is necessary TODO
 * (Agnes) check this
 * 	- in case the device has enough shared memory, temporary arrays used for
 * the different stages (accumulators) are stored into the shared memory
 * 	- the accumulators serve to combine the results for all decomposition
 * levels TODO (Agnes) check this
 * 	- the constant memory (64K) is used for storing the roots of identity
 * values for the FFT
 */
void cuda_bootstrap_low_latency_lwe_ciphertext_vector_32(
        void *v_stream,
        void *lwe_out,
        void *lut_vector,
        void *lut_vector_indexes,
        void *lwe_in,
        void *bootstrapping_key,
        uint32_t lwe_dimension,
        uint32_t polynomial_size,
        uint32_t base_log,
        uint32_t l_gadget,
        uint32_t num_samples,
        uint32_t num_lut_vectors,
        uint32_t lwe_idx,
        uint32_t max_shared_memory) {

  switch (polynomial_size) {
  case 512:
    host_bootstrap_low_latency<uint32_t, Degree<512>>(
        v_stream, (uint32_t *)lwe_out, (uint32_t *)lut_vector,
        (uint32_t *)lut_vector_indexes, (uint32_t *)lwe_in,
        (double2 *)bootstrapping_key, lwe_dimension, polynomial_size,
        base_log, l_gadget, num_samples,
        num_lut_vectors);
    break;
  case 1024:
    host_bootstrap_low_latency<uint32_t, Degree<1024>>(
        v_stream, (uint32_t *)lwe_out, (uint32_t *)lut_vector,
        (uint32_t *)lut_vector_indexes, (uint32_t *)lwe_in,
        (double2 *)bootstrapping_key, lwe_dimension, polynomial_size,
        base_log, l_gadget, num_samples,
        num_lut_vectors);
    break;
  case 2048:
    host_bootstrap_low_latency<uint32_t, Degree<2048>>(
        v_stream, (uint32_t *)lwe_out, (uint32_t *)lut_vector,
        (uint32_t *)lut_vector_indexes, (uint32_t *)lwe_in,
        (double2 *)bootstrapping_key, lwe_dimension, polynomial_size,
        base_log, l_gadget, num_samples,
        num_lut_vectors);
    break;
  case 4096:
    host_bootstrap_low_latency<uint32_t, Degree<4096>>(
        v_stream, (uint32_t *)lwe_out, (uint32_t *)lut_vector,
        (uint32_t *)lut_vector_indexes, (uint32_t *)lwe_in,
        (double2 *)bootstrapping_key, lwe_dimension, polynomial_size,
        base_log, l_gadget, num_samples,
        num_lut_vectors);
    break;
  case 8192:
    host_bootstrap_low_latency<uint32_t, Degree<8192>>(
        v_stream, (uint32_t *)lwe_out, (uint32_t *)lut_vector,
        (uint32_t *)lut_vector_indexes, (uint32_t *)lwe_in,
        (double2 *)bootstrapping_key, lwe_dimension, polynomial_size,
        base_log, l_gadget, num_samples,
        num_lut_vectors);
    break;
  default:
    break;
  }
}

void cuda_bootstrap_low_latency_lwe_ciphertext_vector_64(
        void *v_stream,
        void *lwe_out,
        void *lut_vector,
        void *lut_vector_indexes,
        void *lwe_in,
        void *bootstrapping_key,
        uint32_t lwe_dimension,
        uint32_t polynomial_size,
        uint32_t base_log,
        uint32_t l_gadget,
        uint32_t num_samples,
        uint32_t num_lut_vectors,
        uint32_t lwe_idx,
        uint32_t max_shared_memory) {

  switch (polynomial_size) {
  case 512:
    host_bootstrap_low_latency<uint64_t, Degree<512>>(
        v_stream, (uint64_t *)lwe_out, (uint64_t *)lut_vector,
        (uint32_t *)lut_vector_indexes, (uint64_t *)lwe_in,
        (double2 *)bootstrapping_key, lwe_dimension, polynomial_size,
        base_log, l_gadget, num_samples,
        num_lut_vectors);
    break;
  case 1024:
    host_bootstrap_low_latency<uint64_t, Degree<1024>>(
        v_stream, (uint64_t *)lwe_out, (uint64_t *)lut_vector,
        (uint32_t *)lut_vector_indexes, (uint64_t *)lwe_in,
        (double2 *)bootstrapping_key, lwe_dimension, polynomial_size,
        base_log, l_gadget, num_samples,
        num_lut_vectors);
    break;
  case 2048:
    host_bootstrap_low_latency<uint64_t, Degree<2048>>(
        v_stream, (uint64_t *)lwe_out, (uint64_t *)lut_vector,
        (uint32_t *)lut_vector_indexes, (uint64_t *)lwe_in,
        (double2 *)bootstrapping_key, lwe_dimension, polynomial_size,
        base_log, l_gadget, num_samples,
        num_lut_vectors);
    break;
  case 4096:
    host_bootstrap_low_latency<uint64_t, Degree<4096>>(
        v_stream, (uint64_t *)lwe_out, (uint64_t *)lut_vector,
        (uint32_t *)lut_vector_indexes, (uint64_t *)lwe_in,
        (double2 *)bootstrapping_key, lwe_dimension, polynomial_size,
        base_log, l_gadget, num_samples,
        num_lut_vectors);
    break;
  case 8192:
    host_bootstrap_low_latency<uint64_t, Degree<8192>>(
        v_stream, (uint64_t *)lwe_out, (uint64_t *)lut_vector,
        (uint32_t *)lut_vector_indexes, (uint64_t *)lwe_in,
        (double2 *)bootstrapping_key, lwe_dimension, polynomial_size,
        base_log, l_gadget, num_samples,
        num_lut_vectors);
    break;
  default:
    break;
  }
}

