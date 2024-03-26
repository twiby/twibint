use crate::traits::Digit;

use rayon::prelude::*;

#[cfg(test)]
const MIN_CHUNK_SIZE: usize = 5;
#[cfg(not(test))]
const MIN_CHUNK_SIZE: usize = 1000;
// const MIN_CHUNK_SIZE: usize = 10;

#[cfg(test)]
const DEFAULT_CPUS: usize = 4;
#[cfg(not(test))]
const DEFAULT_CPUS: usize = 20;

pub(super) fn parallel_add_assign<T: Digit>(rhs: &mut [T], lhs: &[T]) -> bool {
    debug_assert!(rhs.len() >= lhs.len());
    if lhs.len() < MIN_CHUNK_SIZE {
        return super::add::schoolbook_add_assign(rhs, lhs, false);
    }

    let chunk_size = (lhs.len() >> 1) + 1;
    let nb_chunks = lhs.len() / chunk_size;
    assert_eq!(nb_chunks, 1);

    let (rhs_chunk0, rhs_chunk1) = rhs.split_at_mut(chunk_size);
    let (lhs_chunk0, lhs_chunk1) = lhs.split_at(chunk_size);

    // precompute carry
    let mut n = rhs_chunk0.len();
    let (mut sum, mut carry): (T, bool);
    loop {
        n -= 1;
        unsafe {
            (sum, carry) = rhs_chunk0
                .get_unchecked(n)
                .overflowing_add(*lhs_chunk0.get_unchecked(n));
        }

        if sum != T::MAX || n == 0 {
            break;
        }
    }

    let (_, final_carry) = rayon::join(
        || super::add::schoolbook_add_assign(rhs_chunk0, lhs_chunk0, false),
        || super::add::schoolbook_add_assign(rhs_chunk1, lhs_chunk1, carry),
    );
    final_carry
}

// #[allow(unused)]
// pub(super) fn parallel_add_assign_dyn<T: Digit>(rhs: &mut [T], lhs: &[T]) -> bool {
//     debug_assert!(rhs.len() >= lhs.len());
//     if lhs.len() < DEFAULT_CPUS * MIN_CHUNK_SIZE {
//         return super::add::schoolbook_add_assign(rhs, lhs, false);
//     }

//     let chunk_size = lhs.len() / DEFAULT_CPUS;
//     let nb_chunks = lhs.len() / chunk_size;

//     let mut pre_computed_carries = Vec::<bool>::with_capacity(nb_chunks + 1);
//     pre_computed_carries.push(false);
//     let mut prev = false;
//     for (a, b) in rhs
//         .chunks_exact(chunk_size)
//         .zip(lhs.chunks_exact(chunk_size))
//     {
//         let mut n = a.len();
//         let (mut sum, mut carry): (T, bool);
//         loop {
//             n -= 1;
//             unsafe {
//                 (sum, carry) = a.get_unchecked(n).overflowing_add(*b.get_unchecked(n));
//             }

//             if sum != T::MAX || n == 0 {
//                 prev = ((sum == T::MAX) & prev) | ((sum != T::MAX) & carry);
//                 break;
//             }
//         }

//         pre_computed_carries.push(prev);
//     }

//     let treated = nb_chunks * chunk_size;
//     let (rhs_chunks, rhs_remaining) = rhs.split_at_mut(treated);
//     let (lhs_chunks, lhs_remaining) = lhs.split_at(treated);

//     let (_, final_carry) = rayon::join(
//         || {
//             let mut data = Vec::<(&mut [T], &[T], bool)>::new();
//             for ((a, b), c) in rhs_chunks
//                 .chunks_exact_mut(chunk_size)
//                 .zip(lhs_chunks.chunks_exact(chunk_size))
//                 .zip(pre_computed_carries.iter())
//             {
//                 data.push((a, b, *c));
//             }

//             data.into_par_iter().for_each(|(a, b, c)| {
//                 super::add::schoolbook_add_assign(a, b, c);
//             });
//         },
//         || {
//             super::add::schoolbook_add_assign(
//                 rhs_remaining,
//                 lhs_remaining,
//                 *pre_computed_carries.last().unwrap(),
//             )
//         },
//     );

//     final_carry
// }
