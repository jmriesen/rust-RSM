use std::{
    fmt::Display,
    fs::OpenOptions,
    io::Read,
    mem::transmute,
    num::NonZeroI32,
    os::fd::AsRawFd,
    path::Path,
    ptr::{from_mut, null_mut},
};

use ffi::{
    shared_memory_id, DAEMONS, DATA_UNION, DB_STAT, GBD, GBD_HASH, LABEL_BLOCK, MAX_DAEMONS,
    MIN_DAEMONS, RBD, RBD_HASH, RSM_SYSTEM, VOL_DEF, VOL_FILENAME_MAX, WD_TAB,
};
use libc::{c_char, c_void};

use derive_more::{AsMut, AsRef};
use ref_cast::RefCast;

use crate::{start::Error, units::Bytes};

use self::{global_buf::init_global_buffer_descriptors, label::Label};

use super::alloc::{Allocation, TabLayout};

#[derive(RefCast, AsMut, AsRef)]
#[repr(transparent)]
pub struct Volume(VOL_DEF);

impl Volume {
    pub fn file_name(&self) -> String {
        use core::ffi::CStr;
        let file_name = self.0.file_name.map(|x| x as u8);
        CStr::from_bytes_until_nul(&file_name)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }
    pub fn shm_id(&self) -> i32 {
        self.0.shm_id
    }

    #[must_use]
    pub fn label(&self) -> &Label {
        Label::ref_cast(unsafe { self.0.vollab.as_ref() }.unwrap())
    }

    pub fn label_mut(&mut self) -> &mut Label {
        Label::ref_cast_mut(unsafe { self.0.vollab.as_mut() }.unwrap())
    }

    fn global_buffer_size(&self) -> Bytes {
        Bytes(unsafe { self.0.zero_block.byte_offset_from(self.0.global_buf) } as usize)
    }

    fn routine_buffer_size(&self) -> Bytes {
        Bytes(unsafe { self.0.rbd_end.byte_offset_from(self.0.rbd_head) } as usize)
    }
}

impl Display for Volume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "*** Volume %d ***")?;
        writeln!(f, "DB File Path:\t\t{}", self.file_name())?;
        write!(f, "{}", self.label())?;
        writeln!(
            f,
            "Global Buffers:\t\t{} ({} Buffers)",
            self.global_buffer_size().megbi_floor(),
            { self.0.num_gbd }
        )?;

        writeln!(
            f,
            "Routine Buffers Space:\t{}",
            self.routine_buffer_size().megbi_floor()
        )?;

        writeln!(f, "shared Memory ID: \t{}", { self.0.shm_id })?;

        for pid in self.0.wd_tab.iter().filter_map(|x| NonZeroI32::new(x.pid)) {
            //NOTE C code wraps values a 80 columns
            //I find it incredibly annoying when applications assume my terminal size
            //I think the terminal should be responsible for the wrapping behavior.
            //Or better yet, Just put everything on its own line.
            writeln!(f, "{pid}")?;
        }

        Ok(())
    }
}

#[cfg(test)]
fn map_as_slice(val: &VOL_DEF) -> &[u8] {
    use std::{ops::Range, slice::from_ptr_range};
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
#[must_use]
pub fn format_name(path: &Path) -> [libc::c_char; VOL_FILENAME_MAX as usize + 1] {
    //test canonicalize depends on the actual file system and is therefore hard to mock.
    //for right now I have decided to not worry about getting canonicalize into a test harness, mostly for simplicities sake.
    //testing this will become much easier if I decide to containorize the unit tests.
    format_file_name_helper(std::fs::canonicalize(path).unwrap().to_str().unwrap())
}

pub mod global_buf;
/// clips/pads with zeros the file name
/// This should only be used by `format_name`, but was pulled out so it was easier to test.
pub mod label;
/// (canonicalized file names are absolute/have to actually exist witch makes it a pain to construct test file names of the correct length)
fn format_file_name_helper(file_name: &str) -> [libc::c_char; VOL_FILENAME_MAX as usize + 1] {
    file_name
        .bytes()
        .rev()
        .take(VOL_FILENAME_MAX as usize)
        .rev()
        .chain(std::iter::repeat(0))
        //NOTE It looks like they added another element to the array.
        //This way the string is always null terminated
        .take(VOL_FILENAME_MAX as usize + 1)
        .map(|x| TryInto::<c_char>::try_into(x).unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

pub unsafe fn new<'a>(
    name: &Path,
    jobs: usize,
    tab: *mut c_void,
    //TODO I am ignoring this for now but this should probably be converted to some sort of builder
    layout: &TabLayout<VOL_DEF, u8, GBD, u8, u8, RBD>,
) -> Result<&'a mut Volume, Error> {
    let (volume, header, gbd_head, global_buf, zero_block, rbd_head, end) =
        unsafe { layout.calculate_offsets(tab) };
    let (label, map) = init_header_section(name, header)?;

    let gbd_blocks = init_global_buffer_descriptors(gbd_head, &global_buf, label.block_size());
    let rbd_head = init_routine(rbd_head);

    let vol_def = VOL_DEF {
        file_name: format_name(name),
        vollab: label.as_mut(),
        map,
        map_dirty_flag: 0,
        first_free: map.cast(),

        global_buf: global_buf.into_void_ptr(),
        zero_block: zero_block.into_void_ptr(),
        num_gbd: gbd_blocks.len().try_into().unwrap(),
        gbd_head: gbd_blocks.as_mut_ptr(),
        gbd_hash: {
            let mut temp = [std::ptr::null_mut(); GBD_HASH as usize + 1];
            temp[GBD_HASH as usize] = gbd_blocks.as_mut_ptr();
            temp
        },
        rbd_head: from_mut(rbd_head).cast::<c_void>(),
        rbd_end: end,
        rbd_hash: {
            let mut temp = [std::ptr::null_mut(); RBD_HASH as usize + 1];
            temp[RBD_HASH as usize] = from_mut(rbd_head).cast();
            temp
        },

        //TODO remove the unwrap or default.
        //This function currently depends on if the mem segement has been set up
        //this also means we "have to use a real mem segment"
        //I don't like that.
        shm_id: shared_memory_id(name, RSM_SYSTEM as i32).unwrap_or_default(),
        //TODO add test that cover these bounds.
        num_of_daemons: (jobs as u32 / DAEMONS)
            .clamp(MIN_DAEMONS, MAX_DAEMONS)
            .try_into()
            .unwrap(),
        wd_tab: [WD_TAB {
            pid: 0,
            doing: 0,
            currmsg: DATA_UNION { intdata: 0 },
        }; 16],
        dismount_flag: 0,
        writelock: 0,
        upto: 0,
        dirtyQ: [std::ptr::null_mut(); 1024],
        dirtyQw: 0,
        dirtyQr: 0,
        garbQ: [0; 8192],
        garbQw: 0,
        garbQr: 0,
        jrn_next: 0,
        stats: DB_STAT {
            dbget: 0,
            dbset: 0,
            dbkil: 0,
            dbdat: 0,
            dbord: 0,
            dbqry: 0,
            lasttry: 0,
            lastok: 0,
            logrd: 0,
            phyrd: 0,
            logwt: 0,
            phywt: 0,
            blkalloc: 0,
            blkdeall: 0,
            blkreorg: 0,
            diskerrors: 0,
        },
    };
    //TODO this could be cleaned up.
    unsafe { volume.ptr.as_mut() }.unwrap().write(vol_def);
    let volume = Volume::ref_cast_mut(unsafe { volume.ptr.cast::<VOL_DEF>().as_mut() }.unwrap());
    {
        // Was the volume cleanly dismounted?
        if volume.label().clean() {
            eprintln!("WARNING: Volume was not dismounted properly!");
            // mark for cleaning
            volume.0.upto = 1;
        } else {
            //TODO branch is untested
            // mark as mounted
            volume.label_mut().set_dirty(true);
            // and map needs writing
            volume.0.map_dirty_flag = 1;
        }
    }
    Ok(volume)
}

/// This will panic if the path is not a valid db file or if the header allocation is 1= the header size.
/// TODO considering factoring out the header file init section.
pub fn init_header_section<'a>(
    path: &Path,
    header: Allocation<u8>,
) -> Result<(&'a mut Label, *mut c_void), Error> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(path)
        .map_err(|_| Error::CouldNotOpenDatabase(path.to_path_buf()))?;
    let buf_size = Bytes(header.layout.size());
    //NOTE it is safe to transmute right away since
    //all bit patterns are a valid [u8]
    #[allow(clippy::transmute_ptr_to_ptr)]
    let header: &mut [u8] = unsafe { transmute(header.into_slice()) };

    file.read_exact(&mut header[..])
        .map_err(|_| Error::CouldNotReadLableSlashMapBlock)?;
    // #Safety The database file layout starts with the header section (Label + Map).
    // so, as long as the file is not corrupted we can just read in the header section.
    // NOTE The Caller is responsible for giving ups a properly sized allocation
    let label =
        Label::ref_cast_mut(unsafe { header.as_mut_ptr().cast::<LABEL_BLOCK>().as_mut() }.unwrap());
    let map = unsafe { from_mut(label).add(1).cast() };
    //Panic if header size on file does not match the header allocation's size
    assert!(buf_size == label.header_size());

    //NOTE the C code says this was to facilitate forking.
    //I am not comfortable really comfortable forking so this may not be needed.
    unsafe {
        ffi::partab.vol_fds[0] = file.as_raw_fd();
    }

    Ok((label, map))
}

#[allow(clippy::needless_pass_by_value)]
fn init_routine<'a>(alloc: Allocation<RBD>) -> &'a mut RBD {
    let rbd = alloc.as_mut();
    rbd.write(RBD {
        fwd_link: null_mut(),
        chunk_size: alloc.layout.size() as u32,
        attached: 0,
        last_access: 0,
        rnam: "".try_into().unwrap(),
        uci: 0,
        vol: 0,
        rou_size: 0,
        comp_ver: 0,
        comp_user: 0,
        comp_date: 0,
        comp_time: 0,
        tag_tbl: 0,
        num_tags: 0,
        var_tbl: 0,
        num_vars: 0,
        code: 0,
        code_size: 0,
    });
    unsafe { rbd.assume_init_mut() }
}

#[cfg(test)]
pub mod tests {
    use std::ptr::from_ref;

    use tests::global_buf::test::assert_gbd_eq;

    use crate::test_helper::relitive_ptr;

    use super::*;

    //TODO it would be nice if I could auto generate most of these asserts with a derive macro
    pub fn assert_vol_def_eq(left: &Volume, right: &Volume) {
        let left = &left.0;
        let right = &right.0;
        assert_eq!({ left.num_gbd }, { right.num_gbd });
        assert_eq!({ left.num_of_daemons }, { right.num_of_daemons });
        //assert_eq!(unsafe { (*left).wd_tab}, unsafe { (*right).wd_tab });
        assert_eq!({ left.dismount_flag }, { right.dismount_flag });
        assert_eq!({ left.map_dirty_flag }, { right.map_dirty_flag });
        assert_eq!({ left.writelock }, { right.writelock });
        assert_eq!({ left.upto }, { right.upto });
        //assert_eq!(unsafe { (*left).shm_id }, unsafe { (*right).shm_id });
        assert_eq!({ left.dirtyQr }, { right.dirtyQr });
        assert_eq!({ left.dirtyQw }, { right.dirtyQw });
        assert_eq!({ left.garbQ }, { right.garbQ });
        assert_eq!({ left.garbQw }, { right.garbQw });
        assert_eq!({ left.garbQr }, { right.garbQr });
        assert_eq!({ left.jrn_next }, { right.jrn_next });
        assert_eq!({ left.file_name }, { right.file_name });
        assert_stat_eq(&left.stats, &right.stats);
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
            l_dirty_q,
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
            r_dirty_q,
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
        assert_eq!(l_dirty_q, r_dirty_q);
        assert_eq!(map_as_slice(left), map_as_slice(right));

        assert_eq!({ left.num_gbd }, { right.num_gbd });
        assert_eq!(l_gbd_head, r_gbd_head);
        let left_base = from_ref(left).cast();
        let right_base = from_ref(right).cast();
        let l_gbds = unsafe { core::slice::from_raw_parts(left.gbd_head, left.num_gbd as usize) };
        let r_gbds = unsafe { core::slice::from_raw_parts(right.gbd_head, right.num_gbd as usize) };
        for (left_gbd, right_gbd) in l_gbds.iter().zip(r_gbds) {
            assert_gbd_eq(left_gbd, left_base, right_gbd, right_base);
        }

        let l_rbd = unsafe { left.rbd_head.cast::<RBD>().as_ref().unwrap() };
        let r_rbd = unsafe { right.rbd_head.cast::<RBD>().as_ref().unwrap() };
        assert_eq!(
            relitive_ptr(l_rbd.fwd_link, left_base),
            relitive_ptr(r_rbd.fwd_link, right_base)
        );
        assert_eq!({ l_rbd.chunk_size }, { r_rbd.chunk_size });
        assert_eq!({ l_rbd.attached }, { r_rbd.attached });
        assert_eq!({ l_rbd.last_access }, { r_rbd.last_access });
        assert_eq!(l_rbd.rnam, r_rbd.rnam);
        assert_eq!(l_rbd.uci, r_rbd.uci);
        assert_eq!(l_rbd.vol, r_rbd.vol);
        assert_eq!({ l_rbd.rou_size }, { r_rbd.rou_size });
    }

    #[allow(clippy::type_complexity)]
    //TODO the type complexity lint is valid, however they type is conceptually quite simple
    //I have higher priorities then fixing this right not, but it should be fixed at some point.
    fn offsets(
        def: &VOL_DEF,
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
        let base = core::ptr::from_ref(def).cast::<c_void>();
        (
            relitive_ptr(def.vollab, base),
            relitive_ptr(def.map, base),
            relitive_ptr(def.first_free, base),
            def.gbd_hash.map(|x| relitive_ptr(x, base)),
            relitive_ptr(def.gbd_head, base),
            relitive_ptr(def.global_buf, base),
            relitive_ptr(def.zero_block, base),
            def.rbd_hash.map(|x| relitive_ptr(x, base)),
            relitive_ptr(def.rbd_head, base),
            relitive_ptr(def.rbd_end, base),
            def.dirtyQ.map(|x| relitive_ptr(x, base)),
        )
    }

    fn assert_stat_eq(left: &DB_STAT, right: &DB_STAT) {
        //NOTE the brackets are needed so the value is copied.
        //otherwise assert_eq would end up creating unaligned references.
        assert_eq!({ left.dbget }, { right.dbget });
        assert_eq!({ left.dbset }, { right.dbset });
        assert_eq!({ left.dbkil }, { right.dbkil });
        assert_eq!({ left.dbdat }, { right.dbdat });
        assert_eq!({ left.dbord }, { right.dbord });
        assert_eq!({ left.dbqry }, { right.dbqry });
        assert_eq!({ left.lasttry }, { right.lasttry });
        assert_eq!({ left.lastok }, { right.lastok });
        assert_eq!({ left.logrd }, { right.logrd });
        assert_eq!({ left.phyrd }, { right.phyrd });
        assert_eq!({ left.logwt }, { right.logwt });
        assert_eq!({ left.phywt }, { right.phywt });
        assert_eq!({ left.blkalloc }, { right.blkalloc });
        assert_eq!({ left.blkdeall }, { right.blkdeall });
        assert_eq!({ left.blkreorg }, { right.blkreorg });
        assert_eq!({ left.diskerrors }, { right.diskerrors });
        assert_eq!({ left.diskerrors }, { right.diskerrors });
    }

    #[test]
    fn name_is_small() {
        let zeros = [0; VOL_FILENAME_MAX as usize + 1];
        let path = String::from("short_name");
        let name = format_file_name_helper(&path);
        let mut path = path.as_bytes().iter().map(|x| *x as i8).collect::<Vec<_>>();
        path.push(0);
        assert_eq!(&name[0..path.len()], &path[..]);
        assert_eq!(&name[path.len()..], &zeros[path.len()..]);
    }

    #[test]
    fn name_exact_size() {
        let path: String = "a".repeat(VOL_FILENAME_MAX as usize);
        let name = format_file_name_helper(&path);
        let mut path = path.as_bytes().iter().map(|x| *x as i8).collect::<Vec<_>>();
        path.push(0);
        assert_eq!(&name[..], &path[..]);
    }

    #[test]
    fn name_is_to_large() {
        use std::iter::{once, repeat};
        // path = ab...bc
        let path: String = once('a')
            .chain(repeat('b').take(VOL_FILENAME_MAX as usize))
            .chain(once('c'))
            .collect();
        let name = format_file_name_helper(&path);
        let mut path = path.as_bytes().iter().map(|x| *x as i8).collect::<Vec<_>>();
        path.push(0);
        //If the name is to long we only store the end of it.
        assert_eq!(
            &name[..],
            &path[path.len() - (VOL_FILENAME_MAX as usize + 1)..]
        );
    }
}
