// SPDX-License-Identifier: GPL-3.0-or-later
use core::ffi::{c_char,c_int};
unsafe extern "C" { fn rboxc_updatedb_environment()->c_int; }
pub unsafe extern "C" fn main(argc:c_int,argv:*mut *mut c_char)->c_int {
    if rboxc_updatedb_environment()!=0 {
        super::write_all(2,b"updatedb: cannot prepare internal command paths\n");return 1
    }
    super::bash_dispatch::run_script(argc,argv,include_str!("generated/updatedb.sh").to_owned())
}
