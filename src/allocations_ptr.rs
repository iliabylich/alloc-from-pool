macro_rules! allocations_ptr {
    ($this:expr) => {{
        #[allow(unused_assignments, unused_mut)]
        let mut ptr = std::ptr::null_mut::<usize>();
        #[cfg(test)]
        {
            ptr = $this.allocations;
        }
        ptr
    }};
}

pub(crate) use allocations_ptr;
