pub trait ContainerTrait {
    type Item;
    fn base_len(&self) -> usize;
}

impl<'a, C: ?Sized + ContainerTrait> ContainerTrait for &'a C {
    type Item = C::Item;

    #[inline(always)]
    fn base_len(&self) -> usize {
        (**self).base_len()
    }
}

impl<'a, C: ?Sized + ContainerTrait> ContainerTrait for &'a mut C {
    type Item = C::Item;

    #[inline(always)]
    fn base_len(&self) -> usize {
        (**self).base_len()
    }
}

pub trait Contiguous: ContainerTrait {
    fn begin(&self) -> *const Self::Item;
    fn end(&self) -> *const Self::Item;
    fn as_slice(&self) -> &[Self::Item];
}

impl<'a, C: ?Sized + Contiguous> Contiguous for &'a C {
    #[inline(always)]
    fn begin(&self) -> *const Self::Item {
        (**self).begin()
    }

    #[inline(always)]
    fn end(&self) -> *const Self::Item {
        (**self).end()
    }

    #[inline(always)]
    fn as_slice(&self) -> &[Self::Item] {
        (**self).as_slice()
    }
}

impl<'a, C: ?Sized + Contiguous> Contiguous for &'a mut C {
    #[inline(always)]
    fn begin(&self) -> *const Self::Item {
        (**self).begin()
    }

    #[inline(always)]
    fn end(&self) -> *const Self::Item {
        (**self).end()
    }

    #[inline(always)]
    fn as_slice(&self) -> &[Self::Item] {
        (**self).as_slice()
    }
}

pub trait ContiguousMut: Contiguous {
    #[inline(always)]
    fn begin_mut(&mut self) -> *mut Self::Item {
        self.begin() as *mut _
    }

    #[inline(always)]
    fn end_mut(&mut self) -> *mut Self::Item {
        self.end() as *mut _
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Item];
}

impl<'a, C: ?Sized + ContiguousMut> ContiguousMut for &'a mut C {
    #[inline(always)]
    fn as_mut_slice(&mut self) -> &mut [Self::Item] {
        (**self).as_mut_slice()
    }
}

pub trait GetUnchecked: ContainerTrait {
    #[inline(always)]
    unsafe fn unchecked(&self, index: usize) -> &Self::Item;
}

impl<'a, C: ?Sized + GetUnchecked> GetUnchecked for &'a C {
    #[inline(always)]
    unsafe fn unchecked(&self, index: usize) -> &Self::Item {
        (**self).unchecked(index)
    }
}

impl<'a, C: ?Sized + GetUnchecked> GetUnchecked for &'a mut C {
    #[inline(always)]
    unsafe fn unchecked(&self, index: usize) -> &Self::Item {
        (**self).unchecked(index)
    }
}

pub trait GetUncheckedMut: GetUnchecked {
    #[inline(always)]
    unsafe fn unchecked_mut(&mut self, index: usize) -> &mut Self::Item;
}

impl<'a, C: ?Sized + GetUncheckedMut> GetUncheckedMut for &'a mut C {
    #[inline(always)]
    unsafe fn unchecked_mut(&mut self, index: usize) -> &mut Self::Item {
        (**self).unchecked_mut(index)
    }
}

impl<T> ContainerTrait for [T] {
    type Item = T;

    #[inline(always)]
    fn base_len(&self) -> usize {
        self.len()
    }
}

impl<T> Contiguous for [T] {
    #[inline(always)]
    fn begin(&self) -> *const Self::Item {
        self.as_ptr()
    }

    #[inline(always)]
    fn end(&self) -> *const Self::Item {
        unsafe { self.begin().offset(self.len() as isize) }
    }

    #[inline(always)]
    fn as_slice(&self) -> &[Self::Item] {
        self
    }
}

impl<T> ContiguousMut for [T] {
    #[inline(always)]
    fn as_mut_slice(&mut self) -> &mut [Self::Item] {
        self
    }
}

impl<T> GetUnchecked for [T] {
    #[inline(always)]
    unsafe fn unchecked(&self, index: usize) -> &Self::Item {
        self.get_unchecked(index)
    }
}

impl<T> GetUncheckedMut for [T] {
    #[inline(always)]
    unsafe fn unchecked_mut(&mut self, index: usize) -> &mut Self::Item {
        self.get_unchecked_mut(index)
    }
}
