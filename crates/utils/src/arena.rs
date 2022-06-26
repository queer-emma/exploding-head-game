use std::{
    convert::TryFrom,
    fmt,
    iter::Enumerate,
    num::NonZeroU16,
};

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(transparent)]
pub struct Index {
    index: NonZeroU16,
}

impl Index {
    fn new(index: usize) -> Self {
        let index = u16::try_from(index).unwrap_or_else(|e| panic!("Index overflowed: {}", e));
        let index = unsafe { NonZeroU16::new_unchecked(index + 1) };
        Self { index }
    }

    fn index(self) -> usize {
        (self.index.get() - 1).into()
    }
}

impl fmt::Debug for Index {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Index").field(&self.index).finish()
    }
}

/// append-only data structure to store objects efficiently and reference them by index.
#[derive(Serialize, Deserialize)]
pub struct Arena<T> {
    items: Vec<T>,
}

impl<T: fmt::Debug> fmt::Debug for Arena<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.items).finish()
    }
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self { items: vec![] }
    }
}

impl<T> Arena<T> {
    pub fn insert(&mut self, item: T) -> Index {
        let index = self.items.len();
        self.items.push(item);
        Index::new(index)
    }

    pub fn get(&self, index: Index) -> Option<&T> {
        self.items.get(index.index())
    }

    pub fn get_mut(&mut self, index: Index) -> Option<&mut T> {
        self.items.get_mut(index.index())
    }

    pub fn map<U, F: FnMut(T) -> U>(self, f: F) -> Arena<U> {
        Arena {
            items: self.items.into_iter().map(f).collect(),
        }
    }

    pub fn try_map<U, E, F: FnMut(T) -> Result<U, E>>(self, f: F) -> Result<Arena<U>, E> {
        Ok(Arena {
            items: self
                .items
                .into_iter()
                .map(f)
                .collect::<Result<Vec<U>, E>>()?,
        })
    }

    pub fn map_ref<U, F: FnMut(&T) -> U>(&self, f: F) -> Arena<U> {
        Arena {
            items: self.items.iter().map(f).collect(),
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.into_iter()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn as_slice(&self) -> &'_ [T] {
        self.items.as_slice()
    }

    pub fn into_inner(self) -> Vec<T> {
        self.items
    }

    pub fn shrink_to_fit(&mut self) {
        self.items.shrink_to_fit();
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl<T> std::ops::Index<Index> for Arena<T> {
    type Output = T;

    fn index(&self, index: Index) -> &Self::Output {
        &self.items[index.index()]
    }
}

impl<'a, T> IntoIterator for &'a Arena<T> {
    type Item = (Index, &'a T);
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            inner: self.items.iter().enumerate(),
        }
    }
}

pub struct Iter<'a, T> {
    inner: Enumerate<std::slice::Iter<'a, T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (Index, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|(index, item)| (Index::new(index), item))
    }
}

impl<'a, T> IntoIterator for &'a mut Arena<T> {
    type Item = (Index, &'a mut T);
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            inner: self.items.iter_mut().enumerate(),
        }
    }
}

pub struct IterMut<'a, T> {
    inner: Enumerate<std::slice::IterMut<'a, T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = (Index, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|(index, item)| (Index::new(index), item))
    }
}
