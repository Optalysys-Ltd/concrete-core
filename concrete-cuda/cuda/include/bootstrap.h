#ifndef CUDA_BOOTSTRAP_H
#define CUDA_BOOTSTRAP_H

#include <cstdint>

extern "C" {

void cuda_initialize_twiddles(uint32_t polynomial_size, uint32_t gpu_index);

void cuda_convert_lwe_bootstrap_key_32(void *dest, void *src, void *v_stream,
                                  uint32_t gpu_index, uint32_t input_lwe_dim, uint32_t glwe_dim,
                                  uint32_t l_gadget, uint32_t polynomial_size);

void cuda_convert_lwe_bootstrap_key_64(void *dest, void *src, void *v_stream,
                                  uint32_t gpu_index, uint32_t input_lwe_dim, uint32_t glwe_dim,
                                  uint32_t l_gadget, uint32_t polynomial_size);

void cuda_bootstrap_amortized_lwe_ciphertext_vector_32(
    void *v_stream,
    void *lwe_out,
    void *test_vector,
    void *test_vector_indexes,
    void *lwe_in,
    void *bootstrapping_key,
    uint32_t input_lwe_dimension,
    uint32_t polynomial_size,
    uint32_t base_log,
    uint32_t l_gadget,
    uint32_t num_samples,
    uint32_t num_test_vectors,
    uint32_t lwe_idx,
    uint32_t max_shared_memory);

void cuda_bootstrap_amortized_lwe_ciphertext_vector_64(
    void *v_stream,
    void *lwe_out,
    void *test_vector,
    void *test_vector_indexes,
    void *lwe_in,
    void *bootstrapping_key,
    uint32_t input_lwe_dimension,
    uint32_t polynomial_size,
    uint32_t base_log,
    uint32_t l_gadget,
    uint32_t num_samples,
    uint32_t num_test_vectors,
    uint32_t lwe_idx,
    uint32_t max_shared_memory);

void cuda_bootstrap_low_latency_lwe_ciphertext_vector_32(
    void *v_stream,
    void *lwe_out,
    void *test_vector,
    void *test_vector_indexes,
    void *lwe_in,
    void *bootstrapping_key,
    uint32_t input_lwe_dimension,
    uint32_t polynomial_size,
    uint32_t base_log,
    uint32_t l_gadget,
    uint32_t num_samples,
    uint32_t num_test_vectors,
    uint32_t lwe_idx,
    uint32_t max_shared_memory);

void cuda_bootstrap_low_latency_lwe_ciphertext_vector_64(
    void *v_stream,
    void *lwe_out,
    void *test_vector,
    void *test_vector_indexes,
    void *lwe_in,
    void *bootstrapping_key,
    uint32_t input_lwe_dimension,
    uint32_t polynomial_size,
    uint32_t base_log,
    uint32_t l_gadget,
    uint32_t num_samples,
    uint32_t num_test_vectors,
    uint32_t lwe_idx,
    uint32_t max_shared_memory);
};

__device__ inline int get_start_ith_ggsw(int i, uint32_t polynomial_size,
                                         int glwe_dimension,
                                         uint32_t l_gadget);

__device__ double2*
get_ith_mask_kth_block(double2* ptr, int i, int k, int level, uint32_t polynomial_size,
                       int glwe_dimension, uint32_t l_gadget);

__device__ double2*
get_ith_body_kth_block(double2 *ptr, int i, int k, int level, uint32_t polynomial_size,
                       int glwe_dimension, uint32_t l_gadget);
#endif // CUDA_BOOTSTRAP_H
