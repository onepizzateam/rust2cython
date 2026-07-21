pub struct Db { conn: String }
pub extern "C" fn db_new() -> *mut Db {
    Box::into_raw(Box::new(Db { conn: String::new() }))
}
pub extern "C" fn db_free(ptr: *mut Db) { unsafe { drop(Box::from_raw(ptr)) } }
