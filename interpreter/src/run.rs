use crate::sys_tab::SYSTAB;

use ffi::{
    partab, strstk, systab, CleanJob, SemOp, UTIL_Share, Vhorolog, DO_FRAME, IN_TERM, JOBTAB,
    MAX_SSTK, MVAR, SEM_SYS, SERVERTAB, SQ_CHAN, TYPE_JOB, TYPE_RUN, VAR_U, VOL_DEF,
};
use std::ffi::CString;
use std::fs::OpenOptions;

/*
static mut strstk : [u_char;MAX_SSTK as usize] = [0;MAX_SSTK as usize];
static mut indstk : [u_char;MAX_SSTK as usize] = [0;MAX_SSTK as usize];
static isp:u64;                                                                    // indirect stack pointer
int     failed_tty = -1;                                                        // flag for tty reset
int     gbd_expired = GBD_EXPIRED;                                              // Set this
u_char  *rsmpc;                                                                 // RSM prog pointer

char    history[MAX_HISTORY][MAX_STR_LEN];                                      // history buffer
u_short hist_next = 0;                                                          // next history pointer
u_short hist_curr = 0;                                                          // history entry pointer
short   in_hist = FALSE;                                                        // are we in the history buffer
u_short prompt_len = 8;                                                         // length of the current direct mode prompt

*/

fn run(file: &str, env: Option<&str>, _command: &str) -> Result<(), String> {
    use std::os::fd::AsRawFd;
    unsafe {
        partab.jobtab = std::ptr::null::<JOBTAB>().cast_mut();
    }
    let start_type = TYPE_RUN;
    let cfile = CString::new(file.to_string()).unwrap();
    if start_type == TYPE_RUN {
        let mut file = OpenOptions::new().read(true).open(file).unwrap();
        (unsafe { UTIL_Share(cfile.into_raw()) } == 0)
            .then_some(0)
            .ok_or("RSM environment is not initialized.".to_string())?;

        let vol = unsafe { (*systab).vol[0] };
        (vol != std::ptr::null::<VOL_DEF>().cast_mut())
            .then_some(0)
            .ok_or(
                "Error occurred in process - Environment does not match runtime image version."
                    .to_string(),
            )?;
        //TODO exit


        let env_num =  env.and_then(|env|{
            let sys_tab = unsafe{systab.cast::<SYSTAB>().as_ref()}.unwrap();
            sys_tab.get_env_index(env)
        })
            .unwrap_or(1);


        let pid = unsafe { libc::getpid() };

        let ret = unsafe { SemOp(SEM_SYS as i32, -((*systab).maxjob as i32)) };
        (ret < 0).then_some(()).ok_or("EXIT")?;
        for i in 0..unsafe { *systab }.maxjob as usize {
            let other_pid = unsafe { *(*systab).jobtab.add(i) }.pid;

            if ((other_pid == 0) && (start_type == TYPE_RUN)) ||       // this one ?
            ((other_pid == pid) && (start_type == TYPE_JOB))
            {
                // or already done (JOB)
                let job_tab = JOBTAB {
                    pid,
                    async_error: 0,
                    attention: 0,
                    commands: 0,
                    cur_do: 0,
                    dostk: [DO_FRAME {
                        routine: std::ptr::null_mut(),
                        pc: std::ptr::null_mut(),
                        symbol: std::ptr::null_mut(),
                        newtab: std::ptr::null_mut(),
                        endlin: std::ptr::null_mut(),
                        rounam: "".try_into().unwrap(),
                        vol: 0,
                        uci: 0,
                        line_num: 0,
                        estack: 0,
                        type_: 0,
                        level: 0,
                        flags: 0,
                        savasp: 0,
                        savssp: 0,
                        asp: 0,
                        ssp: 0,
                        isp: 0,
                    }; 128],
                    error_frame: 0,
                    etrap_at: 0,
                    grefs: 0,
                    io: 0,
                    last_block_flags: 0,
                    last_ref: MVAR {
                        key: [0; 256],
                        name: "".try_into().unwrap(),
                        slen: 0,
                        uci: 0,
                        volset: 0,
                    },
                    luci: 0,
                    lvol: 0,
                    precision: 0,
                    priv_: 0,
                    ruci: 0,
                    rvol: 0,
                    seqio: [SQ_CHAN {
                        dkey: [0; 17],
                        dkey_len: 0,
                        dx: 0,
                        dy: 0,
                        fid: 0,
                        in_term: IN_TERM { iterm: 0 },
                        mode: 0,
                        name: [0; 256],
                        namespace: "".try_into().unwrap(),
                        options: 0,
                        out_len: 0,
                        out_term: [0; 6],
                        s: SERVERTAB {
                            slots: 0,
                            taken: 0,
                            cid: 0,
                            name: [0; 256],
                            forked: std::ptr::null_mut(),
                        },
                        type_: 0,
                    }; 64],
                    start_dh: [0; 14],
                    start_len: 0,
                    test: 0,
                    trap: 0,
                    uci: 0,
                    user: 0,
                    view: [std::ptr::null_mut(); 1],
                    vol: 0,
                };
                unsafe { *(*systab).jobtab.add(i) = job_tab };
                //partab.jobtab = &systab.jobtab[i];                                 // and save our jobtab address
                unsafe { partab.jobtab = (*systab).jobtab.add(i) };
                break;
            }

            let _ret = unsafe { SemOp(SEM_SYS as i32, (*systab).maxjob as i32) };
            /*
                if (partab.jobtab == NULL) {                                                // if that failed
                ret = ENOMEM;                                                           // error message
                goto exit;                                                              // and exit
            }
                 */
            unsafe { (*partab.jobtab).user = libc::getuid() as i32 };
        }
        if unsafe { (*partab.jobtab).user == (*systab).start_user }
            || unsafe { (*partab.jobtab).user == 0 }
        {
            // if he started it or is root
            unsafe {
                (*partab.jobtab).priv_ = 1;
            }
        } else {
            //TODO
            //Deal with latter.
        }
        {
            //Making this alias should be safe since it is scoped.
            let jobtab = unsafe { &mut (*partab.jobtab) };
            jobtab.precision = unsafe { *systab }.precision as i16;
            jobtab.uci = env_num;
            jobtab.vol = 1;
            jobtab.luci = env_num;
            jobtab.lvol = 1;
            jobtab.ruci = env_num;
            jobtab.rvol = 1;
            jobtab.start_len = unsafe { Vhorolog(jobtab.start_dh.as_ptr().cast_mut()) };
            jobtab.dostk[0].type_ = TYPE_RUN as u8;
        }
        let mut tty_settings = libc::termios {
            //These values will be overridden as soon as we call tcgetattr
            c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_cc: [0; 20],
            c_ispeed: 0,
            c_ospeed: 0,
        };

        let _failed_tty = unsafe { libc::tcgetattr(0, &mut tty_settings) };
        //unsafe {i = SQ_Init();}
        let index = unsafe { partab.jobtab.offset_from((*systab).jobtab) } as usize;
        unsafe { *systab }.last_blk_used[index] = 0;
        unsafe {
            partab.debug = 0;
        }
        unsafe { partab.strstk_start = strstk.as_mut_ptr() }; // address of strstk
        unsafe { partab.strstk_last = strstk.as_mut_ptr().add(MAX_SSTK as usize) };
        unsafe { partab.varlst = std::ptr::null_mut() }; // used by compiler
        unsafe { partab.vol_fds[0] = file.as_raw_fd() }; // make sure fd is right
                                                         //ST_Init();
    }
    todo!();
}

fn clean_old_job(pid: i32) {
    for i in 0..unsafe { *systab }.maxjob as isize {
        let ret = unsafe { *(*systab).jobtab.offset(i) }.pid;

        if (ret != pid) && ret != 0 {
            if unsafe { libc::kill(ret, 0) } != 0 {
                //TODO I do not know if this is correct.
                if unsafe { *libc::__error() } == libc::ESRCH {
                    unsafe { CleanJob(i as i32 + 1) };
                    break;
                }
            }
        } else {
            break;
        }
    }
}
