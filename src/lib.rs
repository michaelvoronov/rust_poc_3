#[no_mangle]
pub fn allocate(_size: usize) -> usize {
	0
}

#[no_mangle]
pub fn invoke(_ptr: i32, _size: usize) -> usize {
	0
}

#[no_mangle]
pub unsafe fn foo() {
	ffi::store(1, 1);
	ffi::load(1);
	deallocate_extern(1, 1);
	allocate_extern(1);
	ffi::invoke(1, 1);
}

#[link(wasm_import_module = "sqlite")]
extern "C" {
    #[link_name="allocate"]
    pub fn allocate_extern(size: usize) -> usize;
    #[link_name="deallocate"]
    pub fn deallocate_extern(ptr: i32, size: usize);
}

mod ffi {
	#[link(wasm_import_module="sqlite")]
	extern "C" {
	    pub fn invoke(ptr: i32, size: usize) -> usize;
	    pub fn store(ptr: i32, byte: u8);
	    pub fn load(ptr: i32) -> u8;
	}
}

