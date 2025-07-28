use std::ops::Deref;
use std::ops::DerefMut;
use std::fmt;
#[derive(Clone, Debug, PartialEq)]
pub struct FixedVec<T, const N: usize> {
    data: [T; N],
    len: usize,
}

impl<T: Default + Clone, const N: usize> FixedVec<T, N> {
    pub fn new() -> Self {
        Self {
            data: array_init::array_init(|_| T::default()),
            len: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.len < N {
            self.data[self.len] = item;
            self.len += 1;
        } else {
            eprintln!("FixedVec overflow: capacity {} reached.", N);
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn capacity(&self) -> usize {
        N
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.data[..self.len].iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.data[..self.len].iter_mut()
    }
}

impl<T: Default + Clone, const N: usize> Default for FixedVec<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Default + Clone, const N: usize> From<Vec<T>> for FixedVec<T, N> {
    fn from(vec: Vec<T>) -> Self {
        let mut fv = Self::new();
        for item in vec {
            fv.push(item);
        }
        fv
    }
}

impl<T: Default + Clone, const N: usize> From<&[T]> for FixedVec<T, N> {
    fn from(slice: &[T]) -> Self {
        let mut fv = Self::new();
        for item in slice {
            fv.push(item.clone());
        }
        fv
    }
}

impl<T: Default + Clone, const N: usize> Deref for FixedVec<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.data[..self.len]
    }
}

impl<T: Default + Clone, const N: usize> DerefMut for FixedVec<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data[..self.len]
    }
}

use serde::ser::{Serialize, Serializer};
use serde::de::{Deserialize, Deserializer, Visitor, SeqAccess};
use std::marker::PhantomData;

impl<T, const N: usize> Serialize for FixedVec<T, N>
where
    T: Serialize + Default + Clone,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self.iter())
    }
}

impl<'de, T, const N: usize> Deserialize<'de> for FixedVec<T, N>
where
    T: Deserialize<'de> + Default + Clone,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FixedVecVisitor<T, const N: usize> {
            phantom_t: PhantomData<T>,
        }

        impl<'de, T, const N: usize> Visitor<'de> for FixedVecVisitor<T, N>
        where
            T: Deserialize<'de> + Default + Clone,
        {
            type Value = FixedVec<T, N>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut fv = FixedVec::new();
                while let Some(element) = seq.next_element()? {
                    fv.push(element);
                }
                Ok(fv)
            }
        }

        deserializer.deserialize_seq(FixedVecVisitor { phantom_t: PhantomData })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_vec_new() {
        let fv: FixedVec<u8, 5> = FixedVec::new();
        assert_eq!(fv.len(), 0);
        assert!(fv.is_empty());
        assert_eq!(fv.capacity(), 5);
    }

    #[test]
    fn test_fixed_vec_push() {
        let mut fv: FixedVec<u8, 3> = FixedVec::new();
        fv.push(1);
        fv.push(2);
        assert_eq!(fv.len(), 2);
        assert_eq!(fv.iter().collect::<Vec<_>>(), vec![&1, &2]);
        fv.push(3);
        assert_eq!(fv.len(), 3);
        assert_eq!(fv.iter().collect::<Vec<_>>(), vec![&1, &2, &3]);
        fv.push(4); // Should overflow and print error, but not panic
        assert_eq!(fv.len(), 3); // Length should not change
        assert_eq!(fv.iter().collect::<Vec<_>>(), vec![&1, &2, &3]);
    }

    #[test]
    fn test_fixed_vec_from_vec() {
        let v = vec![10, 20, 30];
        let fv: FixedVec<i32, 5> = v.into();
        assert_eq!(fv.len(), 3);
        assert_eq!(fv.iter().collect::<Vec<_>>(), vec![&10, &20, &30]);

        let v_overflow = vec![1, 2, 3, 4, 5, 6];
        let fv_overflow: FixedVec<i32, 4> = v_overflow.into();
        assert_eq!(fv_overflow.len(), 4);
        assert_eq!(fv_overflow.iter().collect::<Vec<_>>(), vec![&1, &2, &3, &4]);
    }

    #[test]
    fn test_fixed_vec_from_slice() {
        let s = &[100, 200];
        let fv: FixedVec<u8, 5> = s.into();
        assert_eq!(fv.len(), 2);
        assert_eq!(fv.iter().collect::<Vec<_>>(), vec![&100, &200]);
    }

    #[test]
    fn test_fixed_vec_deref() {
        let mut fv: FixedVec<u8, 5> = FixedVec::new();
        fv.push(1);
        fv.push(2);
        let slice: &[u8] = &fv;
        assert_eq!(slice, &[1, 2]);

        let mut_slice: &mut [u8] = &mut fv;
        mut_slice[0] = 10;
        assert_eq!(fv.iter().collect::<Vec<_>>(), vec![&10, &2]);
    }
}
