use std::path::Path;

use libc::{c_char};
use rsm::bindings::VOL_FILENAME_MAX;

fn map_as_slice(val: &rsm::bindings::vol_def) -> &[u8] {
    use std::ops::Range;
    use std::slice::from_ptr_range;
    //TODO verify that gdb_head always Corresponds to the end off the map section.
    unsafe {
        from_ptr_range(Range {
            start: val.map.cast(),
            end: val.gbd_head.cast(),
        })
    }
}

/// Formats the file name so it fits in `vol_def`.
/// If to long grab the last `VOL_FILENAME_MAX` chars.
/// If to short otherwise pad with trailing 0s.
#[must_use] pub fn format_name(path: &Path) -> [libc::c_char; VOL_FILENAME_MAX as usize] {
    //TODO test edge cases
    std::fs::canonicalize(path)
        .unwrap()
        .to_str()
        .unwrap()
        .bytes()
        .rev()
        .take(VOL_FILENAME_MAX as usize)
        .rev()
        .chain(std::iter::repeat(0))
        .take(VOL_FILENAME_MAX as usize)
        .map(|x| TryInto::<c_char>::try_into(x).unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use libc::c_void;
    use rsm::bindings::vol_def;
    use rsm::bindings::GBD;
    use rsm::bindings::RBD;
    pub fn helper<T>(ptr: *const T, base: *const c_void) -> Option<isize> {
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { ptr.byte_offset_from(base) })
        }
    }
    pub unsafe fn assert_vol_def_eq(left: *mut vol_def, right: *mut vol_def) {
        assert_eq!(unsafe { (*left).num_gbd }, unsafe { (*right).num_gbd });
        assert_eq!(unsafe { (*left).num_of_daemons }, unsafe {
            (*right).num_of_daemons
        });
        //assert_eq!(unsafe { (*left).wd_tab}, unsafe { (*right).wd_tab });
        //assert_eq!(unsafe { (*left).dismount_flag }, unsafe { (*right).dismount_flag });
        //assert_eq!(unsafe { (*left).map_dirty_flag }, unsafe { (*right).map_dirty_flag });
        //assert_eq!(unsafe { (*left).writelock }, unsafe { (*right).writelock });
        //assert_eq!(unsafe { (*left).upto }, unsafe { (*right).upto });
        //assert_eq!(unsafe { (*left).shm_id }, unsafe { (*right).shm_id });
        //assert_eq!(unsafe { (*left).dirtyQr }, unsafe { (*right).dirtyQr });
        //assert_eq!(unsafe { (*left).dirtyQw}, unsafe { (*right).dirtyQw });
        //assert_eq!(unsafe { (*left).garbQ}, unsafe { (*right).garbQ});
        //assert_eq!(unsafe { (*left).garbQw}, unsafe { (*right).garbQw});
        //assert_eq!(unsafe { (*left).garbQr}, unsafe { (*right).garbQr});
        //assert_eq!(unsafe { (*left).jrn_next}, unsafe { (*right).jrn_next});
        assert_eq!(unsafe { (*left).file_name }, unsafe { (*right).file_name });
        //assert_eq!(unsafe { (*left).stats}, unsafe { (*right).stats});
        let (
            l_vollab,
            l_map,
            l_first_free,
            l_gdb_hash,
            l_gbd_head,
            l_global_buf,
            l_zero_block,
            l_rbd_hash,
            l_rbd_head,
            l_rbd_end,
            l_dirtyQ,
        ) = offsets(left);
        let (
            r_vollab,
            r_map,
            r_first_free,
            r_gdb_hash,
            r_gbd_head,
            r_global_buf,
            r_zero_block,
            r_rbd_hash,
            r_rbd_head,
            r_rbd_end,
            r_dirtyQ,
        ) = offsets(right);

        assert_eq!(l_vollab, r_vollab);
        assert_eq!(l_map, r_map);
        assert_eq!(l_first_free, r_first_free);
        assert_eq!(l_gdb_hash, r_gdb_hash);
        assert_eq!(l_global_buf, r_global_buf);
        assert_eq!(l_zero_block, r_zero_block);
        assert_eq!(l_rbd_hash, r_rbd_hash);
        assert_eq!(l_rbd_head, r_rbd_head);
        assert_eq!(l_rbd_end, r_rbd_end);
        //assert_eq!(l_dirtyQ,r_dirtyQ);
        assert_eq!(
            map_as_slice(unsafe { left.as_ref() }.unwrap()),
            map_as_slice(unsafe { right.as_ref() }.unwrap())
        );

        assert_eq!({ (*left).num_gbd }, { (*right).num_gbd });
        assert_eq!(l_gbd_head, r_gbd_head);
        for i in 0..(*left).num_gbd as usize {
            assert_gbd_eq(
                (*left).gbd_head.add(i),
                left.cast(),
                (*right).gbd_head.add(i),
                right.cast(),
            );
        }

        let l_rbd = (*left).rbd_head.cast::<RBD>();
        let r_rbd = (*left).rbd_head.cast::<RBD>();
        /*assert_eq!(
            helper((*l_rbd).fwd_link,left.cast()),
            helper((*r_rbd).fwd_link,right.cast())
        );*/
        assert_eq!({ (*l_rbd).chunk_size }, { (*r_rbd).chunk_size });
        assert_eq!({ (*l_rbd).attached }, { (*r_rbd).attached });
        assert_eq!({ (*l_rbd).last_access }, { (*r_rbd).last_access });
        assert_eq!((*l_rbd).rnam, (*r_rbd).rnam);
        assert_eq!((*l_rbd).uci, (*r_rbd).uci);
        assert_eq!((*l_rbd).vol, (*r_rbd).vol);
        assert_eq!({ (*l_rbd).rou_size }, { (*r_rbd).rou_size });
    }

    fn offsets(
        def: *const vol_def,
    ) -> (
        Option<isize>,
        Option<isize>,
        Option<isize>,
        [Option<isize>; 1025],
        Option<isize>,
        Option<isize>,
        Option<isize>,
        [Option<isize>; 1024],
        Option<isize>,
        Option<isize>,
        [Option<isize>; 1024],
    ) {
        let base = def.cast::<c_void>();
        unsafe {
            (
                helper((*def).vollab, base),
                helper((*def).map, base),
                helper((*def).first_free, base),
                (*def).gbd_hash.map(|x| helper(x, base)),
                helper((*def).gbd_head, base),
                helper((*def).global_buf, base),
                helper((*def).zero_block, base),
                (*def).rbd_hash.map(|x| helper(x, base)),
                helper((*def).rbd_head, base),
                helper((*def).rbd_end, base),
                (*def).dirtyQ.map(|x| helper(x, base)),
            )
        }
    }

    pub unsafe fn assert_gbd_eq(
        left: *const GBD,
        left_base: *const c_void,
        right: *const GBD,
        right_base: *const c_void,
    ) {
        assert_eq!(helper(left, left_base), helper(right, right_base));
        //assert_eq!({(*left).block},{(*right).block});
        assert_eq!(
            helper((*left).mem, left_base),
            helper((*right).mem, right_base)
        );
        assert_eq!(
            helper((*left).next, left_base),
            helper((*right).next, right_base)
        );
        //assert_eq!(helper((*left).dirty,left_base),helper((*right).dirty,right_base));
        //assert_eq!({(*left).last_accessed},{(*right).last_accessed});
    }
}
