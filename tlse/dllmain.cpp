#include "pch.h"

bool* GFullscreen = (bool*) 0x137544a;

void init(HINSTANCE hInstance) {
    *GFullscreen = false;

    IDirectInput8* pDirectInput = NULL;

    if (DirectInput8Create(hInstance, DIRECTINPUT_VERSION, IID_IDirectInput8, (LPVOID*)&pDirectInput, NULL) != DI_OK) {
        printf("Hello");
        return;
    }
}

BOOL APIENTRY DllMain(HINSTANCE hInstance, DWORD ul_reason_for_call, LPVOID lpReserved) {
    switch (ul_reason_for_call) {
        case DLL_PROCESS_ATTACH:
            AllocConsole();
            init(hInstance);
            break;
        case DLL_PROCESS_DETACH:
            FreeConsole();
            break;
        default:
            break;
    }

    return TRUE;
}

// use std::mem::transmute;
// use std::fmt;
// use std::ops::{Deref, DerefMut};
// use std::marker::PhantomData;
// use std::os::raw::{c_char, c_float, c_long, c_uchar, c_ulong, c_void, c_double, c_uint, c_int};

// use winapi::shared::d3d9::{IDirect3DBaseTexture9,IDirect3D9,IDirect3DDevice9,IDirect3DSurface9,IDirect3DTexture9,IDirect3DQuery9,IDirect3DIndexBuffer9,IDirect3DVertexDeclaration9,IDirect3DVertexShader9};
// use winapi::shared::d3d9caps::D3DCAPS9;
// use winapi::shared::d3d9types::{D3DDISPLAYMODE,D3DPRESENT_PARAMETERS,D3DVIEWPORT9,D3DTEXTURESTAGESTATETYPE,D3DRENDERSTATETYPE,D3DSAMPLERSTATETYPE,D3DVERTEXELEMENT9};
// use winapi::shared::minwindef::HINSTANCE;
// use winapi::shared::ntdef::WCHAR;
// use winapi::shared::windef::HWND;
// use winapi::shared::basetsd::UINT32;
// use winapi::shared::guiddef::GUID;
// use winapi::um::winnt::RTL_CRITICAL_SECTION;
// use winapi::um::winuser::WNDCLASSEXW;
// use winapi::ctypes::wchar_t;

// use lazy_static::lazy_static;

// // macro_rules! class {
// //     ($name:ident ; $($rest:tt)*) => {
// //         mod $name {
// //             specify! { $($rest)* }
// //         }
// //     }
// // }

// // macro_rules! specify {
// //     ($addr:literal : unsafe fn $name:ident ( $($t:ty),* ) ; $($rest:tt)*) => {
// //         specify! {$addr : unsafe fn $name:ident ( $($t),* ) -> () ; $($rest)*};
// //     };
// //     ($addr:literal : unsafe fn $name:ident ( $($t:ty),* ) -> $r:ty ; $($rest:tt)*) => {
// //         specify! {$addr : unsafe extern "Rust" fn $name:ident ( $($t),* ) -> $r ; $($rest)*};
// //     };
// //     ($addr:literal : unsafe extern $abi:literal fn $name:ident ( $($t:ty),* ) ; $($rest:tt)*) => {
// //         specify! {$addr : unsafe extern $abi fn $name ( $($t),* ) -> () ; $($rest)*};
// //     };
// //     ($addr:literal : unsafe extern $abi:literal fn $name:ident ( $($t:ty),* ) -> $r:ty ; $($rest:tt)*) => {
// //         specify_item! {$addr : unsafe extern $abi fn $name ( $($t),* ) -> $r}
// //         specify! {$($rest)*}
// //     };
// //     () => ()
// // }

// // macro_rules! specify_item {
// //     ($addr:literal : unsafe extern $abi:literal fn $name:ident ( $($t:ty),* ) -> $r:ty) => {
// //         struct $name ;

// //         static mut CODE_SPLICE: Option<[u8; 8]> = None;

// //         impl std::ops::Deref for $name {
// //             type Target = unsafe extern $abi fn ( $($t),* ) -> $r;
// //             fn deref(&self) -> &Self::Target {
// //                 unsafe { transmute($addr) }
// //             }
// //         }

// //         impl $name {
// //             fn hook()
// //         }
// //     }
// // }

// // specify! {
// //     0x00402510: unsafe extern "fastcall" fn GFMain(*mut HINSTANCE, *mut c_char, c_int) -> c_long ;
// // }

// // macro_rules! class {
// //     ($c:ident $b:tt $($r:tt)*) => {
// //         class_block! { $c @ $b }
// //         class! { $($r)* }
// //     };
// //     () => ()
// // }

// // macro_rules! class_block {
// //     ($c:ident @ { $($r:tt)* }) => {
// //         class_block! { $c @ $($r)* }
// //     };
// //     ($c:ident @ vmt $n:literal $b:tt $($r:tt)*) => {
// //         class_virtual! { $c @ $n $b }
// //         class_block! { $c @ $($r)* }
// //     };
// //     ($c:ident @ extends $($extends:ident),* ; $($rest:tt)*) => {
// //         // TODO
// //         class_block! { $c @ $($rest)* }
// //     };
// //     ($c:ident @ layout $b:tt $($r:tt)*) => {
// //         class_layout! { $c @ $b }
// //         class_block! { $c @ $($r)* }
// //     };
// //     ($c:ident @) => ()
// // }

// // macro_rules! class_virtual {
// //     ($c:ident @ $n:literal { $($r:tt)* }) => {
// //         // TODO
// //         class_virtual! { $c @ $($r)* }
// //     };
// //     ($c:ident @) => ()
// // }

// // macro_rules! class_layout {
// //     ($c:ident @ { $($r:tt)* }) => {
// //         // TODO
// //         class_virtual! { $c @ $($r)* }
// //     };
// //     ($c:ident @) => ()
// // }

// // class! {
// //     CMainGameComponent {
// //         extends Foo, Bar;
// //         vmt 0x0 {}
// //         layout {}
// //     }
// //     Bar {

// //     }
// // }

// macro_rules! specify {
//     ($p:literal : unsafe fn $f:ident ( $($i:ident : $t:ty),* ) ; $($x:tt)*) => {
//         specify! {$p : unsafe fn $f ( $($i : $t),* ) -> () ; $($x)*};
//     };

//     ($p:literal : unsafe fn $f:ident ( $($i:ident : $t:ty),* ) -> $r:ty ; $($x:tt)*) => {
//         specify! {$p : unsafe extern "Rust" fn $f ( $($i : $t),* ) -> $r ; $($x)*};
//     };

//     ($p:literal : unsafe extern $a:literal fn $f:ident ( $($i:ident : $t:ty),* ) ; $($x:tt)*) => {
//         specify! {$p : unsafe extern $a fn $f ( $($i : $t),* ) -> () ; $($x)*}
//     };

//     ($p:literal : unsafe extern $a:literal fn $f:ident ( $($i:ident : $t:ty),* ) -> $r:ty ; $($x:tt)*) => {
//         struct $f;

//         impl std::ops::Deref for $f {
//             type Target = unsafe extern $a fn ( $($i : $t),* ) -> $r;
//             fn deref(&self) -> &Self::Target {
//                 unsafe { std::mem::transmute($p) }
//             }
//         }

//         impl $f {
//             fn hook(&self) {
//                 // TODO
//             }
//         }

//         specify! { $($x)* }
//     };

//     () => ()
// }

// specify! {
//     0x00402510: unsafe extern "fastcall" fn GFMain(a: *mut HINSTANCE, b: *mut c_char, c: c_int) -> c_long;
//     0x00416e78: unsafe extern "fastcall" fn CMainGameComponent__GetInputs(x: *mut CMainGameComponent);
// }

// // specify! {
// //     CMainGameComponent {
// //         0x00416e78: unsafe extern "fastcall" fn CMainGameComponent__GetInputs(x: *mut CMainGameComponent);
// //     }
// //     // 0x00402510: unsafe extern "fastcall" fn ::GFMain(a: *mut HINSTANCE, b: *mut c_char, c: c_int) -> c_long;
// //     // // 0x00413c50: unsafe extern "stdcall" fn GFRunInitScripts();
// //     // 0x0041649c: unsafe extern "fastcall" fn CMainGameComponent::AddGameEvent(a: *mut CMainGameComponent, b: *mut CGameEvent);
// //     // 0x00416e78: unsafe extern "fastcall" fn CMainGameComponent__GetInputs(*mut CMainGameComponent);
// //     // 0x00417001: unsafe extern "fastcall" fn CMainGameComponent__Render(*mut CMainGameComponent);
// //     // 0x00418289: unsafe extern "fastcall" fn CMainGameComponent__Update(*mut CMainGameComponent);
// //     // 0x004184bd: unsafe extern "fastcall" fn CMainGameComponent__Init(*mut CMainGameComponent);
// //     // 0x004189c2: unsafe extern "fastcall" fn CMainGameComponent__Run(*mut CMainGameComponent, *mut *mut CGameComponent) -> c_uint;
// //     // 0x004189c2: unsafe extern "fastcall" fn Run(*mut CMainGameComponent);
// //     // 0x00435530: unsafe extern "thiscall" fn CDisplayEngine__DoRender(*mut CDisplayEngine, *mut CInterpolationInfo, bool, bool);
// //     // 0x00554600: unsafe extern "thiscall" fn CPlayer__GetCurrentMode(this: *mut CPlayer) -> EPlayerMode;
// //     // 0x009c0e50: unsafe extern "fastcall" fn CDisplayManager__Init(*mut CDisplayManager, *mut CDisplayManagerInit, bool);
// //     // 0x009dd8f0: unsafe extern "thiscall" fn CRenderManager2D__Draw2DText(*mut CRenderManager2D, *mut C2DVector, CWideString, *mut CFontBank, *mut CRGBColour, c_ulong) -> bool;
// //     // 0x009f57a0: unsafe extern "fastcall" fn CInputManager__Update(*mut CInputManager);
// // }

// pub static mut CMainGameComponentFns: *mut CMainGameComponentVmt = unsafe { transmute::<usize,_>(0x122f180) };
// // pub static mut CInputManagerFns: *mut CInputManagerVmt = unsafe { transmute::<usize,_>(0x29d620) };

// pub static mut GFullscreen: *mut bool = unsafe { transmute::<usize,_>(0x137544a) };
// pub static mut GRunIniScripts: *mut bool = unsafe { transmute::<usize,_>(0x137548f) };
// pub static mut GSystemManager: *mut CSystemManager = unsafe { transmute::<usize,_>(0x13ca618) };
// pub static mut GForceRenderFramesPerSec: *mut c_long = unsafe { transmute::<usize,_>(0x13b8630) };
// pub static mut GPRenderManager: *mut CRenderManager = unsafe { transmute::<usize,_>(0x13b8384) };

// #[derive(Debug)]
// #[repr(C)]
// pub struct BoostScopedPtr<T>(pub *mut T);

// impl<T> Deref for BoostScopedPtr<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         // unsafe { &*self.0 }
//         // NOTE: For some reason reborrowing hangs the program. Instead I just convert the pointer into a reference.
//         // unsafe { transmute::<*const T, &T>(self.0) }
//         unsafe { self.0.as_ref().unwrap() }
//     }
// }

// impl<T> DerefMut for BoostScopedPtr<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         unsafe { self.0.as_mut().unwrap()  }
//         // unsafe {&mut *self.0}
//     }
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CxxAllocator<T> {
//     t: PhantomData<T>,
// }

// /// This only works when the value type is WCHAR because the buffer and alias sizes are 16 / 2.
// /// If Rust becomes smarter then `[A; 16 / mem::size_of::<A>()]` could be possible.
// #[derive(Debug)]
// #[repr(C)]
// pub struct CxxBasicString<A: Sized, B = CxxCharTraits<A>, C = CxxAllocator<A>> {
//     pub buf: [A; 8],
//     pub ptr: *mut A,
//     pub alias: [c_char; 8],
//     pub size: UINT32,
//     pub res: UINT32,
//     pub alloc: C,
//     _traits: PhantomData<B>,
// }

// /// This has no fields and only exists for static methods.
// #[derive(Debug)]
// #[repr(C)]
// pub struct CxxCharTraits<A> {
//     _value_type: PhantomData<A>,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CxxLess<T> {
//     _elem_type: PhantomData<T>,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CxxList<T, A = CxxAllocator<T>> {
//     pub proxy: *mut (),
//     pub head: *mut CxxListNode<T>,
//     pub size: u32,
//     pub _alloc_node: CxxAllocator<CxxListNode<T>>,
//     pub _alloc_value: A,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CxxListNode<T> {
//     pub next: *mut CxxListNode<T>,
//     pub prev: *mut CxxListNode<T>,
//     pub value: T,
// }

// // #[repr(C)]
// // pub struct CxxListIterator<T> {
// // }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CxxMap<
//     Key,
//     T,
//     Compare = CxxLess<Key>,
//     Alloc = CxxAllocator<CxxPair<Key, T>>,
// > {
//     pub proxy: *mut (),
//     pub comp: Compare,
//     pub head: *mut CxxMap<T, Key, Compare, Alloc>,
//     pub _aloc_node: CxxAllocator<CxxMap<T, Key, Compare, Alloc>>,
//     pub _alloc_value: Alloc,
// }
// #[derive(Debug)]
// #[repr(C)]
// pub struct CxxPair<A, B>(pub A, pub B);

// #[derive(Debug)]
// #[repr(C)]
// pub struct CxxSet<T, C = CxxLess<T>, A = CxxAllocator<T>> {
//     pub comp: C,
//     pub head: *mut CxxSetNode,
//     pub size: u32,
//     pub alloc_node: CxxAllocator<CxxSet<T, C, A>>,
//     pub alloc_value: A,
//     _elem_type: PhantomData<T>,
// }

// pub struct CxxSetNode {
//     pub left: *mut CxxSetNode,
//     pub parent: *mut CxxSetNode,
//     pub right: *mut CxxSetNode,
//     // Should this be a type parameter?
//     pub val: c_ulong,
//     pub color: c_char,
//     pub is_nil: c_char,
// }

// #[repr(C)]
// pub struct CxxVector<T, A = CxxAllocator<T>> {
//     pub first: *mut T,
//     pub last: *mut T,
//     pub end: *mut T,
//     pub allocator: A,
// }

// impl<T: Sized, A> CxxVector<T, A> {
//     pub fn as_slice(&self) -> &[T] {
//         unsafe { std::slice::from_raw_parts(self.first, self.len()) }
//     }

//     pub fn as_mut_slice(&mut self) -> &mut [T] {
//         unsafe { std::slice::from_raw_parts_mut(self.first, self.len()) }
//     }

//     pub fn len(&self) -> usize {
//         Self::ptr_offset_from(self.first, self.last)
//     }

//     pub fn capacity(&self) -> usize {
//         Self::ptr_offset_from(self.first, self.end)
//     }

//     pub fn push(&mut self, x: T) {
//         unsafe {
//             self.last = self.last.add(1);

//             if self.last > self.end {
//                 let item_size = std::mem::size_of::<T>();
//                 let new_vec_capacity = self.capacity() * 2;
//                 let restore_vec_len = self.len();
//                 let layout = std::alloc::Layout::from_size_align_unchecked(self.capacity() * item_size, item_size);

//                 self.first = std::alloc::realloc(self.first as *mut u8, layout, new_vec_capacity * item_size) as *mut T;
//                 self.end = self.first.add(new_vec_capacity);
//                 self.last = self.first.add(restore_vec_len + 1);
//             }

//             *self.last = x;
//         }
//     }

//     // Using this scary method until ptr_offset_from is stabilized.
//     // https://doc.rust-lang.org/std/primitive.pointer.html#method.offset_from
//     fn ptr_offset_from(a: *mut T, b: *mut T) -> usize {
//         let size = std::mem::size_of::<T>();
//         if size == 0 {
//             return 0;
//         }
//         // debug_assert!(
//         //     !a.is_null() && !b.is_null(),
//         //     "The pointers must be non-null."
//         // );
//         debug_assert!(
//             a <= b,
//             "The first pointer cannot come after the second pointer."
//         );
//         let dist = b as usize - a as usize;
//         dist / size
//     }
// }

// impl<T: fmt::Debug, A> fmt::Debug for CxxVector<T, A> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_list().entries(self.as_slice()).finish()
//     }
// }


// /// Top left and bottom right positions.
// #[derive(Debug)]
// #[repr(C)]
// pub struct C2DBoxF {
//     pub tl_x: c_float,
//     pub tl_y: c_float,
//     pub br_x: c_float,
//     pub br_y: c_float,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct C2DCoordI {
//     pub x: c_long,
//     pub y: c_long,
// }

// #[derive(Debug,Copy,Clone)]
// #[repr(C)]
// pub struct C2DVector {
//     pub x: c_float,
//     pub y: c_float,
// }
// #[derive(Debug)]
// #[repr(C)]
// pub struct C3DAnimationManager {}

// #[derive(Debug,Copy,Clone)]
// #[repr(C)]
// pub struct C3DVector {
//     pub x: c_float,
//     pub y: c_float,
//     pub z: c_float,
// }
// #[derive(Debug)]
// #[repr(C)]
// pub struct CAIGameCameraBase {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CArray<T> {
//     pub cxx_std_vector: CxxVector<T, CxxAllocator<T>>,
// }

// impl<T> CArray<T> {}
// #[derive(Debug)]
// #[repr(C)]
// pub struct CAtmosCopyInfo {}
// #[derive(Debug)]
// #[repr(C)]
// pub struct CAtmosProcessor {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CBankFile {
//     pub c_base_class_non_copyable: CBaseClassNonCopyable,
//     pub size: c_ulong,
//     pub symbols: CArray<CCharString>,
//     pub checksums: CArray<c_ulong>,
//     pub runtime_data: CArray<self::CRuntimeData>,
//     pub update_data: CArray<*mut CBankFileEntryUpdateData>,
//     pub packed_data_offset: CPackedUIntArray,
// }

// impl CBankFile {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CRuntimeData {
//     pub data_offset: c_ulong,
//     pub data_size: c_ulong,
//     pub data_type: c_uchar,
//     pub valid: bool,
// }

// impl CRuntimeData {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CBankFileAsync {
//     pub c_bank_file: CBankFile,
// }
// // use std::os::raw::{c_ulong,c_char};

// // use crate::{CBankStateBlock,CCharString,CCountedPointer,CSmallVector};

// /// TODO: I've left this empty for now because its behind a pointer.
// #[derive(Debug)]
// #[repr(C)]
// pub struct CBankFileEntryUpdateData {
//     // pub state_block_crc: c_ulong,
// // pub info_size: c_ulong,
// // pub info_buffer: *mut c_char,
// // pub filenames: CSmallVector<CCharString, 8>,
// // pub requires_update: bool,
// // pub exists: bool,
// // pub state_block: CCountedPointer<CBankStateBlock>,
// }
// // #[repr(C)]
// // #[derive(Debug)]
// // pub struct CBankStateBlock {}
// #[derive(Debug)]
// #[repr(C)]
// pub struct CBaseClass {}

// impl CBaseClass {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CBaseClassNonCopyable {
//     pub c_base_class: CBaseClass,
// }

// impl CBaseClassNonCopyable {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CBaseIntelligentPointer {
//     pub p_data: *mut CBaseObjectPointer,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CBaseObject {
//     pub vmt: *mut (),
//     pub c_base_class: CBaseClass,
//     pub intelligent_pointer: *mut CBaseObjectPointer,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CBaseObjectPointer {
//     pub object: *mut CBaseObject,
//     pub ref_count: c_ulong,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CBasicString<T> {
//     pub p_data: *mut c_char,
//     pub string_length: c_ulong,
//     /// This type was unnamed but 4 bytes long.
//     pub data_length: u32,
//     /// This type was unnamed but 4 bytes long.
//     pub use_fast_extend: u32,
//     pub elem_type: PhantomData<T>,
// }
// #[derive(Debug)]
// #[repr(C)]
// pub struct CBulletTimeManager {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CCamera {
//     pub world_position: C3DVector,
//     pub rh_set: CRightHandedSet,
//     pub height_locked: bool,
//     pub view_vec_z_locked: bool,
//     pub fov_flags: c_ulong,
//     pub horizontal_fov: c_float,
//     pub vertical_fov: c_float,
//     pub zoom: c_float,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CCharString {
//     data: *mut CCharStringData,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CCharStringData {
//     pub data: CBasicString<c_char>,
//     pub refs_count: c_long,
// }

// impl CCharString {
//     pub fn new(mut s: String) -> CCharString {
//         CCharString {
//             data: &mut CCharStringData {
//                 data: CBasicString {
//                     p_data: s.as_mut_ptr() as *mut c_char,
//                     string_length: s.len() as u32,
//                     data_length: s.as_bytes().len() as u32,
//                     use_fast_extend: 0,
//                     elem_type: PhantomData,
//                 },
//                 refs_count: 0,
//             } as *mut CCharStringData,
//         }
//     }
// }

// // impl CCharString {
// //     fn as_str(&self) -> &str {
// //         let buf = self.data.data.p_data as *mut u8;
// //         let len = self.data.data.string_length;
// //     }
// // }
// #[derive(Debug)]
// #[repr(C)]
// pub struct CCombatManager {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CCountedPointer<T> {
//     pub data: *mut T,
//     pub info: *mut CCPPointerInfo,
// }

// impl<T> CCountedPointer<T> {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CCPPointerInfo {
//     pub ref_count: c_long,
//     /// Needs looking into. Produced an unnamed type that is 4 bytes. Its probably some function pointer.
//     pub delete_func: usize,
//     pub data: *mut c_void,
// }

// impl CCPPointerInfo {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CCounter {
//     pub relevant_world_frame: c_long,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CCRCSymbolMap {
//     pub c_base_class_non_copyable: CBaseClassNonCopyable,
//     pub s_long_map: CVectorMap<c_ulong, c_ulong, CKeyPairCompareLess<c_ulong, c_long>>,
// }

// impl CCRCSymbolMap {}
// // use winapi::um::winnt::WCHAR;

// #[derive(Debug)]
// #[repr(C)]
// pub struct CDebugManager {
//     // pub system_manager: *mut CSystemManager,
// // pub vcalid: bool,
// // pub exclusive: bool,
// // pub verbose: bool,
// // pub can_do_verbose: bool,
// // pub debug_manager_critical_section: RTL_CRITICAL_SECTION,
// // pub log_messages: bool,
// // pub log_errors: bool,
// // // This is an stdio.h FILE but I don't know whats best to bind to.
// // pub log_file: *mut (),
// // pub project_directory: [WCHAR; 263],
// // pub error_display_info: [CErrorDisplayInfo; 5],
// // pub p_exclusive_assert: CExclusiveAssert,
// // pub win_instance: HINSTANCE,
// // pub win_handle: HWND,
// // pub errors_off: CxxSet<c_ulong>,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CDefClassBase {
//     pub vmt: *mut (),
//     pub c_def_pointee_base: CDefPointeeBase,
//     pub p_def_manager: CDefinitionManager,
//     pub global_index: c_ulong,
//     pub setup: UnknownEmptyType,
//     pub template: UnknownEmptyType,
//     pub default_vals_applied: UnknownEmptyType,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CSubDefInfo {
//     pub def_index: c_long,
//     pub original_parent_def_index: c_long,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CDefPointer<T> {
//     pub object: *mut CDefPointeeBase,
//     _elem_type: PhantomData<T>,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CDefPointeeBase {
//     pub vmt: *mut (),
//     pub c_resource: CResource,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CResourceList {
//     pub vmt: *mut (),
//     pub c_failed_allocation_handler: CFailedAllocationHandler,
//     pub head: CResource,
//     pub resource_count: c_ulong,
//     pub allocated_memory: c_long,
//     pub maximum_memory: c_long,
//     pub current_frame: c_ulong,
//     pub debug_stats_frame: c_ulong,
//     pub unloaded_delay: c_ulong,
//     pub unload_this_frame: c_ulong,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CResource {
//     pub vmt: *mut (),
//     pub c_iv_counted_pointee_base: CIVCountedPointeeBase,
//     pub resource_list: *mut CResourceList,
//     pub prev_resource: *mut CResource,
//     pub next_resource: *mut CResource,
//     pub resource_size: c_ulong,
//     pub last_used_frame: c_ulong,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CIVCountedPointeeBase {
//     pub iv_ref_count: c_ulong,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CFailedAllocationHandler {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CDefString {
//     pub table_pos: c_long,
// }
// /// TODO: This is behind a pointer so I've left it empty for now.
// #[derive(Debug)]
// #[repr(C)]
// pub struct CDefinitionManager {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CDeviceResetCallback {}

// impl CDeviceResetCallback {}

// /// TODO: This is behind a pointer so I've left it empty for now.
// #[derive(Debug)]
// #[repr(C)]
// pub struct CDiskFileWin32 {
//     pub vmt: *mut (),
// }

// /// TODO: WIP
// #[derive(Debug)]
// #[repr(C)]
// pub struct CDisplayEngine {
//     pub vmt: *mut (),
//     pub c_base_class_non_copyable: CBaseClassNonCopyable,
//     pub engine_preload_flags: c_ulong,
//     pub component: *const CMainGameComponent,
//     pub player_manager: *const CPlayerManager,
//     pub definition_manager: *const CGameDefinitionManager,
//     pub engine_graphic_bank: *const CGraphicDataBank,
//     pub mesh_bank: *mut CMeshDataBank,
//     pub main_window: C2DBoxF,
//     pub engine_3d: *mut CIEngine,
//     pub camera: CCamera,
//     pub frame: c_long,
//     pub draw_game: bool,
//     pub letter_box: CLetterBoxModeInfo,
//     pub camera_inputs_on: bool,
//     pub last_refresh_time: c_double,
//     pub last_render_time_length: c_double,
//     pub last_world_update_render_time: c_double,
//     pub prepare_primitives: bool,
//     pub gamma_ramp: c_float,
//     pub old_gamma_ramp: c_float,
//     pub screen_fade_out_info: CFadeInFadeOutBase,
//     pub screen_fade_out_locked: bool,
//     pub draw_memory_use: bool,
//     pub draw_debug_page: c_long,
//     pub p_view_manager: CCountedPointer<CDisplayViewManager>,
//     pub initial_faide_readyness_count: c_long,
//     pub initial_fade_active: bool,
//     pub screen_fade_duration: c_float,
//     pub time_since_fade_started: c_float,
//     pub world: *const CWorld,
// }

// // #[derive(Debug)]
// #[repr(C)]
// pub struct CDisplayManager {
//     pub vmt: *mut (),
//     pub c_base_class_non_copyable: CBaseClassNonCopyable,
//     pub system_manager: *mut CSystemManager,
//     pub p_render_manager: BoostScopedPtr<CRenderManager>,
//     pub render_target_surface: CSurface,
//     pub render_target_depth_buffer: CSurface,
//     pub back_buffer_surface: CSurface,
//     pub depth_buffer_surface: CSurface,
//     pub back_buffer_copy_texture: CTexture,
//     pub d3d: *mut IDirect3D9,
//     pub d3d_device: *mut IDirect3DDevice9,
//     pub d3d_device_caps: D3DCAPS9,
//     pub back_buffer_dimensions: C2DExtentsI,
//     pub render_target_dimensions: C2DExtentsI,
//     pub back_buffers_count: c_long,
//     pub exclusive_mode: bool,
//     pub multisample_type: ESurfaceMultisampleType,
//     pub back_buffer_format: CPixelFormat,
//     pub depth_format_type: CPixelFormat,
//     pub using_render_manager: bool,
//     pub using_shader_render_manager: bool,
//     pub window_handle: *mut HWND,
//     pub current_mode: D3DDISPLAYMODE,
//     pub refresh_rate: c_long,
//     pub presentation_interval: CPresentationIntervalDesc,
//     pub presentation_interval_time: c_float,
//     pub viewport_offscreen: bool,
//     pub d3d_viewport: D3DVIEWPORT9,
//     pub viewport_box: C2DBoxI,
//     pub viewport_box_float: C2DBoxF,
//     pub rendering: bool,
//     pub can_begin_render: bool,
//     pub current_screen_index: c_long,
//     pub present_params: D3DPRESENT_PARAMETERS,
//     pub virtual_coords_resolution_independent: bool,
//     pub device_lost_flag: bool,
//     pub shader_resource_path: CWideString,
//     pub frame_counter: c_long,
//     pub render_manager_init_flags: c_ulong,
//     pub device_reset_callbacks: CxxList<*mut CDeviceResetCallback>,
//     pub last_device_creation_parameters: CCreateDeviceParameters,
//     pub vsync_callback_timer: BoostScopedPtr<CVsyncCallbackTimer>,
//     pub back_buffer_formats: CArray<CPixelFormat>,
//     pub multisample_types: CArray<ESurfaceMultisampleType>,
//     pub busy_resource_manager: CBusyResourceManager,
//     pub gamma: c_float,
//     pub want_hi_res_screenshot: bool,
//     pub taking_screenshot: bool,
// }

// // #[derive(Debug)]
// #[repr(C)]
// pub struct CRenderManager {
//     pub vmt: *mut (),
//     pub c_render_manager_2d: CRenderManager2D,
// }

// // #[derive(Debug)]
// #[repr(C)]
// pub struct CRenderManager2D {
//     pub vmt: *mut (),
//     pub c_render_manager_core: CRenderManagerCore,
//     pub vertex_shader_bank: CShaderDataBank,
//     pub vertex_shader_resource: CCountedPointer<CShaderResource>,
//     pub vertex_shader_2d: CVertexShader,
//     pub quick_draw_vertices: CxxVector<CTVertexRHWColSpecTex1>,
//     pub quick_draw_tri_info: CxxVector<CQuickDrawTriInfo>,
//     pub quick_draw_alpha_vertices: CxxVector<CTVertexRHWColSpecTex1>,
//     pub quick_draw_alpha_tri_info: CxxVector<CQuickDrawTriInfo>,
//     pub quick_draw_2d_vertices: CxxVector<CTVertexRHWColSpecTex1>,
//     pub quick_draw_2d_tri_info: CxxVector<CQuickDrawTriInfo>,
//     pub alternate_render_target_clear_enabled: bool,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CQuickDrawTriInfo {
//     pub line: UnknownEmptyType,
//     pub is_text: UnknownEmptyType,
//     pub window: C2DBoxF,
//     pub string: CWideString,
//     pub uploaded_texture: CTexture,
//     pub blend_mode: c_ulong,
//     pub z_average: c_float,
//     pub string_x: c_float,
//     pub string_y: c_float,
//     pub string_z: c_float,
//     pub string_scale: c_float,
//     pub string_flags: c_ulong,
//     pub clour: CRGBColour,
//     pub p_font: *const CFontBank,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CTVertexRHWColSpecTex1 {
//     pub c_generic_vertex: CGenericVertex<CTVertexRHWColSpecTex1Base>,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CGenericVertex<T> {
//     pub ext: T,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CTVertexRHWColSpecTex1Base {
//     pub x: c_float,
//     pub y: c_float,
//     pub z: c_float,
//     pub rhw: c_float,
//     pub diffuse_colour: c_ulong,
//     pub specular_colour: c_ulong,
//     pub u1: c_float,
//     pub v1: c_float,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CVertexShader {
//     pub vmt: *mut (),
//     pub c_shader_base: CShaderBase,
//     pub data: *mut CVertexShaderData,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CVertexShaderData {
//     pub vmt: *mut (),
//     pub c_base_class_non_copyable: CBaseClassNonCopyable,
//     pub vertex_declaration: *mut IDirect3DVertexDeclaration9,
//     pub vertex_shader: *mut IDirect3DVertexShader9,
//     pub format_decl: *const D3DVERTEXELEMENT9,
//     pub reference_count: c_ulong,
// }

// // #[derive(Debug)]
// #[repr(C)]
// pub struct CShaderResource {
//     pub c_base_class_non_copyable: CBaseClassNonCopyable,
//     pub shader_bank: *const CShaderDataBank,
//     pub shaders: CArray<*mut CShaderBase>,
//     pub shader_render_manager: *mut CShaderRenderManager,
//     pub vertex_shader_input: CVertexShaderInput,
//     pub vertex_shader_declaration: *const D3DVERTEXELEMENT9,
//     pub vertex_format: EVertexType,
// }

// // #[derive(Debug)]
// #[repr(C)]
// pub struct CVertexShaderInput {
//     pub vmt: *mut (),
//     pub c_base_class: CBaseClass,
//     pub data_streams: CArray<CStreamInfo>,
//     pub d3d_declaration: D3DVERTEXELEMENT9,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CStreamInfo {
//     pub stream_index: c_ulong,
//     pub stream_format: CVertexStreamFormat,
// }



// #[derive(Debug)]
// #[repr(C)]
// pub struct CVertexStreamFormat {
//     pub vmt: *mut (),
//     pub c_base_class: CBaseClass,
//     pub data_elements: CArray<CDataElement>,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CDataElement {
//     pub register: c_ulong,
//     pub data_type: EVertexStreamDataType,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CShaderRenderManager {
//     pub vmt: *mut (),
//     pub c_base_class_non_copyable: CBaseClassNonCopyable,
//     // TODO: Unfinished
// }

// // NOTE: This only has static methods.
// #[derive(Debug)]
// #[repr(C)]
// pub struct CShaderBase;

// #[derive(Debug)]
// #[repr(C)]
// pub struct CShaderDataBank {
//     pub vmt: *mut (),
//     pub c_bank_file: CBankFile,
//     pub shaders: CArray<*mut CShaderBankEntry>,
//     pub shader_pre_parser: *mut CShaderPreParser,
//     pub entry_parser_states: CxxMap<c_ulong, CCountedPointer<CShaderParserState>>,
//     pub immediate_file_folder: CCharString,
//     pub state_block: CShaderBankStateBlock,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CShaderPreParser {
//     pub include_directories: CArray<CWideString>,
//     pub state: CShaderParserState,
//     pub state_stack: CxxList<CShaderParserState>,
//     pub entered_symbols: CxxSet<CShaderParserSymbolSignature>,
//     pub current_debug_context_line_number: c_ulong,
//     pub current_debug_context_string: CCharString,
//     pub constant_names: CArray<CCharString>,
//     pub parse_errors: CArray<CCharString>,
//     pub legal_symbol_chars: CArray<bool>,
//     pub next_const_register: CCharString,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CShaderParserState {
//     pub symbols: CxxMap<CCharString, CShaderParserSymbol>,
//     pub exclusive_names: CxxSet<CCharString>,
//     pub const_start_register: CCharString,
//     pub include_dependencies: CArray<CWideString>,
//     pub vs_constant_layout: CCharString,
//     pub pixel_shader_Version: c_long,
//     pub registers: CArray<CRegister>,
//     pub macros: CArray<CMacro>,
// }
// #[derive(Debug)]
// #[repr(C)]
// pub struct CRegister {
//     pub symbol_name: CCharString,
//     pub name: CCharString,
//     pub free: bool,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CMacro {
//     pub name: CCharString,
//     pub code: CCharString,
//     pub param_def: CArray<CCharString>,
//     pub debug_context_string: CCharString,
//     pub debug_context_line_number: c_long,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CShaderParserSymbol {
//     pub c_shader_parser_symbol_signature: CShaderParserSymbolSignature,
//     pub value: CCharString,
//     pub typ: EType,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CShaderParserSymbolSignature {
//     pub name: CCharString,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CShaderBankStateBlock {
//     pub vmt: *mut (),
//     pub c_bank_state_block: CBankStateBlock,
//     pub state: CStateBlock,
// }

// // NOTE: This only has static methods.
// #[derive(Debug)]
// #[repr(C)]
// pub struct CBankStateBlock;

// #[derive(Debug)]
// #[repr(C)]
// pub struct CStateBlock {
//     shader_target: EShaderTarget,
//     enable_shader_optimisation: bool,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CShaderBankEntry {
//     pub vmt: *mut (),
//     pub c_base_class_non_copyable: CBaseClassNonCopyable,
//     pub shader_type: c_ulong,
//     pub shader_code: CArray<c_ulong>,
//     pub shader_constants: CArray<CCharString>,
//     pub vs_constant_layout: CCharString
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CSurface {
//     pub p_d3d_surface: *mut IDirect3DSurface9,
//     pub allocation_source: c_ulong,
//     pub p_allocated_memory: *mut c_void,
// }

// #[derive(Debug)]
// #[repr(C, align(8))]
// pub struct CTexture {
//     pub p_d3d_surface: *mut IDirect3DTexture9,
//     pub byte_length: UnknownEmptyType, // TODO: Seems like it should be c_ulong?
//     pub p_allocated_memory: UnknownEmptyType, // TODO: Seems like it should be c_ulong?
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct C2DExtentsI {
//     pub x: c_long,
//     pub y: c_long,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CPixelFormat {
//     pub format_index: c_long,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CPresentationIntervalDesc {
//     pub present_immediate: bool,
//     pub present_immediate_if_sync_missed: bool,
//     pub vsync_count: c_long,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct C2DBoxI {
//     pub tlx: c_long,
//     pub tly: c_long,
//     pub brx: c_long,
//     pub bry: c_long,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CCreateDeviceParameters {
//     pub win_handle: HWND,
//     pub back_buffer_dimensions: C2DExtentsI,
//     pub back_buffer_bit_depth: c_long,
//     pub z_buffer_bit_depth: c_long,
//     pub back_buffer_count: c_long,
//     pub refresh_rate: c_long,
//     pub exclusive_mode: bool,
//     pub presentation_interval: CPresentationIntervalDesc,
//     pub multisample_type: ESurfaceMultisampleType,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CVsyncCallbackTimer {
//     pub device: *mut IDirect3DDevice9,
//     pub free_query_objects: CxxList<CVsyncCallbackTimer__CPendingQuery>,
//     pub pending_queries: CxxList<CVsyncCallbackTimer__CPendingQuery>,
//     pub pending_frame: c_long,
//     pub finished_frame: c_long,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CVsyncCallbackTimer__CPendingQuery {
//     pub query: *mut IDirect3DQuery9,
//     pub frame: c_long,
//     pub timestamp: c_double,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CBusyResourceManager {
//     pub free_query_objects: CxxList<CBusyResourceManager__CPendingQuery>,
//     pub pending_queries: CxxList<CBusyResourceManager__CPendingQuery>,
//     pub pending_frame: c_long,
//     pub finished_frame: c_long,
//     pub d3d_device: *mut IDirect3DDevice9,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CBusyResourceManager__CPendingQuery {
//     pub query: *mut IDirect3DQuery9,
//     pub frame: c_long,
// }

// // #[derive(Debug)]
// #[repr(C)]
// pub struct CRenderManagerCore {
//     pub vmt: *mut (),
//     pub c_render_state_manager: CRenderStateManager,
//     pub display_manager: *mut CDisplayManager,
//     pub last_render_end_time: c_float,
//     pub tris_per_second: c_long,
//     pub screen_pixel_format: CPixelFormat,
//     pub system_font: CSystemFont,
//     pub rendering: bool,
//     pub resolution_reinit: bool,
//     pub resolution_set: bool,
//     pub textures_enabled: bool,
//     pub d3d_device_caps: D3DCAPS9,
//     pub d3d_device_software_caps: D3DCAPS9,
//     pub texture_stages_in_use: c_long,
//     pub z_sorted_polys_count: c_ulong,
//     pub z_non_sorted_polys_count: c_ulong,
//     pub render_state_changes_count: c_ulong,
//     pub polys_count: c_ulong,
//     pub highest_mipmap_to_render: c_long,
//     pub tris_drawn_count: c_long,
//     pub vertices_drawn: c_long,
//     pub draw_prim_calls_count: c_long,
//     pub last_frame_z_sorted_polys: c_ulong,
//     pub last_frame_non_z_sorted_polys: c_ulong,
//     pub last_frame_cached_texture_uploads: c_ulong,
//     pub p_d3d_device: *mut IDirect3DDevice9,
//     pub p_selected_index_buffer: *mut CIndexBuffer,
//     pub current_vertex_format: c_ulong,
//     pub xbox_shader_constants_enabled: bool,
//     pub fixed_function_pipeline_enabled: bool,
//     pub currently_selected_textures: [*const IDirect3DBaseTexture9; 8]
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CRenderStateManager {
//     pub vmt: *mut (),
//     pub c_base_class_non_copyable: CBaseClassNonCopyable,
//     pub p_d3d_device: *mut IDirect3DDevice9,
//     pub state_stack: [CStateStackElement; 1024],
//     pub state_update_list: [*mut CStateInfo; 256],
//     pub all_states_list: [*mut CStateInfo; 256],
//     pub state_stack_size: c_long,
//     pub state_update_list_size: c_long,
//     pub all_states_list_size: c_long,
//     pub current_bookmark_mask: c_ulong,
//     pub device_state: CDeviceState,
//     pub bookmark_dummy_state: CStateInfo,
//     pub init_flags: c_ulong,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CStateStackElement {
//     pub state: *mut CStateInfo,
//     pub value: c_ulong,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CStateInfo {
//     pub current_state: c_ulong,
//     pub desired_state: c_ulong,
//     pub bookmark_mask: c_ulong,
//     pub texture_state_state_param: D3DTEXTURESTAGESTATETYPE,
//     pub render_state_param: D3DRENDERSTATETYPE,
//     pub sampler_stage_state_param: D3DSAMPLERSTATETYPE,
//     pub dirty_list_flag: bool,
//     pub state_type: c_uchar,
//     pub texture_stage: c_uchar,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CDeviceState {
//     pub poly_shade_mode: CStateInfo,
//     pub poly_fill_mode: CStateInfo,
//     pub colour_write_enable: CStateInfo,
//     pub z_buffer_write_enable: CStateInfo,
//     pub z_buffer_enable: CStateInfo,
//     pub z_buffer_func: CStateInfo,
//     pub poly_cull_mode: CStateInfo,
//     pub alpha_test_func: CStateInfo,
//     pub alpha_blend_enable: CStateInfo,
//     pub alpha_test_enable: CStateInfo,
//     pub alpha_ref: CStateInfo,
//     pub vertex_blend_mode: CStateInfo,
//     pub software_vertex_processing: CStateInfo,
//     pub source_alpha_blend_mode: CStateInfo,
//     pub dest_alpha_blend_mode: CStateInfo,
//     pub fog_enable: CStateInfo,
//     pub fog_colour: CStateInfo,
//     pub fog_start: CStateInfo,
//     pub fog_end: CStateInfo,
//     pub fog_density: CStateInfo,
//     pub fog_mode: CStateInfo,
//     pub fog_vertex_mode: CStateInfo,
//     pub texture_factor: CStateInfo,
//     pub specular_enable: CStateInfo,
//     pub antialiasing_on: CStateInfo,dither_enable: CStateInfo,
//     pub clipping_eanble: CStateInfo,
//     pub lighting_enable: CStateInfo,
//     pub normalize_normals: CStateInfo,
//     pub ambient_material_colour_source: CStateInfo,
//     pub diffuse_material_colour_source: CStateInfo,
//     pub specular_material_colour_source: CStateInfo,
//     pub point_sprite_enable: CStateInfo,
//     pub point_scale_enable: CStateInfo,
//     pub point_size: CStateInfo,
//     pub point_size_min: CStateInfo,
//     pub point_size_max: CStateInfo,
//     pub point_scale_constant_factor: CStateInfo,
//     pub point_scale_distance_factor: CStateInfo,
//     pub point_scale_distance_sqrt_factor: CStateInfo,
//     pub shadow_func: CStateInfo,
//     pub solid_offset_enable: CStateInfo,
//     pub depth_offset: CStateInfo,
//     pub depth_offset_slope: CStateInfo,
//     pub stencil_enable: CStateInfo,
//     pub stencil_op_fail: CStateInfo,
//     pub stencil_op_z_fail: CStateInfo,
//     pub stencil_op_pass: CStateInfo,
//     pub stencil_func: CStateInfo,
//     pub stencil_ref: CStateInfo,
//     pub stencil_mask: CStateInfo,
//     pub stencil_write_mask: CStateInfo,
//     pub stipple_enable: CStateInfo,
//     pub blend_op_mode: CStateInfo,
//     pub blend_colour: CStateInfo,
//     pub texture_coord_wrap_mode: [CStateInfo; 8],
//     pub texture_max_anisotropy: [CStateInfo; 8],
//     pub texture_mipmap_lod_bias: [CStateInfo; 8],
//     pub texture_mip_map_mode: [CStateInfo; 8],
//     pub texture_uv_mode_u: [CStateInfo; 8],
//     pub texture_uv_mode_v: [CStateInfo; 8],
//     pub texture_uv_mode_w: [CStateInfo; 8],
//     pub texture_filter_mode_min: [CStateInfo; 8],
//     pub texture_filter_mode_mag: [CStateInfo; 8],
//     pub texture_operation: [CStateInfo; 8],
//     pub texture_alpha_operation: [CStateInfo; 8],
//     pub texture_argument1: [CStateInfo; 8],
//     pub texture_argument2: [CStateInfo; 8],
//     pub texture_alpha_argument_1: [CStateInfo; 8],
//     pub texture_alpha_argument_2: [CStateInfo; 8],
//     pub texture_coord_index: [CStateInfo; 8],
//     pub texture_transform_mode: [CStateInfo; 8],
//     pub texture_border_colour: [CStateInfo; 8],
//     pub texture_bump_env_mat_00: [CStateInfo; 8],
//     pub texture_bump_env_mat_01: [CStateInfo; 8],
//     pub texture_bump_env_mat_10: [CStateInfo; 8],
//     pub texture_bump_env_mat_11: [CStateInfo; 8],
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CSystemFont {
//     pub vmt: *mut (),
//     pub c_init_base_class: CInitBaseClass,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CIndexBuffer {
//     pub vmt: *mut (),
//     pub c_movable_resource: CMovableResource,
//     pub p_d3d_index_buffer: *mut IDirect3DIndexBuffer9,
//     pub byte_length: c_ulong,
//     pub primitive_type: EPrimitiveType,
//     pub p_allocated_buffer: *mut c_void,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CDisplayManagerInit {
//     pub width: c_long,
//     pub height: c_long,
//     pub depth: c_long,
//     pub refresh_rate: c_long,
//     pub use_render_manager: bool,
//     pub use_shader_render_manager: bool,
//     pub window_handle: *mut HWND,
//     pub z_buffer_depth: c_uchar,
//     pub back_buffers_count: c_uchar,
//     pub multisample_type: ESurfaceMultisampleType,
//     pub push_buffer_size: c_long,
//     pub kick_off_size: c_long,
//     pub present_immediate: bool,
//     pub present_immediately_if_missed_vsync: bool,
//     pub presentation_interval_vsync_count: c_long,
//     pub skip_config_detection: bool,
//     pub render_manager_init_flags: c_ulong,
//     pub shader_resource_path: CWideString,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CDisplayViewManager {
//     // pub p_current_view: CCountedPointer<NDisplayView::CViewBase>,
// }

// // NOTE: This only has static methods.
// #[derive(Debug)]
// #[repr(C)]
// pub struct CMovableResource;

// #[derive(Debug)]
// #[repr(C)]
// pub struct CEnginePrimitiveHandle {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CEnvironment {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CErrorDisplayInfo {
//     pub display_errors: bool,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CEventPackageFileHeader {
//     pub no_players: c_long,
// }
// #[derive(Debug)]
// #[repr(C)]
// pub struct CFactionManager {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CFadeInFadeOutBase {
//     pub active: bool,
//     pub fade_in_time: c_float,
//     pub fade_out_time: c_float,
//     pub closing: bool,
//     pub opening: bool,
//     pub open_timer: c_float,
//     pub close_timer: c_float,
//     pub to_colour: CRGBColour,
// }
// /// TODO: This is behind a pointer so I've left it empty for now.
// #[derive(Debug)]
// #[repr(C)]
// pub struct CFontBank {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CFrameRateSmoother {
//     pub vmt: *mut (),
//     pub c_init_base_class: CInitBaseClass,
//     pub times: CArray<c_float>,
//     pub max_no_times: c_long,
//     pub no_times: c_long,
//     pub no_frames_to_change_frame_rate_over: c_long,
//     pub first_time: c_long,
//     pub last_time: c_long,
//     pub smoothed_time: c_double,
// }

// #[repr(C)]
// pub struct CGame {
//     pub vmt: *mut (),
//     pub c_init_base_class: CInitBaseClass,
//     pub current_game_component: *mut CGameComponent,
//     pub parameter_buffer: [c_uchar; 512],
//     pub quit: bool,
// }

// impl CGame {}

// impl fmt::Debug for CGame {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         f.debug_struct("CGame")
//             .field("c_init_base_class", &self.c_init_base_class)
//             .field("current_game_component", &self.current_game_component)
//             .field("parameter_buffer", &&self.parameter_buffer[..])
//             .field("quit", &self.quit)
//             .finish()
//     }
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CGameComponent {
//     pub vmt: *mut (),
//     pub c_base_class_non_copyable: CBaseClassNonCopyable,
//     pub c_device_reset_callback: CDeviceResetCallback,
//     pub quit: bool,
//     pub running: bool,
//     pub game: *mut CGame,
// }

// impl CGameComponent {}
// /// Apparently this is a forward declaration with no actual definition? See also CDefinitionManager.
// #[derive(Debug)]
// #[repr(C)]
// pub struct CGameDefinitionManager {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CGameEvent {
//     pub event_type: c_long,
//     pub player: c_char,
//     pub data: [c_uchar; 32],
//     pub end_pos: c_uchar,
//     /// Many of the events have data that is invalid, e.g. the event_type and data are seemingly random.
//     /// This flag tells the game the event is valid and in use.
//     pub valid: bool,
//     /// I think this indicates whether the event should be replaced?
//     /// Some events that were seemingly valid, but marked invalid, are marked with replacement.
//     pub replacement: bool,
// }
// /// TODO: This is behind a pointer so I've left it empty for now.
// #[derive(Debug)]
// #[repr(C)]
// pub struct CGameEventDispatch {}

// #[repr(C)]
// pub struct CGameEventPackage {
//     pub timestamp: c_long,
//     pub events_count: c_ulong,
//     pub events: [CGameEvent; 40],
// }

// impl fmt::Debug for CGameEventPackage {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         f.debug_struct("CGameEventPackage")
//             .field("timestamp", &self.timestamp)
//             .field("events_count", &self.events_count)
//             .field("events", &&self.events[..])
//             .finish()
//     }
// }

// #[repr(C)]
// pub struct CGameEventPackageSet {
//     pub packages_count: c_ulong,
//     // pub packages: Box<[CGameEventPackage; 80400]>,
//     pub packages: [CGameEventPackage; 50],
// }

// impl fmt::Debug for CGameEventPackageSet {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         f.debug_struct("CGameEventPackageSet")
//             .field("packages_count", &self.packages_count)
//             .field("packages", &&self.packages[..])
//             .finish()
//     }
// }
// /// TODO: This is behind a pointer so I've left it empty for now.
// #[derive(Debug)]
// #[repr(C)]
// pub struct CGamePlayerInterface {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CGameScriptInterface {
//     pub vmt: *mut CGameScriptInterfaceVmt,
//     pub c_game_script_interface_base: CGameScriptInterfaceBase,
//     pub world: *mut CWorld,
//     pub component: *mut CMainGameComponent,
//     pub display_engine: *mut CDisplayEngine,
//     pub definition_manager: *const CGameDefinitionManager,
//     pub player_manager: *mut CPlayerManager,
//     pub thing_search_tools: *const CThingSearchTools,
//     pub current_player: c_long,
//     pub current_level_id: c_long,
//     pub current_script_level_id: c_long,
//     pub current_script_id: c_long,
//     pub in_movie_sequence: bool,
//     pub alow_screen_fading_if_already_faded: bool,
//     pub hero_script_thing: CScriptThing,
//     pub script_timers: CxxMap<c_long, c_long>,
//     pub camera_rset_to_view_behind_hero_count: c_long,
//     pub create_creature_delay_frames: c_long,
// }

// impl CGameScriptInterface {
//     pub fn end_letter_box(&mut self) {
//         unsafe { ((*self.vmt).end_letter_box)(self as *mut CGameScriptInterface) }
//     }

//     pub fn error(&mut self, a: *const CCharString, b: *const CCharString, c: c_ulong) {
//         unsafe { ((*self.vmt).error)(self as *mut CGameScriptInterface, a, b, c) }
//     }

//     pub fn trace_message(&mut self, a: *const CCharString) {
//         unsafe { ((*self.vmt).trace_message)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn validate(&mut self) {
//         unsafe { ((*self.vmt).validate)(self as *mut CGameScriptInterface) }
//     }

//     pub fn set_debug_camera_type(&mut self, a: c_long) {
//         unsafe { ((*self.vmt).set_debug_camera_type)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn deactivate_boast_ui(&mut self) {
//         unsafe { ((*self.vmt).deactivate_boast_ui)(self as *mut CGameScriptInterface) }
//     }

//     pub fn is_xbox(&mut self) -> bool {
//         unsafe { ((*self.vmt).is_xbox)(self as *mut CGameScriptInterface) }
//     }

//     pub fn new_script_frame(&mut self) {
//         unsafe { ((*self.vmt).new_script_frame)(self as *mut CGameScriptInterface) }
//     }

//     pub fn start_scripting_entity(
//         &mut self,
//         a: *const CScriptThing,
//         b: *mut CScriptGameResourceObjectScriptedThingBase,
//         c: EScriptAIPriority,
//     ) -> bool {
//         unsafe { ((*self.vmt).start_scripting_entity)(self as *mut CGameScriptInterface, a, b, c) }
//     }

//     pub fn is_entity_under_scripted_control(&mut self, a: *const CScriptThing) -> bool {
//         unsafe {
//             ((*self.vmt).is_entity_under_scripted_control)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn is_active_thread_terminating(&mut self, a: *const CScriptThing) {
//         unsafe { ((*self.vmt).is_active_thread_terminating)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn is_level_loaded(&mut self, a: *const CCharString) -> bool {
//         unsafe { ((*self.vmt).is_level_loaded)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn is_region_loaded(&mut self, a: *const CCharString) -> bool {
//         unsafe { ((*self.vmt).is_region_loaded)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn is_region_loaded_and_preloaded(&mut self, a: *const CCharString) -> bool {
//         unsafe {
//             ((*self.vmt).is_region_loaded_and_preloaded)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn is_region_def_loaded(&mut self, a: *const CCharString) -> bool {
//         unsafe { ((*self.vmt).is_region_def_loaded)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_region_name(&mut self) -> *const CCharString {
//         unsafe { ((*self.vmt).get_region_name)(self as *mut CGameScriptInterface) }
//     }

//     pub fn msg_is_level_loaded(&mut self, a: *const CCharString) -> bool {
//         unsafe { ((*self.vmt).msg_is_level_loaded)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn msg_is_level_unloaded(&mut self, a: *const CCharString) -> bool {
//         unsafe { ((*self.vmt).msg_is_level_unloaded)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn msg_on_level_loaded(&mut self, a: *mut CxxList<CCharString>) -> bool {
//         unsafe { ((*self.vmt).msg_on_level_loaded)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn msg_on_level_unloaded(&mut self, a: *mut CxxList<CCharString>) -> bool {
//         unsafe { ((*self.vmt).msg_on_level_unloaded)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn msg_is_region_loaded(&mut self, a: *const CCharString) -> bool {
//         unsafe { ((*self.vmt).msg_is_region_loaded)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn msg_is_region_unloaded(&mut self, a: *const CCharString) -> bool {
//         unsafe { ((*self.vmt).msg_is_region_unloaded)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn msg_on_region_loaded(&mut self, a: *mut CCharString) -> bool {
//         unsafe { ((*self.vmt).msg_on_region_loaded)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn msg_on_region_unloaded(&mut self, a: *const CCharString) -> bool {
//         unsafe { ((*self.vmt).msg_on_region_unloaded)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn msg_on_region_preunloaded(&mut self, a: *mut CCharString) -> bool {
//         unsafe { ((*self.vmt).msg_on_region_preunloaded)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn msg_on_quest_completed(&mut self, a: *mut CCharString) -> bool {
//         unsafe { ((*self.vmt).msg_on_quest_completed)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn msg_on_any_quest_completed(&mut self, a: *mut CCharString) -> bool {
//         unsafe { ((*self.vmt).msg_on_any_quest_completed)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn msg_on_quest_failed(&mut self, a: *const CCharString) -> bool {
//         unsafe { ((*self.vmt).msg_on_quest_failed)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn msg_on_quest_completed_before_screen_shown(&mut self, a: *const CCharString) -> bool {
//         unsafe {
//             ((*self.vmt).msg_on_quest_completed_before_screen_shown)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn msg_on_quest_failed_before_screen_shown(&mut self, a: *const CCharString) -> bool {
//         unsafe {
//             ((*self.vmt).msg_on_quest_failed_before_screen_shown)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn msg_on_quest_accepted(&mut self, a: *mut CCharString) -> bool {
//         unsafe { ((*self.vmt).msg_on_quest_accepted)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn msg_on_feat_accepted(&mut self, a: *mut c_long) -> bool {
//         unsafe { ((*self.vmt).msg_on_feat_accepted)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn msg_is_boast_made(&mut self, a: c_long) -> bool {
//         unsafe { ((*self.vmt).msg_is_boast_made)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn msg_on_boast_made(&mut self, a: *mut c_long, b: *mut CCharString) -> bool {
//         unsafe { ((*self.vmt).msg_on_boast_made)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn msg_on_boasts_made(&mut self, a: *mut CxxVector<CCharString>) -> bool {
//         unsafe { ((*self.vmt).msg_on_boasts_made)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn remove_boast_message(&mut self) {
//         unsafe { ((*self.vmt).remove_boast_message)(self as *mut CGameScriptInterface) }
//     }

//     pub fn is_quest_start_screen_active(&mut self, a: *mut CCharString) -> bool {
//         unsafe { ((*self.vmt).is_quest_start_screen_active)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn msg_on_leaving_quest_start_screen(&mut self, a: *mut CCharString) -> bool {
//         unsafe {
//             ((*self.vmt).msg_on_leaving_quest_start_screen)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn msg_on_leaving_experience_spending_screen(&mut self) -> bool {
//         unsafe {
//             ((*self.vmt).msg_on_leaving_experience_spending_screen)(
//                 self as *mut CGameScriptInterface,
//             )
//         }
//     }

//     pub fn msg_is_answered_yes_or_no(&mut self) -> c_long {
//         unsafe { ((*self.vmt).msg_is_answered_yes_or_no)(self as *mut CGameScriptInterface) }
//     }

//     pub fn msg_is_game_info_clicked_past(&mut self) -> bool {
//         unsafe { ((*self.vmt).msg_is_game_info_clicked_past)(self as *mut CGameScriptInterface) }
//     }

//     pub fn msg_is_tutorial_click_past(&mut self) -> bool {
//         unsafe { ((*self.vmt).msg_is_tutorial_click_past)(self as *mut CGameScriptInterface) }
//     }

//     pub fn msg_is_action_mode_button_pressed(&mut self) -> bool {
//         unsafe {
//             ((*self.vmt).msg_is_action_mode_button_pressed)(self as *mut CGameScriptInterface)
//         }
//     }

//     pub fn msg_on_expression_performed(&mut self, a: *mut CCharString) -> bool {
//         unsafe { ((*self.vmt).msg_on_expression_performed)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn msg_is_cut_scene_skipped(&mut self) -> bool {
//         unsafe { ((*self.vmt).msg_is_cut_scene_skipped)(self as *mut CGameScriptInterface) }
//     }

//     pub fn remove_all_cut_scene_skipped_messages(&mut self, a: *mut CCharString) {
//         unsafe {
//             ((*self.vmt).remove_all_cut_scene_skipped_messages)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn msg_on_hero_hair_type_changed(
//         &mut self,
//         a: EClothingCoversArea,
//         b: *mut CCharString,
//     ) -> bool {
//         unsafe {
//             ((*self.vmt).msg_on_hero_hair_type_changed)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn msg_on_hero_used_teleporter(&mut self, a: *mut CCharString) -> bool {
//         unsafe { ((*self.vmt).msg_on_hero_used_teleporter)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn msg_on_hero_used_guild_seal(&mut self) -> bool {
//         unsafe { ((*self.vmt).msg_on_hero_used_guild_seal)(self as *mut CGameScriptInterface) }
//     }

//     pub fn msg_on_game_saved_manually(&mut self) -> bool {
//         unsafe { ((*self.vmt).msg_on_game_saved_manually)(self as *mut CGameScriptInterface) }
//     }

//     pub fn msg_on_hero_slept(&mut self, a: *mut bool) -> bool {
//         unsafe { ((*self.vmt).msg_on_hero_slept)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn msg_on_hero_fired_ranged_weapon(&mut self) -> bool {
//         unsafe { ((*self.vmt).msg_on_hero_fired_ranged_weapon)(self as *mut CGameScriptInterface) }
//     }

//     pub fn msg_on_hero_cast_spell(&mut self, a: *mut EHeroAbility) -> bool {
//         unsafe { ((*self.vmt).msg_on_hero_cast_spell)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn msg_on_hero_picked_pocket(&mut self, a: *mut CScriptThing) -> bool {
//         unsafe { ((*self.vmt).msg_on_hero_picked_pocket)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn msg_on_hero_picked_lock(&mut self, a: *mut CScriptThing) -> bool {
//         unsafe { ((*self.vmt).msg_on_hero_picked_lock)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn msg_on_fishing_game_finished(&mut self, a: *mut CScriptThing) -> bool {
//         unsafe { ((*self.vmt).msg_on_fishing_game_finished)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn msg_on_tavern_game_finished(&mut self, a: *const CScriptThing) -> bool {
//         unsafe { ((*self.vmt).msg_on_tavern_game_finished)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn msg_on_hero_rewarded_with_items_from(&mut self, a: *mut CCharString) -> bool {
//         unsafe {
//             ((*self.vmt).msg_on_hero_rewarded_with_items_from)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn msg_on_chest_opening_cancelled(&mut self) -> bool {
//         unsafe { ((*self.vmt).msg_on_chest_opening_cancelled)(self as *mut CGameScriptInterface) }
//     }

//     pub fn dont_populate_next_loaded_region(&mut self) {
//         unsafe { ((*self.vmt).dont_populate_next_loaded_region)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_wandering_population_script_def_name_in_current_region(
//         &mut self,
//         a: *mut CCharString,
//     ) {
//         unsafe {
//             ((*self.vmt).get_wandering_population_script_def_name_in_current_region)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn get_wandering_population_script_def_name_in_region(
//         &mut self,
//         a: *const CCharString,
//         b: *mut CCharString,
//     ) {
//         unsafe {
//             ((*self.vmt).get_wandering_population_script_def_name_in_region)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn is_hero_allowed_henchmen_in_current_region(&mut self) -> bool {
//         unsafe {
//             ((*self.vmt).is_hero_allowed_henchmen_in_current_region)(
//                 self as *mut CGameScriptInterface,
//             )
//         }
//     }

//     pub fn is_hero_allowed_henchmen_in_region(&mut self, a: *const CCharString) -> bool {
//         unsafe {
//             ((*self.vmt).is_hero_allowed_henchmen_in_region)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn post_add_scripted_entities(&mut self) {
//         unsafe { ((*self.vmt).post_add_scripted_entities)(self as *mut CGameScriptInterface) }
//     }

//     pub fn is_player_holding_unsheathe_ranged_weapon_button(&mut self) -> bool {
//         unsafe {
//             ((*self.vmt).is_player_holding_unsheathe_ranged_weapon_button)(
//                 self as *mut CGameScriptInterface,
//             )
//         }
//     }

//     pub fn is_player_holding_lock_target_button(&mut self) -> bool {
//         unsafe {
//             ((*self.vmt).is_player_holding_lock_target_button)(self as *mut CGameScriptInterface)
//         }
//     }

//     pub fn is_player_holding_fire_ranged_weapon_button(&mut self) -> bool {
//         unsafe {
//             ((*self.vmt).is_player_holding_fire_ranged_weapon_button)(
//                 self as *mut CGameScriptInterface,
//             )
//         }
//     }

//     pub fn is_player_holding_first_person_targeting_button(&mut self) -> bool {
//         unsafe {
//             ((*self.vmt).is_player_holding_first_person_targeting_button)(
//                 self as *mut CGameScriptInterface,
//             )
//         }
//     }

//     pub fn is_hero_in_projectile_weapon_mode(&mut self) -> bool {
//         unsafe {
//             ((*self.vmt).is_hero_in_projectile_weapon_mode)(self as *mut CGameScriptInterface)
//         }
//     }

//     pub fn register_timer(&mut self) -> c_long {
//         unsafe { ((*self.vmt).register_timer)(self as *mut CGameScriptInterface) }
//     }

//     pub fn deregister_timer(&mut self, a: c_long) {
//         unsafe { ((*self.vmt).deregister_timer)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_timer(&mut self, a: c_long, b: c_long) {
//         unsafe { ((*self.vmt).set_timer)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn get_timer(&mut self, a: c_long) -> c_long {
//         unsafe { ((*self.vmt).get_timer)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_hero(&mut self) -> *mut CScriptThing {
//         unsafe { ((*self.vmt).get_hero)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_hero_targeted_thing(&mut self) -> CScriptThing {
//         unsafe { ((*self.vmt).get_hero_targeted_thing)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_thing_with_script_name(&mut self, a: *const CCharString) -> CScriptThing {
//         unsafe { ((*self.vmt).get_thing_with_script_name)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_thing_with_script_name_2(
//         &mut self,
//         a: *const CCharString,
//         b: *const CCharString,
//     ) -> CScriptThing {
//         unsafe {
//             ((*self.vmt).get_thing_with_script_name_2)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn get_random_thing_with_script_name(&mut self, a: *const CCharString) -> CScriptThing {
//         unsafe {
//             ((*self.vmt).get_random_thing_with_script_name)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn get_all_things_with_script_name(
//         &mut self,
//         a: *const CCharString,
//         b: *mut CxxVector<CScriptThing>,
//     ) -> c_long {
//         unsafe {
//             ((*self.vmt).get_all_things_with_script_name)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn get_all_creatures_in_area_with_script_name(
//         &mut self,
//         a: *const CCharString,
//         b: *const C3DVector,
//         c: c_float,
//         d: *const CxxVector<CScriptThing>,
//     ) -> c_long {
//         unsafe {
//             ((*self.vmt).get_all_creatures_in_area_with_script_name)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//                 c,
//                 d,
//             )
//         }
//     }

//     pub fn get_nearest_with_script_name(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CCharString,
//     ) -> CScriptThing {
//         unsafe {
//             ((*self.vmt).get_nearest_with_script_name)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn get_furthest_with_script_name(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CCharString,
//     ) -> CScriptThing {
//         unsafe {
//             ((*self.vmt).get_furthest_with_script_name)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn get_all_things_with_def_name(
//         &mut self,
//         a: *const CCharString,
//         b: *mut CxxVector<CScriptThing>,
//     ) -> c_long {
//         unsafe {
//             ((*self.vmt).get_all_things_with_def_name)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn get_all_things_with_def_name_by_distance_from(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CCharString,
//         c: *mut CxxVector<CScriptThing>,
//     ) -> c_long {
//         unsafe {
//             ((*self.vmt).get_all_things_with_def_name_by_distance_from)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//                 c,
//             )
//         }
//     }

//     pub fn get_nearest_with_def_name(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CCharString,
//     ) -> CScriptThing {
//         unsafe { ((*self.vmt).get_nearest_with_def_name)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn get_furthest_with_def_name(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CCharString,
//     ) -> CScriptThing {
//         unsafe { ((*self.vmt).get_furthest_with_def_name)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn get_thing_with_uid(&mut self, a: c_ulong) -> CScriptThing {
//         unsafe { ((*self.vmt).get_thing_with_uid)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_all_creatures_excluding_hero(
//         &mut self,
//         a: *mut CxxVector<CScriptThing>,
//     ) -> c_long {
//         unsafe {
//             ((*self.vmt).get_all_creatures_excluding_hero)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn get_all_things_in_level(
//         &mut self,
//         a: *const CCharString,
//         b: *mut CxxVector<CScriptThing>,
//     ) -> c_long {
//         unsafe { ((*self.vmt).get_all_things_in_level)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn is_thing_with_this_uid_alive(&mut self, a: c_ulong) -> bool {
//         unsafe { ((*self.vmt).is_thing_with_this_uid_alive)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn create_creature(
//         &mut self,
//         a: *const CCharString,
//         b: *const C3DVector,
//         c: *const CCharString,
//         d: bool,
//     ) -> CScriptThing {
//         unsafe { ((*self.vmt).create_creature)(self as *mut CGameScriptInterface, a, b, c, d) }
//     }

//     pub fn create_creature_nearby(
//         &mut self,
//         a: *const CCharString,
//         b: *const C3DVector,
//         c: c_float,
//         d: *const CCharString,
//         e: bool,
//     ) -> CScriptThing {
//         unsafe {
//             ((*self.vmt).create_creature_nearby)(self as *mut CGameScriptInterface, a, b, c, d, e)
//         }
//     }

//     pub fn create_creature_on_entity(
//         &mut self,
//         a: *const CCharString,
//         b: *const CScriptThing,
//         c: *const CCharString,
//     ) -> CScriptThing {
//         unsafe {
//             ((*self.vmt).create_creature_on_entity)(self as *mut CGameScriptInterface, a, b, c)
//         }
//     }

//     pub fn turn_creature_into(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CCharString,
//     ) -> CScriptThing {
//         unsafe { ((*self.vmt).turn_creature_into)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn set_creature_creation_delay_frames(&mut self, a: c_long) {
//         unsafe {
//             ((*self.vmt).set_creature_creation_delay_frames)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn reset_creature_creation_delay_frames(&mut self) {
//         unsafe {
//             ((*self.vmt).reset_creature_creation_delay_frames)(self as *mut CGameScriptInterface)
//         }
//     }

//     pub fn create_object(
//         &mut self,
//         a: *const CCharString,
//         b: *const C3DVector,
//         c: c_float,
//         d: *const CCharString,
//     ) -> CScriptThing {
//         unsafe { ((*self.vmt).create_object)(self as *mut CGameScriptInterface, a, b, c, d) }
//     }

//     pub fn create_object_2(
//         &mut self,
//         a: *const CCharString,
//         b: *const C3DVector,
//         c: *const CCharString,
//     ) -> CScriptThing {
//         unsafe { ((*self.vmt).create_object_2)(self as *mut CGameScriptInterface, a, b, c) }
//     }

//     pub fn create_object_on_entity(
//         &mut self,
//         a: *const CCharString,
//         b: *const CScriptThing,
//         c: *const CCharString,
//     ) -> CScriptThing {
//         unsafe { ((*self.vmt).create_object_on_entity)(self as *mut CGameScriptInterface, a, b, c) }
//     }

//     pub fn create_effect(
//         &mut self,
//         a: *const CCharString,
//         b: *const C3DVector,
//         c: *const CCharString,
//         d: c_float,
//         e: bool,
//         f: bool,
//     ) -> CScriptThing {
//         unsafe { ((*self.vmt).create_effect)(self as *mut CGameScriptInterface, a, b, c, d, e, f) }
//     }

//     pub fn create_effect_2(
//         &mut self,
//         a: *const CCharString,
//         b: *const CScriptThing,
//         c: *const CCharString,
//         d: *const CCharString,
//         e: bool,
//         f: bool,
//     ) -> CScriptThing {
//         unsafe {
//             ((*self.vmt).create_effect_2)(self as *mut CGameScriptInterface, a, b, c, d, e, f)
//         }
//     }

//     pub fn create_light(
//         &mut self,
//         a: *const C3DVector,
//         b: *const CRGBColour,
//         c: *const CCharString,
//         d: c_float,
//         e: c_float,
//         f: bool,
//     ) -> CScriptThing {
//         unsafe { ((*self.vmt).create_light)(self as *mut CGameScriptInterface, a, b, c, d, e, f) }
//     }

//     pub fn create_experience_orb(&mut self, a: *const C3DVector, b: c_long) -> CScriptThing {
//         unsafe { ((*self.vmt).create_experience_orb)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn create_explosion(
//         &mut self,
//         a: *const CCharString,
//         b: *const C3DVector,
//         c: *const C3DVector,
//         d: CCharString,
//     ) -> CScriptThing {
//         unsafe { ((*self.vmt).create_explosion)(self as *mut CGameScriptInterface, a, b, c, d) }
//     }

//     pub fn create_physical_barrier(
//         &mut self,
//         a: c_float,
//         b: *const C3DVector,
//         c: *const C3DVector,
//         d: CCharString,
//     ) -> CScriptThing {
//         unsafe {
//             ((*self.vmt).create_physical_barrier)(self as *mut CGameScriptInterface, a, b, c, d)
//         }
//     }

//     pub fn create_rumble(
//         &mut self,
//         a: *const C3DVector,
//         b: c_float,
//         c: c_float,
//         d: CCharString,
//     ) -> CScriptThing {
//         unsafe { ((*self.vmt).create_rumble)(self as *mut CGameScriptInterface, a, b, c, d) }
//     }

//     pub fn clear_all_rumbles(&mut self) {
//         unsafe { ((*self.vmt).clear_all_rumbles)(self as *mut CGameScriptInterface) }
//     }

//     pub fn remove_thing(&mut self, a: *const CScriptThing, b: bool, c: bool) {
//         unsafe { ((*self.vmt).remove_thing)(self as *mut CGameScriptInterface, a, b, c) }
//     }

//     pub fn show_on_screen_message(
//         &mut self,
//         a: *const C2DVector,
//         b: *const CCharString,
//         c: *const CRGBColour,
//         d: *const CCharString,
//     ) {
//         unsafe {
//             ((*self.vmt).show_on_screen_message)(self as *mut CGameScriptInterface, a, b, c, d)
//         }
//     }

//     pub fn show_on_screen_message_2(
//         &mut self,
//         a: *const CCharString,
//         b: *const CCharString,
//         c: *const CCharString,
//         d: c_float,
//     ) {
//         unsafe {
//             ((*self.vmt).show_on_screen_message_2)(self as *mut CGameScriptInterface, a, b, c, d)
//         }
//     }

//     pub fn show_on_screen_message_3(&mut self, a: *const CCharString, b: c_float) {
//         unsafe { ((*self.vmt).show_on_screen_message_3)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn add_screen_message(&mut self, a: *const CCharString, b: ETextGroupSelectionMethod) {
//         unsafe { ((*self.vmt).add_screen_message)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn add_screen_title_message(&mut self, a: *const CCharString, b: c_float, c: bool) {
//         unsafe {
//             ((*self.vmt).add_screen_title_message)(self as *mut CGameScriptInterface, a, b, c)
//         }
//     }

//     pub fn give_hero_yes_no_question(
//         &mut self,
//         a: *const CCharString,
//         b: *const CCharString,
//         c: *const CCharString,
//         d: *const CCharString,
//         e: bool,
//     ) {
//         unsafe {
//             ((*self.vmt).give_hero_yes_no_question)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//                 c,
//                 d,
//                 e,
//             )
//         }
//     }

//     pub fn display_game_info(&mut self, a: *const CCharString) {
//         unsafe { ((*self.vmt).display_game_info)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn display_game_info_text(&mut self, a: *const CWideString) {
//         unsafe { ((*self.vmt).display_game_info_text)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn is_safe_to_display_game_info(&mut self) -> bool {
//         unsafe { ((*self.vmt).is_safe_to_display_game_info)(self as *mut CGameScriptInterface) }
//     }

//     pub fn display_tutorial(&mut self, a: ETutorialCategory) -> bool {
//         unsafe { ((*self.vmt).display_tutorial)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn is_tutorial_system_enabled(&mut self) -> bool {
//         unsafe { ((*self.vmt).is_tutorial_system_enabled)(self as *mut CGameScriptInterface) }
//     }

//     pub fn give_hero_weapon(&mut self, a: *const CCharString, b: bool) {
//         unsafe { ((*self.vmt).give_hero_weapon)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn give_hero_object(&mut self, a: *const CCharString, b: c_long, c: bool) {
//         unsafe { ((*self.vmt).give_hero_object)(self as *mut CGameScriptInterface, a, b, c) }
//     }

//     pub fn set_weapon_as_heros_active_weapon(&mut self, a: *const CCharString) {
//         unsafe {
//             ((*self.vmt).set_weapon_as_heros_active_weapon)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn give_hero_item(&mut self, a: *const CScriptThing) {
//         unsafe { ((*self.vmt).give_hero_item)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn give_hero_items_from_container(&mut self, a: *const CScriptThing, b: bool) -> bool {
//         unsafe {
//             ((*self.vmt).give_hero_items_from_container)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn take_object_from_hero(&mut self, a: *const CCharString) {
//         unsafe { ((*self.vmt).take_object_from_hero)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn give_hero_gold(&mut self, a: c_long) {
//         unsafe { ((*self.vmt).give_hero_gold)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_hero_gold(&mut self) -> c_long {
//         unsafe { ((*self.vmt).get_hero_gold)(self as *mut CGameScriptInterface) }
//     }

//     pub fn give_hero_experience(&mut self, a: c_long) {
//         unsafe { ((*self.vmt).give_hero_experience)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_hero_able_to_gain_experience(&mut self, a: bool) {
//         unsafe {
//             ((*self.vmt).set_hero_able_to_gain_experience)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn sheathe_hero_weapons(&mut self) {
//         unsafe { ((*self.vmt).sheathe_hero_weapons)(self as *mut CGameScriptInterface) }
//     }

//     pub fn set_hero_will_as_usable(&mut self, a: bool) {
//         unsafe { ((*self.vmt).set_hero_will_as_usable)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_hero_weapons_as_usable(&mut self, a: bool) {
//         unsafe { ((*self.vmt).set_hero_weapons_as_usable)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_weapon_out_crime_enabled(&mut self, a: bool) {
//         unsafe { ((*self.vmt).set_weapon_out_crime_enabled)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_guards_ignore_crimes(&mut self, a: bool) {
//         unsafe { ((*self.vmt).set_guards_ignore_crimes)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn remove_all_hero_weapons(&mut self) {
//         unsafe { ((*self.vmt).remove_all_hero_weapons)(self as *mut CGameScriptInterface) }
//     }

//     pub fn is_reported_or_unreported_crime_known(&mut self, a: *const CScriptThing) -> bool {
//         unsafe {
//             ((*self.vmt).is_reported_or_unreported_crime_known)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn confiscate_all_hero_items(&mut self) {
//         unsafe { ((*self.vmt).confiscate_all_hero_items)(self as *mut CGameScriptInterface) }
//     }

//     pub fn confiscate_all_hero_weapons(&mut self) {
//         unsafe { ((*self.vmt).confiscate_all_hero_weapons)(self as *mut CGameScriptInterface) }
//     }

//     pub fn confiscate_items_of_type_from_hero(&mut self, a: *const CCharString) {
//         unsafe {
//             ((*self.vmt).confiscate_items_of_type_from_hero)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn return_all_confiscated_items_to_hero(&mut self) {
//         unsafe {
//             ((*self.vmt).return_all_confiscated_items_to_hero)(self as *mut CGameScriptInterface)
//         }
//     }

//     pub fn give_hero_tutorial(&mut self, a: ETutorialCategory) -> bool {
//         unsafe { ((*self.vmt).give_hero_tutorial)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn make_hero_carry_item_in_hand(&mut self, a: *const CScriptThing, b: bool, c: bool) {
//         unsafe {
//             ((*self.vmt).make_hero_carry_item_in_hand)(self as *mut CGameScriptInterface, a, b, c)
//         }
//     }

//     pub fn make_hero_carry_item_in_hand_2(&mut self, a: *const CCharString) {
//         unsafe {
//             ((*self.vmt).make_hero_carry_item_in_hand_2)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn add_tattoo_to_hero(&mut self, a: *const CCharString) {
//         unsafe { ((*self.vmt).add_tattoo_to_hero)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn give_hero_ability(&mut self, a: EHeroAbility, b: bool) {
//         unsafe { ((*self.vmt).give_hero_ability)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn is_player_z_targeting_thing(&mut self, a: *const CScriptThing) -> bool {
//         unsafe { ((*self.vmt).is_player_z_targeting_thing)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn is_player_creature_blocking(&mut self) -> bool {
//         unsafe { ((*self.vmt).is_player_creature_blocking)(self as *mut CGameScriptInterface) }
//     }

//     pub fn is_player_creature_ready_to_fire_projectile_weapon(&mut self, a: *mut c_float) -> bool {
//         unsafe {
//             ((*self.vmt).is_player_creature_ready_to_fire_projectile_weapon)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn get_player_creature_combat_multiplier(&mut self) -> c_long {
//         unsafe {
//             ((*self.vmt).get_player_creature_combat_multiplier)(self as *mut CGameScriptInterface)
//         }
//     }

//     pub fn get_player_creature_combat_multiplier_running_num_hits(&mut self) -> c_long {
//         unsafe {
//             ((*self.vmt).get_player_creature_combat_multiplier_running_num_hits)(
//                 self as *mut CGameScriptInterface,
//             )
//         }
//     }

//     pub fn reset_player_creature_combat_multiplier(&mut self) {
//         unsafe {
//             ((*self.vmt).reset_player_creature_combat_multiplier)(self as *mut CGameScriptInterface)
//         }
//     }

//     pub fn is_player_creature_flourish_enabled(&mut self) -> bool {
//         unsafe {
//             ((*self.vmt).is_player_creature_flourish_enabled)(self as *mut CGameScriptInterface)
//         }
//     }

//     pub fn set_player_creature_only_target(&mut self, a: *const CScriptThing) {
//         unsafe {
//             ((*self.vmt).set_player_creature_only_target)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn reset_player_creature_only_target(&mut self) {
//         unsafe {
//             ((*self.vmt).reset_player_creature_only_target)(self as *mut CGameScriptInterface)
//         }
//     }

//     pub fn respawn_hero(&mut self, a: bool) {
//         unsafe { ((*self.vmt).respawn_hero)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn give_hero_morality(&mut self, a: c_float) {
//         unsafe { ((*self.vmt).give_hero_morality)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_hero_morality(&mut self) -> c_float {
//         unsafe { ((*self.vmt).get_hero_morality)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_hero_morality_category(&mut self) -> EMorality {
//         unsafe { ((*self.vmt).get_hero_morality_category)(self as *mut CGameScriptInterface) }
//     }

//     pub fn change_hero_morality_due_to_theft(&mut self) {
//         unsafe {
//             ((*self.vmt).change_hero_morality_due_to_theft)(self as *mut CGameScriptInterface)
//         }
//     }

//     pub fn change_hero_morality_due_to_picklock(&mut self) {
//         unsafe {
//             ((*self.vmt).change_hero_morality_due_to_picklock)(self as *mut CGameScriptInterface)
//         }
//     }

//     pub fn give_hero_renown_points(&mut self, a: c_long) {
//         unsafe { ((*self.vmt).give_hero_renown_points)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_hero_renown_level(&mut self) -> c_long {
//         unsafe { ((*self.vmt).get_hero_renown_level)(self as *mut CGameScriptInterface) }
//     }

//     pub fn is_hero_renown_level_full(&mut self) -> bool {
//         unsafe { ((*self.vmt).is_hero_renown_level_full)(self as *mut CGameScriptInterface) }
//     }

//     pub fn increase_hero_renown_level(&mut self) {
//         unsafe { ((*self.vmt).increase_hero_renown_level)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_hero_strength_level(&mut self) -> c_long {
//         unsafe { ((*self.vmt).get_hero_strength_level)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_hero_skill_level(&mut self) -> c_long {
//         unsafe { ((*self.vmt).get_hero_skill_level)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_hero_will_level(&mut self) -> c_long {
//         unsafe { ((*self.vmt).get_hero_will_level)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_hero_stat_level(&mut self, a: EHeroTrainableStatType) -> c_long {
//         unsafe { ((*self.vmt).get_hero_stat_level)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_hero_stat_max(&mut self, a: EHeroTrainableStatType) -> c_long {
//         unsafe { ((*self.vmt).get_hero_stat_max)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_hero_age(&mut self, a: c_float) {
//         unsafe { ((*self.vmt).set_hero_age)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_hero_age(&mut self) -> c_float {
//         unsafe { ((*self.vmt).get_hero_age)(self as *mut CGameScriptInterface) }
//     }

//     pub fn set_hero_as_teenager(&mut self, a: bool) {
//         unsafe { ((*self.vmt).set_hero_as_teenager)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_hero_as_apprentice(&mut self, a: bool) {
//         unsafe { ((*self.vmt).set_hero_as_apprentice)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_distance_hero_can_be_heard_from(&mut self) -> c_float {
//         unsafe {
//             ((*self.vmt).get_distance_hero_can_be_heard_from)(self as *mut CGameScriptInterface)
//         }
//     }

//     pub fn get_hero_rough_experience_level(&mut self) -> c_long {
//         unsafe { ((*self.vmt).get_hero_rough_experience_level)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_hero_experience_available_to_spend(&mut self) -> c_long {
//         unsafe {
//             ((*self.vmt).get_hero_experience_available_to_spend)(self as *mut CGameScriptInterface)
//         }
//     }

//     pub fn get_hero_fatness(&mut self) -> c_float {
//         unsafe { ((*self.vmt).get_hero_fatness)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_hero_scariness(&mut self) -> c_float {
//         unsafe { ((*self.vmt).get_hero_scariness)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_hero_attractiveness(&mut self) -> c_float {
//         unsafe { ((*self.vmt).get_hero_attractiveness)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_hero_will_energy_level(&mut self) -> c_float {
//         unsafe { ((*self.vmt).get_hero_will_energy_level)(self as *mut CGameScriptInterface) }
//     }

//     pub fn set_hero_will_energy_level(&mut self, a: c_float) {
//         unsafe { ((*self.vmt).set_hero_will_energy_level)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_hero_will_energy_as_able_to_refill(&mut self, a: bool) {
//         unsafe {
//             ((*self.vmt).set_hero_will_energy_as_able_to_refill)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn get_number_of_items_of_type_in_inventory(&mut self, a: *const CCharString) -> c_long {
//         unsafe {
//             ((*self.vmt).get_number_of_items_of_type_in_inventory)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn is_object_in_things_possession(
//         &mut self,
//         a: *const CCharString,
//         b: *const CScriptThing,
//     ) -> bool {
//         unsafe {
//             ((*self.vmt).is_object_in_things_possession)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn is_hero_hand_lamp_lit(&mut self) -> bool {
//         unsafe { ((*self.vmt).is_hero_hand_lamp_lit)(self as *mut CGameScriptInterface) }
//     }

//     pub fn set_hero_hand_lamp_as_lit(&mut self, a: bool) {
//         unsafe { ((*self.vmt).set_hero_hand_lamp_as_lit)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn is_wearing_clothing_item(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CCharString,
//     ) -> bool {
//         unsafe { ((*self.vmt).is_wearing_clothing_item)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn is_hero_naked(&mut self) -> bool {
//         unsafe { ((*self.vmt).is_hero_naked)(self as *mut CGameScriptInterface) }
//     }

//     pub fn remove_hero_clothing(&mut self) {
//         unsafe { ((*self.vmt).remove_hero_clothing)(self as *mut CGameScriptInterface) }
//     }

//     pub fn set_hero_as_wearing(&mut self, a: *const CCharString) {
//         unsafe { ((*self.vmt).set_hero_as_wearing)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn change_hero_hairstyle(&mut self, a: *const CCharString) {
//         unsafe { ((*self.vmt).change_hero_hairstyle)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn remove_hero_hairstyle(&mut self) {
//         unsafe { ((*self.vmt).remove_hero_hairstyle)(self as *mut CGameScriptInterface) }
//     }

//     pub fn is_wearing_hairstyle(&mut self, a: *mut CScriptThing, b: *const CCharString) -> bool {
//         unsafe { ((*self.vmt).is_wearing_hairstyle)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn is_player_carrying_item_of_type(&mut self, a: *const CCharString) -> bool {
//         unsafe {
//             ((*self.vmt).is_player_carrying_item_of_type)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn is_player_wielding_weapon(&mut self) -> bool {
//         unsafe { ((*self.vmt).is_player_wielding_weapon)(self as *mut CGameScriptInterface) }
//     }

//     pub fn is_entity_wielding_weapon(&mut self, a: *const CScriptThing) -> bool {
//         unsafe { ((*self.vmt).is_entity_wielding_weapon)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn is_entity_wielding_melee_weapon(&mut self, a: *const CScriptThing) -> bool {
//         unsafe {
//             ((*self.vmt).is_entity_wielding_melee_weapon)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn is_entity_wielding_ranged_weapon(&mut self, a: *const CScriptThing) -> bool {
//         unsafe {
//             ((*self.vmt).is_entity_wielding_ranged_weapon)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn get_previously_wielded_melee_weapon_name(&mut self, a: *mut CCharString) -> bool {
//         unsafe {
//             ((*self.vmt).get_previously_wielded_melee_weapon_name)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn get_previously_wielded_ranged_weapon_name(&mut self, a: *mut CCharString) -> bool {
//         unsafe {
//             ((*self.vmt).get_previously_wielded_ranged_weapon_name)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn is_entity_able_to_attack(&mut self, a: *const CScriptThing) -> bool {
//         unsafe { ((*self.vmt).is_entity_able_to_attack)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn entity_get_thing_in_primary_slot(&mut self, a: *const CScriptThing) -> CScriptThing {
//         unsafe {
//             ((*self.vmt).entity_get_thing_in_primary_slot)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn apply_hero_penalty_for_death(&mut self) {
//         unsafe { ((*self.vmt).apply_hero_penalty_for_death)(self as *mut CGameScriptInterface) }
//     }

//     pub fn give_hero_title(&mut self, a: *const CCharString) {
//         unsafe { ((*self.vmt).give_hero_title)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_hero_title(&mut self) -> EHeroTitle {
//         unsafe { ((*self.vmt).get_hero_title)(self as *mut CGameScriptInterface) }
//     }

//     pub fn entity_set_as_marryable(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).entity_set_as_marryable)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_set_as_able_to_region_follow_when_married(
//         &mut self,
//         a: *const CScriptThing,
//         b: bool,
//     ) {
//         unsafe {
//             ((*self.vmt).entity_set_as_able_to_region_follow_when_married)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn entity_force_marriage_to_hero(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe {
//             ((*self.vmt).entity_force_marriage_to_hero)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn is_entity_married_to_hero(&mut self, a: *const CScriptThing) -> bool {
//         unsafe { ((*self.vmt).is_entity_married_to_hero)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn is_entity_marriable(&mut self, a: *const CScriptThing) -> bool {
//         unsafe { ((*self.vmt).is_entity_marriable)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_hero_has_married(&mut self) -> bool {
//         unsafe { ((*self.vmt).get_hero_has_married)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_hero_has_current_marriage(&mut self) -> bool {
//         unsafe { ((*self.vmt).get_hero_has_current_marriage)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_hero_has_divorced_marriage(&mut self) -> bool {
//         unsafe { ((*self.vmt).get_hero_has_divorced_marriage)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_hero_has_children(&mut self) -> bool {
//         unsafe { ((*self.vmt).get_hero_has_children)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_hero_has_murdered_wife(&mut self) -> bool {
//         unsafe { ((*self.vmt).get_hero_has_murdered_wife)(self as *mut CGameScriptInterface) }
//     }

//     pub fn is_hero_child(&mut self) -> bool {
//         unsafe { ((*self.vmt).is_hero_child)(self as *mut CGameScriptInterface) }
//     }

//     pub fn cancel_hero_teleport_effects(&mut self) {
//         unsafe { ((*self.vmt).cancel_hero_teleport_effects)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_number_of_times_hero_has_had_sex(&mut self) -> c_long {
//         unsafe {
//             ((*self.vmt).get_number_of_times_hero_has_had_sex)(self as *mut CGameScriptInterface)
//         }
//     }

//     pub fn set_number_of_times_hero_has_had_sex(&mut self, a: c_long) {
//         unsafe {
//             ((*self.vmt).set_number_of_times_hero_has_had_sex)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn set_hero_as_having_had_sex(&mut self, a: bool) {
//         unsafe { ((*self.vmt).set_hero_as_having_had_sex)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_hero_as_having_had_gay_sex(&mut self, a: bool) {
//         unsafe {
//             ((*self.vmt).set_hero_as_having_had_gay_sex)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn give_thing_hero_reward_item(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CCharString,
//         c: *const CCharString,
//     ) {
//         unsafe {
//             ((*self.vmt).give_thing_hero_reward_item)(self as *mut CGameScriptInterface, a, b, c)
//         }
//     }

//     pub fn give_thing_item_in_hand(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CCharString,
//         c: bool,
//     ) {
//         unsafe { ((*self.vmt).give_thing_item_in_hand)(self as *mut CGameScriptInterface, a, b, c) }
//     }

//     pub fn give_thing_item_in_slot(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CCharString,
//         c: *const CCharString,
//     ) {
//         unsafe { ((*self.vmt).give_thing_item_in_slot)(self as *mut CGameScriptInterface, a, b, c) }
//     }

//     pub fn give_hero_expression(&mut self, a: *const CCharString, b: c_long, c: bool) {
//         unsafe { ((*self.vmt).give_hero_expression)(self as *mut CGameScriptInterface, a, b, c) }
//     }

//     pub fn hero_has_expression(&mut self, a: *const CCharString) -> bool {
//         unsafe { ((*self.vmt).hero_has_expression)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn is_hero_performing_expression(&mut self, a: *const CCharString) -> bool {
//         unsafe { ((*self.vmt).is_hero_performing_expression)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn is_d_pad_button_held_for_expression(&mut self, a: *const CCharString) -> bool {
//         unsafe {
//             ((*self.vmt).is_d_pad_button_held_for_expression)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn entity_follow_thing(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CScriptThing,
//         c: c_float,
//         d: bool,
//     ) {
//         unsafe { ((*self.vmt).entity_follow_thing)(self as *mut CGameScriptInterface, a, b, c, d) }
//     }

//     pub fn entity_stop_following(&mut self, a: *const CScriptThing) {
//         unsafe { ((*self.vmt).entity_stop_following)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_following_entity_list(
//         &mut self,
//         a: *const CScriptThing,
//         b: *mut CxxVector<CScriptThing>,
//     ) -> bool {
//         unsafe { ((*self.vmt).get_following_entity_list)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn get_perceiving_hero_entity_list(
//         &mut self,
//         a: *mut CxxVector<CScriptThing>,
//     ) -> bool {
//         unsafe {
//             ((*self.vmt).get_perceiving_hero_entity_list)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn get_hero_summoned_creatures_list(
//         &mut self,
//         a: *mut CxxVector<CScriptThing>,
//     ) -> bool {
//         unsafe {
//             ((*self.vmt).get_hero_summoned_creatures_list)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn is_entity_following_hero(&mut self, a: *const CScriptThing) -> bool {
//         unsafe { ((*self.vmt).is_entity_following_hero)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn entity_set_as_allowed_to_follow_hero(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe {
//             ((*self.vmt).entity_set_as_allowed_to_follow_hero)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn entity_set_as_allowed_to_change_region_following_state(
//         &mut self,
//         a: *const CScriptThing,
//         b: bool,
//     ) {
//         unsafe {
//             ((*self.vmt).entity_set_as_allowed_to_change_region_following_state)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn entity_set_as_responding_to_follow_and_wait_expressions(
//         &mut self,
//         a: *const CScriptThing,
//         b: bool,
//     ) {
//         unsafe {
//             ((*self.vmt).entity_set_as_responding_to_follow_and_wait_expressions)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn entity_set_as_mirroring_hero_enemy_relations_while_following(
//         &mut self,
//         a: *const CScriptThing,
//         b: bool,
//     ) {
//         unsafe {
//             ((*self.vmt).entity_set_as_mirroring_hero_enemy_relations_while_following)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn teleport_all_followers_to_hero_position(&mut self) {
//         unsafe {
//             ((*self.vmt).teleport_all_followers_to_hero_position)(self as *mut CGameScriptInterface)
//         }
//     }

//     pub fn entity_teleport_to_hero_position(&mut self, a: *const CScriptThing) {
//         unsafe {
//             ((*self.vmt).entity_teleport_to_hero_position)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn send_entity_event(
//         &mut self,
//         a: EEventType,
//         b: *const CScriptThing,
//         c: *const CScriptThing,
//         d: *mut CThing,
//     ) {
//         unsafe { ((*self.vmt).send_entity_event)(self as *mut CGameScriptInterface, a, b, c, d) }
//     }

//     pub fn get_water_height_at_position(&mut self, a: *const C3DVector) -> c_float {
//         unsafe { ((*self.vmt).get_water_height_at_position)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn is_fishing_spot_enabled(&mut self, a: *const CScriptThing) -> bool {
//         unsafe { ((*self.vmt).is_fishing_spot_enabled)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn disable_fishing_spot(&mut self, a: *const CScriptThing) {
//         unsafe { ((*self.vmt).disable_fishing_spot)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn update_fish_weight(&mut self, a: *const CScriptThing) {
//         unsafe { ((*self.vmt).update_fish_weight)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_best_fish_weight(&mut self, a: *const CCharString) -> c_float {
//         unsafe { ((*self.vmt).get_best_fish_weight)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn hero_go_fishing(&mut self, a: bool) {
//         unsafe { ((*self.vmt).hero_go_fishing)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_hero_fishing_level(&mut self) -> c_long {
//         unsafe { ((*self.vmt).get_hero_fishing_level)(self as *mut CGameScriptInterface) }
//     }

//     pub fn hero_go_digging(&mut self) {
//         unsafe { ((*self.vmt).hero_go_digging)(self as *mut CGameScriptInterface) }
//     }

//     pub fn hero_stop_digging(&mut self) {
//         unsafe { ((*self.vmt).hero_stop_digging)(self as *mut CGameScriptInterface) }
//     }

//     pub fn hero_play_oracle_minigame(&mut self) {
//         unsafe { ((*self.vmt).hero_play_oracle_minigame)(self as *mut CGameScriptInterface) }
//     }

//     pub fn is_hero_playing_oracle_minigame(&mut self) -> bool {
//         unsafe { ((*self.vmt).is_hero_playing_oracle_minigame)(self as *mut CGameScriptInterface) }
//     }

//     pub fn has_hero_won_oracle_minigame(&mut self) -> bool {
//         unsafe { ((*self.vmt).has_hero_won_oracle_minigame)(self as *mut CGameScriptInterface) }
//     }

//     pub fn hero_play_fireheart_minigame(&mut self) {
//         unsafe { ((*self.vmt).hero_play_fireheart_minigame)(self as *mut CGameScriptInterface) }
//     }

//     pub fn hero_quit_fireheart_minigame(&mut self) {
//         unsafe { ((*self.vmt).hero_quit_fireheart_minigame)(self as *mut CGameScriptInterface) }
//     }

//     pub fn has_hero_force_quit_fireheart_minigame(&mut self) -> bool {
//         unsafe {
//             ((*self.vmt).has_hero_force_quit_fireheart_minigame)(self as *mut CGameScriptInterface)
//         }
//     }

//     pub fn get_hero_health(&mut self) -> c_float {
//         unsafe { ((*self.vmt).get_hero_health)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_hero_health_max(&mut self) -> c_float {
//         unsafe { ((*self.vmt).get_hero_health_max)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_hero_health_percentage(&mut self) -> c_float {
//         unsafe { ((*self.vmt).get_hero_health_percentage)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_hero_will_energy(&mut self) -> c_long {
//         unsafe { ((*self.vmt).get_hero_will_energy)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_hero_will_energy_max(&mut self) -> c_long {
//         unsafe { ((*self.vmt).get_hero_will_energy_max)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_hero_will_energy_percentage(&mut self) -> c_long {
//         unsafe { ((*self.vmt).get_hero_will_energy_percentage)(self as *mut CGameScriptInterface) }
//     }

//     pub fn change_hero_health_by(&mut self, a: c_float, b: bool, c: bool) {
//         unsafe { ((*self.vmt).change_hero_health_by)(self as *mut CGameScriptInterface, a, b, c) }
//     }

//     pub fn set_thing_as_killed(&mut self, a: *const CScriptThing) {
//         unsafe { ((*self.vmt).set_thing_as_killed)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_health(&mut self, a: *const CScriptThing) -> c_float {
//         unsafe { ((*self.vmt).get_health)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn modify_thing_health(&mut self, a: *const CScriptThing, b: c_float, c: bool) {
//         unsafe { ((*self.vmt).modify_thing_health)(self as *mut CGameScriptInterface, a, b, c) }
//     }

//     pub fn entity_set_max_health(&mut self, a: *const CScriptThing, b: c_float, c: bool) {
//         unsafe { ((*self.vmt).entity_set_max_health)(self as *mut CGameScriptInterface, a, b, c) }
//     }

//     pub fn give_hero_new_quest_objective(&mut self, a: *const CCharString, b: c_ulong) {
//         unsafe {
//             ((*self.vmt).give_hero_new_quest_objective)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn tell_hero_quest_objective_completed(&mut self, a: c_ulong) {
//         unsafe {
//             ((*self.vmt).tell_hero_quest_objective_completed)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn tell_hero_quest_objective_failed(&mut self, a: c_ulong) {
//         unsafe {
//             ((*self.vmt).tell_hero_quest_objective_failed)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn add_quest_region(&mut self, a: *const CCharString, b: *const CCharString) {
//         unsafe { ((*self.vmt).add_quest_region)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn set_quest_world_map_offset(&mut self, a: *const CCharString, b: *const C2DCoordI) {
//         unsafe { ((*self.vmt).set_quest_world_map_offset)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn is_hero_on_quest(&mut self) -> bool {
//         unsafe { ((*self.vmt).is_hero_on_quest)(self as *mut CGameScriptInterface) }
//     }

//     pub fn hero_receive_message_from_guild_master(
//         &mut self,
//         a: *const CCharString,
//         b: *const CCharString,
//         c: bool,
//         d: bool,
//     ) {
//         unsafe {
//             ((*self.vmt).hero_receive_message_from_guild_master)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//                 c,
//                 d,
//             )
//         }
//     }

//     pub fn set_guild_master_messages(&mut self, a: bool) {
//         unsafe { ((*self.vmt).set_guild_master_messages)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn activate_quest(&mut self, a: *const CCharString) {
//         unsafe { ((*self.vmt).activate_quest)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn activate_multiple_quests(&mut self, a: *const CArray<CCharString>) {
//         unsafe { ((*self.vmt).activate_multiple_quests)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn activate_quest_without_loading_resources(&mut self, a: *const CCharString) {
//         unsafe {
//             ((*self.vmt).activate_quest_without_loading_resources)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn activate_multiple_quests_without_loading_resources(
//         &mut self,
//         a: *const CArray<CCharString>,
//     ) {
//         unsafe {
//             ((*self.vmt).activate_multiple_quests_without_loading_resources)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn deactivate_quest(&mut self, a: *const CCharString, b: c_ulong) {
//         unsafe { ((*self.vmt).deactivate_quest)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn deactivate_quest_later(&mut self, a: *const CCharString, b: c_ulong) {
//         unsafe { ((*self.vmt).deactivate_quest_later)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn prepare_quests_when_final_quest_is_activated(&mut self, a: *const CCharString) {
//         unsafe {
//             ((*self.vmt).prepare_quests_when_final_quest_is_activated)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn prepare_quests_when_final_quest_is_completed(&mut self) {
//         unsafe {
//             ((*self.vmt).prepare_quests_when_final_quest_is_completed)(
//                 self as *mut CGameScriptInterface,
//             )
//         }
//     }

//     pub fn is_quest_active(&mut self, a: *const CCharString) -> bool {
//         unsafe { ((*self.vmt).is_quest_active)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn is_quest_registered(&mut self, a: *const CCharString) -> bool {
//         unsafe { ((*self.vmt).is_quest_registered)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn is_quest_completed(&mut self, a: *const CCharString) -> bool {
//         unsafe { ((*self.vmt).is_quest_completed)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn is_quest_failed(&mut self, a: *const CCharString) -> bool {
//         unsafe { ((*self.vmt).is_quest_failed)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_quest_as_completed(&mut self, a: *const CCharString, b: bool, c: bool, d: bool) {
//         unsafe {
//             ((*self.vmt).set_quest_as_completed)(self as *mut CGameScriptInterface, a, b, c, d)
//         }
//     }

//     pub fn set_quest_as_failed(
//         &mut self,
//         a: *const CCharString,
//         b: bool,
//         c: *const CWideString,
//         d: bool,
//     ) {
//         unsafe { ((*self.vmt).set_quest_as_failed)(self as *mut CGameScriptInterface, a, b, c, d) }
//     }

//     pub fn set_quest_as_persistent(&mut self, a: *const CCharString, b: bool) {
//         unsafe { ((*self.vmt).set_quest_as_persistent)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn get_exclusive_quest_script_name(&mut self) -> *const CCharString {
//         unsafe { ((*self.vmt).get_exclusive_quest_script_name)(self as *mut CGameScriptInterface) }
//     }

//     pub fn add_quest_card(
//         &mut self,
//         a: *const CCharString,
//         b: *const CCharString,
//         c: bool,
//         d: bool,
//     ) {
//         unsafe { ((*self.vmt).add_quest_card)(self as *mut CGameScriptInterface, a, b, c, d) }
//     }

//     pub fn remove_quest_card_from_guild(&mut self, a: *const CCharString) {
//         unsafe { ((*self.vmt).remove_quest_card_from_guild)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn remove_quest_card_from_hero(&mut self, a: *const CCharString) {
//         unsafe { ((*self.vmt).remove_quest_card_from_hero)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn give_hero_quest_card_directly(
//         &mut self,
//         a: *const CCharString,
//         b: *const CCharString,
//         c: bool,
//     ) {
//         unsafe {
//             ((*self.vmt).give_hero_quest_card_directly)(self as *mut CGameScriptInterface, a, b, c)
//         }
//     }

//     pub fn set_quest_card_objective(
//         &mut self,
//         a: *const CCharString,
//         b: *const CCharString,
//         c: *const CCharString,
//         d: *const CCharString,
//     ) {
//         unsafe {
//             ((*self.vmt).set_quest_card_objective)(self as *mut CGameScriptInterface, a, b, c, d)
//         }
//     }

//     pub fn set_quest_card_gold_reward(&mut self, a: *const CCharString, b: c_long) {
//         unsafe { ((*self.vmt).set_quest_card_gold_reward)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn set_quest_card_renown_reward(&mut self, a: *const CCharString, b: c_long) {
//         unsafe {
//             ((*self.vmt).set_quest_card_renown_reward)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn remove_all_available_quest_cards_from_guild(&mut self) {
//         unsafe {
//             ((*self.vmt).remove_all_available_quest_cards_from_guild)(
//                 self as *mut CGameScriptInterface,
//             )
//         }
//     }

//     pub fn fail_all_active_quests(&mut self) {
//         unsafe { ((*self.vmt).fail_all_active_quests)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_all_active_quest_info(
//         &mut self,
//         a: *mut CxxVector<CCharString>,
//         b: *mut CxxVector<CCharString>,
//     ) {
//         unsafe { ((*self.vmt).get_all_active_quest_info)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn add_feat_card(&mut self, a: c_long, b: *const CCharString, c: *const CCharString) {
//         unsafe { ((*self.vmt).add_feat_card)(self as *mut CGameScriptInterface, a, b, c) }
//     }

//     pub fn add_boast(
//         &mut self,
//         a: *const CCharString,
//         b: c_long,
//         c: c_long,
//         d: c_long,
//         e: bool,
//         f: *const CCharString,
//         g: c_long,
//     ) {
//         unsafe { ((*self.vmt).add_boast)(self as *mut CGameScriptInterface, a, b, c, d, e, f, g) }
//     }

//     pub fn remove_boast(&mut self, a: c_long, b: *const CCharString) {
//         unsafe { ((*self.vmt).remove_boast)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn set_boast_as_failed(&mut self, a: c_long, b: *const CCharString) {
//         unsafe { ((*self.vmt).set_boast_as_failed)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn set_boast_as_completed(&mut self, a: c_long, b: *const CCharString) {
//         unsafe { ((*self.vmt).set_boast_as_completed)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn is_boast_taken(&mut self, a: c_long, b: *const CCharString) -> bool {
//         unsafe { ((*self.vmt).is_boast_taken)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn add_log_book_entry(
//         &mut self,
//         a: *const CWideString,
//         b: *const CWideString,
//         c: *const CWideString,
//         d: ECategory,
//     ) {
//         unsafe { ((*self.vmt).add_log_book_entry)(self as *mut CGameScriptInterface, a, b, c, d) }
//     }

//     pub fn kick_off_quest_start_screen(&mut self, a: *const CCharString, b: bool, c: bool) {
//         unsafe {
//             ((*self.vmt).kick_off_quest_start_screen)(self as *mut CGameScriptInterface, a, b, c)
//         }
//     }

//     pub fn kick_off_death_screen(&mut self) {
//         unsafe { ((*self.vmt).kick_off_death_screen)(self as *mut CGameScriptInterface) }
//     }

//     pub fn kick_off_credits_screen(&mut self, a: *mut CCharString) {
//         unsafe { ((*self.vmt).kick_off_credits_screen)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_preferred_quick_access_item(&mut self, a: *const CCharString, b: c_long, c: c_long) {
//         unsafe {
//             ((*self.vmt).set_preferred_quick_access_item)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//                 c,
//             )
//         }
//     }

//     pub fn get_death_recovery_marker_name(&mut self) -> CCharString {
//         unsafe { ((*self.vmt).get_death_recovery_marker_name)(self as *mut CGameScriptInterface) }
//     }

//     pub fn set_death_recovery_marker_name(&mut self, a: *const CCharString) {
//         unsafe {
//             ((*self.vmt).set_death_recovery_marker_name)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn reset_death_recovery_marker_name_to_default(&mut self) {
//         unsafe {
//             ((*self.vmt).reset_death_recovery_marker_name_to_default)(
//                 self as *mut CGameScriptInterface,
//             )
//         }
//     }

//     pub fn is_to_fail_quest_on_death(&mut self) -> bool {
//         unsafe { ((*self.vmt).is_to_fail_quest_on_death)(self as *mut CGameScriptInterface) }
//     }

//     pub fn set_whether_to_fail_quest_on_death(&mut self, a: bool) {
//         unsafe {
//             ((*self.vmt).set_whether_to_fail_quest_on_death)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn reset_whether_to_fail_quest_on_death_to_default(&mut self) {
//         unsafe {
//             ((*self.vmt).reset_whether_to_fail_quest_on_death_to_default)(
//                 self as *mut CGameScriptInterface,
//             )
//         }
//     }

//     pub fn get_most_recent_valid_used_target(&mut self) -> CScriptThing {
//         unsafe {
//             ((*self.vmt).get_most_recent_valid_used_target)(self as *mut CGameScriptInterface)
//         }
//     }

//     pub fn get_most_recent_valid_used_target_name(&mut self) -> CCharString {
//         unsafe {
//             ((*self.vmt).get_most_recent_valid_used_target_name)(self as *mut CGameScriptInterface)
//         }
//     }

//     pub fn display_quest_info(&mut self, a: bool) {
//         unsafe { ((*self.vmt).display_quest_info)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_quest_info_name(&mut self, a: *const c_char) {
//         unsafe { ((*self.vmt).set_quest_info_name)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_quest_info_text(&mut self, a: *const c_char) {
//         unsafe { ((*self.vmt).set_quest_info_text)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn add_quest_info_bar(
//         &mut self,
//         a: c_float,
//         b: c_float,
//         c: *const CRGBColour,
//         d: *const CRGBColour,
//         e: *const CCharString,
//         f: *const CCharString,
//         g: c_float,
//     ) -> c_long {
//         unsafe {
//             ((*self.vmt).add_quest_info_bar)(self as *mut CGameScriptInterface, a, b, c, d, e, f, g)
//         }
//     }

//     pub fn add_quest_info_bar_health(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CRGBColour,
//         c: *const CCharString,
//         d: c_float,
//     ) -> c_long {
//         unsafe {
//             ((*self.vmt).add_quest_info_bar_health)(self as *mut CGameScriptInterface, a, b, c, d)
//         }
//     }

//     pub fn add_quest_info_timer(
//         &mut self,
//         a: *const CTimer,
//         b: *const CCharString,
//         c: c_float,
//     ) -> c_long {
//         unsafe { ((*self.vmt).add_quest_info_timer)(self as *mut CGameScriptInterface, a, b, c) }
//     }

//     pub fn add_quest_info_counter(
//         &mut self,
//         a: *const CCharString,
//         b: c_long,
//         c: c_float,
//     ) -> c_long {
//         unsafe { ((*self.vmt).add_quest_info_counter)(self as *mut CGameScriptInterface, a, b, c) }
//     }

//     pub fn add_quest_info_counter_2(
//         &mut self,
//         a: *const CCounter,
//         b: *const CCharString,
//         c: c_float,
//     ) -> c_long {
//         unsafe {
//             ((*self.vmt).add_quest_info_counter_2)(self as *mut CGameScriptInterface, a, b, c)
//         }
//     }

//     pub fn add_quest_info_counter_list(
//         &mut self,
//         a: *const CCharString,
//         b: c_long,
//         c: c_float,
//     ) -> c_long {
//         unsafe {
//             ((*self.vmt).add_quest_info_counter_list)(self as *mut CGameScriptInterface, a, b, c)
//         }
//     }

//     pub fn add_quest_info_tick(&mut self, a: EGameAction, b: bool, c: c_float) -> c_long {
//         unsafe { ((*self.vmt).add_quest_info_tick)(self as *mut CGameScriptInterface, a, b, c) }
//     }

//     pub fn add_quest_info_tick_2(&mut self, a: *const CCharString, b: bool, c: c_float) -> c_long {
//         unsafe { ((*self.vmt).add_quest_info_tick_2)(self as *mut CGameScriptInterface, a, b, c) }
//     }

//     pub fn update_quest_info_bar(&mut self, a: c_long, b: c_float, c: c_float, d: c_float) {
//         unsafe {
//             ((*self.vmt).update_quest_info_bar)(self as *mut CGameScriptInterface, a, b, c, d)
//         }
//     }

//     pub fn change_quest_info_bar_colour(
//         &mut self,
//         a: c_long,
//         b: *const CRGBColour,
//         c: *const CRGBColour,
//     ) {
//         unsafe {
//             ((*self.vmt).change_quest_info_bar_colour)(self as *mut CGameScriptInterface, a, b, c)
//         }
//     }

//     pub fn update_quest_info_timer(&mut self, a: c_long, b: c_float) {
//         unsafe { ((*self.vmt).update_quest_info_timer)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn update_quest_info_counter(&mut self, a: c_long, b: c_long, c: c_long) {
//         unsafe {
//             ((*self.vmt).update_quest_info_counter)(self as *mut CGameScriptInterface, a, b, c)
//         }
//     }

//     pub fn update_quest_info_counter_list(&mut self, a: c_long, b: c_long, c: c_long) {
//         unsafe {
//             ((*self.vmt).update_quest_info_counter_list)(self as *mut CGameScriptInterface, a, b, c)
//         }
//     }

//     pub fn update_quest_info_tick(&mut self, a: c_long, b: bool) {
//         unsafe { ((*self.vmt).update_quest_info_tick)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn remove_quest_info_element(&mut self, a: c_long) {
//         unsafe { ((*self.vmt).remove_quest_info_element)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn remove_all_quest_info_elements(&mut self) {
//         unsafe { ((*self.vmt).remove_all_quest_info_elements)(self as *mut CGameScriptInterface) }
//     }

//     pub fn display_time(&mut self, a: bool) {
//         unsafe { ((*self.vmt).display_time)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn display_money_bag(&mut self, a: bool) {
//         unsafe { ((*self.vmt).display_money_bag)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn display_mini_game_info(&mut self, a: bool, b: EMiniGameType) {
//         unsafe { ((*self.vmt).display_mini_game_info)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn update_mini_game_info_bar(&mut self, a: c_float) {
//         unsafe { ((*self.vmt).update_mini_game_info_bar)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn is_entity_pick_pocketable(&mut self, a: *const CScriptThing) -> bool {
//         unsafe { ((*self.vmt).is_entity_pick_pocketable)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn is_entity_pick_lockable(&mut self, a: *const CScriptThing) -> bool {
//         unsafe { ((*self.vmt).is_entity_pick_lockable)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn is_entity_stealable(&mut self, a: *const CScriptThing) -> bool {
//         unsafe { ((*self.vmt).is_entity_stealable)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn entity_set_as_pick_pocketed(&mut self, a: *const CScriptThing) {
//         unsafe { ((*self.vmt).entity_set_as_pick_pocketed)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn entity_set_as_pick_locked(&mut self, a: *const CScriptThing) {
//         unsafe { ((*self.vmt).entity_set_as_pick_locked)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn entity_set_as_stolen(&mut self, a: *const CScriptThing) {
//         unsafe { ((*self.vmt).entity_set_as_stolen)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn mini_map_add_marker(&mut self, a: *const CScriptThing, b: *const CCharString) {
//         unsafe { ((*self.vmt).mini_map_add_marker)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn mini_map_set_marker_graphic(&mut self, a: *const CScriptThing, b: *const CCharString) {
//         unsafe {
//             ((*self.vmt).mini_map_set_marker_graphic)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn mini_map_remove_marker(&mut self, a: *const CScriptThing) {
//         unsafe { ((*self.vmt).mini_map_remove_marker)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn mini_map_remove_all_markers(&mut self) {
//         unsafe { ((*self.vmt).mini_map_remove_all_markers)(self as *mut CGameScriptInterface) }
//     }

//     pub fn mini_map_allow_route_between_regions(
//         &mut self,
//         a: *const CCharString,
//         b: *const CCharString,
//         c: bool,
//     ) {
//         unsafe {
//             ((*self.vmt).mini_map_allow_route_between_regions)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//                 c,
//             )
//         }
//     }

//     pub fn mini_map_set_as_enabled(&mut self, a: bool) {
//         unsafe { ((*self.vmt).mini_map_set_as_enabled)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn entity_set_as_hidden_on_mini_map(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe {
//             ((*self.vmt).entity_set_as_hidden_on_mini_map)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn set_hud_enabled(&mut self, a: bool) {
//         unsafe { ((*self.vmt).set_hud_enabled)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn text_entry_exists(&mut self, a: *const CCharString) -> bool {
//         unsafe { ((*self.vmt).text_entry_exists)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_valid_text_entry_name_with_attitude(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CCharString,
//     ) -> CCharString {
//         unsafe {
//             ((*self.vmt).get_valid_text_entry_name_with_attitude)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn set_thing_has_information(&mut self, a: *const CScriptThing, b: bool, c: bool, d: bool) {
//         unsafe {
//             ((*self.vmt).set_thing_has_information)(self as *mut CGameScriptInterface, a, b, c, d)
//         }
//     }

//     pub fn clear_thing_has_information(&mut self, a: *const CScriptThing) {
//         unsafe { ((*self.vmt).clear_thing_has_information)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn entity_set_will_be_using_narrator(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CCharString,
//     ) {
//         unsafe {
//             ((*self.vmt).entity_set_will_be_using_narrator)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn entity_reset_as_pure_ai_narrator(&mut self, a: *const CScriptThing) {
//         unsafe {
//             ((*self.vmt).entity_reset_as_pure_ai_narrator)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn add_new_conversation(&mut self, a: *const CScriptThing, b: bool, c: bool) -> c_long {
//         unsafe { ((*self.vmt).add_new_conversation)(self as *mut CGameScriptInterface, a, b, c) }
//     }

//     pub fn add_person_to_conversation(&mut self, a: c_long, b: *const CScriptThing) {
//         unsafe { ((*self.vmt).add_person_to_conversation)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn add_line_to_conversation(
//         &mut self,
//         a: c_long,
//         b: *const CCharString,
//         c: bool,
//         d: *const CScriptThing,
//         e: *const CScriptThing,
//     ) {
//         unsafe {
//             ((*self.vmt).add_line_to_conversation)(self as *mut CGameScriptInterface, a, b, c, d, e)
//         }
//     }

//     pub fn remove_conversation(&mut self, a: c_long, b: bool) {
//         unsafe { ((*self.vmt).remove_conversation)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn is_conversation_active(&mut self, a: c_long) -> bool {
//         unsafe { ((*self.vmt).is_conversation_active)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn play_avi_movie(&mut self, a: *const CCharString) {
//         unsafe { ((*self.vmt).play_avi_movie)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn start_movie_sequence(
//         &mut self,
//         a: *const CCharString,
//         b: *mut CScriptGameResourceObjectMovieBase,
//     ) {
//         unsafe { ((*self.vmt).start_movie_sequence)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn fix_movie_sequence_camera(&mut self, a: bool) {
//         unsafe { ((*self.vmt).fix_movie_sequence_camera)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn fade_screen_out_until_next_call_to_fade_screen_in(&mut self, a: c_float, b: c_float) {
//         unsafe {
//             ((*self.vmt).fade_screen_out_until_next_call_to_fade_screen_in)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn fade_screen_out(&mut self, a: c_float, b: c_float, c: CRGBColour) -> bool {
//         unsafe { ((*self.vmt).fade_screen_out)(self as *mut CGameScriptInterface, a, b, c) }
//     }

//     pub fn fade_screen_in(&mut self) {
//         unsafe { ((*self.vmt).fade_screen_in)(self as *mut CGameScriptInterface) }
//     }

//     pub fn is_screen_fading_out(&mut self) -> bool {
//         unsafe { ((*self.vmt).is_screen_fading_out)(self as *mut CGameScriptInterface) }
//     }

//     pub fn pause(&mut self, a: c_float) {
//         unsafe { ((*self.vmt).pause)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn end_cut_fade(&mut self) {
//         unsafe { ((*self.vmt).end_cut_fade)(self as *mut CGameScriptInterface) }
//     }

//     pub fn pause_all_non_scripted_entities(&mut self, a: bool) {
//         unsafe {
//             ((*self.vmt).pause_all_non_scripted_entities)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn pause_all_entities(&mut self, a: bool) {
//         unsafe { ((*self.vmt).pause_all_entities)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_allow_screen_fading_on_next_region_change(&mut self, a: bool) {
//         unsafe {
//             ((*self.vmt).set_allow_screen_fading_on_next_region_change)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn set_allow_screen_fading_if_already_faded(&mut self, a: bool) {
//         unsafe {
//             ((*self.vmt).set_allow_screen_fading_if_already_faded)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn set_to_keep_hero_abilities_during_cutscenes(&mut self, a: bool) {
//         unsafe {
//             ((*self.vmt).set_to_keep_hero_abilities_during_cutscenes)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn set_to_display_tutorials_during_cutscenes(&mut self, a: bool) {
//         unsafe {
//             ((*self.vmt).set_to_display_tutorials_during_cutscenes)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn set_cutscene_mode(&mut self, a: bool, b: bool) {
//         unsafe { ((*self.vmt).set_cutscene_mode)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn is_in_cutscene(&mut self) -> bool {
//         unsafe { ((*self.vmt).is_in_cutscene)(self as *mut CGameScriptInterface) }
//     }

//     pub fn set_cutscene_skippable(&mut self, a: bool) {
//         unsafe { ((*self.vmt).set_cutscene_skippable)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_cutscene_skippable_while_paused(&mut self, a: bool) {
//         unsafe {
//             ((*self.vmt).set_cutscene_skippable_while_paused)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn set_cutscene_action_mode(&mut self, a: bool, b: *const CCharString) {
//         unsafe { ((*self.vmt).set_cutscene_action_mode)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn preload_new_scene(&mut self, a: c_float) {
//         unsafe { ((*self.vmt).preload_new_scene)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn start_progress_display(&mut self) {
//         unsafe { ((*self.vmt).start_progress_display)(self as *mut CGameScriptInterface) }
//     }

//     pub fn stop_progress_display(&mut self) {
//         unsafe { ((*self.vmt).stop_progress_display)(self as *mut CGameScriptInterface) }
//     }

//     pub fn set_screen_messages_enabled(&mut self, a: bool) {
//         unsafe { ((*self.vmt).set_screen_messages_enabled)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn is_hero_controlled_by_player(&mut self) -> bool {
//         unsafe { ((*self.vmt).is_hero_controlled_by_player)(self as *mut CGameScriptInterface) }
//     }

//     pub fn is_in_movie_sequence(&mut self) -> bool {
//         unsafe { ((*self.vmt).is_in_movie_sequence)(self as *mut CGameScriptInterface) }
//     }

//     pub fn cancel_abilities_for_cutscene(&mut self) {
//         unsafe { ((*self.vmt).cancel_abilities_for_cutscene)(self as *mut CGameScriptInterface) }
//     }

//     pub fn resume_abilities_for_cutscene(&mut self) {
//         unsafe { ((*self.vmt).resume_abilities_for_cutscene)(self as *mut CGameScriptInterface) }
//     }

//     pub fn cancel_using_ability(&mut self, a: EHeroAbility) {
//         unsafe { ((*self.vmt).cancel_using_ability)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_ability_availability(&mut self, a: EHeroAbility, b: bool) {
//         unsafe { ((*self.vmt).set_ability_availability)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn set_environmental_effects_always_update(&mut self, a: bool) {
//         unsafe {
//             ((*self.vmt).set_environmental_effects_always_update)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn set_dead_creatures_and_experience_orbs_and_drop_bags_as_hidden(&mut self, a: bool) {
//         unsafe {
//             ((*self.vmt).set_dead_creatures_and_experience_orbs_and_drop_bags_as_hidden)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn remove_dead_creature(&mut self, a: *const CCharString) {
//         unsafe { ((*self.vmt).remove_dead_creature)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn camera_set_camera_preload_flag(&mut self, a: bool) {
//         unsafe {
//             ((*self.vmt).camera_set_camera_preload_flag)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn camera_circle_around_thing(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const C3DVector,
//         c: c_float,
//     ) {
//         unsafe {
//             ((*self.vmt).camera_circle_around_thing)(self as *mut CGameScriptInterface, a, b, c)
//         }
//     }

//     pub fn camera_circle_around_pos(
//         &mut self,
//         a: *const C3DVector,
//         b: *const C3DVector,
//         c: c_float,
//     ) {
//         unsafe {
//             ((*self.vmt).camera_circle_around_pos)(self as *mut CGameScriptInterface, a, b, c)
//         }
//     }

//     pub fn camera_move_to_pos_and_look_at_pos(
//         &mut self,
//         a: *const C3DVector,
//         b: *const C3DVector,
//         c: c_float,
//     ) {
//         unsafe {
//             ((*self.vmt).camera_move_to_pos_and_look_at_pos)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//                 c,
//             )
//         }
//     }

//     pub fn camera_move_to_pos_and_look_at_thing(
//         &mut self,
//         a: *const C3DVector,
//         b: *const CScriptThing,
//         c: c_float,
//     ) {
//         unsafe {
//             ((*self.vmt).camera_move_to_pos_and_look_at_thing)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//                 c,
//             )
//         }
//     }

//     pub fn camera_move_between_looking_at(
//         &mut self,
//         a: *const C3DVector,
//         b: *const C3DVector,
//         c: *const CScriptThing,
//         d: c_float,
//         e: c_float,
//     ) {
//         unsafe {
//             ((*self.vmt).camera_move_between_looking_at)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//                 c,
//                 d,
//                 e,
//             )
//         }
//     }

//     pub fn camera_move_between_looking_at_2(
//         &mut self,
//         a: *const C3DVector,
//         b: *const C3DVector,
//         c: *const C3DVector,
//         d: c_float,
//         e: c_float,
//     ) {
//         unsafe {
//             ((*self.vmt).camera_move_between_looking_at_2)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//                 c,
//                 d,
//                 e,
//             )
//         }
//     }

//     pub fn camera_move_between_look_from_and_look_to(
//         &mut self,
//         a: *const C3DVector,
//         b: *const C3DVector,
//         c: *const C3DVector,
//         d: *const C3DVector,
//         e: c_float,
//     ) {
//         unsafe {
//             ((*self.vmt).camera_move_between_look_from_and_look_to)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//                 c,
//                 d,
//                 e,
//             )
//         }
//     }

//     pub fn camera_use_camera_point(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const C3DVector,
//         c: *const CRightHandedSet,
//         d: c_float,
//         e: c_long,
//         f: c_long,
//     ) {
//         unsafe {
//             ((*self.vmt).camera_use_camera_point)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//                 c,
//                 d,
//                 e,
//                 f,
//             )
//         }
//     }

//     pub fn camera_use_camera_point_2(
//         &mut self,
//         a: *const CCharString,
//         b: *const C3DVector,
//         c: *const CRightHandedSet,
//         d: c_float,
//         e: c_long,
//         f: c_long,
//     ) {
//         unsafe {
//             ((*self.vmt).camera_use_camera_point_2)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//                 c,
//                 d,
//                 e,
//                 f,
//             )
//         }
//     }

//     pub fn camera_use_camera_point_3(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CScriptThing,
//         c: c_float,
//         d: c_long,
//         e: c_long,
//     ) {
//         unsafe {
//             ((*self.vmt).camera_use_camera_point_3)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//                 c,
//                 d,
//                 e,
//             )
//         }
//     }

//     pub fn camera_use_camera_point_4(
//         &mut self,
//         a: *const CCharString,
//         b: *const CScriptThing,
//         c: c_float,
//         d: c_long,
//         e: c_long,
//     ) {
//         unsafe {
//             ((*self.vmt).camera_use_camera_point_4)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//                 c,
//                 d,
//                 e,
//             )
//         }
//     }

//     pub fn camera_do_conversation(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CScriptThing,
//         c: ECameraOp,
//         d: bool,
//     ) {
//         unsafe {
//             ((*self.vmt).camera_do_conversation)(self as *mut CGameScriptInterface, a, b, c, d)
//         }
//     }

//     pub fn camera_default(&mut self) {
//         unsafe { ((*self.vmt).camera_default)(self as *mut CGameScriptInterface) }
//     }

//     pub fn camera_reset_to_view_behind_hero(&mut self, a: c_float) {
//         unsafe {
//             ((*self.vmt).camera_reset_to_view_behind_hero)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn is_camera_in_scripted_mode(&mut self) -> bool {
//         unsafe { ((*self.vmt).is_camera_in_scripted_mode)(self as *mut CGameScriptInterface) }
//     }

//     pub fn camera_use_screen_effect(&mut self, a: c_float, b: c_float, c: c_float) {
//         unsafe {
//             ((*self.vmt).camera_use_screen_effect)(self as *mut CGameScriptInterface, a, b, c)
//         }
//     }

//     pub fn camera_cancel_screen_effect(&mut self) {
//         unsafe { ((*self.vmt).camera_cancel_screen_effect)(self as *mut CGameScriptInterface) }
//     }

//     pub fn is_camera_pos_on_screen(&mut self, a: *const C3DVector) -> bool {
//         unsafe { ((*self.vmt).is_camera_pos_on_screen)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_game_angle_xy(&mut self) -> c_float {
//         unsafe { ((*self.vmt).get_game_angle_xy)(self as *mut CGameScriptInterface) }
//     }

//     pub fn camera_earthquake_intensity_at_pos(
//         &mut self,
//         a: *const C3DVector,
//         b: c_float,
//         c: c_float,
//     ) {
//         unsafe {
//             ((*self.vmt).camera_earthquake_intensity_at_pos)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//                 c,
//             )
//         }
//     }

//     pub fn camera_shake(&mut self, a: c_float, b: c_float) {
//         unsafe { ((*self.vmt).camera_shake)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn open_door(&mut self, a: *const CScriptThing) {
//         unsafe { ((*self.vmt).open_door)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn close_door(&mut self, a: *const CScriptThing) {
//         unsafe { ((*self.vmt).close_door)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn open_house_doors(&mut self, a: *const CScriptThing) {
//         unsafe { ((*self.vmt).open_house_doors)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn close_house_doors(&mut self, a: *const CScriptThing) {
//         unsafe { ((*self.vmt).close_house_doors)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn jam_door(&mut self, a: *const CScriptThing) {
//         unsafe { ((*self.vmt).jam_door)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_door_trigger_type(&mut self, a: *const CScriptThing, b: EDoorTriggerType) {
//         unsafe { ((*self.vmt).set_door_trigger_type)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn override_automatic_house_locking(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe {
//             ((*self.vmt).override_automatic_house_locking)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn set_house_owned_by_player(&mut self, a: *const CScriptThing, b: bool, c: bool) {
//         unsafe {
//             ((*self.vmt).set_house_owned_by_player)(self as *mut CGameScriptInterface, a, b, c)
//         }
//     }

//     pub fn set_buyable_house_as_scripted(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe {
//             ((*self.vmt).set_buyable_house_as_scripted)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn is_chest_open(&mut self, a: *const CScriptThing) -> bool {
//         unsafe { ((*self.vmt).is_chest_open)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn open_chest(&mut self, a: *const CScriptThing, b: bool) -> bool {
//         unsafe { ((*self.vmt).open_chest)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn close_chest(&mut self, a: *const CScriptThing) {
//         unsafe { ((*self.vmt).close_chest)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_number_of_keys_needed_to_unlock_chest(
//         &mut self,
//         a: *const CScriptThing,
//         b: *mut CCharString,
//     ) -> c_long {
//         unsafe {
//             ((*self.vmt).get_number_of_keys_needed_to_unlock_chest)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn display_locked_chest_message(&mut self, a: *const CScriptThing) {
//         unsafe { ((*self.vmt).display_locked_chest_message)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_trophy_as_mountable(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).set_trophy_as_mountable)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn set_village_limbo(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).set_village_limbo)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn set_creature_not_reload(&mut self, a: *const CScriptThing) {
//         unsafe { ((*self.vmt).set_creature_not_reload)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn is_sleeping_time(&mut self, a: *const CScriptThing) -> bool {
//         unsafe { ((*self.vmt).is_sleeping_time)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn enable_guards(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).enable_guards)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn enable_villager_def_types(
//         &mut self,
//         a: *const CScriptThing,
//         b: bool,
//         c: *const CCharString,
//     ) {
//         unsafe {
//             ((*self.vmt).enable_villager_def_types)(self as *mut CGameScriptInterface, a, b, c)
//         }
//     }

//     pub fn try_to_respawn_def_named(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CCharString,
//         c: *const C3DVector,
//     ) -> CScriptThing {
//         unsafe {
//             ((*self.vmt).try_to_respawn_def_named)(self as *mut CGameScriptInterface, a, b, c)
//         }
//     }

//     pub fn clear_hero_enemy_of_guards(&mut self, a: *const CScriptThing) {
//         unsafe { ((*self.vmt).clear_hero_enemy_of_guards)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_thing_as_usable(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).set_thing_as_usable)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn set_thing_home_building(&mut self, a: *const CScriptThing, b: *const CScriptThing) {
//         unsafe { ((*self.vmt).set_thing_home_building)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn set_village_attitude(&mut self, a: *const CScriptThing, b: EScriptVillageAttitude) {
//         unsafe { ((*self.vmt).set_village_attitude)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn add_crime_committed(
//         &mut self,
//         a: *const CScriptThing,
//         b: ECrime,
//         c: bool,
//         d: *const CScriptThing,
//         e: *const CScriptThing,
//         f: EOpinionPostDeedType,
//     ) {
//         unsafe {
//             ((*self.vmt).add_crime_committed)(self as *mut CGameScriptInterface, a, b, c, d, e, f)
//         }
//     }

//     pub fn give_thing_best_enemy_target(&mut self, a: *const CScriptThing, b: *const CScriptThing) {
//         unsafe {
//             ((*self.vmt).give_thing_best_enemy_target)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn clear_thing_best_enemy_target(&mut self, a: *const CScriptThing) {
//         unsafe { ((*self.vmt).clear_thing_best_enemy_target)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn entity_set_in_limbo(&mut self, a: *const CScriptThing, b: bool, c: bool) {
//         unsafe { ((*self.vmt).entity_set_in_limbo)(self as *mut CGameScriptInterface, a, b, c) }
//     }

//     pub fn is_entity_in_limbo(&mut self, a: *const CScriptThing) -> bool {
//         unsafe { ((*self.vmt).is_entity_in_limbo)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn entity_get_shot_strike_pos(
//         &mut self,
//         a: *const CScriptThing,
//         b: *mut C3DVector,
//     ) -> bool {
//         unsafe { ((*self.vmt).entity_get_shot_strike_pos)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_set_negate_all_hits(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).entity_set_negate_all_hits)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_set_evade_all_hits(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).entity_set_evade_all_hits)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_set_able_to_be_engaged_in_combat(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe {
//             ((*self.vmt).entity_set_able_to_be_engaged_in_combat)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn entity_set_always_block_attacks_from_thing(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CScriptThing,
//         c: bool,
//     ) {
//         unsafe {
//             ((*self.vmt).entity_set_always_block_attacks_from_thing)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//                 c,
//             )
//         }
//     }

//     pub fn entity_set_attack_thing_immediately(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CScriptThing,
//         c: bool,
//         d: bool,
//     ) {
//         unsafe {
//             ((*self.vmt).entity_set_attack_thing_immediately)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//                 c,
//                 d,
//             )
//         }
//     }

//     pub fn entity_set_combat_type(&mut self, a: *const CScriptThing, b: *const CCharString) {
//         unsafe { ((*self.vmt).entity_set_combat_type)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_reset_combat_type_to_default(&mut self, a: *const CScriptThing) {
//         unsafe {
//             ((*self.vmt).entity_reset_combat_type_to_default)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn entity_set_max_number_of_attackers(&mut self, a: *const CScriptThing, b: c_long) {
//         unsafe {
//             ((*self.vmt).entity_set_max_number_of_attackers)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn entity_clear_max_number_of_attackers(&mut self, a: *const CScriptThing) {
//         unsafe {
//             ((*self.vmt).entity_clear_max_number_of_attackers)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn entity_attach_to_script(&mut self, a: *const CScriptThing, b: *const CCharString) {
//         unsafe { ((*self.vmt).entity_attach_to_script)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_set_combat_ability(&mut self, a: *const CScriptThing, b: c_float) {
//         unsafe { ((*self.vmt).entity_set_combat_ability)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_set_ranged_target(&mut self, a: *const CScriptThing, b: *const CScriptThing) {
//         unsafe { ((*self.vmt).entity_set_ranged_target)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_clear_ranged_target(&mut self, a: *const CScriptThing) {
//         unsafe { ((*self.vmt).entity_clear_ranged_target)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn entity_set_targetable(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).entity_set_targetable)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_set_targeting_type(&mut self, a: *const CScriptThing, b: ETargetingType) {
//         unsafe { ((*self.vmt).entity_set_targeting_type)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_set_targeting_valid_target_without_los(
//         &mut self,
//         a: *const CScriptThing,
//         b: bool,
//     ) {
//         unsafe {
//             ((*self.vmt).entity_set_targeting_valid_target_without_los)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn entity_teleport_to_thing(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CScriptThing,
//         c: bool,
//     ) {
//         unsafe {
//             ((*self.vmt).entity_teleport_to_thing)(self as *mut CGameScriptInterface, a, b, c)
//         }
//     }

//     pub fn entity_teleport_to_position(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const C3DVector,
//         c: c_float,
//         d: bool,
//         e: bool,
//     ) {
//         unsafe {
//             ((*self.vmt).entity_teleport_to_position)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//                 c,
//                 d,
//                 e,
//             )
//         }
//     }

//     pub fn entity_set_facing_angle(&mut self, a: *const CScriptThing, b: c_float, c: bool) {
//         unsafe { ((*self.vmt).entity_set_facing_angle)(self as *mut CGameScriptInterface, a, b, c) }
//     }

//     pub fn entity_set_facing_angle_towards_thing(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CScriptThing,
//         c: bool,
//     ) {
//         unsafe {
//             ((*self.vmt).entity_set_facing_angle_towards_thing)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//                 c,
//             )
//         }
//     }

//     pub fn entity_set_perception_variables(
//         &mut self,
//         a: *const CScriptThing,
//         b: c_float,
//         c: c_float,
//         d: c_float,
//     ) {
//         unsafe {
//             ((*self.vmt).entity_set_perception_variables)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//                 c,
//                 d,
//             )
//         }
//     }

//     pub fn set_thing_persistent(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).set_thing_persistent)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_set_thing_as_wanting_money(&mut self, a: *const CScriptThing, b: c_long) {
//         unsafe {
//             ((*self.vmt).entity_set_thing_as_wanting_money)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn entity_set_appearance_morph_seed(&mut self, a: *const CScriptThing, b: c_long) {
//         unsafe {
//             ((*self.vmt).entity_set_appearance_morph_seed)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn set_entity_as_region_following(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CScriptThing,
//         c: bool,
//     ) {
//         unsafe {
//             ((*self.vmt).set_entity_as_region_following)(self as *mut CGameScriptInterface, a, b, c)
//         }
//     }

//     pub fn set_entity_as_following_hero_through_teleporters(
//         &mut self,
//         a: *const CScriptThing,
//         b: bool,
//     ) {
//         unsafe {
//             ((*self.vmt).set_entity_as_following_hero_through_teleporters)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn entity_set_appearance_seed(&mut self, a: *const CScriptThing, b: c_ulong) {
//         unsafe { ((*self.vmt).entity_set_appearance_seed)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_get_appearance_seed(&mut self, a: *const CScriptThing, b: *mut c_ulong) {
//         unsafe { ((*self.vmt).entity_get_appearance_seed)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_set_as_for_sale(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).entity_set_as_for_sale)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_set_stock_item_price(&mut self, a: *const CScriptThing, b: c_long) {
//         unsafe {
//             ((*self.vmt).entity_set_stock_item_price)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn entity_get_stock_item_price(&mut self, a: *const CScriptThing) -> c_long {
//         unsafe { ((*self.vmt).entity_get_stock_item_price)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn entity_play_object_animation(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CCharString,
//         c: bool,
//     ) {
//         unsafe {
//             ((*self.vmt).entity_play_object_animation)(self as *mut CGameScriptInterface, a, b, c)
//         }
//     }

//     pub fn entity_set_max_running_speed(&mut self, a: *const CScriptThing, b: c_float) {
//         unsafe {
//             ((*self.vmt).entity_set_max_running_speed)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn entity_set_max_walking_speed(&mut self, a: *const CScriptThing, b: c_float) {
//         unsafe {
//             ((*self.vmt).entity_set_max_walking_speed)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn entity_reset_max_running_speed(&mut self, a: *const CScriptThing) {
//         unsafe {
//             ((*self.vmt).entity_reset_max_running_speed)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn entity_reset_max_walking_speed(&mut self, a: *const CScriptThing) {
//         unsafe {
//             ((*self.vmt).entity_reset_max_walking_speed)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn entity_attach_to_village(&mut self, a: *const CScriptThing, b: *const CScriptThing) {
//         unsafe { ((*self.vmt).entity_attach_to_village)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_set_as_sitting_on_floor(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe {
//             ((*self.vmt).entity_set_as_sitting_on_floor)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn entity_set_as_scared(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).entity_set_as_scared)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_set_as_drunk(&mut self, a: *mut CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).entity_set_as_drunk)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_set_as_having_bound_hands(&mut self, a: *mut CScriptThing, b: bool) {
//         unsafe {
//             ((*self.vmt).entity_set_as_having_bound_hands)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn entity_set_as_remove_all_movement_blocking_modes(&mut self, a: *mut CScriptThing) {
//         unsafe {
//             ((*self.vmt).entity_set_as_remove_all_movement_blocking_modes)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn entity_force_to_look_at_thing(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CScriptThing,
//     ) {
//         unsafe {
//             ((*self.vmt).entity_force_to_look_at_thing)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn entity_force_to_look_at_camera(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe {
//             ((*self.vmt).entity_force_to_look_at_camera)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn entity_force_to_look_at_nothing(&mut self, a: *const CScriptThing) {
//         unsafe {
//             ((*self.vmt).entity_force_to_look_at_nothing)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn entity_reset_force_to_look_at(&mut self, a: *const CScriptThing) {
//         unsafe { ((*self.vmt).entity_reset_force_to_look_at)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn entity_set_shot_accuracy_percentage(&mut self, a: *const CScriptThing, b: c_long) {
//         unsafe {
//             ((*self.vmt).entity_set_shot_accuracy_percentage)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn entity_get_standing_on_thing(&mut self, a: *const CScriptThing) -> CScriptThing {
//         unsafe { ((*self.vmt).entity_get_standing_on_thing)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn entity_get_standing_inside_building(&mut self, a: *const CScriptThing) -> CScriptThing {
//         unsafe {
//             ((*self.vmt).entity_get_standing_inside_building)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn entity_drop_generic_box(&mut self, a: *const CScriptThing) {
//         unsafe { ((*self.vmt).entity_drop_generic_box)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn entity_sheathe_weapons(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).entity_sheathe_weapons)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_unsheathe_weapons(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).entity_unsheathe_weapons)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_unsheathe_melee_weapon(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe {
//             ((*self.vmt).entity_unsheathe_melee_weapon)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn entity_unsheathe_ranged_weapon(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe {
//             ((*self.vmt).entity_unsheathe_ranged_weapon)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn entity_set_alpha(&mut self, a: *const CScriptThing, b: c_float, c: bool) {
//         unsafe { ((*self.vmt).entity_set_alpha)(self as *mut CGameScriptInterface, a, b, c) }
//     }

//     pub fn entity_set_as_drawable(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).entity_set_as_drawable)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_set_cutscene_behaviour(&mut self, a: *const CScriptThing, b: ECutsceneBehaviour) {
//         unsafe {
//             ((*self.vmt).entity_set_cutscene_behaviour)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn entity_get_sex(&mut self, a: *const CScriptThing) -> ESex {
//         unsafe { ((*self.vmt).entity_get_sex)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn entity_set_as_able_to_walk_through_solid_objects(
//         &mut self,
//         a: *const CScriptThing,
//         b: bool,
//     ) {
//         unsafe {
//             ((*self.vmt).entity_set_as_able_to_walk_through_solid_objects)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn entity_set_as_respond_to_hit(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe {
//             ((*self.vmt).entity_set_as_respond_to_hit)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn entity_set_as_damageable(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).entity_set_as_damageable)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_set_as_killable(&mut self, a: *const CScriptThing, b: bool, c: bool) {
//         unsafe { ((*self.vmt).entity_set_as_killable)(self as *mut CGameScriptInterface, a, b, c) }
//     }

//     pub fn entity_set_as_locked(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).entity_set_as_locked)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_decapitate(&mut self, a: *const CScriptThing) {
//         unsafe { ((*self.vmt).entity_decapitate)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn entity_give_gold(&mut self, a: *const CScriptThing, b: c_long) {
//         unsafe { ((*self.vmt).entity_give_gold)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_set_allow_boss_phase_changes(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe {
//             ((*self.vmt).entity_set_allow_boss_phase_changes)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn entity_get_boss_phase(&mut self, a: *const CScriptThing) -> c_long {
//         unsafe { ((*self.vmt).entity_get_boss_phase)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn entity_set_boss_phase(&mut self, a: *const CScriptThing, b: c_long) {
//         unsafe { ((*self.vmt).entity_set_boss_phase)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_reset_creature_mode(&mut self, a: *const CScriptThing) {
//         unsafe { ((*self.vmt).entity_reset_creature_mode)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn entity_set_as_receiving_events(&mut self, a: *const CScriptThing, b: bool) -> bool {
//         unsafe {
//             ((*self.vmt).entity_set_as_receiving_events)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn entity_set_as_to_add_to_combo_multiplier_when_hit(
//         &mut self,
//         a: *const CScriptThing,
//         b: bool,
//     ) {
//         unsafe {
//             ((*self.vmt).entity_set_as_to_add_to_combo_multiplier_when_hit)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn entity_set_as_to_add_to_stat_changes_when_hit(
//         &mut self,
//         a: *const CScriptThing,
//         b: bool,
//     ) {
//         unsafe {
//             ((*self.vmt).entity_set_as_to_add_to_stat_changes_when_hit)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn entity_leave_combat_stance(&mut self, a: *const CScriptThing) {
//         unsafe { ((*self.vmt).entity_leave_combat_stance)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn entity_set_as_use_movement_in_actions(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe {
//             ((*self.vmt).entity_set_as_use_movement_in_actions)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn entity_set_as_displaying_emote_icon(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe {
//             ((*self.vmt).entity_set_as_displaying_emote_icon)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn entity_set_as_collidable_to_things(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe {
//             ((*self.vmt).entity_set_as_collidable_to_things)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn entity_enable_gravity(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).entity_enable_gravity)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_set_light_as_on(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).entity_set_light_as_on)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_fade_out(&mut self, a: *const CScriptThing, b: c_float) {
//         unsafe { ((*self.vmt).entity_fade_out)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_fade_in(&mut self, a: *const CScriptThing, b: c_float) {
//         unsafe { ((*self.vmt).entity_fade_in)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_begin_loading_animation(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CCharString,
//     ) {
//         unsafe {
//             ((*self.vmt).entity_begin_loading_animation)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn entity_begin_loading_basic_animations(&mut self, a: *const CScriptThing) {
//         unsafe {
//             ((*self.vmt).entity_begin_loading_basic_animations)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn entity_cast_force_push(&mut self, a: *const CScriptThing, b: bool) -> bool {
//         unsafe { ((*self.vmt).entity_cast_force_push)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_cast_lightning_at_target(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CScriptThing,
//     ) {
//         unsafe {
//             ((*self.vmt).entity_cast_lightning_at_target)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn begin_loading_mesh(&mut self, a: *const CCharString) {
//         unsafe { ((*self.vmt).begin_loading_mesh)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn entity_will_teleport_to_area(
//         &mut self,
//         a: *const CScriptThing,
//         b: C3DVector,
//         c: c_float,
//         d: c_float,
//     ) -> bool {
//         unsafe {
//             ((*self.vmt).entity_will_teleport_to_area)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//                 c,
//                 d,
//             )
//         }
//     }

//     pub fn entity_start_screamer_super_attack_thing(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CScriptThing,
//     ) {
//         unsafe {
//             ((*self.vmt).entity_start_screamer_super_attack_thing)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn entity_end_screamer_super_attack_thing(&mut self, a: *const CScriptThing) {
//         unsafe {
//             ((*self.vmt).entity_end_screamer_super_attack_thing)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn set_hero_guide_to_show_quest_cards_when_spoken_to(&mut self, a: bool) {
//         unsafe {
//             ((*self.vmt).set_hero_guide_to_show_quest_cards_when_spoken_to)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn set_light_colour(&mut self, a: *const CScriptThing, b: *const CRGBColour) {
//         unsafe { ((*self.vmt).set_light_colour)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn creature_generator_set_family(&mut self, a: *const CScriptThing, b: *const CCharString) {
//         unsafe {
//             ((*self.vmt).creature_generator_set_family)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn creature_generator_trigger(&mut self, a: *const CScriptThing) {
//         unsafe { ((*self.vmt).creature_generator_trigger)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn creature_generator_set_always_create_creatures_on_trigger(
//         &mut self,
//         a: *const CScriptThing,
//         b: bool,
//     ) {
//         unsafe {
//             ((*self.vmt).creature_generator_set_always_create_creatures_on_trigger)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn creature_generator_is_depleted(&mut self, a: *const CScriptThing) -> bool {
//         unsafe {
//             ((*self.vmt).creature_generator_is_depleted)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn creature_generator_is_destroyed(&mut self, a: *const CScriptThing) -> bool {
//         unsafe {
//             ((*self.vmt).creature_generator_is_destroyed)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn creature_generator_set_generated_creature_script_name(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CCharString,
//     ) {
//         unsafe {
//             ((*self.vmt).creature_generator_set_generated_creature_script_name)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn creature_generator_set_num_triggers(&mut self, a: *const CScriptThing, b: c_long) {
//         unsafe {
//             ((*self.vmt).creature_generator_set_num_triggers)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn creature_generator_get_num_generated_creatures(
//         &mut self,
//         a: *const CScriptThing,
//     ) -> c_long {
//         unsafe {
//             ((*self.vmt).creature_generator_get_num_generated_creatures)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn creature_generator_are_all_creatures_alive(&mut self, a: *const CScriptThing) -> bool {
//         unsafe {
//             ((*self.vmt).creature_generator_are_all_creatures_alive)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn creature_generator_add_triggerer(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CScriptThing,
//     ) {
//         unsafe {
//             ((*self.vmt).creature_generator_add_triggerer)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn creature_generator_remove_triggerer(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CScriptThing,
//     ) {
//         unsafe {
//             ((*self.vmt).creature_generator_remove_triggerer)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn set_creature_generator_enabled(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe {
//             ((*self.vmt).set_creature_generator_enabled)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn set_creature_generators_enabled(&mut self, a: *const CCharString, b: bool) {
//         unsafe {
//             ((*self.vmt).set_creature_generators_enabled)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn set_creature_generators_enabled_during_script(
//         &mut self,
//         a: *const CCharString,
//         b: bool,
//     ) {
//         unsafe {
//             ((*self.vmt).set_creature_generators_enabled_during_script)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn set_creature_generators_creature_group_as_enabled(
//         &mut self,
//         a: ECreatureGroup,
//         b: bool,
//     ) {
//         unsafe {
//             ((*self.vmt).set_creature_generators_creature_group_as_enabled)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn is_creature_generation_enabled_for_region(&mut self, a: *const CCharString) -> bool {
//         unsafe {
//             ((*self.vmt).is_creature_generation_enabled_for_region)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn is_creature_flying(&mut self, a: *const CScriptThing) -> bool {
//         unsafe { ((*self.vmt).is_creature_flying)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_teleporter_as_active(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).set_teleporter_as_active)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn is_teleporter_active(&mut self, a: *const CScriptThing) -> bool {
//         unsafe { ((*self.vmt).is_teleporter_active)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_teleporting_as_active(&mut self, a: bool) {
//         unsafe { ((*self.vmt).set_teleporting_as_active)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn is_teleporting_active(&mut self) -> bool {
//         unsafe { ((*self.vmt).is_teleporting_active)(self as *mut CGameScriptInterface) }
//     }

//     pub fn set_region_exit_as_active(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).set_region_exit_as_active)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn set_region_entrance_as_active(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe {
//             ((*self.vmt).set_region_entrance_as_active)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn set_region_text_display_as_active(&mut self, a: bool) {
//         unsafe {
//             ((*self.vmt).set_region_text_display_as_active)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn set_hero_sleeping_as_enabled(&mut self, a: bool) {
//         unsafe { ((*self.vmt).set_hero_sleeping_as_enabled)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn is_hero_sleeping_enabled(&mut self) -> bool {
//         unsafe { ((*self.vmt).is_hero_sleeping_enabled)(self as *mut CGameScriptInterface) }
//     }

//     pub fn set_experience_spending_as_enabled(&mut self, a: bool) {
//         unsafe {
//             ((*self.vmt).set_experience_spending_as_enabled)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn set_morality_changing_as_enabled(&mut self, a: bool) {
//         unsafe {
//             ((*self.vmt).set_morality_changing_as_enabled)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn set_summoner_death_explosion_affects_hero(&mut self, a: bool) {
//         unsafe {
//             ((*self.vmt).set_summoner_death_explosion_affects_hero)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn get_nearest_enabled_digging_spot(&mut self, a: *const CScriptThing) -> CScriptThing {
//         unsafe {
//             ((*self.vmt).get_nearest_enabled_digging_spot)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn is_digging_spot_enabled(&mut self, a: *const CScriptThing) -> bool {
//         unsafe { ((*self.vmt).is_digging_spot_enabled)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn is_digging_spot_hidden(&mut self, a: *const CScriptThing) -> bool {
//         unsafe { ((*self.vmt).is_digging_spot_hidden)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_digging_spot_as_hidden(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).set_digging_spot_as_hidden)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn check_for_camera_message(&mut self, a: *const CCharString) -> bool {
//         unsafe { ((*self.vmt).check_for_camera_message)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn wait_for_camera_message(&mut self, a: *const CCharString) {
//         unsafe { ((*self.vmt).wait_for_camera_message)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_thing_as_conscious(
//         &mut self,
//         a: *const CScriptThing,
//         b: bool,
//         c: *const CCharString,
//     ) {
//         unsafe { ((*self.vmt).set_thing_as_conscious)(self as *mut CGameScriptInterface, a, b, c) }
//     }

//     pub fn set_fire_to_thing(&mut self, a: *const CScriptThing) {
//         unsafe { ((*self.vmt).set_fire_to_thing)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn extinguish_fires_on_thing(&mut self, a: *const CScriptThing) {
//         unsafe { ((*self.vmt).extinguish_fires_on_thing)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn is_thing_on_fire(&mut self, a: *const CScriptThing) -> bool {
//         unsafe { ((*self.vmt).is_thing_on_fire)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn add_item_to_container(&mut self, a: *const CScriptThing, b: *const CCharString) {
//         unsafe { ((*self.vmt).add_item_to_container)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn remove_item_from_container(&mut self, a: *const CScriptThing, b: *const CCharString) {
//         unsafe { ((*self.vmt).remove_item_from_container)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_set_death_container_as_enabled(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe {
//             ((*self.vmt).entity_set_death_container_as_enabled)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn get_item_def_names_from_container(
//         &mut self,
//         a: *const CScriptThing,
//         b: *mut CxxVector<CCharString>,
//     ) {
//         unsafe {
//             ((*self.vmt).get_item_def_names_from_container)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn set_creature_brain(&mut self, a: *const CScriptThing, b: *const CCharString) {
//         unsafe { ((*self.vmt).set_creature_brain)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_set_stategroup_enabled(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CCharString,
//         c: bool,
//     ) {
//         unsafe {
//             ((*self.vmt).entity_set_stategroup_enabled)(self as *mut CGameScriptInterface, a, b, c)
//         }
//     }

//     pub fn entity_set_all_stategroups_enabled(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe {
//             ((*self.vmt).entity_set_all_stategroups_enabled)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn entity_set_combat_enabled(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).entity_set_combat_enabled)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_set_sleep_enabled(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).entity_set_sleep_enabled)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_set_opinion_reactions_enabled(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe {
//             ((*self.vmt).entity_set_opinion_reactions_enabled)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn entity_set_deed_reactions_enabled(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe {
//             ((*self.vmt).entity_set_deed_reactions_enabled)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn debug_get_all_text_entries_for_targeted_thing(&mut self, a: *mut CxxSet<c_ulong>) {
//         unsafe {
//             ((*self.vmt).debug_get_all_text_entries_for_targeted_thing)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn entity_set_thing_as_enemy_of_thing(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CScriptThing,
//     ) {
//         unsafe {
//             ((*self.vmt).entity_set_thing_as_enemy_of_thing)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn entity_unset_thing_as_enemy_of_thing(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CScriptThing,
//     ) {
//         unsafe {
//             ((*self.vmt).entity_unset_thing_as_enemy_of_thing)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn entity_set_thing_as_ally_of_thing(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CScriptThing,
//     ) {
//         unsafe {
//             ((*self.vmt).entity_set_thing_as_ally_of_thing)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn entity_unset_thing_as_ally_of_thing(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CScriptThing,
//     ) {
//         unsafe {
//             ((*self.vmt).entity_unset_thing_as_ally_of_thing)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn entity_set_in_faction(&mut self, a: *const CScriptThing, b: *const CCharString) {
//         unsafe { ((*self.vmt).entity_set_in_faction)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn set_faction_as_allied_to_faction(
//         &mut self,
//         a: *const CCharString,
//         b: *const CCharString,
//     ) {
//         unsafe {
//             ((*self.vmt).set_faction_as_allied_to_faction)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn set_faction_as_neutral_to_faction(
//         &mut self,
//         a: *const CCharString,
//         b: *const CCharString,
//     ) {
//         unsafe {
//             ((*self.vmt).set_faction_as_neutral_to_faction)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn set_faction_as_enemy_to_faction(
//         &mut self,
//         a: *const CCharString,
//         b: *const CCharString,
//     ) {
//         unsafe {
//             ((*self.vmt).set_faction_as_enemy_to_faction)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn are_entities_enemies(&mut self, a: *const CScriptThing, b: *const CScriptThing) -> bool {
//         unsafe { ((*self.vmt).are_entities_enemies)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn get_next_in_opinion_attitude_graph(
//         &mut self,
//         a: EOpinionAttitudeType,
//     ) -> EOpinionAttitudeType {
//         unsafe {
//             ((*self.vmt).get_next_in_opinion_attitude_graph)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn get_opinion_attitude_as_string(&mut self, a: EOpinionAttitudeType, b: *mut CCharString) {
//         unsafe {
//             ((*self.vmt).get_opinion_attitude_as_string)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn entity_get_opinion_attitude_to_player(
//         &mut self,
//         a: *const CScriptThing,
//     ) -> EOpinionAttitudeType {
//         unsafe {
//             ((*self.vmt).entity_get_opinion_attitude_to_player)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn entity_get_opinion_attitude_to_player_as_string(
//         &mut self,
//         a: *const CScriptThing,
//         b: *mut CCharString,
//     ) {
//         unsafe {
//             ((*self.vmt).entity_get_opinion_attitude_to_player_as_string)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn entity_get_opinion_of_player(&mut self, a: *const CScriptThing, b: EOpinion) -> c_float {
//         unsafe {
//             ((*self.vmt).entity_get_opinion_of_player)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn entity_set_opinion_reaction_mask(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CCharString,
//     ) {
//         unsafe {
//             ((*self.vmt).entity_set_opinion_reaction_mask)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn entity_set_opinion_reaction_mask_2(&mut self, a: *const CScriptThing, b: c_long) {
//         unsafe {
//             ((*self.vmt).entity_set_opinion_reaction_mask_2)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn entity_set_opinion_deed_mask(&mut self, a: *const CScriptThing, b: *const CCharString) {
//         unsafe {
//             ((*self.vmt).entity_set_opinion_deed_mask)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn entity_set_opinion_deed_mask_2(&mut self, a: *const CScriptThing, b: c_long) {
//         unsafe {
//             ((*self.vmt).entity_set_opinion_deed_mask_2)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn entity_set_opinion_deed_type_enabled(
//         &mut self,
//         a: *const CScriptThing,
//         b: EOpinionDeedType,
//         c: bool,
//     ) {
//         unsafe {
//             ((*self.vmt).entity_set_opinion_deed_type_enabled)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//                 c,
//             )
//         }
//     }

//     pub fn entity_set_opinion_attitude_enabled(
//         &mut self,
//         a: *const CScriptThing,
//         b: EOpinionAttitudeType,
//         c: bool,
//     ) {
//         unsafe {
//             ((*self.vmt).entity_set_opinion_attitude_enabled)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//                 c,
//             )
//         }
//     }

//     pub fn entity_set_opinion_reaction_enabled(
//         &mut self,
//         a: *const CScriptThing,
//         b: EOpinionReactionType,
//         c: bool,
//     ) {
//         unsafe {
//             ((*self.vmt).entity_set_opinion_reaction_enabled)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//                 c,
//             )
//         }
//     }

//     pub fn entity_set_personality_override(&mut self, a: *const CScriptThing, b: c_long) {
//         unsafe {
//             ((*self.vmt).entity_set_personality_override)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn entity_set_personality_override_2(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CCharString,
//     ) {
//         unsafe {
//             ((*self.vmt).entity_set_personality_override_2)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn entity_clear_personality_override(&mut self, a: *const CScriptThing) {
//         unsafe {
//             ((*self.vmt).entity_clear_personality_override)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn entity_set_as_opinion_source(&mut self, a: *const CScriptThing, b: c_long) {
//         unsafe {
//             ((*self.vmt).entity_set_as_opinion_source)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn entity_set_as_opinion_source_2(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CCharString,
//     ) {
//         unsafe {
//             ((*self.vmt).entity_set_as_opinion_source_2)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn entity_unset_as_opinion_source(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe {
//             ((*self.vmt).entity_unset_as_opinion_source)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn opinion_source_set_as_exclusive(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe {
//             ((*self.vmt).opinion_source_set_as_exclusive)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn opinion_source_set_as_attention_grabbing(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe {
//             ((*self.vmt).opinion_source_set_as_attention_grabbing)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn entity_post_opinion_deed_to_all(&mut self, a: *const CScriptThing, b: EOpinionDeedType) {
//         unsafe {
//             ((*self.vmt).entity_post_opinion_deed_to_all)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn entity_post_opinion_deed_to_recipient(
//         &mut self,
//         a: *const CScriptThing,
//         b: EOpinionDeedType,
//         c: *const CScriptThing,
//     ) {
//         unsafe {
//             ((*self.vmt).entity_post_opinion_deed_to_recipient)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//                 c,
//             )
//         }
//     }

//     pub fn entity_post_opinion_deed_to_recipient_village(
//         &mut self,
//         a: *const CScriptThing,
//         b: EOpinionDeedType,
//         c: *const CScriptThing,
//     ) {
//         unsafe {
//             ((*self.vmt).entity_post_opinion_deed_to_recipient_village)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//                 c,
//             )
//         }
//     }

//     pub fn entity_post_opinion_deed_keep_searching_for_witnesses(
//         &mut self,
//         a: *const CScriptThing,
//         b: EOpinionDeedType,
//         c: *const CScriptThing,
//     ) -> c_long {
//         unsafe {
//             ((*self.vmt).entity_post_opinion_deed_keep_searching_for_witnesses)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//                 c,
//             )
//         }
//     }

//     pub fn remove_opinion_deed_still_searching_for_witnesses(
//         &mut self,
//         a: *const CScriptThing,
//         b: c_long,
//     ) {
//         unsafe {
//             ((*self.vmt).remove_opinion_deed_still_searching_for_witnesses)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn is_deed_witnessed(&mut self, a: c_long) -> bool {
//         unsafe { ((*self.vmt).is_deed_witnessed)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn can_thing_be_seen_by_other_thing(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CScriptThing,
//     ) -> bool {
//         unsafe {
//             ((*self.vmt).can_thing_be_seen_by_other_thing)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn can_thing_be_nearly_seen_by_other_thing(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CScriptThing,
//     ) -> bool {
//         unsafe {
//             ((*self.vmt).can_thing_be_nearly_seen_by_other_thing)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn can_thing_be_smelled_by_other_thing(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CScriptThing,
//     ) -> bool {
//         unsafe {
//             ((*self.vmt).can_thing_be_smelled_by_other_thing)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn can_thing_be_heard_by_other_thing(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CScriptThing,
//     ) -> bool {
//         unsafe {
//             ((*self.vmt).can_thing_be_heard_by_other_thing)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn is_thing_aware_of_other_thing_in_any_way(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CScriptThing,
//     ) -> bool {
//         unsafe {
//             ((*self.vmt).is_thing_aware_of_other_thing_in_any_way)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn entity_set_as_aware_of_thing(&mut self, a: *const CScriptThing, b: *const CScriptThing) {
//         unsafe {
//             ((*self.vmt).entity_set_as_aware_of_thing)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn entity_set_sound_radius(&mut self, a: *const CScriptThing, b: c_float) {
//         unsafe { ((*self.vmt).entity_set_sound_radius)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_set_smell_radius(&mut self, a: *const CScriptThing, b: c_float) {
//         unsafe { ((*self.vmt).entity_set_smell_radius)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_set_sight_radius(&mut self, a: *const CScriptThing, b: c_float) {
//         unsafe { ((*self.vmt).entity_set_sight_radius)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn entity_set_extended_sight_radius(&mut self, a: *const CScriptThing, b: c_float) {
//         unsafe {
//             ((*self.vmt).entity_set_extended_sight_radius)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn entity_set_give_up_chase_radius(&mut self, a: *const CScriptThing, b: c_float) {
//         unsafe {
//             ((*self.vmt).entity_set_give_up_chase_radius)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn entity_get_hearing_radius(&mut self, a: *const CScriptThing) -> c_float {
//         unsafe { ((*self.vmt).entity_get_hearing_radius)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn manually_trigger_trap(&mut self, a: *const CScriptThing) -> bool {
//         unsafe { ((*self.vmt).manually_trigger_trap)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn manually_reset_trap(&mut self, a: *const CScriptThing) -> bool {
//         unsafe { ((*self.vmt).manually_reset_trap)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_time_of_day(&mut self, a: c_float) {
//         unsafe { ((*self.vmt).set_time_of_day)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_time_of_day(&mut self) -> c_long {
//         unsafe { ((*self.vmt).get_time_of_day)(self as *mut CGameScriptInterface) }
//     }

//     pub fn set_time_as_stopped(&mut self, a: bool, b: *mut c_long) {
//         unsafe { ((*self.vmt).set_time_as_stopped)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn fast_forward_time_to(&mut self, a: c_float, b: c_float) {
//         unsafe { ((*self.vmt).fast_forward_time_to)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn is_time_of_day_between(&mut self, a: c_long, b: c_long) -> bool {
//         unsafe { ((*self.vmt).is_time_of_day_between)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn get_day_of_week(&mut self) -> c_long {
//         unsafe { ((*self.vmt).get_day_of_week)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_day_count(&mut self) -> c_long {
//         unsafe { ((*self.vmt).get_day_count)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_world_frame(&mut self) -> c_long {
//         unsafe { ((*self.vmt).get_world_frame)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_constant_fps(&mut self) -> c_long {
//         unsafe { ((*self.vmt).get_constant_fps)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_active_quest_name(&mut self) -> CCharString {
//         unsafe { ((*self.vmt).get_active_quest_name)(self as *mut CGameScriptInterface) }
//     }

//     pub fn transition_to_theme(&mut self, a: *const CCharString, b: c_float) {
//         unsafe { ((*self.vmt).transition_to_theme)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn reset_to_default_theme(&mut self, a: c_float) {
//         unsafe { ((*self.vmt).reset_to_default_theme)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn transition_to_theme_all_internals(&mut self, a: *const CCharString, b: c_float) {
//         unsafe {
//             ((*self.vmt).transition_to_theme_all_internals)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn reset_to_default_theme_all_internals(&mut self, a: c_float) {
//         unsafe {
//             ((*self.vmt).reset_to_default_theme_all_internals)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn transition_to_theme_externals(&mut self, a: *const CCharString, b: c_float) {
//         unsafe {
//             ((*self.vmt).transition_to_theme_externals)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn reset_to_default_theme_externals(&mut self, a: c_float) {
//         unsafe {
//             ((*self.vmt).reset_to_default_theme_externals)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn set_environment_theme_weight_all_channels(&mut self, a: *const CCharString, b: c_float) {
//         unsafe {
//             ((*self.vmt).set_environment_theme_weight_all_channels)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn set_environment_theme_weight_all_internals(
//         &mut self,
//         a: *const CCharString,
//         b: c_float,
//     ) {
//         unsafe {
//             ((*self.vmt).set_environment_theme_weight_all_internals)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn set_environment_theme_weight_externals(&mut self, a: *const CCharString, b: c_float) {
//         unsafe {
//             ((*self.vmt).set_environment_theme_weight_externals)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn set_sound_themes_as_enabled_for_region(&mut self, a: *const CCharString, b: bool) {
//         unsafe {
//             ((*self.vmt).set_sound_themes_as_enabled_for_region)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn set_all_sounds_as_muted(&mut self, a: bool) {
//         unsafe { ((*self.vmt).set_all_sounds_as_muted)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn radial_blur_fade_to(
//         &mut self,
//         a: c_float,
//         b: c_float,
//         c: c_float,
//         d: c_float,
//         e: c_float,
//         f: c_float,
//         g: c_float,
//     ) -> *mut c_void {
//         unsafe {
//             ((*self.vmt).radial_blur_fade_to)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//                 c,
//                 d,
//                 e,
//                 f,
//                 g,
//             )
//         }
//     }

//     pub fn radial_blur_fade_to_2(
//         &mut self,
//         a: c_float,
//         b: C3DVector,
//         c: c_float,
//         d: C3DVector,
//         e: c_float,
//         f: c_float,
//         g: c_float,
//     ) -> *mut c_void {
//         unsafe {
//             ((*self.vmt).radial_blur_fade_to_2)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//                 c,
//                 d,
//                 e,
//                 f,
//                 g,
//             )
//         }
//     }

//     pub fn radial_blur_fade_out(&mut self, a: c_float, b: *mut c_void) {
//         unsafe { ((*self.vmt).radial_blur_fade_out)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn is_radial_blur_fade_active(&mut self) -> bool {
//         unsafe { ((*self.vmt).is_radial_blur_fade_active)(self as *mut CGameScriptInterface) }
//     }

//     pub fn cancel_radial_blur_fade(&mut self) {
//         unsafe { ((*self.vmt).cancel_radial_blur_fade)(self as *mut CGameScriptInterface) }
//     }

//     pub fn radial_blur_set_center_world_pos(&mut self, a: *mut c_void, b: *const C3DVector) {
//         unsafe {
//             ((*self.vmt).radial_blur_set_center_world_pos)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn displacement_monochrome_effect_colour_fade_to(
//         &mut self,
//         a: c_float,
//         b: *const CRGBFloatColour,
//     ) -> c_void {
//         unsafe {
//             ((*self.vmt).displacement_monochrome_effect_colour_fade_to)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn displacement_monochrome_effect_colour_fade_out(&mut self, a: c_float, b: *mut c_void) {
//         unsafe {
//             ((*self.vmt).displacement_monochrome_effect_colour_fade_out)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn screen_filter_fade_to(
//         &mut self,
//         a: c_float,
//         b: c_float,
//         c: c_float,
//         d: c_float,
//         e: c_float,
//         f: *const CRGBFloatColour,
//         g: *const CxxVector<CScreenFilterSThingByPass>,
//     ) -> c_void {
//         unsafe {
//             ((*self.vmt).screen_filter_fade_to)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//                 c,
//                 d,
//                 e,
//                 f,
//                 g,
//             )
//         }
//     }

//     pub fn screen_filter_fade_out(&mut self, a: c_float, b: *mut c_void) {
//         unsafe { ((*self.vmt).screen_filter_fade_out)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn set_thing_and_carried_items_not_affected_by_screen_filter(
//         &mut self,
//         a: *mut CScriptThing,
//         b: *mut c_void,
//     ) {
//         unsafe {
//             ((*self.vmt).set_thing_and_carried_items_not_affected_by_screen_filter)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn un_set_thing_and_carried_items_not_affected_by_screen_filter(
//         &mut self,
//         a: *mut CScriptThing,
//     ) {
//         unsafe {
//             ((*self.vmt).un_set_thing_and_carried_items_not_affected_by_screen_filter)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn is_gift_romantic(&mut self, a: *const CCharString) -> bool {
//         unsafe { ((*self.vmt).is_gift_romantic)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn is_gift_friendly(&mut self, a: *const CCharString) -> bool {
//         unsafe { ((*self.vmt).is_gift_friendly)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn is_gift_offensive(&mut self, a: *const CCharString) -> bool {
//         unsafe { ((*self.vmt).is_gift_offensive)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn is_thing_a_bed(&mut self, a: *const CScriptThing) -> bool {
//         unsafe { ((*self.vmt).is_thing_a_bed)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn is_thing_a_chest(&mut self, a: *const CScriptThing) -> bool {
//         unsafe { ((*self.vmt).is_thing_a_chest)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn is_thing_a_door(&mut self, a: *const CScriptThing) -> bool {
//         unsafe { ((*self.vmt).is_thing_a_door)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn is_thing_smashable(&mut self, a: *const CScriptThing) -> bool {
//         unsafe { ((*self.vmt).is_thing_smashable)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn is_thing_searchable(&mut self, a: *const CScriptThing) -> bool {
//         unsafe { ((*self.vmt).is_thing_searchable)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn apply_script_brush(&mut self, a: *const CCharString) {
//         unsafe { ((*self.vmt).apply_script_brush)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn enable_decals(&mut self, a: bool) {
//         unsafe { ((*self.vmt).enable_decals)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn play_criteria_sound_on_thing(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CCharString,
//     ) -> c_ulong {
//         unsafe {
//             ((*self.vmt).play_criteria_sound_on_thing)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn play_sound_on_thing(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CCharString,
//     ) -> c_ulong {
//         unsafe { ((*self.vmt).play_sound_on_thing)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn is_sound_playing(&mut self, a: c_ulong) -> bool {
//         unsafe { ((*self.vmt).is_sound_playing)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn stop_sound(&mut self, a: c_ulong) {
//         unsafe { ((*self.vmt).stop_sound)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn play_sound_at_pos(&mut self, a: *const C3DVector, b: *const CCharString) -> c_ulong {
//         unsafe { ((*self.vmt).play_sound_at_pos)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn play_2d_sound(&mut self, a: *const CCharString) -> c_ulong {
//         unsafe { ((*self.vmt).play_2d_sound)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn enable_sounds(&mut self, a: bool) {
//         unsafe { ((*self.vmt).enable_sounds)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn override_music(&mut self, a: EMusicSetType, b: bool, c: bool) {
//         unsafe { ((*self.vmt).override_music)(self as *mut CGameScriptInterface, a, b, c) }
//     }

//     pub fn stop_override_music(&mut self, a: bool) {
//         unsafe { ((*self.vmt).stop_override_music)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn cache_music_set(&mut self, a: EMusicSetType) {
//         unsafe { ((*self.vmt).cache_music_set)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn enable_danger_music(&mut self, a: bool) {
//         unsafe { ((*self.vmt).enable_danger_music)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn is_danger_music_enabled(&mut self) -> bool {
//         unsafe { ((*self.vmt).is_danger_music_enabled)(self as *mut CGameScriptInterface) }
//     }

//     pub fn start_countdown_timer(&mut self, a: c_float) {
//         unsafe { ((*self.vmt).start_countdown_timer)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_countdown_timer(&mut self) -> c_float {
//         unsafe { ((*self.vmt).get_countdown_timer)(self as *mut CGameScriptInterface) }
//     }

//     pub fn auto_save_check_point(&mut self) {
//         unsafe { ((*self.vmt).auto_save_check_point)(self as *mut CGameScriptInterface) }
//     }

//     pub fn auto_save_quest_start(&mut self) {
//         unsafe { ((*self.vmt).auto_save_quest_start)(self as *mut CGameScriptInterface) }
//     }

//     pub fn auto_save(&mut self) {
//         unsafe { ((*self.vmt).auto_save)(self as *mut CGameScriptInterface) }
//     }

//     pub fn set_saving_as_enabled(&mut self, a: bool) {
//         unsafe { ((*self.vmt).set_saving_as_enabled)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn is_saving_enabled(&mut self) -> bool {
//         unsafe { ((*self.vmt).is_saving_enabled)(self as *mut CGameScriptInterface) }
//     }

//     pub fn set_save_game_marker_pos(&mut self, a: *const C3DVector) {
//         unsafe { ((*self.vmt).set_save_game_marker_pos)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn reset_to_front_end(&mut self) {
//         unsafe { ((*self.vmt).reset_to_front_end)(self as *mut CGameScriptInterface) }
//     }

//     pub fn set_guild_seal_recall_location(&mut self, a: *const C3DVector, b: c_float) {
//         unsafe {
//             ((*self.vmt).set_guild_seal_recall_location)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn get_guild_seal_recall_pos(&mut self) -> C3DVector {
//         unsafe { ((*self.vmt).get_guild_seal_recall_pos)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_guild_seal_recall_angle_xy(&mut self) -> c_float {
//         unsafe { ((*self.vmt).get_guild_seal_recall_angle_xy)(self as *mut CGameScriptInterface) }
//     }

//     pub fn set_readable_object_text(&mut self, a: *const CScriptThing, b: *const CWideString) {
//         unsafe { ((*self.vmt).set_readable_object_text)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn set_readable_object_text_tag(&mut self, a: *const CScriptThing, b: *const CCharString) {
//         unsafe {
//             ((*self.vmt).set_readable_object_text_tag)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn get_formatted_string(
//         &mut self,
//         a: *const CCharString,
//         b: *const CxxVector<CWideString>,
//     ) -> CWideString {
//         unsafe { ((*self.vmt).get_formatted_string)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn get_text_string(&mut self, a: *const CCharString) -> CWideString {
//         unsafe { ((*self.vmt).get_text_string)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn add_rumour_category(&mut self, a: *const CCharString) {
//         unsafe { ((*self.vmt).add_rumour_category)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn add_new_rumour_to_category(&mut self, a: *const CCharString, b: *const CCharString) {
//         unsafe { ((*self.vmt).add_new_rumour_to_category)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn remove_rumour_category(&mut self, a: *const CCharString) {
//         unsafe { ((*self.vmt).remove_rumour_category)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_category_activity(&mut self, a: *const CCharString, b: bool) {
//         unsafe { ((*self.vmt).set_category_activity)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn add_gossip_village(&mut self, a: *const CCharString, b: *const CCharString) {
//         unsafe { ((*self.vmt).add_gossip_village)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn add_gossip_faction_to_category(&mut self, a: *const CCharString, b: *const CCharString) {
//         unsafe {
//             ((*self.vmt).add_gossip_faction_to_category)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn set_is_gossip_for_player(&mut self, a: CCharString, b: bool) {
//         unsafe { ((*self.vmt).set_is_gossip_for_player)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn set_is_gossip_for_player_2(&mut self, a: *const CCharString, b: bool) {
//         unsafe { ((*self.vmt).set_is_gossip_for_player_2)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn update_online_score_archery(&mut self, a: c_float) {
//         unsafe { ((*self.vmt).update_online_score_archery)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn update_online_score_chicken_kick(&mut self, a: c_float) {
//         unsafe {
//             ((*self.vmt).update_online_score_chicken_kick)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn update_online_score_chapel_or_temple(&mut self, a: c_float) {
//         unsafe {
//             ((*self.vmt).update_online_score_chapel_or_temple)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn update_online_score_fishing_compo(&mut self, a: c_float) {
//         unsafe {
//             ((*self.vmt).update_online_score_fishing_compo)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn update_score_fishing_competition(&mut self, a: c_float) {
//         unsafe {
//             ((*self.vmt).update_score_fishing_competition)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn get_best_time_pairs(&mut self) -> c_float {
//         unsafe { ((*self.vmt).get_best_time_pairs)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_best_time_sorting(&mut self) -> c_float {
//         unsafe { ((*self.vmt).get_best_time_sorting)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_best_score_blackjack(&mut self) -> c_long {
//         unsafe { ((*self.vmt).get_best_score_blackjack)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_best_score_coin_golf_oak_vale(&mut self) -> c_long {
//         unsafe {
//             ((*self.vmt).get_best_score_coin_golf_oak_vale)(self as *mut CGameScriptInterface)
//         }
//     }

//     pub fn get_best_score_coin_golf_snow_spire(&mut self) -> c_long {
//         unsafe {
//             ((*self.vmt).get_best_score_coin_golf_snow_spire)(self as *mut CGameScriptInterface)
//         }
//     }

//     pub fn get_best_score_shove_ha_penny(&mut self) -> c_long {
//         unsafe { ((*self.vmt).get_best_score_shove_ha_penny)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_best_time_guess_the_addition(&mut self) -> c_float {
//         unsafe { ((*self.vmt).get_best_time_guess_the_addition)(self as *mut CGameScriptInterface) }
//     }

//     pub fn is_hero_in_tavern_game(&mut self) -> bool {
//         unsafe { ((*self.vmt).is_hero_in_tavern_game)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_num_houses_owned(&mut self) -> c_long {
//         unsafe { ((*self.vmt).get_num_houses_owned)(self as *mut CGameScriptInterface) }
//     }

//     pub fn start_sneaking(&mut self) {
//         unsafe { ((*self.vmt).start_sneaking)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_steal_duration(&mut self, a: *const CScriptThing) -> c_long {
//         unsafe { ((*self.vmt).get_steal_duration)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_useable_by_hero(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).set_useable_by_hero)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn set_owned_by_hero(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).set_owned_by_hero)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn set_tavern_table_available_for_use(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe {
//             ((*self.vmt).set_tavern_table_available_for_use)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn set_is_thing_turncoatable(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).set_is_thing_turncoatable)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn set_is_thing_force_pushable(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe {
//             ((*self.vmt).set_is_thing_force_pushable)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn set_is_thing_lightningable(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).set_is_thing_lightningable)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn set_is_thing_epic_spellable(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe {
//             ((*self.vmt).set_is_thing_epic_spellable)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn is_thing_turncoated(&mut self, a: *const CScriptThing) -> bool {
//         unsafe { ((*self.vmt).is_thing_turncoated)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn add_creature_scripted_mode(&mut self, a: *const CScriptThing, b: *const CCharString) {
//         unsafe { ((*self.vmt).add_creature_scripted_mode)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn remove_creature_scripted_mode(&mut self, a: *const CScriptThing) {
//         unsafe { ((*self.vmt).remove_creature_scripted_mode)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn force_ships_visible(&mut self) {
//         unsafe { ((*self.vmt).force_ships_visible)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_sleeping_position_and_orientation_from_bed(
//         &mut self,
//         a: *const CScriptThing,
//         b: *const CScriptThing,
//         c: *mut C3DVector,
//         d: *mut C3DVector,
//     ) -> bool {
//         unsafe {
//             ((*self.vmt).get_sleeping_position_and_orientation_from_bed)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//                 c,
//                 d,
//             )
//         }
//     }

//     pub fn set_bed_availability(&mut self, a: *const CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).set_bed_availability)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn repopulate_village(&mut self, a: *const CScriptThing) {
//         unsafe { ((*self.vmt).repopulate_village)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn smash_all_windows_within_radius_of_point(&mut self, a: *const C3DVector, b: c_float) {
//         unsafe {
//             ((*self.vmt).smash_all_windows_within_radius_of_point)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn set_residency(&mut self, a: *const CScriptThing, b: bool) -> CScriptThing {
//         unsafe { ((*self.vmt).set_residency)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn set_thanking_phrase(&mut self, a: CScriptThing, b: c_ulong) {
//         unsafe { ((*self.vmt).set_thanking_phrase)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn get_thanking_phrase(&mut self, a: CScriptThing) -> c_ulong {
//         unsafe { ((*self.vmt).get_thanking_phrase)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn reset_thanking_phrase(&mut self, a: CScriptThing) {
//         unsafe { ((*self.vmt).reset_thanking_phrase)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_ignoring_phrase(&mut self, a: CScriptThing, b: c_ulong) {
//         unsafe { ((*self.vmt).set_ignoring_phrase)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn get_ignoring_phrase(&mut self, a: CScriptThing) -> c_ulong {
//         unsafe { ((*self.vmt).get_ignoring_phrase)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn reset_ignoring_phrase(&mut self, a: CScriptThing) {
//         unsafe { ((*self.vmt).reset_ignoring_phrase)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_wander_centre_point(&mut self, a: CScriptThing, b: C3DVector) {
//         unsafe { ((*self.vmt).set_wander_centre_point)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn get_wander_centre_point(&mut self, a: CScriptThing) -> C3DVector {
//         unsafe { ((*self.vmt).get_wander_centre_point)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn reset_wander_centre_point(&mut self, a: CScriptThing) {
//         unsafe { ((*self.vmt).reset_wander_centre_point)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_wander_min_distance(&mut self, a: CScriptThing, b: c_float) {
//         unsafe { ((*self.vmt).set_wander_min_distance)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn get_wander_min_distance(&mut self, a: CScriptThing) -> c_float {
//         unsafe { ((*self.vmt).get_wander_min_distance)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn reset_wander_min_distance(&mut self, a: CScriptThing) {
//         unsafe { ((*self.vmt).reset_wander_min_distance)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_wander_max_distance(&mut self, a: CScriptThing, b: c_float) {
//         unsafe { ((*self.vmt).set_wander_max_distance)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn get_wander_max_distance(&mut self, a: CScriptThing) -> c_float {
//         unsafe { ((*self.vmt).get_wander_max_distance)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn reset_wander_max_distance(&mut self, a: CScriptThing) {
//         unsafe { ((*self.vmt).reset_wander_max_distance)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_gossip_counter(&mut self, a: CScriptThing, b: c_long) {
//         unsafe { ((*self.vmt).set_gossip_counter)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn get_gossip_counter(&mut self, a: CScriptThing) -> c_long {
//         unsafe { ((*self.vmt).get_gossip_counter)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn reset_gossip_counter(&mut self, a: CScriptThing) {
//         unsafe { ((*self.vmt).reset_gossip_counter)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_max_gossip_phrase(&mut self, a: CScriptThing, b: c_long) {
//         unsafe { ((*self.vmt).set_max_gossip_phrase)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn get_max_gossip_phrase(&mut self, a: CScriptThing) -> c_long {
//         unsafe { ((*self.vmt).get_max_gossip_phrase)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn reset_max_gossip_phrase(&mut self, a: CScriptThing) {
//         unsafe { ((*self.vmt).reset_max_gossip_phrase)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_warning_phrase(&mut self, a: CScriptThing, b: c_ulong) {
//         unsafe { ((*self.vmt).set_warning_phrase)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn get_warning_phrase(&mut self, a: CScriptThing) -> c_ulong {
//         unsafe { ((*self.vmt).get_warning_phrase)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn reset_warning_phrase(&mut self, a: CScriptThing) {
//         unsafe { ((*self.vmt).reset_warning_phrase)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_beer_request_phrase(&mut self, a: CScriptThing, b: c_ulong) {
//         unsafe { ((*self.vmt).set_beer_request_phrase)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn get_beer_request_phrase(&mut self, a: CScriptThing) -> c_ulong {
//         unsafe { ((*self.vmt).get_beer_request_phrase)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn reset_beer_request_phrase(&mut self, a: CScriptThing) {
//         unsafe { ((*self.vmt).reset_beer_request_phrase)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_scripting_state_group(&mut self, a: CScriptThing, b: EScriptingStateGroups) {
//         unsafe { ((*self.vmt).set_scripting_state_group)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn get_scripting_state_group(&mut self, a: CScriptThing) -> EScriptingStateGroups {
//         unsafe { ((*self.vmt).get_scripting_state_group)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn reset_scripting_state_group(&mut self, a: CScriptThing) {
//         unsafe { ((*self.vmt).reset_scripting_state_group)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_max_hero_reaction_distance(&mut self, a: CScriptThing, b: c_float) {
//         unsafe {
//             ((*self.vmt).set_max_hero_reaction_distance)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn get_max_hero_reaction_distance(&mut self, a: CScriptThing) -> c_float {
//         unsafe {
//             ((*self.vmt).get_max_hero_reaction_distance)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn reset_max_hero_reaction_distance(&mut self, a: CScriptThing) {
//         unsafe {
//             ((*self.vmt).reset_max_hero_reaction_distance)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn set_action_frequency(&mut self, a: CScriptThing, b: c_long) {
//         unsafe { ((*self.vmt).set_action_frequency)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn get_action_frequency(&mut self, a: CScriptThing) -> c_long {
//         unsafe { ((*self.vmt).get_action_frequency)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn reset_action_frequency(&mut self, a: CScriptThing) {
//         unsafe { ((*self.vmt).reset_action_frequency)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_action_frequency_variation(&mut self, a: CScriptThing, b: c_float) {
//         unsafe {
//             ((*self.vmt).set_action_frequency_variation)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn get_action_frequency_variation(&mut self, a: CScriptThing) -> c_float {
//         unsafe {
//             ((*self.vmt).get_action_frequency_variation)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn reset_action_frequency_variation(&mut self, a: CScriptThing) {
//         unsafe {
//             ((*self.vmt).reset_action_frequency_variation)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn set_action(&mut self, a: CScriptThing, b: CCharString) {
//         unsafe { ((*self.vmt).set_action)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn get_action(&mut self, a: CScriptThing) -> CCharString {
//         unsafe { ((*self.vmt).get_action)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn reset_action(&mut self, a: CScriptThing) {
//         unsafe { ((*self.vmt).reset_action)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_face_hero_for_action(&mut self, a: CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).set_face_hero_for_action)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn get_face_hero_for_action(&mut self, a: CScriptThing) -> bool {
//         unsafe { ((*self.vmt).get_face_hero_for_action)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn reset_face_hero_for_action(&mut self, a: CScriptThing) {
//         unsafe { ((*self.vmt).reset_face_hero_for_action)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_target_name(&mut self, a: CScriptThing, b: CCharString) {
//         unsafe { ((*self.vmt).set_target_name)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn get_target_name(&mut self, a: CScriptThing) -> CCharString {
//         unsafe { ((*self.vmt).get_target_name)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn reset_target_name(&mut self, a: CScriptThing) {
//         unsafe { ((*self.vmt).reset_target_name)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_follow_distance(&mut self, a: CScriptThing, b: c_float) {
//         unsafe { ((*self.vmt).set_follow_distance)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn get_follow_distance(&mut self, a: CScriptThing) -> c_float {
//         unsafe { ((*self.vmt).get_follow_distance)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn reset_follow_distance(&mut self, a: CScriptThing) {
//         unsafe { ((*self.vmt).reset_follow_distance)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_attack_hero_on_sight(&mut self, a: CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).set_attack_hero_on_sight)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn get_attack_hero_on_sight(&mut self, a: CScriptThing) -> bool {
//         unsafe { ((*self.vmt).get_attack_hero_on_sight)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn reset_attack_hero_on_sight(&mut self, a: CScriptThing) {
//         unsafe { ((*self.vmt).reset_attack_hero_on_sight)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_time_to_spend_harassing_hero(&mut self, a: CScriptThing, b: c_long) {
//         unsafe {
//             ((*self.vmt).set_time_to_spend_harassing_hero)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn get_time_to_spend_harassing_hero(&mut self, a: CScriptThing) -> c_long {
//         unsafe {
//             ((*self.vmt).get_time_to_spend_harassing_hero)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn reset_time_to_spend_harassing_hero(&mut self, a: CScriptThing) {
//         unsafe {
//             ((*self.vmt).reset_time_to_spend_harassing_hero)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn set_combat_nearby_enemy_fleeing_break_off_range(&mut self, a: CScriptThing, b: c_float) {
//         unsafe {
//             ((*self.vmt).set_combat_nearby_enemy_fleeing_break_off_range)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn get_combat_nearby_enemy_fleeing_break_off_range(&mut self, a: CScriptThing) -> c_float {
//         unsafe {
//             ((*self.vmt).get_combat_nearby_enemy_fleeing_break_off_range)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn reset_combat_nearby_enemy_fleeing_break_off_range(&mut self, a: CScriptThing) {
//         unsafe {
//             ((*self.vmt).reset_combat_nearby_enemy_fleeing_break_off_range)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn set_combat_nearby_break_off_range(&mut self, a: CScriptThing, b: c_float) {
//         unsafe {
//             ((*self.vmt).set_combat_nearby_break_off_range)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn get_combat_nearby_break_off_range(&mut self, a: CScriptThing) -> c_float {
//         unsafe {
//             ((*self.vmt).get_combat_nearby_break_off_range)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn reset_combat_nearby_break_off_range(&mut self, a: CScriptThing) {
//         unsafe {
//             ((*self.vmt).reset_combat_nearby_break_off_range)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn set_steal_stealable_items(&mut self, a: CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).set_steal_stealable_items)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn get_steal_stealable_items(&mut self, a: CScriptThing) -> bool {
//         unsafe { ((*self.vmt).get_steal_stealable_items)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn reset_steal_stealable_items(&mut self, a: CScriptThing) {
//         unsafe { ((*self.vmt).reset_steal_stealable_items)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_recover_stealable_items(&mut self, a: CScriptThing, b: bool) {
//         unsafe {
//             ((*self.vmt).set_recover_stealable_items)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn get_recover_stealable_items(&mut self, a: CScriptThing) -> bool {
//         unsafe { ((*self.vmt).get_recover_stealable_items)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn reset_recover_stealable_items(&mut self, a: CScriptThing) {
//         unsafe { ((*self.vmt).reset_recover_stealable_items)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_take_stealable_item_to_random_destination(&mut self, a: CScriptThing, b: bool) {
//         unsafe {
//             ((*self.vmt).set_take_stealable_item_to_random_destination)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn get_take_stealable_item_to_random_destination(&mut self, a: CScriptThing) -> bool {
//         unsafe {
//             ((*self.vmt).get_take_stealable_item_to_random_destination)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn reset_take_stealable_item_to_random_destination(&mut self, a: CScriptThing) {
//         unsafe {
//             ((*self.vmt).reset_take_stealable_item_to_random_destination)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn set_kill_self_and_stealable_item_after_reaching_destination(
//         &mut self,
//         a: CScriptThing,
//         b: bool,
//     ) {
//         unsafe {
//             ((*self.vmt).set_kill_self_and_stealable_item_after_reaching_destination)(
//                 self as *mut CGameScriptInterface,
//                 a,
//                 b,
//             )
//         }
//     }

//     pub fn get_kill_self_and_stealable_item_after_reaching_destination(
//         &mut self,
//         a: CScriptThing,
//     ) -> bool {
//         unsafe {
//             ((*self.vmt).get_kill_self_and_stealable_item_after_reaching_destination)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn reset_kill_self_and_stealable_item_after_reaching_destination(
//         &mut self,
//         a: CScriptThing,
//     ) {
//         unsafe {
//             ((*self.vmt).reset_kill_self_and_stealable_item_after_reaching_destination)(
//                 self as *mut CGameScriptInterface,
//                 a,
//             )
//         }
//     }

//     pub fn set_allowed_to_follow(&mut self, a: CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).set_allowed_to_follow)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn get_allowed_to_follow(&mut self, a: CScriptThing) -> bool {
//         unsafe { ((*self.vmt).get_allowed_to_follow)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn reset_allowed_to_follow(&mut self, a: CScriptThing) {
//         unsafe { ((*self.vmt).reset_allowed_to_follow)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_table_name(&mut self, a: CScriptThing, b: CCharString) {
//         unsafe { ((*self.vmt).set_table_name)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn get_table_name(&mut self, a: CScriptThing) -> CCharString {
//         unsafe { ((*self.vmt).get_table_name)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn reset_table_name(&mut self, a: CScriptThing) {
//         unsafe { ((*self.vmt).reset_table_name)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_seat_name(&mut self, a: CScriptThing, b: CCharString) {
//         unsafe { ((*self.vmt).set_seat_name)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn get_seat_name(&mut self, a: CScriptThing) -> CCharString {
//         unsafe { ((*self.vmt).get_seat_name)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn reset_seat_name(&mut self, a: CScriptThing) {
//         unsafe { ((*self.vmt).reset_seat_name)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_disable_head_looking(&mut self, a: CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).set_disable_head_looking)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn get_disable_head_looking(&mut self, a: CScriptThing) -> bool {
//         unsafe { ((*self.vmt).get_disable_head_looking)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn reset_disable_head_looking(&mut self, a: CScriptThing) {
//         unsafe { ((*self.vmt).reset_disable_head_looking)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_is_pushable_by_hero(&mut self, a: CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).set_is_pushable_by_hero)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn get_is_pushable_by_hero(&mut self, a: CScriptThing) -> bool {
//         unsafe { ((*self.vmt).get_is_pushable_by_hero)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn reset_is_pushable_by_hero(&mut self, a: CScriptThing) {
//         unsafe { ((*self.vmt).reset_is_pushable_by_hero)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_look_for_finite_time(&mut self, a: CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).set_look_for_finite_time)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn get_look_for_finite_time(&mut self, a: CScriptThing) -> bool {
//         unsafe { ((*self.vmt).get_look_for_finite_time)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn reset_look_for_finite_time(&mut self, a: CScriptThing) {
//         unsafe { ((*self.vmt).reset_look_for_finite_time)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_avoid_region_exits(&mut self, a: CScriptThing, b: bool) {
//         unsafe { ((*self.vmt).set_avoid_region_exits)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn get_avoid_region_exits(&mut self, a: CScriptThing) -> bool {
//         unsafe { ((*self.vmt).get_avoid_region_exits)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn reset_avoid_region_exits(&mut self, a: CScriptThing) {
//         unsafe { ((*self.vmt).reset_avoid_region_exits)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn set_targeting_distance_offset(&mut self, a: CScriptThing, b: c_float) {
//         unsafe {
//             ((*self.vmt).set_targeting_distance_offset)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn get_targeting_distance_offset(&mut self, a: CScriptThing) -> c_float {
//         unsafe { ((*self.vmt).get_targeting_distance_offset)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn reset_targeting_distance_offset(&mut self, a: CScriptThing) {
//         unsafe {
//             ((*self.vmt).reset_targeting_distance_offset)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn set_player_using_melee_dummies(&mut self, a: bool) {
//         unsafe {
//             ((*self.vmt).set_player_using_melee_dummies)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn get_player_using_melee_dummies(&mut self) -> bool {
//         unsafe { ((*self.vmt).get_player_using_melee_dummies)(self as *mut CGameScriptInterface) }
//     }

//     pub fn set_player_using_ranged_dummies(&mut self, a: bool) {
//         unsafe {
//             ((*self.vmt).set_player_using_ranged_dummies)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn get_player_using_ranged_dummies(&mut self) -> bool {
//         unsafe { ((*self.vmt).get_player_using_ranged_dummies)(self as *mut CGameScriptInterface) }
//     }

//     pub fn set_player_using_will_dummies(&mut self, a: bool) {
//         unsafe { ((*self.vmt).set_player_using_will_dummies)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_player_using_will_dummies(&mut self) -> bool {
//         unsafe { ((*self.vmt).get_player_using_will_dummies)(self as *mut CGameScriptInterface) }
//     }

//     pub fn set_cheap_head_looking(&mut self, a: bool) {
//         unsafe { ((*self.vmt).set_cheap_head_looking)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_cheap_head_looking(&mut self) -> bool {
//         unsafe { ((*self.vmt).get_cheap_head_looking)(self as *mut CGameScriptInterface) }
//     }

//     pub fn set_quit_tavern_game(&mut self, a: bool) {
//         unsafe { ((*self.vmt).set_quit_tavern_game)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_quit_tavern_game(&mut self) -> bool {
//         unsafe { ((*self.vmt).get_quit_tavern_game)(self as *mut CGameScriptInterface) }
//     }

//     pub fn set_prize_tavern_table(&mut self, a: bool) {
//         unsafe { ((*self.vmt).set_prize_tavern_table)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_prize_tavern_table(&mut self) -> bool {
//         unsafe { ((*self.vmt).get_prize_tavern_table)(self as *mut CGameScriptInterface) }
//     }

//     pub fn set_betting_active(&mut self, a: bool) {
//         unsafe { ((*self.vmt).set_betting_active)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_betting_active(&mut self) -> bool {
//         unsafe { ((*self.vmt).get_betting_active)(self as *mut CGameScriptInterface) }
//     }

//     pub fn set_betting_accept(&mut self, a: bool) {
//         unsafe { ((*self.vmt).set_betting_accept)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_betting_accept(&mut self) -> bool {
//         unsafe { ((*self.vmt).get_betting_accept)(self as *mut CGameScriptInterface) }
//     }

//     pub fn set_betting_amount(&mut self, a: c_long) {
//         unsafe { ((*self.vmt).set_betting_amount)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_betting_amount(&mut self) -> c_long {
//         unsafe { ((*self.vmt).get_betting_amount)(self as *mut CGameScriptInterface) }
//     }

//     pub fn set_count_bet_money_down(&mut self, a: bool) {
//         unsafe { ((*self.vmt).set_count_bet_money_down)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_count_bet_money_down(&mut self) -> bool {
//         unsafe { ((*self.vmt).get_count_bet_money_down)(self as *mut CGameScriptInterface) }
//     }

//     pub fn set_spot_the_addition_beaten(&mut self, a: bool) {
//         unsafe { ((*self.vmt).set_spot_the_addition_beaten)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_spot_the_addition_beaten(&mut self) -> bool {
//         unsafe { ((*self.vmt).get_spot_the_addition_beaten)(self as *mut CGameScriptInterface) }
//     }

//     pub fn set_global_targeting_distance_offset(&mut self, a: c_float) {
//         unsafe {
//             ((*self.vmt).set_global_targeting_distance_offset)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn get_global_targeting_distance_offset(&mut self) -> c_float {
//         unsafe {
//             ((*self.vmt).get_global_targeting_distance_offset)(self as *mut CGameScriptInterface)
//         }
//     }

//     pub fn set_trading_price_mult(&mut self, a: c_float) {
//         unsafe { ((*self.vmt).set_trading_price_mult)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_trading_price_mult(&mut self) -> c_float {
//         unsafe { ((*self.vmt).get_trading_price_mult)(self as *mut CGameScriptInterface) }
//     }

//     pub fn set_boasting_enabled(&mut self, a: bool) {
//         unsafe { ((*self.vmt).set_boasting_enabled)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_boasting_enabled(&mut self) -> bool {
//         unsafe { ((*self.vmt).get_boasting_enabled)(self as *mut CGameScriptInterface) }
//     }

//     pub fn set_active_gossip_categories(&mut self, a: CCharString, b: bool) {
//         unsafe {
//             ((*self.vmt).set_active_gossip_categories)(self as *mut CGameScriptInterface, a, b)
//         }
//     }

//     pub fn get_active_gossip_categories(&mut self) -> *const CxxMap<CCharString, bool> {
//         unsafe { ((*self.vmt).get_active_gossip_categories)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_active_gossip_categories_2(&mut self, a: CCharString) -> *mut bool {
//         unsafe {
//             ((*self.vmt).get_active_gossip_categories_2)(self as *mut CGameScriptInterface, a)
//         }
//     }

//     pub fn get_active_gossip_categories_size(&mut self) -> i32 {
//         unsafe {
//             ((*self.vmt).get_active_gossip_categories_size)(self as *mut CGameScriptInterface)
//         }
//     }

//     pub fn clear_active_gossip_categories(&mut self) {
//         unsafe { ((*self.vmt).clear_active_gossip_categories)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_is_gossip_for_player(&mut self) -> *const CxxMap<CCharString, bool> {
//         unsafe { ((*self.vmt).get_is_gossip_for_player)(self as *mut CGameScriptInterface) }
//     }

//     pub fn get_is_gossip_for_player_2(&mut self, a: CCharString) -> *mut bool {
//         unsafe { ((*self.vmt).get_is_gossip_for_player_2)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_is_gossip_for_player_size(&mut self) -> i32 {
//         unsafe { ((*self.vmt).get_is_gossip_for_player_size)(self as *mut CGameScriptInterface) }
//     }

//     pub fn clear_is_gossip_for_player(&mut self) {
//         unsafe { ((*self.vmt).clear_is_gossip_for_player)(self as *mut CGameScriptInterface) }
//     }

//     pub fn set_gossip(&mut self, a: CCharString, b: CCharString, c: c_long) {
//         unsafe { ((*self.vmt).set_gossip)(self as *mut CGameScriptInterface, a, b, c) }
//     }

//     pub fn get_gossip(&mut self, a: CCharString) -> *const CxxVector<CCharString> {
//         unsafe { ((*self.vmt).get_gossip)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_gossip_2(&mut self, a: CCharString, b: c_long) -> CCharString {
//         unsafe { ((*self.vmt).get_gossip_2)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn get_gossip_size(&mut self, a: CCharString) -> i32 {
//         unsafe { ((*self.vmt).get_gossip_size)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn clear_gossip(&mut self, a: CCharString) {
//         unsafe { ((*self.vmt).clear_gossip)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn remove_gossip(&mut self, a: CCharString) {
//         unsafe { ((*self.vmt).remove_gossip)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn add_gossip(&mut self, a: CCharString) {
//         unsafe { ((*self.vmt).add_gossip)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn add_gossip_2(&mut self, a: CCharString, b: CCharString) {
//         unsafe { ((*self.vmt).add_gossip_2)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn set_gossip_villages(&mut self, a: CCharString, b: CCharString, c: c_long) {
//         unsafe { ((*self.vmt).set_gossip_villages)(self as *mut CGameScriptInterface, a, b, c) }
//     }

//     pub fn get_gossip_villages(&mut self, a: CCharString) -> *const CxxVector<CCharString> {
//         unsafe { ((*self.vmt).get_gossip_villages)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_gossip_villages_2(&mut self, a: CCharString, b: c_long) -> CCharString {
//         unsafe { ((*self.vmt).get_gossip_villages_2)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn get_gossip_villages_size(&mut self, a: CCharString) -> i32 {
//         unsafe { ((*self.vmt).get_gossip_villages_size)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn clear_gossip_villages(&mut self, a: CCharString) {
//         unsafe { ((*self.vmt).clear_gossip_villages)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn remove_gossip_villages(&mut self, a: CCharString) {
//         unsafe { ((*self.vmt).remove_gossip_villages)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn add_gossip_villages(&mut self, a: CCharString) {
//         unsafe { ((*self.vmt).add_gossip_villages)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn add_gossip_villages_2(&mut self, a: CCharString, b: CCharString) {
//         unsafe { ((*self.vmt).add_gossip_villages_2)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn set_gossip_factions(&mut self, a: CCharString, b: CCharString, c: c_long) {
//         unsafe { ((*self.vmt).set_gossip_factions)(self as *mut CGameScriptInterface, a, b, c) }
//     }

//     pub fn get_gossip_factions(&mut self, a: CCharString) -> *const CxxVector<CCharString> {
//         unsafe { ((*self.vmt).get_gossip_factions)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn get_gossip_factions_2(&mut self, a: CCharString, b: c_long) -> CCharString {
//         unsafe { ((*self.vmt).get_gossip_factions_2)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn get_gossip_factions_size(&mut self, a: CCharString) -> i32 {
//         unsafe { ((*self.vmt).get_gossip_factions_size)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn clear_gossip_factions(&mut self, a: CCharString) {
//         unsafe { ((*self.vmt).clear_gossip_factions)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn remove_gossip_factions(&mut self, a: CCharString) {
//         unsafe { ((*self.vmt).remove_gossip_factions)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn add_gossip_factions(&mut self, a: CCharString) {
//         unsafe { ((*self.vmt).add_gossip_factions)(self as *mut CGameScriptInterface, a) }
//     }

//     pub fn add_gossip_factions_2(&mut self, a: CCharString, b: CCharString) {
//         unsafe { ((*self.vmt).add_gossip_factions_2)(self as *mut CGameScriptInterface, a, b) }
//     }

//     pub fn c_game_script_interface_destructor(&mut self) {
//         unsafe {
//             ((*self.vmt).c_game_script_interface_destructor)(self as *mut CGameScriptInterface)
//         }
//     }
// }

// // #[derive(Debug)]
// #[repr(C)]
// pub struct CGameScriptInterfaceVmt {
//     pub end_letter_box: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub error: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CCharString,
//         *const CCharString,
//         c_ulong,
//     ),
//     pub trace_message: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
//     pub validate: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub set_debug_camera_type: extern "thiscall" fn(*mut CGameScriptInterface, c_long),
//     pub deactivate_boast_ui: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub is_xbox: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub new_script_frame: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub start_scripting_entity: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *mut CScriptGameResourceObjectScriptedThingBase,
//         EScriptAIPriority,
//     ) -> bool,
//     pub is_entity_under_scripted_control:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
//     pub is_active_thread_terminating:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub is_level_loaded:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
//     pub is_region_loaded:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
//     pub is_region_loaded_and_preloaded:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
//     pub is_region_def_loaded:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
//     pub get_region_name: extern "thiscall" fn(*mut CGameScriptInterface) -> *const CCharString,
//     pub msg_is_level_loaded:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
//     pub msg_is_level_unloaded:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
//     pub msg_on_level_loaded:
//         extern "thiscall" fn(*mut CGameScriptInterface, *mut CxxList<CCharString>) -> bool,
//     pub msg_on_level_unloaded:
//         extern "thiscall" fn(*mut CGameScriptInterface, *mut CxxList<CCharString>) -> bool,
//     pub msg_is_region_loaded:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
//     pub msg_is_region_unloaded:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
//     pub msg_on_region_loaded:
//         extern "thiscall" fn(*mut CGameScriptInterface, *mut CCharString) -> bool,
//     pub msg_on_region_unloaded:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
//     pub msg_on_region_preunloaded:
//         extern "thiscall" fn(*mut CGameScriptInterface, *mut CCharString) -> bool,
//     pub msg_on_quest_completed:
//         extern "thiscall" fn(*mut CGameScriptInterface, *mut CCharString) -> bool,
//     pub msg_on_any_quest_completed:
//         extern "thiscall" fn(*mut CGameScriptInterface, *mut CCharString) -> bool,
//     pub msg_on_quest_failed:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
//     pub msg_on_quest_completed_before_screen_shown:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
//     pub msg_on_quest_failed_before_screen_shown:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
//     pub msg_on_quest_accepted:
//         extern "thiscall" fn(*mut CGameScriptInterface, *mut CCharString) -> bool,
//     pub msg_on_feat_accepted: extern "thiscall" fn(*mut CGameScriptInterface, *mut c_long) -> bool,
//     pub msg_is_boast_made: extern "thiscall" fn(*mut CGameScriptInterface, c_long) -> bool,
//     pub msg_on_boast_made:
//         extern "thiscall" fn(*mut CGameScriptInterface, *mut c_long, *mut CCharString) -> bool,
//     pub msg_on_boasts_made:
//         extern "thiscall" fn(*mut CGameScriptInterface, *mut CxxVector<CCharString>) -> bool,
//     pub remove_boast_message: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub is_quest_start_screen_active:
//         extern "thiscall" fn(*mut CGameScriptInterface, *mut CCharString) -> bool,
//     pub msg_on_leaving_quest_start_screen:
//         extern "thiscall" fn(*mut CGameScriptInterface, *mut CCharString) -> bool,
//     pub msg_on_leaving_experience_spending_screen:
//         extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub msg_is_answered_yes_or_no: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
//     pub msg_is_game_info_clicked_past: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub msg_is_tutorial_click_past: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub msg_is_action_mode_button_pressed: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub msg_on_expression_performed:
//         extern "thiscall" fn(*mut CGameScriptInterface, *mut CCharString) -> bool,
//     pub msg_is_cut_scene_skipped: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub remove_all_cut_scene_skipped_messages:
//         extern "thiscall" fn(*mut CGameScriptInterface, *mut CCharString),
//     pub msg_on_hero_hair_type_changed: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         EClothingCoversArea,
//         *mut CCharString,
//     ) -> bool,
//     pub msg_on_hero_used_teleporter:
//         extern "thiscall" fn(*mut CGameScriptInterface, *mut CCharString) -> bool,
//     pub msg_on_hero_used_guild_seal: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub msg_on_game_saved_manually: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub msg_on_hero_slept: extern "thiscall" fn(*mut CGameScriptInterface, *mut bool) -> bool,
//     pub msg_on_hero_fired_ranged_weapon: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub msg_on_hero_cast_spell:
//         extern "thiscall" fn(*mut CGameScriptInterface, *mut EHeroAbility) -> bool,
//     pub msg_on_hero_picked_pocket:
//         extern "thiscall" fn(*mut CGameScriptInterface, *mut CScriptThing) -> bool,
//     pub msg_on_hero_picked_lock:
//         extern "thiscall" fn(*mut CGameScriptInterface, *mut CScriptThing) -> bool,
//     pub msg_on_fishing_game_finished:
//         extern "thiscall" fn(*mut CGameScriptInterface, *mut CScriptThing) -> bool,
//     pub msg_on_tavern_game_finished:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
//     pub msg_on_hero_rewarded_with_items_from:
//         extern "thiscall" fn(*mut CGameScriptInterface, *mut CCharString) -> bool,
//     pub msg_on_chest_opening_cancelled: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub dont_populate_next_loaded_region: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub get_wandering_population_script_def_name_in_current_region:
//         extern "thiscall" fn(*mut CGameScriptInterface, *mut CCharString),
//     pub get_wandering_population_script_def_name_in_region:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *mut CCharString),
//     pub is_hero_allowed_henchmen_in_current_region:
//         extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub is_hero_allowed_henchmen_in_region:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
//     pub post_add_scripted_entities: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub is_player_holding_unsheathe_ranged_weapon_button:
//         extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub is_player_holding_lock_target_button:
//         extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub is_player_holding_fire_ranged_weapon_button:
//         extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub is_player_holding_first_person_targeting_button:
//         extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub is_hero_in_projectile_weapon_mode: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub register_timer: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
//     pub deregister_timer: extern "thiscall" fn(*mut CGameScriptInterface, c_long),
//     pub set_timer: extern "thiscall" fn(*mut CGameScriptInterface, c_long, c_long),
//     pub get_timer: extern "thiscall" fn(*mut CGameScriptInterface, c_long) -> c_long,
//     pub get_hero: extern "thiscall" fn(*mut CGameScriptInterface) -> *mut CScriptThing,
//     pub get_hero_targeted_thing: extern "thiscall" fn(*mut CGameScriptInterface) -> CScriptThing,
//     pub get_thing_with_script_name:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> CScriptThing,
//     pub get_thing_with_script_name_2: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CCharString,
//         *const CCharString,
//     ) -> CScriptThing,
//     pub get_random_thing_with_script_name:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> CScriptThing,
//     pub get_all_things_with_script_name: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CCharString,
//         *mut CxxVector<CScriptThing>,
//     ) -> c_long,
//     pub get_all_creatures_in_area_with_script_name: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CCharString,
//         *const C3DVector,
//         c_float,
//         *const CxxVector<CScriptThing>,
//     ) -> c_long,
//     pub get_nearest_with_script_name: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *const CCharString,
//     ) -> CScriptThing,
//     pub get_furthest_with_script_name: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *const CCharString,
//     ) -> CScriptThing,
//     pub get_all_things_with_def_name: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CCharString,
//         *mut CxxVector<CScriptThing>,
//     ) -> c_long,
//     pub get_all_things_with_def_name_by_distance_from: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *const CCharString,
//         *mut CxxVector<CScriptThing>,
//     ) -> c_long,
//     pub get_nearest_with_def_name: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *const CCharString,
//     ) -> CScriptThing,
//     pub get_furthest_with_def_name: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *const CCharString,
//     ) -> CScriptThing,
//     pub get_thing_with_uid:
//         extern "thiscall" fn(*mut CGameScriptInterface, c_ulong) -> CScriptThing,
//     pub get_all_creatures_excluding_hero: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *mut CxxVector<CScriptThing>,
//     ) -> c_long,
//     pub get_all_things_in_level: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CCharString,
//         *mut CxxVector<CScriptThing>,
//     ) -> c_long,
//     pub is_thing_with_this_uid_alive:
//         extern "thiscall" fn(*mut CGameScriptInterface, c_ulong) -> bool,
//     pub create_creature: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CCharString,
//         *const C3DVector,
//         *const CCharString,
//         bool,
//     ) -> CScriptThing,
//     pub create_creature_nearby: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CCharString,
//         *const C3DVector,
//         c_float,
//         *const CCharString,
//         bool,
//     ) -> CScriptThing,
//     pub create_creature_on_entity: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CCharString,
//         *const CScriptThing,
//         *const CCharString,
//     ) -> CScriptThing,
//     pub turn_creature_into: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *const CCharString,
//     ) -> CScriptThing,
//     pub set_creature_creation_delay_frames: extern "thiscall" fn(*mut CGameScriptInterface, c_long),
//     pub reset_creature_creation_delay_frames: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub create_object: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CCharString,
//         *const C3DVector,
//         c_float,
//         *const CCharString,
//     ) -> CScriptThing,
//     pub create_object_2: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CCharString,
//         *const C3DVector,
//         *const CCharString,
//     ) -> CScriptThing,
//     pub create_object_on_entity: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CCharString,
//         *const CScriptThing,
//         *const CCharString,
//     ) -> CScriptThing,
//     pub create_effect: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CCharString,
//         *const C3DVector,
//         *const CCharString,
//         c_float,
//         bool,
//         bool,
//     ) -> CScriptThing,
//     pub create_effect_2: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CCharString,
//         *const CScriptThing,
//         *const CCharString,
//         *const CCharString,
//         bool,
//         bool,
//     ) -> CScriptThing,
//     pub create_light: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const C3DVector,
//         *const CRGBColour,
//         *const CCharString,
//         c_float,
//         c_float,
//         bool,
//     ) -> CScriptThing,
//     pub create_experience_orb:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const C3DVector, c_long) -> CScriptThing,
//     pub create_explosion: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CCharString,
//         *const C3DVector,
//         *const C3DVector,
//         CCharString,
//     ) -> CScriptThing,
//     pub create_physical_barrier: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         c_float,
//         *const C3DVector,
//         *const C3DVector,
//         CCharString,
//     ) -> CScriptThing,
//     pub create_rumble: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const C3DVector,
//         c_float,
//         c_float,
//         CCharString,
//     ) -> CScriptThing,
//     pub clear_all_rumbles: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub remove_thing:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool, bool),
//     pub show_on_screen_message: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const C2DVector,
//         *const CCharString,
//         *const CRGBColour,
//         *const CCharString,
//     ),
//     pub show_on_screen_message_2: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CCharString,
//         *const CCharString,
//         *const CCharString,
//         c_float,
//     ),
//     pub show_on_screen_message_3:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, c_float),
//     pub add_screen_message: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CCharString,
//         ETextGroupSelectionMethod,
//     ),
//     pub add_screen_title_message:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, c_float, bool),
//     pub give_hero_yes_no_question: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CCharString,
//         *const CCharString,
//         *const CCharString,
//         *const CCharString,
//         bool,
//     ),
//     pub display_game_info: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
//     pub display_game_info_text: extern "thiscall" fn(*mut CGameScriptInterface, *const CWideString),
//     pub is_safe_to_display_game_info: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub display_tutorial:
//         extern "thiscall" fn(*mut CGameScriptInterface, ETutorialCategory) -> bool,
//     pub is_tutorial_system_enabled: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub give_hero_weapon: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, bool),
//     pub give_hero_object:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, c_long, bool),
//     pub set_weapon_as_heros_active_weapon:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
//     pub give_hero_item: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub give_hero_items_from_container:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool) -> bool,
//     pub take_object_from_hero: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
//     pub give_hero_gold: extern "thiscall" fn(*mut CGameScriptInterface, c_long),
//     pub get_hero_gold: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
//     pub give_hero_experience: extern "thiscall" fn(*mut CGameScriptInterface, c_long),
//     pub set_hero_able_to_gain_experience: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub sheathe_hero_weapons: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub set_hero_will_as_usable: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub set_hero_weapons_as_usable: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub set_weapon_out_crime_enabled: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub set_guards_ignore_crimes: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub remove_all_hero_weapons: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub is_reported_or_unreported_crime_known:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
//     pub confiscate_all_hero_items: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub confiscate_all_hero_weapons: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub confiscate_items_of_type_from_hero:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
//     pub return_all_confiscated_items_to_hero: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub give_hero_tutorial:
//         extern "thiscall" fn(*mut CGameScriptInterface, ETutorialCategory) -> bool,
//     pub make_hero_carry_item_in_hand:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool, bool),
//     pub make_hero_carry_item_in_hand_2:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
//     pub add_tattoo_to_hero: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
//     pub give_hero_ability: extern "thiscall" fn(*mut CGameScriptInterface, EHeroAbility, bool),
//     pub is_player_z_targeting_thing:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
//     pub is_player_creature_blocking: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub is_player_creature_ready_to_fire_projectile_weapon:
//         extern "thiscall" fn(*mut CGameScriptInterface, *mut c_float) -> bool,
//     pub get_player_creature_combat_multiplier:
//         extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
//     pub get_player_creature_combat_multiplier_running_num_hits:
//         extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
//     pub reset_player_creature_combat_multiplier: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub is_player_creature_flourish_enabled:
//         extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub set_player_creature_only_target:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub reset_player_creature_only_target: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub respawn_hero: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub give_hero_morality: extern "thiscall" fn(*mut CGameScriptInterface, c_float),
//     pub get_hero_morality: extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
//     pub get_hero_morality_category: extern "thiscall" fn(*mut CGameScriptInterface) -> EMorality,
//     pub change_hero_morality_due_to_theft: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub change_hero_morality_due_to_picklock: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub give_hero_renown_points: extern "thiscall" fn(*mut CGameScriptInterface, c_long),
//     pub get_hero_renown_level: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
//     pub is_hero_renown_level_full: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub increase_hero_renown_level: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub get_hero_strength_level: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
//     pub get_hero_skill_level: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
//     pub get_hero_will_level: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
//     pub get_hero_stat_level:
//         extern "thiscall" fn(*mut CGameScriptInterface, EHeroTrainableStatType) -> c_long,
//     pub get_hero_stat_max:
//         extern "thiscall" fn(*mut CGameScriptInterface, EHeroTrainableStatType) -> c_long,
//     pub set_hero_age: extern "thiscall" fn(*mut CGameScriptInterface, c_float),
//     pub get_hero_age: extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
//     pub set_hero_as_teenager: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub set_hero_as_apprentice: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub get_distance_hero_can_be_heard_from:
//         extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
//     pub get_hero_rough_experience_level: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
//     pub get_hero_experience_available_to_spend:
//         extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
//     pub get_hero_fatness: extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
//     pub get_hero_scariness: extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
//     pub get_hero_attractiveness: extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
//     pub get_hero_will_energy_level: extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
//     pub set_hero_will_energy_level: extern "thiscall" fn(*mut CGameScriptInterface, c_float),
//     pub set_hero_will_energy_as_able_to_refill:
//         extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub get_number_of_items_of_type_in_inventory:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> c_long,
//     pub is_object_in_things_possession: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CCharString,
//         *const CScriptThing,
//     ) -> bool,
//     pub is_hero_hand_lamp_lit: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub set_hero_hand_lamp_as_lit: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub is_wearing_clothing_item: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *const CCharString,
//     ) -> bool,
//     pub is_hero_naked: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub remove_hero_clothing: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub set_hero_as_wearing: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
//     pub change_hero_hairstyle: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
//     pub remove_hero_hairstyle: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub is_wearing_hairstyle: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *mut CScriptThing,
//         *const CCharString,
//     ) -> bool,
//     pub is_player_carrying_item_of_type:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
//     pub is_player_wielding_weapon: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub is_entity_wielding_weapon:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
//     pub is_entity_wielding_melee_weapon:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
//     pub is_entity_wielding_ranged_weapon:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
//     pub get_previously_wielded_melee_weapon_name:
//         extern "thiscall" fn(*mut CGameScriptInterface, *mut CCharString) -> bool,
//     pub get_previously_wielded_ranged_weapon_name:
//         extern "thiscall" fn(*mut CGameScriptInterface, *mut CCharString) -> bool,
//     pub is_entity_able_to_attack:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
//     pub entity_get_thing_in_primary_slot:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> CScriptThing,
//     pub apply_hero_penalty_for_death: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub give_hero_title: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
//     pub get_hero_title: extern "thiscall" fn(*mut CGameScriptInterface) -> EHeroTitle,
//     pub entity_set_as_marryable:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_set_as_able_to_region_follow_when_married:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_force_marriage_to_hero:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub is_entity_married_to_hero:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
//     pub is_entity_marriable:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
//     pub get_hero_has_married: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub get_hero_has_current_marriage: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub get_hero_has_divorced_marriage: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub get_hero_has_children: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub get_hero_has_murdered_wife: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub is_hero_child: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub cancel_hero_teleport_effects: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub get_number_of_times_hero_has_had_sex:
//         extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
//     pub set_number_of_times_hero_has_had_sex:
//         extern "thiscall" fn(*mut CGameScriptInterface, c_long),
//     pub set_hero_as_having_had_sex: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub set_hero_as_having_had_gay_sex: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub give_thing_hero_reward_item: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *const CCharString,
//         *const CCharString,
//     ),
//     pub give_thing_item_in_hand: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *const CCharString,
//         bool,
//     ),
//     pub give_thing_item_in_slot: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *const CCharString,
//         *const CCharString,
//     ),
//     pub give_hero_expression:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, c_long, bool),
//     pub hero_has_expression:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
//     pub is_hero_performing_expression:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
//     pub is_d_pad_button_held_for_expression:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
//     pub entity_follow_thing: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *const CScriptThing,
//         c_float,
//         bool,
//     ),
//     pub entity_stop_following: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub get_following_entity_list: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *mut CxxVector<CScriptThing>,
//     ) -> bool,
//     pub get_perceiving_hero_entity_list:
//         extern "thiscall" fn(*mut CGameScriptInterface, *mut CxxVector<CScriptThing>) -> bool,
//     pub get_hero_summoned_creatures_list:
//         extern "thiscall" fn(*mut CGameScriptInterface, *mut CxxVector<CScriptThing>) -> bool,
//     pub is_entity_following_hero:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
//     pub entity_set_as_allowed_to_follow_hero:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_set_as_allowed_to_change_region_following_state:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_set_as_responding_to_follow_and_wait_expressions:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_set_as_mirroring_hero_enemy_relations_while_following:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub teleport_all_followers_to_hero_position: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub entity_teleport_to_hero_position:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub send_entity_event: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         EEventType,
//         *const CScriptThing,
//         *const CScriptThing,
//         *mut CThing,
//     ),
//     pub get_water_height_at_position:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const C3DVector) -> c_float,
//     pub is_fishing_spot_enabled:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
//     pub disable_fishing_spot: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub update_fish_weight: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub get_best_fish_weight:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> c_float,
//     pub hero_go_fishing: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub get_hero_fishing_level: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
//     pub hero_go_digging: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub hero_stop_digging: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub hero_play_oracle_minigame: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub is_hero_playing_oracle_minigame: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub has_hero_won_oracle_minigame: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub hero_play_fireheart_minigame: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub hero_quit_fireheart_minigame: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub has_hero_force_quit_fireheart_minigame:
//         extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub get_hero_health: extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
//     pub get_hero_health_max: extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
//     pub get_hero_health_percentage: extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
//     pub get_hero_will_energy: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
//     pub get_hero_will_energy_max: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
//     pub get_hero_will_energy_percentage: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
//     pub change_hero_health_by: extern "thiscall" fn(*mut CGameScriptInterface, c_float, bool, bool),
//     pub set_thing_as_killed: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub get_health: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> c_float,
//     pub modify_thing_health:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_float, bool),
//     pub entity_set_max_health:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_float, bool),
//     pub give_hero_new_quest_objective:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, c_ulong),
//     pub tell_hero_quest_objective_completed:
//         extern "thiscall" fn(*mut CGameScriptInterface, c_ulong),
//     pub tell_hero_quest_objective_failed: extern "thiscall" fn(*mut CGameScriptInterface, c_ulong),
//     pub add_quest_region:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const CCharString),
//     pub set_quest_world_map_offset:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const C2DCoordI),
//     pub is_hero_on_quest: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub hero_receive_message_from_guild_master: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CCharString,
//         *const CCharString,
//         bool,
//         bool,
//     ),
//     pub set_guild_master_messages: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub activate_quest: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
//     pub activate_multiple_quests:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CArray<CCharString>),
//     pub activate_quest_without_loading_resources:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
//     pub activate_multiple_quests_without_loading_resources:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CArray<CCharString>),
//     pub deactivate_quest:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, c_ulong),
//     pub deactivate_quest_later:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, c_ulong),
//     pub prepare_quests_when_final_quest_is_activated:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
//     pub prepare_quests_when_final_quest_is_completed:
//         extern "thiscall" fn(*mut CGameScriptInterface),
//     pub is_quest_active:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
//     pub is_quest_registered:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
//     pub is_quest_completed:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
//     pub is_quest_failed:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
//     pub set_quest_as_completed:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, bool, bool, bool),
//     pub set_quest_as_failed: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CCharString,
//         bool,
//         *const CWideString,
//         bool,
//     ),
//     pub set_quest_as_persistent:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, bool),
//     pub get_exclusive_quest_script_name:
//         extern "thiscall" fn(*mut CGameScriptInterface) -> *const CCharString,
//     pub add_quest_card: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CCharString,
//         *const CCharString,
//         bool,
//         bool,
//     ),
//     pub remove_quest_card_from_guild:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
//     pub remove_quest_card_from_hero:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
//     pub give_hero_quest_card_directly: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CCharString,
//         *const CCharString,
//         bool,
//     ),
//     pub set_quest_card_objective: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CCharString,
//         *const CCharString,
//         *const CCharString,
//         *const CCharString,
//     ),
//     pub set_quest_card_gold_reward:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, c_long),
//     pub set_quest_card_renown_reward:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, c_long),
//     pub remove_all_available_quest_cards_from_guild:
//         extern "thiscall" fn(*mut CGameScriptInterface),
//     pub fail_all_active_quests: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub get_all_active_quest_info: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *mut CxxVector<CCharString>,
//         *mut CxxVector<CCharString>,
//     ),
//     pub add_feat_card: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         c_long,
//         *const CCharString,
//         *const CCharString,
//     ),
//     pub add_boast: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CCharString,
//         c_long,
//         c_long,
//         c_long,
//         bool,
//         *const CCharString,
//         c_long,
//     ),
//     pub remove_boast: extern "thiscall" fn(*mut CGameScriptInterface, c_long, *const CCharString),
//     pub set_boast_as_failed:
//         extern "thiscall" fn(*mut CGameScriptInterface, c_long, *const CCharString),
//     pub set_boast_as_completed:
//         extern "thiscall" fn(*mut CGameScriptInterface, c_long, *const CCharString),
//     pub is_boast_taken:
//         extern "thiscall" fn(*mut CGameScriptInterface, c_long, *const CCharString) -> bool,
//     pub add_log_book_entry: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CWideString,
//         *const CWideString,
//         *const CWideString,
//         ECategory,
//     ),
//     pub kick_off_quest_start_screen:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, bool, bool),
//     pub kick_off_death_screen: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub kick_off_credits_screen: extern "thiscall" fn(*mut CGameScriptInterface, *mut CCharString),
//     pub set_preferred_quick_access_item:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, c_long, c_long),
//     pub get_death_recovery_marker_name:
//         extern "thiscall" fn(*mut CGameScriptInterface) -> CCharString,
//     pub set_death_recovery_marker_name:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
//     pub reset_death_recovery_marker_name_to_default:
//         extern "thiscall" fn(*mut CGameScriptInterface),
//     pub is_to_fail_quest_on_death: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub set_whether_to_fail_quest_on_death: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub reset_whether_to_fail_quest_on_death_to_default:
//         extern "thiscall" fn(*mut CGameScriptInterface),
//     pub get_most_recent_valid_used_target:
//         extern "thiscall" fn(*mut CGameScriptInterface) -> CScriptThing,
//     pub get_most_recent_valid_used_target_name:
//         extern "thiscall" fn(*mut CGameScriptInterface) -> CCharString,
//     pub display_quest_info: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub set_quest_info_name: extern "thiscall" fn(*mut CGameScriptInterface, *const c_char),
//     pub set_quest_info_text: extern "thiscall" fn(*mut CGameScriptInterface, *const c_char),
//     pub add_quest_info_bar: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         c_float,
//         c_float,
//         *const CRGBColour,
//         *const CRGBColour,
//         *const CCharString,
//         *const CCharString,
//         c_float,
//     ) -> c_long,
//     pub add_quest_info_bar_health: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *const CRGBColour,
//         *const CCharString,
//         c_float,
//     ) -> c_long,
//     pub add_quest_info_timer: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CTimer,
//         *const CCharString,
//         c_float,
//     ) -> c_long,
//     pub add_quest_info_counter: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CCharString,
//         c_long,
//         c_float,
//     ) -> c_long,
//     pub add_quest_info_counter_2: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CCounter,
//         *const CCharString,
//         c_float,
//     ) -> c_long,
//     pub add_quest_info_counter_list: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CCharString,
//         c_long,
//         c_float,
//     ) -> c_long,
//     pub add_quest_info_tick:
//         extern "thiscall" fn(*mut CGameScriptInterface, EGameAction, bool, c_float) -> c_long,
//     pub add_quest_info_tick_2: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CCharString,
//         bool,
//         c_float,
//     ) -> c_long,
//     pub update_quest_info_bar:
//         extern "thiscall" fn(*mut CGameScriptInterface, c_long, c_float, c_float, c_float),
//     pub change_quest_info_bar_colour: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         c_long,
//         *const CRGBColour,
//         *const CRGBColour,
//     ),
//     pub update_quest_info_timer: extern "thiscall" fn(*mut CGameScriptInterface, c_long, c_float),
//     pub update_quest_info_counter:
//         extern "thiscall" fn(*mut CGameScriptInterface, c_long, c_long, c_long),
//     pub update_quest_info_counter_list:
//         extern "thiscall" fn(*mut CGameScriptInterface, c_long, c_long, c_long),
//     pub update_quest_info_tick: extern "thiscall" fn(*mut CGameScriptInterface, c_long, bool),
//     pub remove_quest_info_element: extern "thiscall" fn(*mut CGameScriptInterface, c_long),
//     pub remove_all_quest_info_elements: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub display_time: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub display_money_bag: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub display_mini_game_info:
//         extern "thiscall" fn(*mut CGameScriptInterface, bool, EMiniGameType),
//     pub update_mini_game_info_bar: extern "thiscall" fn(*mut CGameScriptInterface, c_float),
//     pub is_entity_pick_pocketable:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
//     pub is_entity_pick_lockable:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
//     pub is_entity_stealable:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
//     pub entity_set_as_pick_pocketed:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub entity_set_as_pick_locked:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub entity_set_as_stolen: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub mini_map_add_marker:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
//     pub mini_map_set_marker_graphic:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
//     pub mini_map_remove_marker:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub mini_map_remove_all_markers: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub mini_map_allow_route_between_regions: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CCharString,
//         *const CCharString,
//         bool,
//     ),
//     pub mini_map_set_as_enabled: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub entity_set_as_hidden_on_mini_map:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub set_hud_enabled: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub text_entry_exists:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
//     pub get_valid_text_entry_name_with_attitude: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *const CCharString,
//     ) -> CCharString,
//     pub set_thing_has_information:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool, bool, bool),
//     pub clear_thing_has_information:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub entity_set_will_be_using_narrator:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
//     pub entity_reset_as_pure_ai_narrator:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub add_new_conversation:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool, bool) -> c_long,
//     pub add_person_to_conversation:
//         extern "thiscall" fn(*mut CGameScriptInterface, c_long, *const CScriptThing),
//     pub add_line_to_conversation: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         c_long,
//         *const CCharString,
//         bool,
//         *const CScriptThing,
//         *const CScriptThing,
//     ),
//     pub remove_conversation: extern "thiscall" fn(*mut CGameScriptInterface, c_long, bool),
//     pub is_conversation_active: extern "thiscall" fn(*mut CGameScriptInterface, c_long) -> bool,
//     pub play_avi_movie: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
//     pub start_movie_sequence: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CCharString,
//         *mut CScriptGameResourceObjectMovieBase,
//     ),
//     pub fix_movie_sequence_camera: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub fade_screen_out_until_next_call_to_fade_screen_in:
//         extern "thiscall" fn(*mut CGameScriptInterface, c_float, c_float),
//     pub fade_screen_out:
//         extern "thiscall" fn(*mut CGameScriptInterface, c_float, c_float, CRGBColour) -> bool,
//     pub fade_screen_in: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub is_screen_fading_out: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub pause: extern "thiscall" fn(*mut CGameScriptInterface, c_float),
//     pub end_cut_fade: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub pause_all_non_scripted_entities: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub pause_all_entities: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub set_allow_screen_fading_on_next_region_change:
//         extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub set_allow_screen_fading_if_already_faded:
//         extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub set_to_keep_hero_abilities_during_cutscenes:
//         extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub set_to_display_tutorials_during_cutscenes:
//         extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub set_cutscene_mode: extern "thiscall" fn(*mut CGameScriptInterface, bool, bool),
//     pub is_in_cutscene: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub set_cutscene_skippable: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub set_cutscene_skippable_while_paused: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub set_cutscene_action_mode:
//         extern "thiscall" fn(*mut CGameScriptInterface, bool, *const CCharString),
//     pub preload_new_scene: extern "thiscall" fn(*mut CGameScriptInterface, c_float),
//     pub start_progress_display: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub stop_progress_display: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub set_screen_messages_enabled: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub is_hero_controlled_by_player: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub is_in_movie_sequence: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub cancel_abilities_for_cutscene: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub resume_abilities_for_cutscene: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub cancel_using_ability: extern "thiscall" fn(*mut CGameScriptInterface, EHeroAbility),
//     pub set_ability_availability:
//         extern "thiscall" fn(*mut CGameScriptInterface, EHeroAbility, bool),
//     pub set_environmental_effects_always_update:
//         extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub set_dead_creatures_and_experience_orbs_and_drop_bags_as_hidden:
//         extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub remove_dead_creature: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
//     pub camera_set_camera_preload_flag: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub camera_circle_around_thing: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *const C3DVector,
//         c_float,
//     ),
//     pub camera_circle_around_pos: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const C3DVector,
//         *const C3DVector,
//         c_float,
//     ),
//     pub camera_move_to_pos_and_look_at_pos: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const C3DVector,
//         *const C3DVector,
//         c_float,
//     ),
//     pub camera_move_to_pos_and_look_at_thing: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const C3DVector,
//         *const CScriptThing,
//         c_float,
//     ),
//     pub camera_move_between_looking_at: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const C3DVector,
//         *const C3DVector,
//         *const CScriptThing,
//         c_float,
//         c_float,
//     ),
//     pub camera_move_between_looking_at_2: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const C3DVector,
//         *const C3DVector,
//         *const C3DVector,
//         c_float,
//         c_float,
//     ),
//     pub camera_move_between_look_from_and_look_to: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const C3DVector,
//         *const C3DVector,
//         *const C3DVector,
//         *const C3DVector,
//         c_float,
//     ),
//     pub camera_use_camera_point: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *const C3DVector,
//         *const CRightHandedSet,
//         c_float,
//         c_long,
//         c_long,
//     ),
//     pub camera_use_camera_point_2: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CCharString,
//         *const C3DVector,
//         *const CRightHandedSet,
//         c_float,
//         c_long,
//         c_long,
//     ),
//     pub camera_use_camera_point_3: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *const CScriptThing,
//         c_float,
//         c_long,
//         c_long,
//     ),
//     pub camera_use_camera_point_4: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CCharString,
//         *const CScriptThing,
//         c_float,
//         c_long,
//         c_long,
//     ),
//     pub camera_do_conversation: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *const CScriptThing,
//         ECameraOp,
//         bool,
//     ),
//     pub camera_default: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub camera_reset_to_view_behind_hero: extern "thiscall" fn(*mut CGameScriptInterface, c_float),
//     pub is_camera_in_scripted_mode: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub camera_use_screen_effect:
//         extern "thiscall" fn(*mut CGameScriptInterface, c_float, c_float, c_float),
//     pub camera_cancel_screen_effect: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub is_camera_pos_on_screen:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const C3DVector) -> bool,
//     pub get_game_angle_xy: extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
//     pub camera_earthquake_intensity_at_pos:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const C3DVector, c_float, c_float),
//     pub camera_shake: extern "thiscall" fn(*mut CGameScriptInterface, c_float, c_float),
//     pub open_door: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub close_door: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub open_house_doors: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub close_house_doors: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub jam_door: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub set_door_trigger_type:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, EDoorTriggerType),
//     pub override_automatic_house_locking:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub set_house_owned_by_player:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool, bool),
//     pub set_buyable_house_as_scripted:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub is_chest_open: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
//     pub open_chest:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool) -> bool,
//     pub close_chest: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub get_number_of_keys_needed_to_unlock_chest: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *mut CCharString,
//     ) -> c_long,
//     pub display_locked_chest_message:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub set_trophy_as_mountable:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub set_village_limbo:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub set_creature_not_reload:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub is_sleeping_time:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
//     pub enable_guards: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub enable_villager_def_types: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         bool,
//         *const CCharString,
//     ),
//     pub try_to_respawn_def_named: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *const CCharString,
//         *const C3DVector,
//     ) -> CScriptThing,
//     pub clear_hero_enemy_of_guards:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub set_thing_as_usable:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub set_thing_home_building:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing),
//     pub set_village_attitude: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         EScriptVillageAttitude,
//     ),
//     pub add_crime_committed: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         ECrime,
//         bool,
//         *const CScriptThing,
//         *const CScriptThing,
//         EOpinionPostDeedType,
//     ),
//     pub give_thing_best_enemy_target:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing),
//     pub clear_thing_best_enemy_target:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub entity_set_in_limbo:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool, bool),
//     pub is_entity_in_limbo:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
//     pub entity_get_shot_strike_pos: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *mut C3DVector,
//     ) -> bool,
//     pub entity_set_negate_all_hits:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_set_evade_all_hits:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_set_able_to_be_engaged_in_combat:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_set_always_block_attacks_from_thing: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *const CScriptThing,
//         bool,
//     ),
//     pub entity_set_attack_thing_immediately: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *const CScriptThing,
//         bool,
//         bool,
//     ),
//     pub entity_set_combat_type:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
//     pub entity_reset_combat_type_to_default:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub entity_set_max_number_of_attackers:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_long),
//     pub entity_clear_max_number_of_attackers:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub entity_attach_to_script:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
//     pub entity_set_combat_ability:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_float),
//     pub entity_set_ranged_target:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing),
//     pub entity_clear_ranged_target:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub entity_set_targetable:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_set_targeting_type:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, ETargetingType),
//     pub entity_set_targeting_valid_target_without_los:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_teleport_to_thing: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *const CScriptThing,
//         bool,
//     ),
//     pub entity_teleport_to_position: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *const C3DVector,
//         c_float,
//         bool,
//         bool,
//     ),
//     pub entity_set_facing_angle:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_float, bool),
//     pub entity_set_facing_angle_towards_thing: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *const CScriptThing,
//         bool,
//     ),
//     pub entity_set_perception_variables: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         c_float,
//         c_float,
//         c_float,
//     ),
//     pub set_thing_persistent:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_set_thing_as_wanting_money:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_long),
//     pub entity_set_appearance_morph_seed:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_long),
//     pub set_entity_as_region_following: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *const CScriptThing,
//         bool,
//     ),
//     pub set_entity_as_following_hero_through_teleporters:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_set_appearance_seed:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_ulong),
//     pub entity_get_appearance_seed:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *mut c_ulong),
//     pub entity_set_as_for_sale:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_set_stock_item_price:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_long),
//     pub entity_get_stock_item_price:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> c_long,
//     pub entity_play_object_animation: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *const CCharString,
//         bool,
//     ),
//     pub entity_set_max_running_speed:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_float),
//     pub entity_set_max_walking_speed:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_float),
//     pub entity_reset_max_running_speed:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub entity_reset_max_walking_speed:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub entity_attach_to_village:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing),
//     pub entity_set_as_sitting_on_floor:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_set_as_scared:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_set_as_drunk:
//         extern "thiscall" fn(*mut CGameScriptInterface, *mut CScriptThing, bool),
//     pub entity_set_as_having_bound_hands:
//         extern "thiscall" fn(*mut CGameScriptInterface, *mut CScriptThing, bool),
//     pub entity_set_as_remove_all_movement_blocking_modes:
//         extern "thiscall" fn(*mut CGameScriptInterface, *mut CScriptThing),
//     pub entity_force_to_look_at_thing:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing),
//     pub entity_force_to_look_at_camera:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_force_to_look_at_nothing:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub entity_reset_force_to_look_at:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub entity_set_shot_accuracy_percentage:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_long),
//     pub entity_get_standing_on_thing:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> CScriptThing,
//     pub entity_get_standing_inside_building:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> CScriptThing,
//     pub entity_drop_generic_box:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub entity_sheathe_weapons:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_unsheathe_weapons:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_unsheathe_melee_weapon:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_unsheathe_ranged_weapon:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_set_alpha:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_float, bool),
//     pub entity_set_as_drawable:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_set_cutscene_behaviour:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, ECutsceneBehaviour),
//     pub entity_get_sex:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> ESex,
//     pub entity_set_as_able_to_walk_through_solid_objects:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_set_as_respond_to_hit:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_set_as_damageable:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_set_as_killable:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool, bool),
//     pub entity_set_as_locked:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_decapitate: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub entity_give_gold:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_long),
//     pub entity_set_allow_boss_phase_changes:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_get_boss_phase:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> c_long,
//     pub entity_set_boss_phase:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_long),
//     pub entity_reset_creature_mode:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub entity_set_as_receiving_events:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool) -> bool,
//     pub entity_set_as_to_add_to_combo_multiplier_when_hit:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_set_as_to_add_to_stat_changes_when_hit:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_leave_combat_stance:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub entity_set_as_use_movement_in_actions:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_set_as_displaying_emote_icon:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_set_as_collidable_to_things:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_enable_gravity:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_set_light_as_on:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_fade_out:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_float),
//     pub entity_fade_in:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_float),
//     pub entity_begin_loading_animation:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
//     pub entity_begin_loading_basic_animations:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub entity_cast_force_push:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool) -> bool,
//     pub entity_cast_lightning_at_target:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing),
//     pub begin_loading_mesh: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
//     pub entity_will_teleport_to_area: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         C3DVector,
//         c_float,
//         c_float,
//     ) -> bool,
//     pub entity_start_screamer_super_attack_thing:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing),
//     pub entity_end_screamer_super_attack_thing:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub set_hero_guide_to_show_quest_cards_when_spoken_to:
//         extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub set_light_colour:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CRGBColour),
//     pub creature_generator_set_family:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
//     pub creature_generator_trigger:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub creature_generator_set_always_create_creatures_on_trigger:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub creature_generator_is_depleted:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
//     pub creature_generator_is_destroyed:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
//     pub creature_generator_set_generated_creature_script_name:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
//     pub creature_generator_set_num_triggers:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_long),
//     pub creature_generator_get_num_generated_creatures:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> c_long,
//     pub creature_generator_are_all_creatures_alive:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
//     pub creature_generator_add_triggerer:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing),
//     pub creature_generator_remove_triggerer:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing),
//     pub set_creature_generator_enabled:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub set_creature_generators_enabled:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, bool),
//     pub set_creature_generators_enabled_during_script:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, bool),
//     pub set_creature_generators_creature_group_as_enabled:
//         extern "thiscall" fn(*mut CGameScriptInterface, ECreatureGroup, bool),
//     pub is_creature_generation_enabled_for_region:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
//     pub is_creature_flying:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
//     pub set_teleporter_as_active:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub is_teleporter_active:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
//     pub set_teleporting_as_active: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub is_teleporting_active: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub set_region_exit_as_active:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub set_region_entrance_as_active:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub set_region_text_display_as_active: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub set_hero_sleeping_as_enabled: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub is_hero_sleeping_enabled: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub set_experience_spending_as_enabled: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub set_morality_changing_as_enabled: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub set_summoner_death_explosion_affects_hero:
//         extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub get_nearest_enabled_digging_spot:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> CScriptThing,
//     pub is_digging_spot_enabled:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
//     pub is_digging_spot_hidden:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
//     pub set_digging_spot_as_hidden:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub check_for_camera_message:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
//     pub wait_for_camera_message:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
//     pub set_thing_as_conscious: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         bool,
//         *const CCharString,
//     ),
//     pub set_fire_to_thing: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub extinguish_fires_on_thing:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub is_thing_on_fire:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
//     pub add_item_to_container:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
//     pub remove_item_from_container:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
//     pub entity_set_death_container_as_enabled:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub get_item_def_names_from_container: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *mut CxxVector<CCharString>,
//     ),
//     pub set_creature_brain:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
//     pub entity_set_stategroup_enabled: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *const CCharString,
//         bool,
//     ),
//     pub entity_set_all_stategroups_enabled:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_set_combat_enabled:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_set_sleep_enabled:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_set_opinion_reactions_enabled:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_set_deed_reactions_enabled:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub debug_get_all_text_entries_for_targeted_thing:
//         extern "thiscall" fn(*mut CGameScriptInterface, *mut CxxSet<c_ulong>),
//     pub entity_set_thing_as_enemy_of_thing:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing),
//     pub entity_unset_thing_as_enemy_of_thing:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing),
//     pub entity_set_thing_as_ally_of_thing:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing),
//     pub entity_unset_thing_as_ally_of_thing:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing),
//     pub entity_set_in_faction:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
//     pub set_faction_as_allied_to_faction:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const CCharString),
//     pub set_faction_as_neutral_to_faction:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const CCharString),
//     pub set_faction_as_enemy_to_faction:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const CCharString),
//     pub are_entities_enemies: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *const CScriptThing,
//     ) -> bool,
//     pub get_next_in_opinion_attitude_graph: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         EOpinionAttitudeType,
//     ) -> EOpinionAttitudeType,
//     pub get_opinion_attitude_as_string:
//         extern "thiscall" fn(*mut CGameScriptInterface, EOpinionAttitudeType, *mut CCharString),
//     pub entity_get_opinion_attitude_to_player: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//     ) -> EOpinionAttitudeType,
//     pub entity_get_opinion_attitude_to_player_as_string:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *mut CCharString),
//     pub entity_get_opinion_of_player:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, EOpinion) -> c_float,
//     pub entity_set_opinion_reaction_mask:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
//     pub entity_set_opinion_reaction_mask_2:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_long),
//     pub entity_set_opinion_deed_mask:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
//     pub entity_set_opinion_deed_mask_2:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_long),
//     pub entity_set_opinion_deed_type_enabled: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         EOpinionDeedType,
//         bool,
//     ),
//     pub entity_set_opinion_attitude_enabled: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         EOpinionAttitudeType,
//         bool,
//     ),
//     pub entity_set_opinion_reaction_enabled: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         EOpinionReactionType,
//         bool,
//     ),
//     pub entity_set_personality_override:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_long),
//     pub entity_set_personality_override_2:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
//     pub entity_clear_personality_override:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub entity_set_as_opinion_source:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_long),
//     pub entity_set_as_opinion_source_2:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
//     pub entity_unset_as_opinion_source:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub opinion_source_set_as_exclusive:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub opinion_source_set_as_attention_grabbing:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub entity_post_opinion_deed_to_all:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, EOpinionDeedType),
//     pub entity_post_opinion_deed_to_recipient: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         EOpinionDeedType,
//         *const CScriptThing,
//     ),
//     pub entity_post_opinion_deed_to_recipient_village: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         EOpinionDeedType,
//         *const CScriptThing,
//     ),
//     pub entity_post_opinion_deed_keep_searching_for_witnesses: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         EOpinionDeedType,
//         *const CScriptThing,
//     ) -> c_long,
//     pub remove_opinion_deed_still_searching_for_witnesses:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_long),
//     pub is_deed_witnessed: extern "thiscall" fn(*mut CGameScriptInterface, c_long) -> bool,
//     pub can_thing_be_seen_by_other_thing: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *const CScriptThing,
//     ) -> bool,
//     pub can_thing_be_nearly_seen_by_other_thing: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *const CScriptThing,
//     ) -> bool,
//     pub can_thing_be_smelled_by_other_thing: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *const CScriptThing,
//     ) -> bool,
//     pub can_thing_be_heard_by_other_thing: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *const CScriptThing,
//     ) -> bool,
//     pub is_thing_aware_of_other_thing_in_any_way: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *const CScriptThing,
//     ) -> bool,
//     pub entity_set_as_aware_of_thing:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CScriptThing),
//     pub entity_set_sound_radius:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_float),
//     pub entity_set_smell_radius:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_float),
//     pub entity_set_sight_radius:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_float),
//     pub entity_set_extended_sight_radius:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_float),
//     pub entity_set_give_up_chase_radius:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, c_float),
//     pub entity_get_hearing_radius:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> c_float,
//     pub manually_trigger_trap:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
//     pub manually_reset_trap:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
//     pub set_time_of_day: extern "thiscall" fn(*mut CGameScriptInterface, c_float),
//     pub get_time_of_day: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
//     pub set_time_as_stopped: extern "thiscall" fn(*mut CGameScriptInterface, bool, *mut c_long),
//     pub fast_forward_time_to: extern "thiscall" fn(*mut CGameScriptInterface, c_float, c_float),
//     pub is_time_of_day_between:
//         extern "thiscall" fn(*mut CGameScriptInterface, c_long, c_long) -> bool,
//     pub get_day_of_week: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
//     pub get_day_count: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
//     pub get_world_frame: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
//     pub get_constant_fps: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
//     pub get_active_quest_name: extern "thiscall" fn(*mut CGameScriptInterface) -> CCharString,
//     pub transition_to_theme:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, c_float),
//     pub reset_to_default_theme: extern "thiscall" fn(*mut CGameScriptInterface, c_float),
//     pub transition_to_theme_all_internals:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, c_float),
//     pub reset_to_default_theme_all_internals:
//         extern "thiscall" fn(*mut CGameScriptInterface, c_float),
//     pub transition_to_theme_externals:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, c_float),
//     pub reset_to_default_theme_externals: extern "thiscall" fn(*mut CGameScriptInterface, c_float),
//     pub set_environment_theme_weight_all_channels:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, c_float),
//     pub set_environment_theme_weight_all_internals:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, c_float),
//     pub set_environment_theme_weight_externals:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, c_float),
//     pub set_sound_themes_as_enabled_for_region:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, bool),
//     pub set_all_sounds_as_muted: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub radial_blur_fade_to: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         c_float,
//         c_float,
//         c_float,
//         c_float,
//         c_float,
//         c_float,
//         c_float,
//     ) -> *mut c_void,
//     pub radial_blur_fade_to_2: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         c_float,
//         C3DVector,
//         c_float,
//         C3DVector,
//         c_float,
//         c_float,
//         c_float,
//     ) -> *mut c_void,
//     pub radial_blur_fade_out: extern "thiscall" fn(*mut CGameScriptInterface, c_float, *mut c_void),
//     pub is_radial_blur_fade_active: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub cancel_radial_blur_fade: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub radial_blur_set_center_world_pos:
//         extern "thiscall" fn(*mut CGameScriptInterface, *mut c_void, *const C3DVector),
//     pub displacement_monochrome_effect_colour_fade_to:
//         extern "thiscall" fn(*mut CGameScriptInterface, c_float, *const CRGBFloatColour) -> c_void,
//     pub displacement_monochrome_effect_colour_fade_out:
//         extern "thiscall" fn(*mut CGameScriptInterface, c_float, *mut c_void),
//     pub screen_filter_fade_to: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         c_float,
//         c_float,
//         c_float,
//         c_float,
//         c_float,
//         *const CRGBFloatColour,
//         *const CxxVector<CScreenFilterSThingByPass>,
//     ) -> c_void,
//     pub screen_filter_fade_out:
//         extern "thiscall" fn(*mut CGameScriptInterface, c_float, *mut c_void),
//     pub set_thing_and_carried_items_not_affected_by_screen_filter:
//         extern "thiscall" fn(*mut CGameScriptInterface, *mut CScriptThing, *mut c_void),
//     pub un_set_thing_and_carried_items_not_affected_by_screen_filter:
//         extern "thiscall" fn(*mut CGameScriptInterface, *mut CScriptThing),
//     pub is_gift_romantic:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
//     pub is_gift_friendly:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
//     pub is_gift_offensive:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> bool,
//     pub is_thing_a_bed:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
//     pub is_thing_a_chest:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
//     pub is_thing_a_door:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
//     pub is_thing_smashable:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
//     pub is_thing_searchable:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
//     pub apply_script_brush: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
//     pub enable_decals: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub play_criteria_sound_on_thing: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *const CCharString,
//     ) -> c_ulong,
//     pub play_sound_on_thing: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *const CCharString,
//     ) -> c_ulong,
//     pub is_sound_playing: extern "thiscall" fn(*mut CGameScriptInterface, c_ulong) -> bool,
//     pub stop_sound: extern "thiscall" fn(*mut CGameScriptInterface, c_ulong),
//     pub play_sound_at_pos: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const C3DVector,
//         *const CCharString,
//     ) -> c_ulong,
//     pub play_2d_sound:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> c_ulong,
//     pub enable_sounds: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub override_music: extern "thiscall" fn(*mut CGameScriptInterface, EMusicSetType, bool, bool),
//     pub stop_override_music: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub cache_music_set: extern "thiscall" fn(*mut CGameScriptInterface, EMusicSetType),
//     pub enable_danger_music: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub is_danger_music_enabled: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub start_countdown_timer: extern "thiscall" fn(*mut CGameScriptInterface, c_float),
//     pub get_countdown_timer: extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
//     pub auto_save_check_point: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub auto_save_quest_start: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub auto_save: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub set_saving_as_enabled: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub is_saving_enabled: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub set_save_game_marker_pos: extern "thiscall" fn(*mut CGameScriptInterface, *const C3DVector),
//     pub reset_to_front_end: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub set_guild_seal_recall_location:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const C3DVector, c_float),
//     pub get_guild_seal_recall_pos: extern "thiscall" fn(*mut CGameScriptInterface) -> C3DVector,
//     pub get_guild_seal_recall_angle_xy: extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
//     pub set_readable_object_text:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CWideString),
//     pub set_readable_object_text_tag:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
//     pub get_formatted_string: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CCharString,
//         *const CxxVector<CWideString>,
//     ) -> CWideString,
//     pub get_text_string:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString) -> CWideString,
//     pub add_rumour_category: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
//     pub add_new_rumour_to_category:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const CCharString),
//     pub remove_rumour_category: extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString),
//     pub set_category_activity:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, bool),
//     pub add_gossip_village:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const CCharString),
//     pub add_gossip_faction_to_category:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, *const CCharString),
//     pub set_is_gossip_for_player:
//         extern "thiscall" fn(*mut CGameScriptInterface, CCharString, bool),
//     pub set_is_gossip_for_player_2:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CCharString, bool),
//     pub update_online_score_archery: extern "thiscall" fn(*mut CGameScriptInterface, c_float),
//     pub update_online_score_chicken_kick: extern "thiscall" fn(*mut CGameScriptInterface, c_float),
//     pub update_online_score_chapel_or_temple:
//         extern "thiscall" fn(*mut CGameScriptInterface, c_float),
//     pub update_online_score_fishing_compo: extern "thiscall" fn(*mut CGameScriptInterface, c_float),
//     pub update_score_fishing_competition: extern "thiscall" fn(*mut CGameScriptInterface, c_float),
//     pub get_best_time_pairs: extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
//     pub get_best_time_sorting: extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
//     pub get_best_score_blackjack: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
//     pub get_best_score_coin_golf_oak_vale:
//         extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
//     pub get_best_score_coin_golf_snow_spire:
//         extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
//     pub get_best_score_shove_ha_penny: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
//     pub get_best_time_guess_the_addition:
//         extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
//     pub is_hero_in_tavern_game: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub get_num_houses_owned: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
//     pub start_sneaking: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub get_steal_duration:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> c_long,
//     pub set_useable_by_hero:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub set_owned_by_hero:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub set_tavern_table_available_for_use:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub set_is_thing_turncoatable:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub set_is_thing_force_pushable:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub set_is_thing_lightningable:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub set_is_thing_epic_spellable:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub is_thing_turncoated:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing) -> bool,
//     pub add_creature_scripted_mode:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, *const CCharString),
//     pub remove_creature_scripted_mode:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub force_ships_visible: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub get_sleeping_position_and_orientation_from_bed: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         *const CScriptThing,
//         *const CScriptThing,
//         *mut C3DVector,
//         *mut C3DVector,
//     ) -> bool,
//     pub set_bed_availability:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool),
//     pub repopulate_village: extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing),
//     pub smash_all_windows_within_radius_of_point:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const C3DVector, c_float),
//     pub set_residency:
//         extern "thiscall" fn(*mut CGameScriptInterface, *const CScriptThing, bool) -> CScriptThing,
//     pub set_thanking_phrase: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, c_ulong),
//     pub get_thanking_phrase:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> c_ulong,
//     pub reset_thanking_phrase: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
//     pub set_ignoring_phrase: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, c_ulong),
//     pub get_ignoring_phrase:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> c_ulong,
//     pub reset_ignoring_phrase: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
//     pub set_wander_centre_point:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, C3DVector),
//     pub get_wander_centre_point:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> C3DVector,
//     pub reset_wander_centre_point: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
//     pub set_wander_min_distance:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, c_float),
//     pub get_wander_min_distance:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> c_float,
//     pub reset_wander_min_distance: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
//     pub set_wander_max_distance:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, c_float),
//     pub get_wander_max_distance:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> c_float,
//     pub reset_wander_max_distance: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
//     pub set_gossip_counter: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, c_long),
//     pub get_gossip_counter: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> c_long,
//     pub reset_gossip_counter: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
//     pub set_max_gossip_phrase:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, c_long),
//     pub get_max_gossip_phrase:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> c_long,
//     pub reset_max_gossip_phrase: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
//     pub set_warning_phrase: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, c_ulong),
//     pub get_warning_phrase:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> c_ulong,
//     pub reset_warning_phrase: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
//     pub set_beer_request_phrase:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, c_ulong),
//     pub get_beer_request_phrase:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> c_ulong,
//     pub reset_beer_request_phrase: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
//     pub set_scripting_state_group:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, EScriptingStateGroups),
//     pub get_scripting_state_group:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> EScriptingStateGroups,
//     pub reset_scripting_state_group: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
//     pub set_max_hero_reaction_distance:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, c_float),
//     pub get_max_hero_reaction_distance:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> c_float,
//     pub reset_max_hero_reaction_distance:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
//     pub set_action_frequency: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, c_long),
//     pub get_action_frequency:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> c_long,
//     pub reset_action_frequency: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
//     pub set_action_frequency_variation:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, c_float),
//     pub get_action_frequency_variation:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> c_float,
//     pub reset_action_frequency_variation:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
//     pub set_action: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, CCharString),
//     pub get_action: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> CCharString,
//     pub reset_action: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
//     pub set_face_hero_for_action:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, bool),
//     pub get_face_hero_for_action:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> bool,
//     pub reset_face_hero_for_action: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
//     pub set_target_name: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, CCharString),
//     pub get_target_name:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> CCharString,
//     pub reset_target_name: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
//     pub set_follow_distance: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, c_float),
//     pub get_follow_distance:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> c_float,
//     pub reset_follow_distance: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
//     pub set_attack_hero_on_sight:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, bool),
//     pub get_attack_hero_on_sight:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> bool,
//     pub reset_attack_hero_on_sight: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
//     pub set_time_to_spend_harassing_hero:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, c_long),
//     pub get_time_to_spend_harassing_hero:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> c_long,
//     pub reset_time_to_spend_harassing_hero:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
//     pub set_combat_nearby_enemy_fleeing_break_off_range:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, c_float),
//     pub get_combat_nearby_enemy_fleeing_break_off_range:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> c_float,
//     pub reset_combat_nearby_enemy_fleeing_break_off_range:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
//     pub set_combat_nearby_break_off_range:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, c_float),
//     pub get_combat_nearby_break_off_range:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> c_float,
//     pub reset_combat_nearby_break_off_range:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
//     pub set_steal_stealable_items:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, bool),
//     pub get_steal_stealable_items:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> bool,
//     pub reset_steal_stealable_items: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
//     pub set_recover_stealable_items:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, bool),
//     pub get_recover_stealable_items:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> bool,
//     pub reset_recover_stealable_items:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
//     pub set_take_stealable_item_to_random_destination:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, bool),
//     pub get_take_stealable_item_to_random_destination:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> bool,
//     pub reset_take_stealable_item_to_random_destination:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
//     pub set_kill_self_and_stealable_item_after_reaching_destination:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, bool),
//     pub get_kill_self_and_stealable_item_after_reaching_destination:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> bool,
//     pub reset_kill_self_and_stealable_item_after_reaching_destination:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
//     pub set_allowed_to_follow: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, bool),
//     pub get_allowed_to_follow:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> bool,
//     pub reset_allowed_to_follow: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
//     pub set_table_name: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, CCharString),
//     pub get_table_name:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> CCharString,
//     pub reset_table_name: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
//     pub set_seat_name: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, CCharString),
//     pub get_seat_name: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> CCharString,
//     pub reset_seat_name: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
//     pub set_disable_head_looking:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, bool),
//     pub get_disable_head_looking:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> bool,
//     pub reset_disable_head_looking: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
//     pub set_is_pushable_by_hero:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, bool),
//     pub get_is_pushable_by_hero:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> bool,
//     pub reset_is_pushable_by_hero: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
//     pub set_look_for_finite_time:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, bool),
//     pub get_look_for_finite_time:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> bool,
//     pub reset_look_for_finite_time: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
//     pub set_avoid_region_exits: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, bool),
//     pub get_avoid_region_exits:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> bool,
//     pub reset_avoid_region_exits: extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
//     pub set_targeting_distance_offset:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing, c_float),
//     pub get_targeting_distance_offset:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing) -> c_float,
//     pub reset_targeting_distance_offset:
//         extern "thiscall" fn(*mut CGameScriptInterface, CScriptThing),
//     pub set_player_using_melee_dummies: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub get_player_using_melee_dummies: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub set_player_using_ranged_dummies: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub get_player_using_ranged_dummies: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub set_player_using_will_dummies: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub get_player_using_will_dummies: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub set_cheap_head_looking: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub get_cheap_head_looking: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub set_quit_tavern_game: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub get_quit_tavern_game: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub set_prize_tavern_table: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub get_prize_tavern_table: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub set_betting_active: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub get_betting_active: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub set_betting_accept: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub get_betting_accept: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub set_betting_amount: extern "thiscall" fn(*mut CGameScriptInterface, c_long),
//     pub get_betting_amount: extern "thiscall" fn(*mut CGameScriptInterface) -> c_long,
//     pub set_count_bet_money_down: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub get_count_bet_money_down: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub set_spot_the_addition_beaten: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub get_spot_the_addition_beaten: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub set_global_targeting_distance_offset:
//         extern "thiscall" fn(*mut CGameScriptInterface, c_float),
//     pub get_global_targeting_distance_offset:
//         extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
//     pub set_trading_price_mult: extern "thiscall" fn(*mut CGameScriptInterface, c_float),
//     pub get_trading_price_mult: extern "thiscall" fn(*mut CGameScriptInterface) -> c_float,
//     pub set_boasting_enabled: extern "thiscall" fn(*mut CGameScriptInterface, bool),
//     pub get_boasting_enabled: extern "thiscall" fn(*mut CGameScriptInterface) -> bool,
//     pub set_active_gossip_categories:
//         extern "thiscall" fn(*mut CGameScriptInterface, CCharString, bool),
//     pub get_active_gossip_categories:
//         extern "thiscall" fn(*mut CGameScriptInterface) -> *const CxxMap<CCharString, bool>,
//     pub get_active_gossip_categories_2:
//         extern "thiscall" fn(*mut CGameScriptInterface, CCharString) -> *mut bool,
//     pub get_active_gossip_categories_size: extern "thiscall" fn(*mut CGameScriptInterface) -> i32,
//     pub clear_active_gossip_categories: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub get_is_gossip_for_player:
//         extern "thiscall" fn(*mut CGameScriptInterface) -> *const CxxMap<CCharString, bool>,
//     pub get_is_gossip_for_player_2:
//         extern "thiscall" fn(*mut CGameScriptInterface, CCharString) -> *mut bool,
//     pub get_is_gossip_for_player_size: extern "thiscall" fn(*mut CGameScriptInterface) -> i32,
//     pub clear_is_gossip_for_player: extern "thiscall" fn(*mut CGameScriptInterface),
//     pub set_gossip:
//         extern "thiscall" fn(*mut CGameScriptInterface, CCharString, CCharString, c_long),
//     pub get_gossip: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         CCharString,
//     ) -> *const CxxVector<CCharString>,
//     pub get_gossip_2:
//         extern "thiscall" fn(*mut CGameScriptInterface, CCharString, c_long) -> CCharString,
//     pub get_gossip_size: extern "thiscall" fn(*mut CGameScriptInterface, CCharString) -> i32,
//     pub clear_gossip: extern "thiscall" fn(*mut CGameScriptInterface, CCharString),
//     pub remove_gossip: extern "thiscall" fn(*mut CGameScriptInterface, CCharString),
//     pub add_gossip: extern "thiscall" fn(*mut CGameScriptInterface, CCharString),
//     pub add_gossip_2: extern "thiscall" fn(*mut CGameScriptInterface, CCharString, CCharString),
//     pub set_gossip_villages:
//         extern "thiscall" fn(*mut CGameScriptInterface, CCharString, CCharString, c_long),
//     pub get_gossip_villages: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         CCharString,
//     ) -> *const CxxVector<CCharString>,
//     pub get_gossip_villages_2:
//         extern "thiscall" fn(*mut CGameScriptInterface, CCharString, c_long) -> CCharString,
//     pub get_gossip_villages_size:
//         extern "thiscall" fn(*mut CGameScriptInterface, CCharString) -> i32,
//     pub clear_gossip_villages: extern "thiscall" fn(*mut CGameScriptInterface, CCharString),
//     pub remove_gossip_villages: extern "thiscall" fn(*mut CGameScriptInterface, CCharString),
//     pub add_gossip_villages: extern "thiscall" fn(*mut CGameScriptInterface, CCharString),
//     pub add_gossip_villages_2:
//         extern "thiscall" fn(*mut CGameScriptInterface, CCharString, CCharString),
//     pub set_gossip_factions:
//         extern "thiscall" fn(*mut CGameScriptInterface, CCharString, CCharString, c_long),
//     pub get_gossip_factions: extern "thiscall" fn(
//         *mut CGameScriptInterface,
//         CCharString,
//     ) -> *const CxxVector<CCharString>,
//     pub get_gossip_factions_2:
//         extern "thiscall" fn(*mut CGameScriptInterface, CCharString, c_long) -> CCharString,
//     pub get_gossip_factions_size:
//         extern "thiscall" fn(*mut CGameScriptInterface, CCharString) -> i32,
//     pub clear_gossip_factions: extern "thiscall" fn(*mut CGameScriptInterface, CCharString),
//     pub remove_gossip_factions: extern "thiscall" fn(*mut CGameScriptInterface, CCharString),
//     pub add_gossip_factions: extern "thiscall" fn(*mut CGameScriptInterface, CCharString),
//     pub add_gossip_factions_2:
//         extern "thiscall" fn(*mut CGameScriptInterface, CCharString, CCharString),
//     pub c_game_script_interface_destructor: extern "thiscall" fn(*mut CGameScriptInterface),
// }
// #[derive(Debug)]
// #[repr(C)]
// pub struct CGameScriptInterfaceBase {}
// #[derive(Debug)]
// #[repr(C)]
// pub struct CGameTimeManager {}
// /// TODO: This is only used in a couple methods so I've left it empty for now.
// #[derive(Debug)]
// #[repr(C)]
// pub struct CGenericVar {}
// /// TODO: This is behind a pointer so I've left it empty for now.
// #[derive(Debug)]
// #[repr(C)]
// pub struct CGraphicDataBank {}
// #[derive(Debug)]
// #[repr(C)]
// pub struct CHeroCombatDef {}
// // TODO: Implement this. Ignored for now because its behind a pointer.
// #[derive(Debug)]
// #[repr(C)]
// pub struct CHeroLogBook {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CHeroLogBookEntry {
//     pub name: CWideString,
//     pub abbreviated_name: CWideString,
//     pub content: CWideString,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInitBaseClass {
//     pub vmt: *mut (),
//     pub c_base_class: CBaseClass,
//     // pub valid: bool,
// }

// impl CInitBaseClass {}

// /// TODO: This is behind a pointer so I've left it empty for now.
// #[derive(Debug)]
// #[repr(C)]
// pub struct CDataBank {
//     // pub c_bank_file_async: CBankFileAsync,
//     // pub c_resource_bank: CResourceBank,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessGameBase {
//     pub vmt: *mut (),
//     pub ca_input_process: CAInputProcess,
//     pub world: *mut CWorld,
//     pub player: *mut CPlayer,
//     pub definition_manager: *const CGameDefinitionManager,
//     pub component: *const CMainGameComponent,
//     pub p_game_player_interface: *const CGamePlayerInterface,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessBButtonExitMode {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessBetting {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessBlock {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessBoastUI {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessCameraLookAround {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessClickPastText {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessCombat {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessConsole {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessControlCreature {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessControlCreatureActivateZTarget {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessControlCreatureActivateZTargetOnPress {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessControlCreatureRightStick {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessControlFreeCamera {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessControlSpirit {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessCreatureMovement {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessCreatureMovementWatchForControlAngleChange {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessCreditsUI {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessCutScene {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessCycleSpecialCameraModes {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessDead {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessDebugControls {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessDigging {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessFireheartMinigame {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessFirstPerson {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessFirstPersonLookAround {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessFirstPersonTargeting {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessFishing {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessFreezeControls {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessHeroAbilitiesScreen {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessHeroInformationScreens {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessInGameMenu {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessInventory {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessInventoryClothing {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessInventoryExperienceScreen {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessInventoryMagicScreen {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessInventoryMapScreen {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessInventoryQuestsScreen {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessInventoryStatsScreen {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessInventoryTradeScreen {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessInventoryWeapons {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessJumpingAndRolling {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessLightning {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessMain {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessOracleMinigame {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessParalysed {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessPhotojournalCapture {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessProjectileTargetingAnalogueZoom {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessQuestCompletionUI {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessQuickAccessItems {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessQuickAccessMenu {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessRebootGame {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessSetRangedWeaponMode {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessSetRangedWeaponThirdPersonMode {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessSpecialAbilities {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessStrafe {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessTargetLockCycleTargets {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessTargetLockRightStickTargetSelect {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessTavernGame {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessToggleViewHeroMode {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessUseEnvironment {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessUseRangedWeapon {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessUseRangedWeaponZLock {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessWatchForRangedWeaponThirdPersonModeTermination {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessWatchForWillChargeUpThirdPersonModeTermination {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessYesNoQuestion {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessZTarget {
//     pub c_input_process_game_base: CInputProcessGameBase,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInputProcessManager {
//     pub vmt: *mut (),
//     pub c_base_class_non_copyable: CBaseClassNonCopyable,
//     pub input_process_list: CLinkedList<CAInputProcess>,
//     pub queued_processed_inputs: [CProcessedInput; 10],
//     pub no_queued_processed_inputs: c_long,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CIntelligentPointer<T> {
//     pub vmt: *mut (),
//     pub c_base_intelligent_pointer: CBaseIntelligentPointer,
//     _elem_type: PhantomData<T>,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInterpolationInfo {
//     pub current: self::CInterpolationInfoSet,
//     pub last: self::CInterpolationInfoSet,
//     pub paused_current: self::CInterpolationInfoSet,
//     pub paused_last: self::CInterpolationInfoSet,
//     pub bullet_time_current: self::CInterpolationInfoSet,
//     pub bullet_time_last: self::CInterpolationInfoSet,
//     pub wf_server_current_time: c_ulong,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CInterpolationInfoSet {
//     gt_predicted_time_since_last_render_frame: c_float,
//     wf_interpolate: c_float,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CKeyPairCompareLess<A, B> {
//     a: PhantomData<A>,
//     b: PhantomData<B>,
// }

// impl<A, B> CKeyPairCompareLess<A, B> {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CLetterBoxModeInfo {
//     pub c_fade_in_fade_out_base: CFadeInFadeOutBase,
//     pub ratio: c_float,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CLinkedList<T> {
//     pub head: *mut CLinkedListPosition,
//     pub tail: *mut CLinkedListPosition,
//     pub entries_count: c_long,
//     pub first_scan_info: CLinkedListScanInfo,
//     _elem_type: PhantomData<T>,
// }

// impl<T> CLinkedList<T> {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CLinkedListPosition {
//     pub data: *mut CBaseClass,
//     pub next: *mut CLinkedListPosition,
//     pub prev: *mut CLinkedListPosition,
//     pub list: *mut c_void,
//     pub in_list: bool,
// }

// impl CLinkedListPosition {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CLinkedListScanInfo {
//     pub current: *mut CLinkedListPosition,
//     pub next: *mut CLinkedListPosition,
//     pub prev: *mut CLinkedListPosition,
//     pub started: bool,
//     pub forwards: bool,
//     pub next_scan: *mut CLinkedListScanInfo,
//     pub prev_scan: *mut CLinkedListScanInfo,
//     pub list: *mut c_void,
// }

// impl CLinkedListScanInfo {}
// /// TODO: This is behind a pointer so I've left it empty for now.
// #[derive(Debug)]
// #[repr(C)]
// pub struct CLUA {}

// #[repr(C)]
// pub struct CMainGameComponentVmt {
//     pub _scalar_deleting_destructor: unsafe extern "thiscall" fn(*mut CMainGameComponent, c_uint) -> *mut c_void,
//     pub init: unsafe extern "thiscall" fn(*mut CMainGameComponent),
//     pub run: unsafe extern "thiscall" fn(*mut CMainGameComponent, *mut *mut CGameComponent) -> bool,
//     pub change_texture_colour_depth: unsafe extern "thiscall" fn(*mut CMainGameComponent, c_long),
//     pub on_activate: unsafe extern "thiscall" fn(*mut CActionBase),
//     pub update: unsafe extern "thiscall" fn(*mut CMainGameComponent),
//     pub get_inputs: unsafe extern "thiscall" fn(*mut CMainGameComponent),
//     pub render: unsafe extern "thiscall" fn(*mut CMainGameComponent),
//     pub post_init: unsafe extern "thiscall" fn(*mut CMainGameComponent),
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CActionBase {
//     pub vmt: *mut (),
//     pub c_base_class_non_copyable: CBaseClassNonCopyable,
//     pub ptc_script: *mut (), // TODO: CTCScriptedControl
//     pub activated: bool,
//     pub terminated: bool,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CMainGameComponent {
//     pub vmt: *mut CMainGameComponentVmt,
//     pub c_game_component: CGameComponent,
//     pub p_sample_bank: *mut CASoundBank,
//     pub p_text_bank: CCountedPointer<CDataBank>,
//     pub p_player_manager: BoostScopedPtr<CPlayerManager>,
//     pub p_player_interface: BoostScopedPtr<CGamePlayerInterface>,
//     pub p_world: BoostScopedPtr<CWorld>,
//     pub p_display_engine: BoostScopedPtr<CDisplayEngine>,
//     pub p_lua: CCountedPointer<CLUA>,
//     pub force_update_tick: bool,
//     pub force_update_tick_speed_multiplier: c_float,
//     pub force_update_tick_speed_desired_framerate: c_float,
//     pub force_update_no_failed_updates: c_long,
//     pub first_world_frame_update: bool,
//     /// Could be absent in retail?
//     pub current_server_frame: c_long,
//     /// Could be absent in retail?
//     pub input_server_frame: c_long,
//     pub last_game_turn_force_rendered: c_long,
//     pub current_frame_start_game_time: c_double,
//     pub game_start_time: c_double,
//     pub last_frame_render_duration: c_double,
//     pub last_interpolation_info: CInterpolationInfo,
//     // Cannot figure this out. Temporary fix.
//     pub event_package_set: CGameEventPackageSet,
//     /// Could be absent in retail? The class is still present, so there's a good chance it is.
//     pub client: CNetworkClient,
//     pub render_frames_since_last_game_update_count: c_ulong,
//     pub world_seed: c_ulong,
//     pub local_seed: c_ulong,
//     pub p_debug_font: CCountedPointer<CFontBank>,
//     pub p_cut_scene_main_font: CCountedPointer<CFontBank>,
//     pub event_package_file: CDiskFileWin32,
//     pub loading_event_packages: bool,
//     pub saving_event_packages: bool,
//     pub event_package_file_header: CEventPackageFileHeader,
//     pub frame_rate_smoother: CFrameRateSmoother,
//     pub last_render_frame_start_time: c_double,
//     pub time_passed_since_last_update: c_float,
//     pub last_update_time: c_float,
//     pub world_update_turn: bool,
//     pub rough_fps_counter: CRoughFrameCounter,
//     pub next_component_to_run: *mut CGameComponent,
//     pub p_main_graphic_bank: CCountedPointer<CGraphicDataBank>,
//     pub init_structure: CMainGameComponentInit,
//     pub initialised: bool,
//     pub allow_render: bool,
//     pub rendered: bool,
//     pub debug_frames_unable_to_render_count: c_long,
// }

// impl CMainGameComponent {
//     pub const P_MAIN_GAME_COMPONENT: usize = 0x13b86a0;

//     fn update(&self) -> c_void {
//         unimplemented!()
//     }

//     fn get_inputs(&self) -> c_void {
//         unimplemented!()
//     }

//     fn render(&self) -> c_void {
//         unimplemented!()
//     }

//     fn update_from_event_package_set(
//         &self,
//         event_package_set: *const CGameEventPackageSet,
//     ) -> c_void {
//         unimplemented!()
//     }

//     fn process_event_package(&self, event_package: *const CGameEventPackage) -> c_void {
//         unimplemented!()
//     }

//     fn process_event(&self, event: *const CGameEvent) -> c_void {
//         unimplemented!()
//     }

//     fn event_is_system_event(&self, event: *const CGameEvent) -> bool {
//         unimplemented!()
//     }

//     fn init_definitions(&self, x: bool) -> bool {
//         unimplemented!()
//     }

//     fn init_player_manager(&self) -> c_void {
//         unimplemented!()
//     }

//     fn create_players(&self) -> c_void {
//         unimplemented!()
//     }

//     fn initialise_text(&self) -> c_void {
//         unimplemented!()
//     }

//     fn limit_fps(&self) -> c_void {
//         unimplemented!()
//     }

//     fn check_sync(&self, event_package: *const CGameEventPackage) -> c_void {
//         unimplemented!()
//     }

//     fn init_update_and_render_all_entities(&self) -> c_void {
//         unimplemented!()
//     }

//     fn init_particle_engine(&self) -> c_void {
//         unimplemented!()
//     }

//     fn init_graphics(&self) -> bool {
//         unimplemented!()
//     }

//     fn get_event_package_set_from_save(
//         &self,
//         event_package_set: *mut CGameEventPackageSet,
//     ) -> bool {
//         unimplemented!()
//     }

//     fn add_event_package_set_to_save(
//         &self,
//         event_package_set: *const CGameEventPackageSet,
//     ) -> c_void {
//         unimplemented!()
//     }

//     fn init_event_package_loading(&self, x: *mut WCHAR) -> bool {
//         unimplemented!()
//     }

//     fn init_event_package_saving(&self, x: *mut WCHAR) -> bool {
//         unimplemented!()
//     }

//     fn uninit_event_package_loading(&self) -> c_void {
//         unimplemented!()
//     }

//     fn uninit_event_package_saving(&self) -> c_void {
//         unimplemented!()
//     }

//     fn update_average_frame_duration(&self, x: c_double) -> c_void {
//         unimplemented!()
//     }

//     fn get_average_frame_duration(&self) -> c_float {
//         unimplemented!()
//     }

//     fn get_current_frame_finish_time_approximation(&self) -> c_double {
//         unimplemented!()
//     }

//     fn get_current_frame_start_game_time(&self) -> c_double {
//         unimplemented!()
//     }

//     fn get_game_start_time(&self) -> c_double {
//         unimplemented!()
//     }

//     fn get_current_game_time(&self) -> c_double {
//         unimplemented!()
//     }

//     fn convert_gt_to_wf(&self, x: c_double) -> c_double {
//         unimplemented!()
//     }

//     fn get_render_interpolate(&self) -> c_float {
//         unimplemented!()
//     }

//     fn get_predicted_time_since_last_render_frame(&self) -> c_double {
//         unimplemented!()
//     }

//     fn get_last_frame_render_duration(&self) -> c_double {
//         unimplemented!()
//     }

//     fn get_game_time_of_next_present_completion(&self) -> c_double {
//         unimplemented!()
//     }

//     fn force_render(&self, x: c_long) -> c_void {
//         unimplemented!()
//     }

//     fn init_world(&self) -> c_void {
//         unimplemented!()
//     }

//     fn init_display_engine(&self) -> c_void {
//         unimplemented!()
//     }

//     fn init_sound(&self) -> c_void {
//         unimplemented!()
//     }

//     fn initialise_fonts(&self) -> c_void {
//         unimplemented!()
//     }

//     fn init_lua(&self, x: *const CCharString) -> c_void {
//         unimplemented!()
//     }

//     fn post_init(&self) -> c_void {
//         unimplemented!()
//     }

//     fn validate_definitions(&self) -> c_void {
//         unimplemented!()
//     }

//     fn shutdown(&self) -> c_void {
//         unimplemented!()
//     }

//     fn get_frame_difference_from_current(&self, x: c_long) -> c_long {
//         unimplemented!()
//     }

//     fn begin_input_saving(&self, x: *const CWideString) -> c_void {
//         unimplemented!()
//     }

//     fn begin_input_loading(&self, x: *const CWideString) -> c_void {
//         unimplemented!()
//     }

//     fn get_world(&self) -> *mut CWorld {
//         unimplemented!()
//     }

//     fn peek_world(&self) -> *const CWorld {
//         unimplemented!()
//     }

//     fn get_sample_bank(&self) -> *mut CASoundBank {
//         unimplemented!()
//     }

//     fn peek_sample_bank(&self) -> *const CASoundBank {
//         unimplemented!()
//     }

//     fn get_player_manager(&self) -> *mut CPlayerManager {
//         unimplemented!()
//     }

//     fn peek_player_manager(&self) -> *const CPlayerManager {
//         unimplemented!()
//     }

//     fn peek_event_dispatch_table_entry(&self, x: c_long) -> *const CGameEventDispatch {
//         unimplemented!()
//     }

//     fn is_initialised(&self) -> bool {
//         unimplemented!()
//     }

//     fn has_rendered(&self) -> bool {
//         unimplemented!()
//     }

//     fn is_controller_disconnected(&self) -> bool {
//         unimplemented!()
//     }

//     fn c_main_game_component_1(&self, x: *const CMainGameComponent) -> c_void {
//         unimplemented!()
//     }

//     fn c_main_game_component_2(&self, x: *mut CGame, y: *const CMainGameComponent) -> c_void {
//         unimplemented!()
//     }

//     /// Virtual function
//     fn c_main_game_component_destructor(&self) -> c_void {
//         unimplemented!()
//     }

//     /// Virtual function
//     fn init(&self) -> c_void {
//         unimplemented!()
//     }

//     /// Virtual function
//     fn run(&self, x: *mut *mut CMainGameComponent) -> bool {
//         unimplemented!()
//     }

//     fn update_regular(&self) -> c_void {
//         unimplemented!()
//     }

//     fn update_forced_tick_speed(&self) -> c_void {
//         unimplemented!()
//     }

//     fn start_letter_box_mode(&self, a: c_float, b: c_float, c: c_float) -> c_void {
//         unimplemented!()
//     }

//     fn end_letter_box_mode(&self) -> c_void {
//         unimplemented!()
//     }

//     fn is_display_engine_initialised(&self) -> bool {
//         unimplemented!()
//     }

//     fn get_display_engine(&self) -> *mut CDisplayEngine {
//         unimplemented!()
//     }

//     fn peek_display_engine(&self) -> *const CDisplayEngine {
//         unimplemented!()
//     }

//     fn pre_change_resolution(&self) -> c_void {
//         unimplemented!()
//     }

//     fn post_change_resolution(&self, a: c_ulong, b: c_ulong, c: c_ulong) -> c_void {
//         unimplemented!()
//     }

//     /// Virtual function
//     fn change_texture_colour_depth(&self, x: c_long) -> c_void {
//         unimplemented!()
//     }

//     fn change_max_texture_size(&self, x: c_ulong) -> c_void {
//         unimplemented!()
//     }

//     /// Virtual function
//     fn set_quit(&self) -> c_void {
//         unimplemented!()
//     }

//     fn set_next_component(&self, x: *const CGameComponent) -> c_void {
//         unimplemented!()
//     }

//     fn set_exclusive_mode(&self, x: bool) -> c_void {
//         unimplemented!()
//     }

//     fn set_display_mode(&self, a: c_ulong, b: c_ulong, c: c_uchar) -> c_void {
//         unimplemented!()
//     }

//     fn get_world_seed(&self) -> *mut c_ulong {
//         unimplemented!()
//     }

//     fn get_local_seed(&self) -> *mut c_ulong {
//         unimplemented!()
//     }

//     fn peek_world_seed(&self) -> c_ulong {
//         unimplemented!()
//     }

//     fn peek_local_seed(&self) -> c_ulong {
//         unimplemented!()
//     }

//     fn peek_main_font(&self) -> *const CFontBank {
//         unimplemented!()
//     }

//     fn peek_debug_font(&self) -> *const CFontBank {
//         unimplemented!()
//     }

//     fn peek_cut_scene_main_font(&self) -> *const CFontBank {
//         unimplemented!()
//     }

//     fn is_time_for_server_update(&self, x: c_long) -> bool {
//         unimplemented!()
//     }

//     fn signal_frame_update(&self) -> c_void {
//         unimplemented!()
//     }

//     fn get_p_player_interface(&self) -> *mut CGamePlayerInterface {
//         unimplemented!()
//     }

//     fn peek_definition_manager(&self) -> *const CDefinitionManager {
//         unimplemented!()
//     }

//     fn get_last_fps(&self) -> c_float {
//         unimplemented!()
//     }

//     fn is_editor_active(&self) -> bool {
//         unimplemented!()
//     }

//     fn get_current_world_frame(&self) -> c_long {
//         unimplemented!()
//     }

//     fn get_current_server_frame(&self) -> c_long {
//         unimplemented!()
//     }

//     fn get_engine_graphics(&self) -> *mut CGraphicDataBank {
//         unimplemented!()
//     }

//     fn peek_engine_graphics(&self) -> *const CGraphicDataBank {
//         unimplemented!()
//     }

//     fn get_text_bank(&self) -> *const CDataBank {
//         unimplemented!()
//     }

//     fn is_text_bank_initialised(&self) -> bool {
//         unimplemented!()
//     }

//     fn get_time_since_last_render_frame(&self) -> c_float {
//         unimplemented!()
//     }

//     fn get_time_passed_since_last_update(&self) -> c_float {
//         unimplemented!()
//     }

//     /// Virtual function
//     fn on_pre_device_reset(&self) -> c_void {
//         unimplemented!()
//     }

//     /// Virtual function
//     fn on_post_device_reset(&self) -> c_void {
//         unimplemented!()
//     }

//     fn add_game_event(&self, event: *const CGameEvent) -> c_void {
//         unimplemented!()
//     }

//     fn peek_last_interpolation_info(&self) -> *const CInterpolationInfo {
//         unimplemented!()
//     }

//     /// CMainGameComponent* operator=(const CMainGameComponent*)
//     fn assign(&self, x: *const CMainGameComponent) {
//         unimplemented!()
//     }

//     fn app_reinit_func(x: *mut c_void) -> c_void {
//         unimplemented!()
//     }

//     fn console_set_resolution(
//         mgc: *mut CMainGameComponent,
//         x: CxxVector<CGenericVar>,
//     ) -> c_void {
//         unimplemented!()
//     }

//     fn console_force_update_tick_speed(
//         mgc: *mut CMainGameComponent,
//         x: CxxVector<CGenericVar>,
//     ) -> c_void {
//         unimplemented!()
//     }

//     fn get() -> *mut CMainGameComponent {
//         unimplemented!()
//     }

//     fn get_constant_fps() -> c_long {
//         unimplemented!()
//     }

//     fn convert_wf_to_seconds(x: c_float) -> c_float {
//         unimplemented!()
//     }

//     fn convert_seconds_to_wf(x: c_float) -> c_float {
//         unimplemented!()
//     }

//     fn generate_met_files_from_lut_files() -> c_void {
//         unimplemented!()
//     }
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CMainGameComponentInit {
//     pub initial_world_name: CWideString,
//     pub initial_world_holy_site_name: CWideString,
//     pub initial_quest_name: CCharString,
//     pub save_game_name: CWideString,
// }
// #[derive(Debug)]
// #[repr(C)]
// pub struct CMeshDataBank {

// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CMessageEventManager {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CMusicManager {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CNavigationManager {}

// #[repr(C)]
// pub struct CNetworkClient {
//     pub vmt: *mut (),
//     pub c_init_base_class: CInitBaseClass,
//     pub server: CNetworkServer,
//     pub receive_buffer: [c_uchar; 8192],
//     pub local_event_package: CGameEventPackage,
//     pub last_update_time: c_double,
//     pub first_time: bool,
//     pub host: bool,
//     pub local_game: bool,
//     pub local_player: *const CNetworkPlayer,
//     pub host_player: *const CNetworkPlayer,
//     pub local_frame: c_long,
//     pub checksum1: c_ulong,
//     pub checksum2: c_ulong,
//     pub game_component: *mut CMainGameComponent,
// }

// impl fmt::Debug for CNetworkClient {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         f.debug_struct("CNetworkClient")
//             .field("c_init_base_class", &self.c_init_base_class)
//             .field("server", &self.server)
//             .field("receive_buffer", &&self.receive_buffer[..])
//             .field("local_event_package", &self.local_event_package)
//             .field("last_update_time", &self.last_update_time)
//             .field("first_time", &self.first_time)
//             .field("host", &self.host)
//             .field("local_game", &self.local_game)
//             .field("local_player", &self.local_player)
//             .field("host_player", &self.host_player)
//             .field("local_frame", &self.local_frame)
//             .field("checksum1", &self.checksum1)
//             .field("checksum2", &self.checksum2)
//             .field("game_component", &self.game_component)
//             .finish()
//     }
// }
// /// The definition for this is absent or never existed, but it only appears behind a pointer.
// #[derive(Debug)]
// #[repr(C)]
// pub struct CNetworkPlayer {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CNetworkServer {
//     pub vmt: *mut (),
//     pub c_init_base_class: CInitBaseClass,
// }
// #[derive(Debug)]
// #[repr(C)]
// pub struct COpinionReactionManager {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CPackedUIntArray {
//     pub packed_ints: *mut c_ulong,
//     pub size: c_ulong,
//     pub bits: c_uchar,
//     pub bias: c_ulong,
// }

// impl CPackedUIntArray {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CParentDefClassBase {
//     pub vmt: *mut (),
//     pub c_def_class_base: CDefClassBase,
//     pub instantiation_name: CDefString,
//     pub sub_def_info_map:
//         CVectorMap<c_ulong, self::CSubDefInfo, CKeyPairCompareLess<c_ulong, self::CSubDefInfo>>,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CPlayer {
//     pub vmt: *mut (),
//     pub c_init_base_class: CInitBaseClass,
//     pub drawing_free_cam_debug: bool,
//     pub player_is_character: bool,
//     pub player_interface: *mut CGamePlayerInterface,
//     pub player_manager: *mut CPlayerManager,
//     pub component: *const CMainGameComponent,
//     pub definition_manager: *const CGameDefinitionManager,
//     pub world: *mut CWorld,
//     pub thing_manager: *mut CThingManager,
//     pub p_world_map: *mut CWorldMap,
//     pub number: c_long,
//     pub controlled_creature: CIntelligentPointer<CThingPlayerCreature>,
//     pub player_character: CIntelligentPointer<CThingPlayerCreature>,
//     pub player_respawn_delay: c_long,
//     pub current_free_camera: CCamera,
//     pub free_camera: CCamera,
//     pub old_free_camera: CCamera,
//     pub respawn_info: self::CThingRespawnInfo,
//     pub p_hero_log_book: CCountedPointer<CHeroLogBook>,
//     pub input_process_main: BoostScopedPtr<CInputProcessMain>,
//     pub input_process_control_creature: BoostScopedPtr<CInputProcessControlCreature>,
//     pub input_process_z_target: BoostScopedPtr<CInputProcessZTarget>,
//     pub input_process_dead: BoostScopedPtr<CInputProcessDead>,
//     pub input_process_inventory: BoostScopedPtr<CInputProcessInventory>,
//     pub input_process_inventory_clothing: BoostScopedPtr<CInputProcessInventoryClothing>,
//     pub input_process_inventory_weapons: BoostScopedPtr<CInputProcessInventoryWeapons>,
//     pub input_process_hero_abilities_screen: BoostScopedPtr<CInputProcessHeroAbilitiesScreen>,
//     pub input_process_inventory_map_screen: BoostScopedPtr<CInputProcessInventoryMapScreen>,
//     pub input_process_inventory_stats_screen:
//         BoostScopedPtr<CInputProcessInventoryStatsScreen>,
//     pub input_process_inventory_magic_screen:
//         BoostScopedPtr<CInputProcessInventoryMagicScreen>,
//     pub input_process_inventory_experience_screen:
//         BoostScopedPtr<CInputProcessInventoryExperienceScreen>,
//     pub input_process_inventory_trade_screen:
//         BoostScopedPtr<CInputProcessInventoryTradeScreen>,
//     pub input_process_inventory_quests_screen:
//         BoostScopedPtr<CInputProcessInventoryQuestsScreen>,
//     pub input_process_hero_information_screens:
//         BoostScopedPtr<CInputProcessHeroInformationScreens>,
//     pub input_process_first_person: BoostScopedPtr<CInputProcessFirstPerson>,
//     pub input_process_click_past_text: BoostScopedPtr<CInputProcessClickPastText>,
//     pub input_process_yes_no_question: BoostScopedPtr<CInputProcessYesNoQuestion>,
//     pub input_process_freeze_controls: BoostScopedPtr<CInputProcessFreezeControls>,
//     pub input_process_control_free_camera: BoostScopedPtr<CInputProcessControlFreeCamera>,
//     pub input_process_creature_movement: BoostScopedPtr<CInputProcessCreatureMovement>,
//     pub input_process_use_environment: BoostScopedPtr<CInputProcessUseEnvironment>,
//     pub input_process_block: BoostScopedPtr<CInputProcessBlock>,
//     pub input_process_debug_controls: BoostScopedPtr<CInputProcessDebugControls>,
//     pub input_process_quick_access_items: BoostScopedPtr<CInputProcessQuickAccessItems>,
//     pub input_process_combat: BoostScopedPtr<CInputProcessCombat>,
//     pub input_process_special_abilities: BoostScopedPtr<CInputProcessSpecialAbilities>,
//     pub input_process_control_creature_right_stick:
//         BoostScopedPtr<CInputProcessControlCreatureRightStick>,
//     pub input_process_right_stick_look_around: BoostScopedPtr<CInputProcessCameraLookAround>,
//     pub input_process_cycle_special_camera_modes:
//         BoostScopedPtr<CInputProcessCycleSpecialCameraModes>,
//     pub input_process_reboot_game: BoostScopedPtr<CInputProcessRebootGame>,
//     pub input_process_jumping_and_rolling: BoostScopedPtr<CInputProcessJumpingAndRolling>,
//     pub input_process_first_person_targeting:
//         BoostScopedPtr<CInputProcessFirstPersonTargeting>,
//     pub input_process_use_ranged_weapon: BoostScopedPtr<CInputProcessUseRangedWeapon>,
//     pub input_process_control_creature_activate_z_target:
//         BoostScopedPtr<CInputProcessControlCreatureActivateZTarget>,
//     pub input_process_control_creature_activate_z_target_on_press:
//         BoostScopedPtr<CInputProcessControlCreatureActivateZTargetOnPress>,
//     pub input_process_left_stick_look_around:
//         BoostScopedPtr<CInputProcessFirstPersonLookAround>,
//     pub input_process_in_game_menu: BoostScopedPtr<CInputProcessInGameMenu>,
//     pub input_process_control_spirit: BoostScopedPtr<CInputProcessControlSpirit>,
//     pub input_process_tavern_game: BoostScopedPtr<CInputProcessTavernGame>,
//     pub input_process_set_ranged_weapon_mode: BoostScopedPtr<CInputProcessSetRangedWeaponMode>,
//     pub input_process_cut_scene: BoostScopedPtr<CInputProcessCutScene>,
//     pub input_process_fishing: BoostScopedPtr<CInputProcessFishing>,
//     pub input_process_digging: BoostScopedPtr<CInputProcessDigging>,
//     pub input_process_paralysed: BoostScopedPtr<CInputProcessParalysed>,
//     pub input_process_boast_ui: BoostScopedPtr<CInputProcessBoastUI>,
//     pub input_process_set_ranged_weapon_third_person:
//         BoostScopedPtr<CInputProcessSetRangedWeaponThirdPersonMode>,
//     pub input_process_use_ranged_weapon_z_lock:
//         BoostScopedPtr<CInputProcessUseRangedWeaponZLock>,
//     pub input_process_watch_for_ranged_weapon_third_person_mode_termination:
//         BoostScopedPtr<CInputProcessWatchForRangedWeaponThirdPersonModeTermination>,
//     pub input_process_watch_for_will_charge_up_third_person_mode_termination:
//         BoostScopedPtr<CInputProcessWatchForWillChargeUpThirdPersonModeTermination>,
//     pub input_process_target_lock_cycle_targets:
//         BoostScopedPtr<CInputProcessTargetLockCycleTargets>,
//     pub input_process_quick_access_menu: BoostScopedPtr<CInputProcessQuickAccessMenu>,
//     pub input_process_quest_completion_ui: BoostScopedPtr<CInputProcessQuestCompletionUI>,
//     pub input_process_target_lock_right_stick_target_select:
//         BoostScopedPtr<CInputProcessTargetLockRightStickTargetSelect>,
//     pub input_process_b_button_exit_mode: BoostScopedPtr<CInputProcessBButtonExitMode>,
//     pub input_process_credits_ui: BoostScopedPtr<CInputProcessCreditsUI>,
//     pub input_process_betting: BoostScopedPtr<CInputProcessBetting>,
//     pub input_process_lightning: BoostScopedPtr<CInputProcessLightning>,
//     pub input_process_watch_for_control_angle_change:
//         BoostScopedPtr<CInputProcessCreatureMovementWatchForControlAngleChange>,
//     pub input_process_oracle_minigame: BoostScopedPtr<CInputProcessOracleMinigame>,
//     pub input_process_fireheart_minigame: BoostScopedPtr<CInputProcessFireheartMinigame>,
//     pub input_process_strafe: BoostScopedPtr<CInputProcessStrafe>,
//     pub input_process_console: BoostScopedPtr<CInputProcessConsole>,
//     pub input_process_projectile_targeting_analogue_zoom:
//         BoostScopedPtr<CInputProcessProjectileTargetingAnalogueZoom>,
//     pub input_process_photojournal_capture: BoostScopedPtr<CInputProcessPhotojournalCapture>,
//     pub input_process_toggle_view_hero_mode: BoostScopedPtr<CInputProcessToggleViewHeroMode>,
//     pub local: bool,
//     pub show_world_thing: bool,
//     pub z_targeting: bool,
//     pub projectile_target_locked: bool,
//     pub right_trigger_target_locked: bool,
//     pub player_modes: CxxList<EPlayerMode>,
//     pub supress_full_mode_removal: bool,
//     pub disallow_mode_changes: bool,
//     pub initial_player_mode_set: bool,
//     pub aggressive_mode: bool,
//     pub spell_mode: bool,
//     pub expression_shift_mode: bool,
//     pub pc_projectile_weapon_third_person_aiming_mode: bool,
//     pub kill_everything_mode: bool,
//     pub player_def: CDefPointer<CPlayerDef>,
//     pub using_free_cam: bool,
//     pub free_camera_tracking_player: bool,
//     pub free_camera_tracking_player_yz: bool,
//     pub controlling_free_camera: bool,
//     pub keep_abilities_during_cutscenes: bool,
//     pub joystick_device_number: c_long,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CThingRespawnInfo {
//     pub died_position: C3DVector,
//     pub died_info_set: bool,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CPlayerDef {
//     pub vmt: *mut (),
//     pub c_parent_def_class_base: CParentDefClassBase,
//     pub character_def: c_long,
//     pub colour: CRGBColour,
// }

// /// TODO: This is behind a pointer so I've left it empty for now.
// #[derive(Debug)]
// #[repr(C)]
// pub struct CPlayerManager {
//     pub vmt: *mut (),
//     pub c_base_class_non_copyable: CBaseClassNonCopyable,
//     pub component: *mut CMainGameComponent,
//     /// Apparently this is a forward declaration with no actual definition. See also CDefinitionManager.
//     pub definition_manager: *const CGameDefinitionManager,
//     pub players: CxxVector<*mut CPlayer>,
//     pub player_neutral: c_long,
//     pub main_player: c_long,
//     pub hero_swap_player_script_names: CxxVector<CCharString>,
// }
// #[derive(Debug)]
// #[repr(C)]
// pub struct CPlayerMovementDef {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CProcessedInput {
//     pub player: c_long,
//     pub event_type: EProcessedEventType,
//     pub game_events: [CGameEvent; 4],
//     pub game_events_count: c_char,
//     pub priority: EGameEventPriority,
// }

// #[allow(non_camel_case_types)]
// #[derive(Debug)]
// #[repr(C)]
// pub enum EProcessedEventType {
//     PROCESSED_INPUT_NULL = 0,
//     PROCESSED_INPUT_GAME_EVENTS = 1,
//     PROCESSED_INPUT_PERFORMED_EVENT = 2,
// }

// #[allow(non_camel_case_types)]
// #[derive(Debug)]
// #[repr(C)]
// pub enum EGameEventPriority {
//     GAME_EVENT_PRIORITY_NULL = 0,
//     GAME_EVENT_PRIORITY_MIN = 1,
//     GAME_EVENT_PRIORITY_LOW = 2,
//     GAME_EVENT_PRIORITY_MEDIUM = 3,
//     GAME_EVENT_PRIORITY_HIGH = 4,
//     GAME_EVENT_PRIORITY_MAX = 5,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CRGBColour {
//     pub b: c_uchar,
//     pub g: c_uchar,
//     pub r: c_uchar,
//     pub a: c_uchar,
//     // pub int_value: c_ulong,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CRGBFloatColour {
//     pub r: c_float,
//     pub g: c_float,
//     pub b: c_float,
//     pub a: c_float,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CRightHandedSet {
//     pub up: C3DVector,
//     pub forward: C3DVector,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CRoughFrameCounter {
//     pub frame_start: c_ulong,
//     pub fps: c_float,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CScreenFilterSThingByPass {
//     pub thing: CScriptThing,
//     pub bypass_set: bool,
// }
// #[derive(Debug)]
// #[repr(C)]
// pub struct CScriptConversationManager {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CScriptGameResourceObjectBase {
//     pub vmt: *mut (),
//     pub c_object_base: CBaseObject,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CScriptGameResourceObjectMovieBase {
//     pub c_script_game_resource_object_base: CScriptGameResourceObjectBase,
//     pub p_imp: CCountedPointer<CScriptGameResourceObjectMovieBase>,
// }

// /// Whew lad what a class name
// #[derive(Debug)]
// #[repr(C)]
// pub struct CScriptGameResourceObjectScriptedThingBase {
//     pub vmt: *mut (),
//     pub c_scripted_game_resource_object_base: CScriptGameResourceObjectBase,
//     pub p_imp: CCountedPointer<CScriptGameResourceObjectScriptedThingBase>,
// }
// #[derive(Debug)]
// #[repr(C)]
// pub struct CScriptInfoManager {}
// #[derive(Debug)]
// #[repr(C)]
// pub struct CScriptThing {}

// #[repr(C)]
// pub struct CSystemManager {
//     pub vmt: *mut (),
//     pub c_init_base_class: CInitBaseClass,
//     pub windows_quit: bool,
//     pub application_active: bool,
//     pub application_has_input_focus: bool,
//     pub application_guid: GUID,
//     pub best_guid: GUID,
//     pub p_best_guid: *mut GUID,
//     pub prefer_primary_device: bool,
//     pub critical_section: RTL_CRITICAL_SECTION,
//     pub main_fibre: *mut c_void,
//     pub lib_debug_manager: CCountedPointer<CDebugManager>,
//     pub p_input_manager: CCountedPointer<CInputManager>,
//     pub p_display_manager: CCountedPointer<CDisplayManager>,
//     pub debug_manager: *mut CDebugManager,
//     pub p_graphic_bank_manager: CCountedPointer<CGraphicBankManager>,
//     pub p_mesh_bank_manager: CCountedPointer<CMeshBankManager>,
//     pub p_font_manager: CCountedPointer<CFontManager>,
//     pub window_initialised: bool,
//     pub d_draw_initialised: bool,
//     pub win_instance_handle: *mut HINSTANCE,
//     pub application_win_handle: *mut HWND,
//     pub win_show: u32,
//     pub win_command_line: c_char,
//     pub win_app_name: CWideString,
//     pub win_class: WNDCLASSEXW,
//     pub resolution_set: bool,
//     pub init_flags: c_ulong,
//     pub ever_been_active: bool,
//     pub mouse_button_0_presed: bool,
//     pub mouse_button_1_presed: bool,
//     pub mouse_button_2_presed: bool,
//     pub mouse_button_3_presed: bool,
//     pub mouse_button_4_presed: bool,
//     pub mouse_z_move: c_long,
//     pub window_valid: bool,
//     pub restore_exclusive: bool,
//     pub restore: bool,
//     pub wait_while_inactive: bool,
//     pub attached_to_external_window: bool,
//     pub prevent_exclusive_mode_changes: bool,
//     pub using_accurate_timer: bool,
//     // Pointer to an unknown function type.
//     pub app_reinit_func: *mut (),
//     pub app_reinit_context: *mut c_void,
//     pub profile_manager: CCountedPointer<CProfileManager>,
//     pub system_colours: CGuiColoursInfo,
//     pub temp_path: CWideString,
//     pub application_name: CCharString,
//     pub force_music_play: bool,
//     pub force_music_play_offset: c_float,
//     // Pointer to an unknown function type.
//     pub error_func: *mut (),
//     // Pointer to an unknown function type.
//     pub ime_message_func: *mut (),
//     // Pointer to an unknown function type.
//     pub mesage_filter: *mut (),
// }

// #[repr(C)]
// pub struct CFontManager {
//     // pub vmt: *mut (),
//     // pub c_base_class_non_copyable: CBaseClassNonCopyable,
//     // pub p_font_data_bank: CCountedPointer<CFontDataBank>,
//     // pub p_streaming_font_data_bank: CCountedPointer<CStreamingFontDataBank>,
//     // pub p_symbols: CCountedPointer<CSymbolMap>,
//     // pub p_streaming_symbols: CCountedPOinter<CSymbolMap>,
//     // pub vertex_buffers: CArray<*mut CVertexBufferWin32>,
//     // pub current_vb: c_long,
//     // pub async_glyphs_pending: c_ulong,
//     // pub load_font_glyphs_asynchronously: bool,
// }

// #[repr(C)]
// pub struct CMeshBankManager {
//     pub vmt: *mut (),
//     pub c_base_class_non_copyabe: CBaseClassNonCopyable,
//     pub c_device_reset_callback: CDeviceResetCallback,
//     pub mesh_data_banks: CxxList<CMeshDataBankInfo>,
// }

// #[repr(C)]
// pub struct CMeshDataBankInfo {
//     pub bank_handle: CCharString,
//     pub mesh_data_bank: CCountedPointer<CMeshDataBank>,
// }

// #[repr(C)]
// pub struct CGraphicBankManager {
//     pub vmt: *mut (),
//     pub c_base_class_non_copyable: CBaseClassNonCopyable,
//     pub graphic_data_banks: CxxList<CGraphicDataBankInfo>,
// }

// #[repr(C)]
// pub struct CGraphicDataBankInfo {
//     pub init: CGraphicDataBankInit,
//     pub bank_handle: CCharString,
//     pub graphic_data_bank: CCountedPointer<CGraphicDataBank>,
// }

// #[repr(C)]
// pub struct CGraphicDataBankInit {
//     pub non_alpha_format: CPixelFormat,
//     pub alpha_format: CPixelFormat,
//     pub interpolated_alpha_format: CPixelFormat,
//     pub boolean_alpha_format: CPixelFormat,
//     pub uncompressed_non_alpha_format: CPixelFormat,
//     pub uncompressed_alpha_format: CPixelFormat,
//     pub generate_mipmaps: bool,
//     pub max_graphic_width: c_ulong,
//     pub max_Graphic_height: c_ulong,
//     pub allow_dither: bool,
//     pub allow_compressed_textures: bool,
//     pub allow_dxt1_for_boolean_alpha: bool,
// }

// #[repr(C)]
// pub struct CInputManager {
//     pub vmt: *mut (),
//     pub c_base_class: CBaseClass,
//     pub processed_events: CxxVector<CInputEvent>,
//     pub event_store: CxxVector<CInputEvent>,
//     pub stored_events_count: c_long,
//     pub event_scans_running: c_long,
//     pub p_mouse: BoostScopedPtr<CMouse>,
//     pub p_keyboard: BoostScopedPtr<CKeyboard>,
//     pub p_joysticks: CxxVector<CCountedPointer<CJoystick>>,
//     pub loading: bool,
//     pub saving: bool,
//     pub checksum: CChecksum,
//     pub loaded_event_packages: CxxVector<CInputEventPackage>,
//     pub current_file_package_index: c_long,
//     pub timestamp: c_long,
//     pub package_file: CDiskFileWin32,
//     pub update_count: c_long,

// }

// #[repr(C)]
// pub struct CInputEventPackage {
//     pub events: CxxVector<CInputEvent>,
//     pub timestamp: c_long,
//     pub checksum: CChecksum,
//     pub mouse_pos: C2DVector,
//     pub joystick_pos: C2DVector,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CChecksum {
//     pub checksum_1: c_long,
//     pub checksum_2: c_long,
//     pub checksum_3: c_long,
//     pub checksum_4: c_long,
// }

// #[repr(C)]
// pub struct CJoystick {
//     pub event_store: [CInputEvent; 64], // 3328 bytes
//     pub input_manager: *mut CInputManager,
//     pub exclusive_mode: bool,
//     pub maintained_events_list: CxxList<CMaintainedEvent>,
//     pub stored_events_count: c_long,
//     pub event_scans_running: c_long,
// }

// #[repr(C)]
// pub struct CMaintainedEvent {
//     pub vmt: *mut (),
//     pub c_base_class: CBaseClass,
//     pub event: CInputEvent,
// }

// #[repr(C)]
// pub struct CKeyboard {
//     pub vmt: *mut (),
//     pub c_base_class_non_copyabe: CBaseClassNonCopyable,
//     pub event_store: [CInputEvent; 256], // 13312 bytes
//     pub stored_events_count: c_long,
//     pub event_scans_running: c_long,
//     pub maintained_events_list: CxxList<CMaintainedEvent>,
//     pub input_manager: *mut CInputManager,
// }

// #[repr(C)]
// pub struct CMouse {
//     pub vmt: *mut (),
//     pub c_base_class_non_copyable: CBaseClassNonCopyable,
//     pub event_store: [CInputEvent; 256], // 13312 bytes
//     pub stored_events_count: c_long,
//     pub event_scans_running: c_long,
//     pub maintained_events_list: CxxList<CMaintainedEvent>,
//     pub input_manager: *mut CInputManager,
// }

// #[repr(C)]
// pub struct CInputEvent {
//     pub data: UInputEventData,
//     pub device_type: EInputDeviceType,
//     pub device_number: c_long,
//     pub event_type: EInputEventType,
//     pub current_time: c_float,
//     pub start_time: c_float,
// }

// #[repr(C)]
// pub union UInputEventData {
//     pub key: EInputKey,
//     pub mouse: CInputEventDataMouse,
//     pub unknown: WCHAR,
//     pub joystick: CInputEventDataJoystick,
// }

// #[derive(Debug,Copy,Clone)]
// #[repr(C)]
// pub struct CInputEventDataMouse {
//     pub held_movement: C3DVector,
//     pub movement: C3DVector,
//     pub pos: C2DVector,
// }

// #[derive(Debug,Copy,Clone)]
// #[repr(C)]
// pub struct CInputEventDataJoystick {
//     pub pos: C2DVector,
//     pub button: c_uchar,
//     pub button_preassure: c_float,
// }

// // TODO
// #[repr(C)]
// pub struct CProfileManager {
//     // pub up_pressed: bool,
//     // pub down_pressed: bool,
//     // pub toggle_pressed: bool,
//     // pub flip_all_pressed: bool,
//     // // TODO: Figure this out
//     // // pub cursor_position: CxxListIterator<CProfile>,
//     // pub auto_display_hiearchy: bool,
//     // pub valid: bool,
//     // pub application_timer_start: c_double,
//     // pub application_total_time: c_double,
//     // pub min_updated: bool,
//     // pub min_application_total_time: c_double,
//     // pub stored_total_time: c_double,
//     // pub turn_count: c_long,
//     // pub render_manager: *mut CRenderManager,
//     // pub draw_mode: EDrawMode,
//     // pub eval_mode: EEvalMode,
//     // pub new_profiles_registered: bool,
//     // pub profile_list: CxxList<*mut CProfile>,
//     // pub id_to_profile_name: CxxMap<c_long, CCharString>,
//     // pub profile_name_to_id: CxxMap<CCharString, c_long>,
//     // pub profile_tree: CxxList<CProfileTreeEntry>,
//     // pub current_tree_Entry: *mut CProfileTreeEntry,
//     // pub needs_saving: bool,
// }

// #[repr(C)]
// pub struct CGuiColoursInfo {
//     pub background_shadow3_d: CRGBColour,
//     pub button_face: CRGBColour,
//     pub button_highlight: CRGBColour,
//     pub button_light: CRGBColour,
//     pub button_shadow: CRGBColour,
//     pub button_text: CRGBColour,
//     pub active_border: CRGBColour,
//     pub active_window_left_side_caption: CRGBColour,
//     pub active_window_right_side_caption: CRGBColour,
//     pub inactive_window_left_side_caption: CRGBColour,
//     pub inactive_window_right_side_caption: CRGBColour,
//     pub desktop_colour: CRGBColour,
//     pub caption_text: CRGBColour,
//     pub caption_inactive_text: CRGBColour,
//     pub gray_text: CRGBColour,
//     pub highlight_item: CRGBColour,
//     pub highlight_text: CRGBColour,
//     pub tooltip_background: CRGBColour,
//     pub tooltip_text: CRGBColour,
//     pub menu_background: CRGBColour,
//     pub menu_text: CRGBColour,
//     pub scrollbar: CRGBColour,
//     pub window_background: CRGBColour,
//     pub window_frame: CRGBColour,
//     pub window_text: CRGBColour,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CTestQuest {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CThing {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CThingCreatureBase {
//     // pub c_thing_game_object: CThingGameObject,
//     // pub vmt: *mut (),
//     // pub last_message_event_i_created_id: c_ulong,
//     // pub combat_collision_debug_graphics: CxxVector<CEnginePrimitiveHandle>,
//     // pub p_def: CDefPointer<CThingCreatureDef>,
//     // pub shot_accuracy_percentage: c_long,
//     // pub initial_pos: C3DVector,
//     // pub p_last_attacked_by_creature: CIntelligentPointer<CThingCreatureBase>,
//     // pub wf_last_attacked_by_Creature: c_ulong,
//     // pub p_current_action: CCountedPointer<CCreatureActionBase>,
//     // pub p_queued_actions: CxxList<CCountedPointer<CCreatureActionBase>>,
//     // pub movement_vector: C3DVector,
//     // pub head_pos_offset: C3DVector,
//     // pub idle_counter: c_long,
//     // pub turn_speed: c_float,
//     // pub p_creature_interaction: CCountedPointer<CCreatureInteraction>,
//     // pub p_tc_mode_manager: CTCCreatureModeManager,
//     // pub previous_action_handedness: ECombatAnimationHandedness,
//     // pub previous_action_handedness_wd: c_long,
//     // pub body_reorienter: BoostScopedPtr<CThingBodyReorienter>,
//     // pub combat_debug_graphics: CxxVector<CEnginePrimitiveHandle>,
//     // pub p_item_to_unseathe_after_cutscene: CIntelligentPointer<CThing>,
//     // /// c_rchar
//     // pub debug_text: c_char,
//     // pub currently_frame_updating: UnknownEmptyType,
//     // pub blinking: UnknownEmptyType,
//     // pub melee_attacker: UnknownEmptyType,
//     // pub melee_attacked: UnknownEmptyType,
//     // pub leave_dead_body_override: UnknownEmptyType,
//     // pub leave_experience_orbs_override: UnknownEmptyType,
//     // pub follow_hero_through_teleporters: UnknownEmptyType,
//     // // Unknown one byte type
//     // pub head_pos_cached: u8,
//     // pub use_movement_in_actions: UnknownEmptyType,
//     // pub update_movement_vector_this_frame: UnknownEmptyType,
//     // pub aborting_current_action: UnknownEmptyType,
//     // pub demon_door: UnknownEmptyType,
//     // pub oracle: UnknownEmptyType,
//     // pub had_facing_angle_y_z_set_by_body_orientation: UnknownEmptyType,
//     // pub under_influence_of_epic_spell: UnknownEmptyType,
//     // // Unknown one byte type
//     // pub hidden_on_mini_map: u8,
//     // pub is_showing_debug_info: UnknownEmptyType,
// }

// #[allow(non_camel_case_types)]
// #[derive(Debug)]
// #[repr(C)]
// pub enum ECombatAnimationHandedness {
//     HANDED_RIGHT = 0,
//     HANDED_LEFT = 1,
//     HANDED_NONE = 2,
// }

// pub struct CThingGameObject {
//     // pub c_thing_physical: CThingPhysical,
//     // pub vmt: *mut (),
//     // pub p_thing_standing_on: CIntelligentPointer<CThing>,
//     // pub add_to_combo_multiplier_on_hit: UnknownEmptyType,
//     // pub add_to_combo_multiplier_on_hit_override_set: UnknownEmptyType,
//     // pub give_hero_stat_changes_on_being_hit: UnknownEmptyType,
//     // pub give_hero_stat_changes_on_being_hit_override_set: UnknownEmptyType,
// }

// pub struct CThingPhysical {
//     // pub c_thing: CThing,
// }
// #[derive(Debug)]
// #[repr(C)]
// pub struct CThingManager {
//     // pub c_class_factory: CClassFactory,
//     // pub vmt: *mut (),
//     // pub component: *const CMainGameComponent,
//     // pub definition_manager: *const CGameDefinitionManager,
//     // pub world: *mut CWorld,
//     // pub world_map: *mut CWorldMap,
//     // pub world_seed: c_ulong,
//     // pub player_manager: *mut CPlayerManager,
//     // pub navigation_manager: *mut CNavigatorManager,
//     // pub p_environment: *const CEnvironment,
//     // pub thing_type_info: CxxVector<CThingTypeInfo>,
//     // pub dead_thing_list: CxxVector<*mut CThing>,
//     // pub non_scripted_entities_pause_mode: bool,
//     // pub all_entities_pause_mode: bool,
//     // pub first_render_of_frame: bool,
//     // pub uid_to_thing_map: CxxMap<u64, *mut CThing, CxxLess<u64>>,
//     // pub current_unique_id: u64,
//     // pub current_local_thing_unique_id: u64,
//     // pub draw_combat_collision_debug: bool,
//     // pub draw_attitude_debug: bool,
//     // pub max_thing_draw_dist: c_long,
//     // pub global_thing_loading_behavior: self::EGlobalThingLoadingBehavior,
//     // pub render_list: ThingLList,
//     // pub update_lists: CxxVector<ThingLList>,
//     // pub use_map_specific_uids: bool,
//     // pub update_and_render_all_flags: bool,
//     // pub serialising_game_state: bool,
//     // pub currently_loading: bool,
//     // pub loading_entities_from_save_game: bool,
//     // pub entitiy_runtime_persisitence: n_thing_manager::CEntityRuntimePersistence,
//     // pub error_flag: bool,
//     // pub npc_name_map: CxxMap<u64, CxxPair<CCharString, CWideString>, CxxLess<u64>>,
//     // pub serialise_version_number: c_long,
//     // pub level_being_loaded: c_long,
//     // pub current_selection_name: CCharString,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CThingPlayerCreature {
//     pub vmt: *mut (),
//     pub c_thing_creature_base: CThingCreatureBase,
//     pub control_pos: C3DVector,
//     pub control_move_by_vector: C3DVector,
//     pub moved_by_player: UnknownEmptyType,
//     pub jumping: UnknownEmptyType,
//     /// An unknown type with 4 bytes.
//     pub pad1: u32,
//     pub controlled_movement_type: EControlledMovementType,
//     pub movement_acceleration: C3DVector,
//     pub max_slow_walking_speed: c_long,
//     pub max_walking_speed: c_float,
//     pub max_jogging_speed: c_float,
//     pub max_running_speed: c_float,
//     pub max_springing_speed: c_float,
//     pub max_flying_speed: c_float,
//     pub max_strafing_speed: c_float,
//     pub relative_movement_acceleration_components:
//         CxxMap<EGameAction, CxxPair<C3DVector, c_long>, CxxLess<EGameAction>>,
//     pub relative_movement_acceleration: C3DVector,
//     pub last_relative_movement_acceleration: C3DVector,
//     pub collided_with_thing: bool,
//     pub last_thing_collision_normal: C3DVector,
//     pub last_thing_collision_position: C3DVector,
//     pub p_hero_combat_def: CDefPointer<CHeroCombatDef>,
//     pub p_player_movement_def: CDefPointer<CPlayerMovementDef>,
//     pub impulse_velocity: C3DVector,
//     pub lean_angle: c_float,
//     pub movement_band_type: EMovementBandType,
// }
// #[derive(Debug)]
// #[repr(C)]
// pub struct CThingSearchTools {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CTimer {
//     pub timer_index: c_long,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CVectorMap<K, V, C> {
//     pub c_array: CArray<CxxPair<K, V>>,
//     pub compare: C,
//     pub dirty: bool,
// }

// impl<K, V, C> CVectorMap<K, V, C> {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CWideString {
//     pub p_string_data: *mut CWideStringData,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CWideStringData {
//     pub data: CxxBasicString<wchar_t, CxxCharTraits<wchar_t>, CxxAllocator<wchar_t>>,
//     pub refs_count: c_long,
// }

// /// TODO: This is behind a pointer so I've left it empty for now.
// #[derive(Debug)]
// #[repr(C)]
// pub struct CWorld {
//     pub vmt: *mut (),
//     pub c_base_class_non_copyable: CBaseClassNonCopyable,
//     pub ci_draw_world: CIDrawWorld,
//     pub component: *mut CMainGameComponent,
//     pub player_manager: *mut CPlayerManager,
//     pub definition_manager: *const CGameDefinitionManager,
//     pub p_world_map: BoostScopedPtr<CWorldMap>,
//     pub p_environment: BoostScopedPtr<CEnvironment>,
//     pub p_game_time_manager: BoostScopedPtr<CGameTimeManager>,
//     pub p_thing_search_tools: BoostScopedPtr<CThingSearchTools>,
//     pub p_atmos_processor: CCountedPointer<CAtmosProcessor>,
//     pub p_game_camera: BoostScopedPtr<CAIGameCameraBase>,
//     pub p_game_camera_manager: BoostScopedPtr<CAGameCameraBase>,
//     pub p_current_game_camera: *mut CAIGameCameraBase,
//     pub p_game_script_interface: BoostScopedPtr<CGameScriptInterface>,
//     pub p_main_mesh_bank: CCountedPointer<CMeshDataBank>,
//     pub p_animation_manager: *mut C3DAnimationManager,
//     pub p_navigation_manager: BoostScopedPtr<CNavigationManager>,
//     pub p_thing_combat_manager: BoostScopedPtr<CCombatManager>,
//     pub p_thing_manager: BoostScopedPtr<CThingManager>,
//     pub p_faction_manager: BoostScopedPtr<CFactionManager>,
//     pub p_script_info_manager: CCountedPointer<CScriptInfoManager>,
//     pub p_message_event_manager: CCountedPointer<CMessageEventManager>,
//     pub p_bullet_time_manager: BoostScopedPtr<CBulletTimeManager>,
//     pub p_music_manager: CCountedPointer<CMusicManager>,
//     pub p_opinion_reaction_manager: CCountedPointer<COpinionReactionManager>,
//     pub p_script_conversation_manager: BoostScopedPtr<CScriptConversationManager>,
//     pub just_loaded: bool,
//     pub current_world_name: CCharString,
//     pub console_pause_at_frame_number: c_long,
//     pub frame_started_3d_rendering: c_long,
//     pub last_update_time_length: c_double,
//     pub last_update_time: c_double,
//     pub countdown_timer: c_long,
//     pub paused: bool,
//     pub slow_motion: bool,
//     pub show_debug_text: bool,
//     pub show_fps_text: bool,
//     pub show_profile_text: bool,
//     pub initial_active_quests: CArray<CCharString>,
//     pub registered_quests: CxxVector<CCharString>,
//     pub active_test_quests: CxxVector<CTestQuest>,
//     pub creature_generation_disabled_groups: c_ulong,
//     pub creature_generation_enabled: bool,
//     pub teleporting_enabled: bool,
//     pub experience_spending_enabled: bool,
//     pub saving_enabled: bool,
//     pub dont_populate_next_loaded_region: bool,
//     pub hero_sleeping_enabled: bool,
//     pub map_table_show_quest_cards_on_used: bool,
//     pub screen_to_fade_in_on_next_region_change: bool,
//     pub done_extra_frame_update_before_region_load_screen_fade_in: bool,
//     pub mini_map_enabled: bool,
//     pub mini_map_active_before_disabled: bool,
//     pub region_loaded_display_region: bool,
//     pub guild_master_messages_enabled: bool,
//     pub summoner_death_explosion_affects_hero: bool,
//     pub waiting_for_inventory_tutorial_to_finish: bool,
//     pub hero_information_screen_mode_after_tutorial: bool,
//     pub frame_cached_lod_center: c_long,
//     pub cached_lod_center: C3DVector,
//     // pub save_game_load_status: self::ESaveGameLoadStatus,
//     pub save_game_load_status: u32,
//     pub save_game_path_name: CWideString,
//     pub auto_save_loacked: bool,
//     pub serialising_about_to_load_hero_state: bool,
//     pub serialising_hero_state: bool,
//     pub serialising_non_persistent_quest_items: bool,
//     // pub region_load_status: self::ERegionLoadStatus,
//     pub region_load_status: u32,
//     pub region_load_start_pos: C3DVector,
//     pub region_load_start_angle_xy: c_float,
//     pub region_load_followers: CxxVector<CIntelligentPointer<CThing>>,
//     pub pervious_region: c_long,
//     pub number_of_times_freeze_controls_mode_added_during_region_load: c_long,
//     pub region_load_via_teleport: bool,
//     pub region_load_via_door: bool,
//     pub put_into_pause_mode_on_region_change: bool,
//     pub region_load_waiting_for_confirmation: bool,
//     pub region_load_screen_fully_faded: bool,
//     pub region_load_screen_was_faded_out: bool,
//     pub waiting_for_reset_to_front_end_confirmation: bool,
//     // pub most_recent_save_type: self::ESaveType,
//     pub most_recent_save_type: u32,
//     // pub most_recent_save_type_before_manual_save: self::ESaveType,
//     pub most_recent_save_type_before_manual_save: u32,
//     pub most_recent_manual_save_name: CWideString,
//     pub auto_save_check_point_exists: bool,
//     pub save_game_marker_pos: C3DVector,
//     pub save_game_marker_angle_xy: c_float,
//     pub guild_seal_recall_pos: C3DVector,
//     pub guild_seal_recall_angle_xy: c_float,
//     pub weather_masking_primitives_sent: bool,
//     pub weather_masking_primitive_handles: CxxVector<CEnginePrimitiveHandle>,
//     pub atmos_banks_waiting_to_copy: CxxList<CAtmosCopyInfo>,
//     // pub player_spawn_status: self::EPlayerSpawnStatus,
//     pub player_spawn_status: u32,
//     pub villager_reaction_debug: bool,
//     pub start_time: c_double,
//     pub time_played: c_double,
//     pub has_initialised_start_time: bool,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct CIDrawWorld {
//     // Does it really have a vtable?
//     pub vmt: *mut (),
// }

// #[allow(non_camel_case_types)]
// #[derive(Debug)]
// #[repr(C)]
// pub enum ESaveType {
//     SAVE_TYPE_NONE = 0,
//     SAVE_TYPE_MANUAL_SAVE = 1,
//     SAVE_TYPE_AUTO_SAVE = 2,
//     SAVE_TYPE_QUEST_START_SAVE = 3,
// }

// #[allow(non_camel_case_types)]
// #[derive(Debug)]
// #[repr(C)]
// pub enum EPlayerSpawnStatus {
//     PLAYER_SPAWN_STATUS_NULL = 0,
//     PLAYER_SPAWN_STATUS_START = 1,
//     PLAYER_SPAWN_STATUS_END = 2,
// }

// #[allow(non_camel_case_types)]
// #[derive(Debug)]
// #[repr(C)]
// pub enum ERegionLoadStatus {
//     NOT_LOADING_REGION = 0,
//     WAITING_FOR_LOCKED_REGION_CONFIRMATION = 1,
//     WAITING_FOR_CONFIRMATION = 2,
//     WAITING_FOR_TELEPORT_EFFECT = 3,
//     READY_TO_BEGIN_FADE_OUT = 4,
//     WAITING_FOR_FADE_OUT = 5,
//     LOADING_NEW_REGION = 6,
//     LOADING_RESOURCES = 7,
//     READY_FOR_FADE_IN = 8,
//     WAITING_FOR_FADE_IN = 9,
// }

// #[allow(non_camel_case_types)]
// #[derive(Debug)]
// #[repr(C)]
// pub enum ESaveGameLoadStatus {
//     SAVE_LOAD_STATUS_NONE = 0,
//     SAVE_LOAD_STATUS_FADE_OUT = 1,
//     SAVE_LOAD_STATUS_FADING_OUT = 2,
//     SAVE_LOAD_STATUS_LOADING = 3,
//     SAVE_LOAD_STATUS_FADE_IN = 4,
// }
// #[derive(Debug)]
// #[repr(C)]
// pub struct CWorldMap {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CAGameCameraBase {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CAInputProcess {
//     pub vmt: *mut (),
//     pub c_base_class_non_copyable: CBaseClassNonCopyable,
//     pub p_player_interface: CInputProcessManager,
//     pub input_process_list_pos: CLinkedListPosition,
// }

// impl CAInputProcess {}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CASoundBank {
//     pub c_base_class_non_copyable: CBaseClassNonCopyable,
//     pub p_symbol_map: CCountedPointer<CCRCSymbolMap>,
// }

// impl CASoundBank {}

// /// The methods on this are more interesting. Maybe CI = Class Interface?
// #[derive(Debug)]
// #[repr(C)]
// pub struct CIEngine {
//     pub vmt: *mut (),
//     pub c_init_base_class: CInitBaseClass,
//     pub active: bool,
// }

// impl CIEngine {}

// /// Unknown variants.
// #[derive(Debug)]
// #[repr(C)]
// pub enum ECameraOp {
//     UNKNOWN = 0,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum ECategory {
//     CATEGORY_QUEST = 0,
//     CATEGORY_STORY = 1,
//     CATEGORY_TUTORIAL = 2,
//     CATEGORY_BASICS = 3,
//     CATEGORY_OBJECTS = 4,
//     CATEGORY_TOWNS = 5,
//     CATEGORY_HERO = 6,
//     CATEGORY_COMBAT = 7,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum EClothingCoversArea {
//     COVERS_BODY_AREA_NULL = 0,
//     COVERS_BODY_AREA_FEET = 1,
//     COVERS_BODY_AREA_LEGS = 2,
//     COVERS_BODY_AREA_ARSE = 4,
//     COVERS_BODY_AREA_BODY = 8,
//     COVERS_BODY_AREA_HEAD = 16,
//     COVERS_BODY_AREA_ARMS = 32,
//     COVERS_BODY_AREA_HANDS = 64,
//     COVERS_BODY_AREA_FACE = 128,
//     COVERS_BODY_AREA_MOUSTACHE = 256,
//     COVERS_BODY_AREA_HORN = 512,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum EControlledMovementType {
//     CONTROLLED_MOVEMENT_NULL = 0,
//     CONTROLLED_MOVEMENT_WALKING = 1,
//     CONTROLLED_MOVEMENT_FLYING = 2,
//     CONTROLLED_MOVEMENT_FIRST_PERSON = 3,
// }
// /// Unknown variants.
// #[derive(Debug)]
// #[repr(C)]
// pub enum ECreatureGroup {
//     UNKNOWN = 0,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum ECrime {
//     CRIME_NONE = 0,
//     CRIME_WEAPON_OUT = 1,
//     CRIME_TRESPASS = 2,
//     CRIME_VANDALISM = 3,
//     CRIME_LOCKPICKING = 4,
//     CRIME_PICK_POCKETS = 5,
//     CRIME_THEFT = 6,
//     CRIME_ASSAULT = 7,
//     CRIME_GBH = 8,
//     CRIME_MURDER = 9,
//     CRIME_LAST = 10,
// }

// /// Unknown variants.
// #[derive(Debug)]
// #[repr(C)]
// pub enum ECutsceneBehaviour {
//     UNKNOWN = 0,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum EDoorTriggerType {
//     DOOR_TRIGGER_ON_PERSON = 0,
//     DOOR_TRIGGER_MANUAL = 1,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum EGameAction {
//     GAME_ACTION_NULL = 0,
//     GAME_ACTION_LOCK_TARGET = 1,
//     GAME_ACTION_OPEN_INVENTORY = 2,
//     GAME_ACTION_OPEN_IN_GAME_MENU = 3,
//     GAME_ACTION_TOGGLE_MINI_MAP = 4,
//     GAME_ACTION_PAUSE = 5,
//     GAME_ACTION_INTERACT = 6,
//     GAME_ACTION_BLOCK = 7,
//     GAME_ACTION_SPECIAL_ATTACK = 8,
//     GAME_ACTION_ATTACK = 9,
//     GAME_ACTION_FIRE_RANGED_WEAPON = 10,
//     GAME_ACTION_UNARMED_ATTACK = 11,
//     GAME_ACTION_ARMED_MELEE_ATTACK = 12,
//     GAME_ACTION_UNSHEATHE_MELEE_WEAPON = 13,
//     GAME_ACTION_UNSHEATHE_RANGED_WEAPON = 14,
//     GAME_ACTION_SHEATHE_MELEE_WEAPON = 15,
//     GAME_ACTION_SHEATHE_RANGED_WEAPON = 16,
//     GAME_ACTION_TOGGLE_SILENT_MOVE = 17,
//     GAME_ACTION_TOGGLE_CINEMATIC_AND_USER_CAMERA = 18,
//     GAME_ACTION_TOGGLE_FIRST_PERSON_VIEW = 19,
//     GAME_ACTION_SKIP_PAST_TEXT = 20,
//     GAME_ACTION_SKIP_CUT_SCENE = 21,
//     GAME_ACTION_ANSWER_QUESTION_YES = 22,
//     GAME_ACTION_ANSWER_QUESTION_NO = 23,
//     GAME_ACTION_ANSWER_QUESTION_THIRD = 24,
//     GAME_ACTION_INVENTORY_SELECT = 25,
//     GAME_ACTION_ATTRACT_EXPERIENCE_ORBS = 26,
//     GAME_ACTION_ROTATE_GUI_SCREENS_LEFT = 27,
//     GAME_ACTION_ROTATE_GUI_SCREENS_RIGHT = 28,
//     GAME_ACTION_JUMP = 29,
//     GAME_ACTION_SPRINT = 30,
//     GAME_ACTION_RUN = 31,
//     GAME_ACTION_SNEAK = 32,
//     GAME_ACTION_INVENTORY_A = 33,
//     GAME_ACTION_INVENTORY_B = 34,
//     GAME_ACTION_INVENTORY_X = 35,
//     GAME_ACTION_INVENTORY_Y = 36,
//     GAME_ACTION_INVENTORY_UP = 37,
//     GAME_ACTION_INVENTORY_DOWN = 38,
//     GAME_ACTION_INVENTORY_LEFT = 39,
//     GAME_ACTION_INVENTORY_RIGHT = 40,
//     GAME_ACTION_INVENTORY_WHITE = 41,
//     GAME_ACTION_TAVERN_GAMES_INSTRUCTIONS = 42,
//     GAME_ACTION_FISHING_REEL_IN = 43,
//     GAME_ACTION_FISHING_CANCEL = 44,
//     GAME_ACTION_TOGGLE_FIRST_PERSON_TARGETING = 45,
//     GAME_ACTION_FIRST_PERSON_TARGET_LOCK = 46,
//     GAME_ACTION_FIRST_PERSON_ZOOM_IN = 47,
//     GAME_ACTION_GENERAL_LEAVE_PLAYER_MODE = 48,
//     GAME_ACTION_DEBUG_JUMP_1 = 49,
//     GAME_ACTION_DEBUG_JUMP_2 = 50,
//     GAME_ACTION_DEBUG_CAMERA = 51,
//     GAME_ACTION_DEBUG_SHIFT = 52,
//     GAME_ACTION_TAKE_PHOTO_FOR_PHOTOJOURNAL = 53,
//     GAME_ACTION_ASSIGNABLE_SPECIAL_MOVE = 54,
//     GAME_ACTION_QUICK_ACCESS_ITEM = 55,
//     GAME_ACTION_CONTEXT_SENSITIVE_ITEM = 56,
//     GAME_ACTION_CYCLE_THROUGH_SPELLS = 57,
//     GAME_ACTION_COIN_GOLF_CANCEL_AIM = 58,
//     GAME_ACTION_CONFIRM_RESET_TO_FRONT_END = 59,
//     GAME_ACTION_MOVEMENT = 60,
//     GAME_ACTION_CAMERA_ROTATE = 61,
//     GAME_ACTION_CAMERA_ROTATE_LEFT = 62,
//     GAME_ACTION_CAMERA_ROTATE_RIGHT = 63,
//     GAME_ACTION_CAMERA_ZOOM_IN = 64,
//     GAME_ACTION_CAMERA_ZOOM_OUT = 65,
//     GAME_ACTION_ORACLE_MINIGAME_UP = 66,
//     GAME_ACTION_ORACLE_MINIGAME_DOWN = 67,
//     GAME_ACTION_ORACLE_MINIGAME_LEFT = 68,
//     GAME_ACTION_ORACLE_MINIGAME_RIGHT = 69,
//     GAME_ACTION_ORACLE_MINIGAME_QUIT = 70,
//     GAME_ACTION_MOVE_MOUSE_ON_GUI = 71,
//     GAME_ACTION_TOGGLE_LIVE_GUI = 72,
//     GAME_ACTION_OPEN_EXPRESSION_MENU = 73,
//     GAME_ACTION_TOGGLE_DEACTIVATE_LOCK_TARGET = 74,
//     GAME_ACTION_FIRST_PERSON_LOOKAROUND = 75,
//     GAME_ACTION_INVENTORY_UNSELECT = 76,
//     GAME_ACTION_CAMERA_MOVE_DOUBLE_AXIS = 77,
//     GAME_ACTION_CHARGE_GUILD_SEAL = 78,
//     GAME_ACTION_TAVERN_GAME_MOVEMENT = 79,
//     GAME_ACTION_TAVERN_GAME_CAMERA = 80,
//     GAME_ACTION_TAVERN_GAME_ACTION_BUTTON = 81,
//     GAME_ACTION_TAVERN_GAME_ALTERNATE_BUTTON = 82,
//     GAME_ACTION_TAVERN_GAME_QUIT = 83,
//     GAME_ACTION_PROJECTILE_TARGETING_ANALOGUE_ZOOM = 84,
//     GAME_ACTION_TOGGLE_PASSIVE_AGGRESSIVE_MODE = 85,
//     GAME_ACTION_ACTIVATE_SPELL_MODE = 86,
//     GAME_ACTION_EXPRESSION_SHIFT = 87,
//     GAME_ACTION_SCROLL_DESCRIPTION_UP = 88,
//     GAME_ACTION_SCROLL_DESCRIPTION_DOWN = 89,
//     GAME_ACTION_OPEN_WEAPONS_MENU = 90,
//     GAME_ACTION_OPEN_CLOTHING_MENU = 91,
//     GAME_ACTION_OPEN_ITEMS_MENU = 92,
//     GAME_ACTION_OPEN_CURRENT_QUESTS_MENU = 93,
//     GAME_ACTION_CYCLE_THROUGH_SPELLS_KEYBOARD = 94,
//     GAME_ACTION_TOGGLE_KILL_EVERYTHING_MODE = 95,
//     GAME_ACTION_OPEN_MAGIC_MENU = 96,
//     GAME_ACTION_OPEN_EXPRESSIONS_MENU = 97,
//     GAME_ACTION_OPEN_PERSONALITY_MENU = 98,
//     GAME_ACTION_OPEN_LOGBOOK_MENU = 99,
//     GAME_ACTION_OPEN_MAP_MENU = 100,
//     GAME_ACTION_SCROLL_MENU = 101,
//     GAME_ACTION_ZOOM_MAP_OUT = 102,
//     GAME_ACTION_ZOOM_MAP_IN = 103,
//     GAME_ACTION_SCROLL_MAP_LEFT = 104,
//     GAME_ACTION_SCROLL_MAP_RIGHT = 105,
//     GAME_ACTION_SCROLL_MAP_DOWN = 106,
//     GAME_ACTION_SCROLL_MAP_UP = 107,
//     GAME_ACTION_OPTIONS_SLIDER_LEFT = 108,
//     GAME_ACTION_OPTIONS_SLIDER_RIGHT = 109,
//     GAME_ACTION_DIGITAL_ANALOGUE_ZOOM_IN = 110,
//     GAME_ACTION_DIGITAL_ANALOGUE_ZOOM_OUT = 111,
//     GAME_ACTION_TOGGLE_VIEW_HERO_MODE = 112,
//     GAME_ACTION_CENTRE_CAMERA = 113,
//     GAME_ACTION_BETTING = 114,
//     GAME_ACTION_COUNT = 115,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum EGlobalThingLoadingBehaviour {
//     LOAD_ON_STARTUP = 1,
//     LOAD_PER_LEVEL = 2,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum EHeroAbility {
//     HERO_ABILITY_NULL = 0,
//     HERO_ABILITY_FORCE_PUSH = 1,
//     HERO_ABILITY_TIME_SPELL = 2,
//     HERO_ABILITY_ENFLAME_SPELL = 3,
//     HERO_ABILITY_PHYSICAL_SHIELD_SPELL = 4,
//     HERO_ABILITY_TURNCOAT_SPELL = 5,
//     HERO_ABILITY_DRAIN_LIFE_SPELL = 6,
//     HERO_ABILITY_RAISE_DEAD_SPELL = 7,
//     HERO_ABILITY_BERSERK = 8,
//     HERO_ABILITY_DOUBLE_STRIKE = 9,
//     HERO_ABILITY_SUMMON_SPELL = 10,
//     HERO_ABILITY_LIGHTNING_SPELL = 11,
//     HERO_ABILITY_BATTLE_CHARGE = 12,
//     HERO_ABILITY_ASSASSIN_RUSH = 13,
//     HERO_ABILITY_HEAL_LIFE_SPELL = 14,
//     HERO_ABILITY_GHOST_SWORD_SPELL = 15,
//     HERO_ABILITY_FIREBALL_SPELL = 16,
//     HERO_ABILITY_MULTI_ARROW = 17,
//     HERO_ABILITY_DIVINE_WRATH_SPELL = 18,
//     HERO_ABILITY_UNHOLY_POWER_SPELL = 19,
//     MAX_NUMBER_OF_HERO_ABILITIES = 20,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum EHeroTitle /* stored as int32_t */ {
//     TITLE_NONE = 0,
//     TITLE_REAPER = 1,
//     TITLE_SHADOWHUNTER = 2,
//     TITLE_MALEFICUS = 3,
//     TITLE_DEATHBRINGER = 4,
//     TITLE_ASSASSIN = 5,
//     TITLE_NECROMANCER = 6,
//     TITLE_AVATAR = 7,
//     TITLE_PILGRIM = 8,
//     TITLE_LIBERATOR = 9,
//     TITLE_PALADIN = 10,
//     TITLE_DRUID = 11,
//     TITLE_RANGER = 12,
//     TITLE_RUNEMASTER = 13,
//     TITLE_HOOD = 14,
//     TITLE_GLADIATOR = 15,
//     TITLE_SABRE = 16,
//     TITLE_ARROWDODGER = 17,
//     TITLE_PIEMASTER = 18,
//     TITLE_CHICKEN_CHASER = 19,
//     TITLE_ARSEFACE = 20,
//     TITLE_JACK = 21,
//     TITLE_MAZE = 22,
//     TITLE_SCARLET_ROBE = 23,
//     TITLE_SCYTHE = 24,
//     TITLE_THUNDER = 25,
//     TITLE_WHISPER = 26,
//     TITLE_TWINBLADE = 27,
//     TITLE_BRIAR_ROSE = 28,
//     TITLE_LADY_GREY = 29,
//     TITLE_GUILDMASTER = 30,
//     TITLE_SCORPION_SLAYER = 31,
//     TITLE_DEATH_BRINGER = 32,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum EHeroTrainableStatType {
//     HERO_STAT_STRENGTH_PHYSIQUE = 0,
//     HERO_STAT_STRENGTH_HEALTH = 1,
//     HERO_STAT_STRENGTH_TOUGHNESS = 2,
//     HERO_STAT_SKILL_SPEED = 3,
//     HERO_STAT_SKILL_ACCURACY = 4,
//     HERO_STAT_SKILL_STEALTH = 5,
//     HERO_STAT_WILL_WEAPON_MAGIC = 6,
//     HERO_STAT_WILL_ABILITY_MAGIC = 7,
//     HERO_STAT_WILL_PURE_MAGIC = 8,
//     HERO_STAT_WILL_MAGIC_POWER = 9,
//     NUMBER_OF_TRAINABLE_HERO_STATS = 10,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum EMiniGameType {
//     MINIGAME_NONE = 0,
//     MINIGAME_FISHING = 1,
//     MINIGAME_DIGGING = 2,
//     MINIGAME_PICKPOCKET = 3,
//     MINIGAME_PICKLOCK = 4,
//     MINIGAME_STEAL = 5,
//     MINIGAME_TROPHY = 6,
//     MINIGAME_ORACLE = 7,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum EMorality {
//     MORALITY_SUPER_EVIL = 0,
//     MORALITY_VERY_EVIL = 1,
//     MORALITY_EVIL = 2,
//     MORALITY_NEUTRAL = 3,
//     MORALITY_GOOD = 4,
//     MORALITY_VERY_GOOD = 5,
//     MORALITY_SUPER_GOOD = 6,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum EMovementBandType {
//     MOVEMENT_BAND_STATIONARY = 0,
//     MOVEMENT_BAND_SLOW_WALKING = 1,
//     MOVEMENT_BAND_WALKING = 2,
//     MOVEMENT_BAND_RUNNING = 3,
//     MOVEMENT_BAND_JOGGING = 4,
//     MOVEMENT_BAND_SNEAKING = 5,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum EMusicSetType {
//     MUSIC_SET_NULL = 1,
//     MUSIC_SET_FRESCO_WEDDING = 2,
//     MUSIC_SET_CHAPEL_OF_EVIL = 3,
//     MUSIC_SET_GUILD = 4,
//     MUSIC_SET_GUILD_DAY = 5,
//     MUSIC_SET_GUILD_NIGHT = 6,
//     MUSIC_SET_HALL_OF_HEROES = 7,
//     MUSIC_SET_TEMPLE_OF_LIGHT = 8,
//     MUSIC_SET_ARENA_FANFARE = 9,
//     MUSIC_SET_BANDIT_CAMP = 10,
//     MUSIC_SET_BOWERSTONE = 11,
//     MUSIC_SET_CAVES = 12,
//     MUSIC_SET_DARKWOOD = 13,
//     MUSIC_SET_GRAVEYARD = 14,
//     MUSIC_SET_GRAVEYARD_PASSAGE = 15,
//     MUSIC_SET_GREATWOOD = 16,
//     MUSIC_SET_HAUNTED_HOUSE = 17,
//     MUSIC_SET_LOOKOUT_POINT = 18,
//     MUSIC_SET_OAKVALE = 19,
//     MUSIC_SET_WITCHWOOD = 20,
//     MUSIC_SET_QUEST_SUCCEEDED = 21,
//     MUSIC_SET_QUEST_FAILED = 22,
//     MUSIC_SET_BOSS = 23,
//     MUSIC_SET_DANGER = 24,
//     MUSIC_SET_PRISON = 25,
//     MUSIC_SET_HOOK_COAST = 26,
//     MUSIC_SET_KNOTHOLE_GLADE = 27,
//     MUSIC_SET_EXECUTION_TREE = 28,
//     MUSIC_SET_GIBBET_WOODS = 29,
//     MUSIC_SET_KRAKEN_CHAMBER = 30,
//     MUSIC_SET_INTERLUDE = 31,
//     MUSIC_SET_DANGER_ONLY = 32,
//     MUSIC_SET_DRAGON = 33,
//     MUSIC_SET_ARENA_FIGHT = 34,
//     MUSIC_SET_ARENA_FANFARE_01 = 35,
//     MUSIC_SET_ARENA_FANFARE_02 = 36,
//     MUSIC_SET_ARENA_FANFARE_03 = 37,
//     MUSIC_SET_ARENA_FANFARE_04 = 38,
//     MUSIC_SET_ARENA_FANFARE_05 = 39,
//     MUSIC_SET_ARENA_FANFARE_06 = 40,
//     MUSIC_SET_ARENA_FANFARE_07 = 41,
//     MUSIC_SET_ARENA_FANFARE_08 = 42,
//     MUSIC_SET_ARENA_FANFARE_09 = 43,
//     MUSIC_SET_ARENA_FANFARE_10 = 44,
//     MUSIC_SET_CUTSCENE_DEAD_DAD = 45,
//     MUSIC_SET_CUTSCENE_FEET = 46,
//     MUSIC_SET_CUTSCENE_GUILD_CEREMONY = 47,
//     MUSIC_SET_CUTSCENE_TWINBLADE = 48,
//     MUSIC_SET_CUTSCENE_THERESA_01 = 49,
//     MUSIC_SET_CUTSCENE_THERESA_02 = 50,
//     MUSIC_SET_CUTSCENE_WIZARD_BATTLE_INTRO = 51,
//     MUSIC_SET_CUTSCENE_WIZARD_BATTLE_OUTRO = 52,
//     MUSIC_SET_CUTSCENE_PRISON_MOTHER = 53,
//     MUSIC_SET_CUTSCENE_JACK_BOSS_FIGHT_INTRO = 54,
//     MUSIC_SET_CUTSCENE_JACK_BOSS_FIGHT_OUTRO = 55,
//     MUSIC_SET_CUTSCENE_JACK_CAPTURES = 56,
//     MUSIC_SET_CUTSCENE_GUILD_ARRIVAL = 57,
//     MUSIC_SET_CUTSCENE_COLLECT_FIREHEART = 58,
//     MUSIC_SET_CUTSCENE_DRAGON_DEATH = 59,
//     MUSIC_SET_CUTSCENE_DRAGON_FIGHT_INTRO = 60,
//     MUSIC_SET_CUTSCENE_DRAGON_FIGHT_OUTRO_EVIL = 61,
//     MUSIC_SET_CUTSCENE_DRAGON_FIGHT_OUTRO_GOOD = 62,
//     MUSIC_SET_CUTSCENE_DRAGON_FIGHT_OUTRO_CHOICE = 63,
//     MUSIC_SET_CUTSCENE_ONE_YEAR_LATER = 64,
//     MUSIC_SET_CUTSCENE_ORACLE_AWAKENS = 65,
//     MUSIC_SET_CUTSCENE_SCYTHE_MESSAGE = 66,
//     MUSIC_SET_CUTSCENE_SOUL2_MOTHER_INTRO = 67,
//     MUSIC_SET_CUTSCENE_SOUL2_MOTHER_SUCCESS = 68,
//     MUSIC_SET_CUTSCENE_SOUL3_GUILDMASTER_INTRO = 69,
//     MUSIC_SET_CUTSCENE_SUMMON_SHIP_INTRO = 70,
//     MUSIC_SET_CUTSCENE_SUMMON_SHIP_OUTRO = 71,
//     MUSIC_SET_CUTSCENE_SOUL3_GUILDMASTER_OUTRO_EVIL = 72,
//     MUSIC_SET_CUTSCENE_SOUL3_GUILDMASTER_OUTRO_GOOD = 73,
//     MUSIC_SET_CUTSCENE_GATE_OPENS_BRIAR_ROSE = 74,
//     MUSIC_SET_CUTSCENE_GATE_OPENS_SCYTHE = 75,
//     MUSIC_SET_INTRO = 76,
//     NO_MUSIC_SETS = 77,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum EOpinion {
//     OPINION_MORALITY = 0,
//     OPINION_RENOWN = 1,
//     OPINION_SCARINESS = 2,
//     OPINION_AGREEABLENESS = 3,
//     OPINION_ATTRACTIVENESS = 4,
//     OPINION_LAST = 5,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum EOpinionAttitudeType {
//     OPINION_ATTITUDE_TYPE_NONE = 0,
//     OPINION_ATTITUDE_TYPE_RESPECT = 1,
//     OPINION_ATTITUDE_TYPE_AWE = 2,
//     OPINION_ATTITUDE_TYPE_DISDAIN = 3,
//     OPINION_ATTITUDE_TYPE_FEAR = 4,
//     OPINION_ATTITUDE_TYPE_FRIENDLINESS = 5,
//     OPINION_ATTITUDE_TYPE_WORSHIP = 6,
//     OPINION_ATTITUDE_TYPE_RIDICULOUS = 7,
//     OPINION_ATTITUDE_TYPE_OFFENSIVE = 8,
//     OPINION_ATTITUDE_TYPE_AGREEABLE = 9,
//     OPINION_ATTITUDE_TYPE_UGLY = 10,
//     OPINION_ATTITUDE_TYPE_ATTRACTED = 11,
//     OPINION_ATTITUDE_TYPE_LOVE = 12,
//     // OPINION_ATTITUDE_TYPE_WIFE_FIRST = 13,
//     OPINION_ATTITUDE_TYPE_WIFE_LOVE = 13,
//     OPINION_ATTITUDE_TYPE_WIFE_LIKE = 14,
//     OPINION_ATTITUDE_TYPE_WIFE_NEUTRAL = 15,
//     OPINION_ATTITUDE_TYPE_WIFE_DISLIKE = 16,
//     OPINION_ATTITUDE_TYPE_WIFE_HATE = 17,
//     OPINION_ATTITUDE_TYPE_LAST = 18,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum EOpinionDeedType {
//     OPINION_DEED_TYPE_NONE = 0,
//     OPINION_DEED_TYPE_CRIME_WEAPON_OUT = 1,
//     OPINION_DEED_TYPE_CRIME_TRESPASS_THIRD = 2,
//     OPINION_DEED_TYPE_CRIME_VANDALISM = 3,
//     OPINION_DEED_TYPE_CRIME_LOCKPICKING = 4,
//     OPINION_DEED_TYPE_CRIME_PICK_POCKETS = 5,
//     OPINION_DEED_TYPE_CRIME_THEFT = 6,
//     OPINION_DEED_TYPE_CRIME_ASSAULT = 7,
//     OPINION_DEED_TYPE_CRIME_GBH = 8,
//     OPINION_DEED_TYPE_CRIME_MURDER = 9,
//     OPINION_DEED_TYPE_CRIME_TRESPASS_FIRST = 10,
//     OPINION_DEED_TYPE_CRIME_TRESPASS_SECOND = 11,
//     OPINION_DEED_TYPE_EXPRESSION_HEROIC_STANCE = 12,
//     OPINION_DEED_TYPE_EXPRESSION_FLIRT = 13,
//     OPINION_DEED_TYPE_EXPRESSION_APOLOGY_NO_CRIME = 14,
//     OPINION_DEED_TYPE_EXPRESSION_SNEER = 15,
//     OPINION_DEED_TYPE_EXPRESSION_EVIL_LAUGH = 16,
//     OPINION_DEED_TYPE_EXPRESSION_BATTLE_CRY = 17,
//     OPINION_DEED_TYPE_EXPRESSION_PELVIC_THRUST = 18,
//     OPINION_DEED_TYPE_EXPRESSION_MIDDLE_FINGER = 19,
//     OPINION_DEED_TYPE_EXPRESSION_BELCH = 20,
//     OPINION_DEED_TYPE_EXPRESSION_FART = 21,
//     OPINION_DEED_TYPE_EXPRESSION_VICTORY_PUMP = 22,
//     OPINION_DEED_TYPE_EXPRESSION_CLAP = 23,
//     OPINION_DEED_TYPE_EXPRESSION_GIGGLE = 24,
//     OPINION_DEED_TYPE_EXPRESSION_SHIT = 25,
//     OPINION_DEED_TYPE_EXPRESSION_THANKS = 26,
//     OPINION_DEED_TYPE_EXPRESSION_COCK_A_DOODLE_DO = 27,
//     OPINION_DEED_TYPE_EXPRESSION_CROTCH_GRAB = 28,
//     OPINION_DEED_TYPE_EXPRESSION_KISS_MY_ASS = 29,
//     OPINION_DEED_TYPE_EXPRESSION_FLAMENCO = 30,
//     OPINION_DEED_TYPE_EXPRESSION_COSSACK = 31,
//     OPINION_DEED_TYPE_EXPRESSION_AIR_GUITAR = 32,
//     OPINION_DEED_TYPE_EXPRESSION_BALLET = 33,
//     OPINION_DEED_TYPE_EXPRESSION_SATURDAY_NIGHT_FEVER = 34,
//     OPINION_DEED_TYPE_EXPRESSION_TAP = 35,
//     OPINION_DEED_TYPE_EXPRESSION_Y = 36,
//     OPINION_DEED_TYPE_EXPRESSION_M = 37,
//     OPINION_DEED_TYPE_EXPRESSION_C = 38,
//     OPINION_DEED_TYPE_EXPRESSION_A = 39,
//     OPINION_DEED_TYPE_EXPRESSION_THREATEN_SMALL = 40,
//     OPINION_DEED_TYPE_EXPRESSION_THREATEN_LARGE = 41,
//     OPINION_DEED_TYPE_SCRIPT_ACTION_ANNOYING_SMALL = 42,
//     OPINION_DEED_TYPE_SCRIPT_ACTION_ANNOYING_LARGE = 43,
//     OPINION_DEED_TYPE_SCRIPT_ACTION_NICE_SMALL = 44,
//     OPINION_DEED_TYPE_SCRIPT_ACTION_NICE_LARGE = 45,
//     OPINION_DEED_TYPE_FOLLOWER_ACCEPT = 46,
//     OPINION_DEED_TYPE_FOLLOWER_REFUSE = 47,
//     OPINION_DEED_TYPE_FOLLOWER_DISMISSED = 48,
//     OPINION_DEED_TYPE_FOLLOWER_QUIT = 49,
//     OPINION_DEED_TYPE_FOLLOWER_ENEMYSPOTTED = 50,
//     OPINION_DEED_TYPE_FOLLOWER_IDLEEXCITED = 51,
//     OPINION_DEED_TYPE_FOLLOWER_IDLEBORED = 52,
//     OPINION_DEED_TYPE_WITNESSED_ASSAULT_OR_GBH = 53,
//     OPINION_DEED_TYPE_TOO_FREQUENT_OTHER_DEED = 54,
//     OPINION_DEED_TYPE_SHOW_TROPHY_EVIL = 55,
//     OPINION_DEED_TYPE_SHOW_TROPHY_GOOD = 56,
//     OPINION_DEED_TYPE_SHOW_TROPHY_BORED = 57,
//     OPINION_DEED_TYPE_KILL_BAD_GUY = 58,
//     OPINION_DEED_TYPE_MURDER_SPOUSE = 59,
//     OPINION_DEED_TYPE_RECEIVE_GIFT_ROMANTIC = 60,
//     OPINION_DEED_TYPE_RECEIVE_GIFT_FRIENDLY = 61,
//     OPINION_DEED_TYPE_RECEIVE_GIFT_OFFENSIVE = 62,
//     OPINION_DEED_TYPE_RECEIVE_WEDDING_RING = 63,
//     OPINION_DEED_TYPE_BOAST_ANTICIPATION = 64,
//     OPINION_DEED_TYPE_BOAST_ENCOURAGE_FIRST = 65,
//     OPINION_DEED_TYPE_BOAST_ENCOURAGE_MIDDLE = 66,
//     OPINION_DEED_TYPE_BOAST_ENCOURAGE_FINAL = 67,
//     OPINION_DEED_TYPE_BOAST_WELL_WISHES = 68,
//     OPINION_DEED_TYPE_BOAST_ANNOYED_NO_BOASTING = 69,
//     OPINION_DEED_TYPE_COMMENT_AT_HERO = 70,
//     OPINION_DEED_TYPE_COMMENT_TO_SELF = 71,
//     OPINION_DEED_TYPE_COMMENT_ABOUT_HERO = 72,
//     OPINION_DEED_TYPE_GENERIC_INCOMPREHENSION = 73,
//     OPINION_DEED_TYPE_HIGH_PRIORITY_INCOMPREHENSION = 74,
//     OPINION_DEED_TYPE_HUSBAND_RAGE = 75,
//     OPINION_DEED_TYPE_TAVERN_GAME_WIN = 76,
//     OPINION_DEED_TYPE_INDOORS_GREETING = 77,
//     OPINION_DEED_TYPE_APOLOGY_ACCEPTED = 78,
//     OPINION_DEED_TYPE_APOLOGY_REFUSED = 79,
//     OPINION_DEED_TYPE_WIFE_GREETED = 80,
//     OPINION_DEED_TYPE_WIFE_TIME_SINCE_SEEING = 81,
//     OPINION_DEED_TYPE_WIFE_GIFT_RECEIVE_ALREADY_GOT = 82,
//     OPINION_DEED_TYPE_WIFE_JUSTMARRIED = 83,
//     OPINION_DEED_TYPE_WIFE_GIFT_WANTED = 84,
//     OPINION_DEED_TYPE_WIFE_WITNESSED_FLIRT = 85,
//     OPINION_DEED_TYPE_WIFE_HOUSE_DRESSING_GOOD = 86,
//     OPINION_DEED_TYPE_WIFE_HOUSE_DRESSING_BAD = 87,
//     OPINION_DEED_TYPE_WIFE_DIVORCE_WARNING = 88,
//     OPINION_DEED_TYPE_WIFE_DIVORCE_OCCURRED = 89,
//     OPINION_DEED_TYPE_WIFE_SEX_OFFER_TO_GO_TO_BED = 90,
//     OPINION_DEED_TYPE_WIFE_SEX_COMMENT_AFTERWARDS = 91,
//     OPINION_DEED_TYPE_LAST = 92,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum EOpinionPostDeedType {
//     OPINION_POST_DEED_TYPE_NULL = 0,
//     OPINION_POST_DEED_TYPE_RECIPIENT = 1,
//     OPINION_POST_DEED_TYPE_WITNESSES = 2,
//     OPINION_POST_DEED_TYPE_VILLAGE = 4,
//     OPINION_POST_DEED_TYPE_GUARDS = 16,
//     OPINION_POST_DEED_TYPE_RECIPIENT_AND_WITNESSES = 3,
//     OPINION_POST_DEED_TYPE_ALL = 7,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum EOpinionReactionType {
//     OPINION_REACTION_TYPE_NONE = 0,
//     OPINION_REACTION_TYPE_ATTITUDE_RESPECT = 1,
//     OPINION_REACTION_TYPE_ATTITUDE_AWE = 2,
//     OPINION_REACTION_TYPE_ATTITUDE_DISDAIN = 3,
//     OPINION_REACTION_TYPE_ATTITUDE_FEAR = 4,
//     OPINION_REACTION_TYPE_ATTITUDE_FRIENDLINESS = 5,
//     OPINION_REACTION_TYPE_ATTITUDE_WORSHIP = 6,
//     OPINION_REACTION_TYPE_ATTITUDE_RIDICULOUS = 7,
//     OPINION_REACTION_TYPE_ATTITUDE_OFFENSIVE = 8,
//     OPINION_REACTION_TYPE_ATTITUDE_AGREEABLE = 9,
//     OPINION_REACTION_TYPE_ATTITUDE_UGLY = 10,
//     OPINION_REACTION_TYPE_ATTITUDE_ATTRACTED = 11,
//     OPINION_REACTION_TYPE_ATTITUDE_LOVE = 12,
//     OPINION_REACTION_TYPE_ATTITUDE_WIFE_LOVE = 13,
//     OPINION_REACTION_TYPE_ATTITUDE_WIFE_LIKE = 14,
//     OPINION_REACTION_TYPE_ATTITUDE_WIFE_NEUTRAL = 15,
//     OPINION_REACTION_TYPE_ATTITUDE_WIFE_DISLIKE = 16,
//     OPINION_REACTION_TYPE_ATTITUDE_WIFE_HATE = 17,
//     OPINION_REACTION_TYPE_ANGRY_POINT = 18,
//     OPINION_REACTION_TYPE_BACK_AWAY = 19,
//     OPINION_REACTION_TYPE_BELLY_LAUGH = 20,
//     OPINION_REACTION_TYPE_BOO = 21,
//     OPINION_REACTION_TYPE_BOWING_LARGE = 22,
//     OPINION_REACTION_TYPE_BOWING_SMALL = 23,
//     OPINION_REACTION_TYPE_CALLING_CHILDREN = 24,
//     OPINION_REACTION_TYPE_CHEER_LARGE = 25,
//     OPINION_REACTION_TYPE_CHEER_SMALL = 26,
//     OPINION_REACTION_TYPE_CLAP_LARGE = 27,
//     OPINION_REACTION_TYPE_CLAP_SMALL = 28,
//     OPINION_REACTION_TYPE_COMMENT_ABOUT_HERO = 29,
//     OPINION_REACTION_TYPE_COMMENT_AT_HERO = 30,
//     OPINION_REACTION_TYPE_COMMENT_TO_SELF = 31,
//     OPINION_REACTION_TYPE_COWER = 32,
//     OPINION_REACTION_TYPE_DISMISS = 33,
//     OPINION_REACTION_TYPE_FLEE = 34,
//     OPINION_REACTION_TYPE_FOLLOW_CLOSE = 35,
//     OPINION_REACTION_TYPE_FOLLOW_FAR = 36,
//     OPINION_REACTION_TYPE_FRIENDLY_GREET = 37,
//     OPINION_REACTION_TYPE_GET_OUT = 38,
//     OPINION_REACTION_TYPE_GROVEL_LARGE = 39,
//     OPINION_REACTION_TYPE_GROVEL_SMALL = 40,
//     OPINION_REACTION_TYPE_HERO_IMITATION_PLAY = 41,
//     OPINION_REACTION_TYPE_HEROTITLE_AT_HERO = 42,
//     OPINION_REACTION_TYPE_HEROTITLE_TO_SELF = 43,
//     OPINION_REACTION_TYPE_HIDE = 44,
//     OPINION_REACTION_TYPE_MARRIAGE_COMMENT = 45,
//     OPINION_REACTION_TYPE_OFFER_GIFT_FRIENDLY = 46,
//     OPINION_REACTION_TYPE_OFFER_GIFT_WORSHIP = 47,
//     OPINION_REACTION_TYPE_PICK_FIGHT = 48,
//     OPINION_REACTION_TYPE_POINT = 49,
//     OPINION_REACTION_TYPE_POINT_LAUGH = 50,
//     OPINION_REACTION_TYPE_RESPECTFUL_GREET = 51,
//     OPINION_REACTION_TYPE_SHAKE_FIST = 52,
//     OPINION_REACTION_TYPE_SHOUT_AWE = 53,
//     OPINION_REACTION_TYPE_SNIGGER = 54,
//     OPINION_REACTION_TYPE_THUMBS_DOWN = 55,
//     OPINION_REACTION_TYPE_WATCH = 56,
//     // OPINION_REACTION_TYPE_WIFE_FIRST = 57,
//     OPINION_REACTION_TYPE_WIFE_FEELING_LOVE = 57,
//     OPINION_REACTION_TYPE_WIFE_GENERAL_LOVE = 58,
//     OPINION_REACTION_TYPE_WIFE_TOHERSELF_LOVE = 59,
//     OPINION_REACTION_TYPE_WIFE_CLOTHING_LOVE = 60,
//     OPINION_REACTION_TYPE_WIFE_FEELING_LIKE = 61,
//     OPINION_REACTION_TYPE_WIFE_GENERAL_LIKE = 62,
//     OPINION_REACTION_TYPE_WIFE_TOHERSELF_LIKE = 63,
//     OPINION_REACTION_TYPE_WIFE_CLOTHING_LIKE = 64,
//     OPINION_REACTION_TYPE_WIFE_FEELING_NEUTRAL = 65,
//     OPINION_REACTION_TYPE_WIFE_GENERAL_NEUTRAL = 66,
//     OPINION_REACTION_TYPE_WIFE_TOHERSELF_NEUTRAL = 67,
//     OPINION_REACTION_TYPE_WIFE_CLOTHING_NEUTRAL = 68,
//     OPINION_REACTION_TYPE_WIFE_FEELING_DISLIKE = 69,
//     OPINION_REACTION_TYPE_WIFE_GENERAL_DISLIKE = 70,
//     OPINION_REACTION_TYPE_WIFE_TOHERSELF_DISLIKE = 71,
//     OPINION_REACTION_TYPE_WIFE_CLOTHING_DISLIKE = 72,
//     OPINION_REACTION_TYPE_WIFE_FEELING_HATE = 73,
//     OPINION_REACTION_TYPE_WIFE_GENERAL_HATE = 74,
//     OPINION_REACTION_TYPE_WIFE_TOHERSELF_HATE = 75,
//     OPINION_REACTION_TYPE_WIFE_CLOTHING_HATE = 76,
//     OPINION_REACTION_TYPE_WIFE_JUSTMARRIED = 77,
//     OPINION_REACTION_TYPE_WIFE_GIFT_WANTED = 78,
//     // OPINION_REACTION_TYPE_WIFE_LAST = 79,
//     OPINION_REACTION_TYPE_LAST = 79,
// }

// #[allow(non_camel_case_types)]
// #[derive(Debug)]
// #[repr(C)]
// pub enum EPlayerMode {
//     PLAYER_MODE_NULL = 0x0000,
//     PLAYER_MODE_CONTROL_CREATURE = 0x0001,
//     PLAYER_MODE_Z_TARGET = 0x0002,
//     PLAYER_MODE_DEAD = 0x0003,
//     PLAYER_MODE_NAVIGATE_INVENTORY = 0x0004,
//     PLAYER_MODE_NAVIGATE_INVENTORY_CLOTHING = 0x0005,
//     PLAYER_MODE_NAVIGATE_INVENTORY_WEAPONS = 0x0006,
//     PLAYER_MODE_NAVIGATE_INVENTORY_ABILITIES_SCREEN = 0x0007,
//     PLAYER_MODE_NAVIGATE_INVENTORY_MAP_SCREEN = 0x0008,
//     PLAYER_MODE_NAVIGATE_INVENTORY_MAGIC_SCREEN = 0x0009,
//     PLAYER_MODE_NAVIGATE_INVENTORY_STATS_SCREEN = 0x000a,
//     PLAYER_MODE_NAVIGATE_INVENTORY_EXPERIENCE_SCREEN = 0x000b,
//     PLAYER_MODE_NAVIGATE_INVENTORY_TRADE_SCREEN = 0x000c,
//     PLAYER_MODE_NAVIGATE_INVENTORY_QUESTS_SCREEN = 0x000d,
//     PLAYER_MODE_CLICK_PAST_TEXT = 0x000e,
//     PLAYER_MODE_YES_NO_QUESTION = 0x000f,
//     PLAYER_MODE_FIRST_PERSON = 0x0010,
//     PLAYER_MODE_FREEZE_CONTROLS = 0x0011,
//     PLAYER_MODE_SPECIAL_ABILITY_FREEZE_CONTROLS = 0x0012,
//     PLAYER_MODE_CONTROL_CAMERA = 0x0013,
//     PLAYER_MODE_LOOK_THROUGH_WINDOW = 0x0014,
//     PLAYER_MODE_REBOOT_GAME = 0x0015,
//     PLAYER_MODE_FIRST_PERSON_TARGETING = 0x0016,
//     PLAYER_MODE_NAVIGATE_IN_GAME_MENU = 0x0017,
//     PLAYER_MODE_CONTROL_SPIRIT = 0x0018,
//     PLAYER_MODE_USE_PROJECTILE_WEAPON = 0x0019,
//     PLAYER_MODE_TAVERN_GAME = 0x001a,
//     PLAYER_MODE_CUT_SCENE = 0x001b,
//     PLAYER_MODE_CHARGE_QUICK_ACCESS = 0x001c,
//     PLAYER_MODE_FISHING = 0x001d,
//     PLAYER_MODE_DIGGING = 0x001e,
//     PLAYER_MODE_PARALYSED = 0x001f,
//     PLAYER_MODE_BOAST_UI = 0x0020,
//     PLAYER_MODE_BERSERK = 0x0021,
//     PLAYER_MODE_USE_PROJECTILE_WEAPON_THIRD_PERSON = 0x0022,
//     PLAYER_MODE_CHARGE_UP_WILL_SPELL = 0x0023,
//     PLAYER_MODE_QUICK_ACCESS_MENU = 0x0024,
//     PLAYER_MODE_QUEST_COMPLETION_UI = 0x0025,
//     PLAYER_MODE_CREDITS_UI = 0x0026,
//     PLAYER_MODE_BETTING = 0x0027,
//     PLAYER_MODE_LIGHTNING = 0x0028,
//     PLAYER_MODE_ORACLE_MINIGAME = 0x0029,
//     PLAYER_MODE_FIREHEART_MINIGAME = 0x002a,
//     PLAYER_MODE_LIVE_GUI = 0x002b,
//     PLAYER_MODE_CONSOLE = 0x002c,
//     PLAYER_MODE_TAKE_SCREENSHOT_FOR_PHOTOJOURNAL = 0x002d,
//     PLAYER_MODE_PC_PROJECTILE_WEAPON_THIRD_PERSON_AIMING = 0x002e,
//     PLAYER_MODE_VIEW_HERO = 0x002f,
//     MAX_NO_PLAYER_MODES = 0x0030,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum EScriptAIPriority {
//     SCRIPT_AI_PRIORITY_LOWEST = 0,
//     SCRIPT_AI_PRIORITY_LOW = 1,
//     SCRIPT_AI_PRIORITY_MEDIUM = 2,
//     SCRIPT_AI_PRIORITY_HIGH = 3,
//     SCRIPT_AI_PRIORITY_HIGHEST = 4,
// }

// /// Unknown variants.
// #[derive(Debug)]
// #[repr(C)]
// pub enum EScriptVillageAttitude {
//     UNKNOWN = 0,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum EScriptingStateGroups {
//     ESSG_NONE = 0,
//     ESSG_PERFORM_ACTION_PHYSICAL = 1,
//     ESSG_PERFORM_ACTION_VERBAL = 2,
//     ESSG_PERFORM_ACTION_AURAL = 3,
//     ESSG_WANDER_NEAR = 4,
//     ESSG_FOLLOW_PATH = 5,
//     ESSG_FOLLOW_RANDOM = 6,
//     ESSG_FOLLOW_NEAREST = 7,
//     ESSG_WALK_TO_RANDOM = 8,
//     ESSG_WALK_TO_NEAREST_DIFFERENT = 9,
//     ESSG_RUN_AT_HERO_AND_ATTACK_UNTIL_DEAD = 10,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum ESex {
//     SEX_NULL = 0,
//     SEX_MALE = 1,
//     SEX_FEMALE = 2,
//     NO_OF_SEXES = 3,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum ESystemCode {
//     SYS_OK = 1,
//     SYS_ERROR_INIT_DISPLAY_MANAGER = 2,
//     SYS_ERROR_INIT_SOUND_MANAGER = 3,
//     SYS_ERROR_INIT_WINDOW = 4,
//     SYS_ERROR_INIT_DRIVE_MANAGER = 5,
//     SYS_ERROR_INIT_JOYSTICK1 = 6,
//     SYS_ERROR_INIT_JOYSTICK2 = 7,
//     SYS_ERROR_INIT_JOYSTICK3 = 8,
//     SYS_ERROR_INIT_JOYSTICK4 = 9,
//     SYS_ERROR_INIT_MOUSE = 10,
//     SYS_ERROR_INIT_KEYBOARD = 11,
//     SYS_ERROR_INIT_INPUT_MANAGER = 12,
//     SYS_ERROR_INIT_RENDER_MANAGER = 13,
//     SYS_ERROR_INIT_DEBUG_MANAGER = 14,
//     SYS_ERROR_INIT_SYSTEM_MANAGER = 15,
//     SYS_ERROR_INIT_DDRAW = 16,
//     SYS_ERROR_INIT_NETWORK_MANAGER = 17,
//     SYS_ERROR_INIT_DLL_MANAGER = 18,
//     SYS_ERROR_INIT_GENERIC = 19,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum ETargetingType {
//     TARGETING_NULL = 0,
//     TARGETING_USABLE = 1,
//     TARGETING_TALKABLE = 2,
//     TARGETING_STAB = 4,
//     TARGETING_SHOOTABLE = 8,
//     TARGETING_MELEE = 16,
//     TARGETING_ZTARGETING = 32,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum ETextGroupSelectionMethod {
//     GROUP_SELECT_NONE = 0,
//     GROUP_SELECT_RANDOM = 1,
//     GROUP_SELECT_RANDOM_NO_REPEAT = 2,
//     GROUP_SELECT_SEQUENTIAL = 3,
//     GROUP_SELECT_FIRST_ONLY = 4,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum ETutorialCategory {
//     TUTORIAL_CATEGORY_NONE = 0,
//     TUTORIAL_CATEGORY_ABILITY_ASSIGNING = 1,
//     TUTORIAL_CATEGORY_ABILITY_CYCLING = 2,
//     TUTORIAL_CATEGORY_BASIC_OBJECTS = 3,
//     TUTORIAL_CATEGORY_BED = 4,
//     TUTORIAL_CATEGORY_BOASTING = 5,
//     TUTORIAL_CATEGORY_CAMERA = 6,
//     TUTORIAL_CATEGORY_CHARITY_SHOP = 7,
//     TUTORIAL_CATEGORY_CHEST = 8,
//     TUTORIAL_CATEGORY_COMBAT_MULTIPLIER = 9,
//     TUTORIAL_CATEGORY_CREATURE_DROP = 10,
//     TUTORIAL_CATEGORY_DYING = 11,
//     TUTORIAL_CATEGORY_DEMON_DOOR = 12,
//     TUTORIAL_CATEGORY_DOOR = 13,
//     TUTORIAL_CATEGORY_EXPERIENCE = 14,
//     TUTORIAL_CATEGORY_EXPERIENCE_SPENDING = 15,
//     TUTORIAL_CATEGORY_EXPRESSION = 16,
//     TUTORIAL_CATEGORY_FLIRTING = 17,
//     TUTORIAL_CATEGORY_FLOURISHING_MOVE = 18,
//     TUTORIAL_CATEGORY_GOLDMARKERS = 19,
//     TUTORIAL_CATEGORY_GUILD_SEAL = 20,
//     TUTORIAL_CATEGORY_INTERACTING = 21,
//     TUTORIAL_CATEGORY_INVENTORY = 22,
//     TUTORIAL_CATEGORY_INVENTORY_ASSIGNING = 23,
//     TUTORIAL_CATEGORY_LEVELLING_UP = 24,
//     TUTORIAL_CATEGORY_MORALITY = 25,
//     TUTORIAL_CATEGORY_MOVEMENT = 26,
//     TUTORIAL_CATEGORY_QUEST = 27,
//     TUTORIAL_CATEGORY_QUEST_CARD = 28,
//     TUTORIAL_CATEGORY_RENOWN = 29,
//     TUTORIAL_CATEGORY_TAKING_QUESTS = 30,
//     TUTORIAL_CATEGORY_TELEPORTING = 31,
//     TUTORIAL_CATEGORY_TRADE_ITEM = 32,
//     TUTORIAL_CATEGORY_SEARCHING = 33,
//     TUTORIAL_CATEGORY_SNEAK = 34,
//     TUTORIAL_CATEGORY_BUILDING_OWNERSHIP = 35,
//     TUTORIAL_CATEGORY_FISHING_GAME = 36,
//     TUTORIAL_CATEGORY_ORACLE_GAME = 37,
//     TUTORIAL_CATEGORY_WORLD_MAP = 38,
//     TUTORIAL_CATEGORY_ALCOHOL = 39,
//     TUTORIAL_CATEGORY_AUGMENTATION = 40,
//     TUTORIAL_CATEGORY_ARMOUR = 41,
//     TUTORIAL_CATEGORY_BOMB = 42,
//     TUTORIAL_CATEGORY_CLOTHES = 43,
//     TUTORIAL_CATEGORY_FOOD = 44,
//     TUTORIAL_CATEGORY_FISHING_ROD = 45,
//     TUTORIAL_CATEGORY_GIFT = 46,
//     TUTORIAL_CATEGORY_HAIRSTYLE = 47,
//     TUTORIAL_CATEGORY_POTION = 48,
//     TUTORIAL_CATEGORY_RESURRECTION_PHIAL = 49,
//     TUTORIAL_CATEGORY_SILVER_KEY = 50,
//     TUTORIAL_CATEGORY_SPADE = 51,
//     TUTORIAL_CATEGORY_TATTOO = 52,
//     TUTORIAL_CATEGORY_TROPHY = 53,
//     TUTORIAL_CATEGORY_WEAPON = 54,
//     TUTORIAL_CATEGORY_WEAPON_LEGENDARY = 55,
//     TUTORIAL_CATEGORY_APOLOGY = 56,
//     TUTORIAL_CATEGORY_BATTLE_CRY = 57,
//     TUTORIAL_CATEGORY_BELCH = 58,
//     TUTORIAL_CATEGORY_EVIL_LAUGH = 59,
//     TUTORIAL_CATEGORY_FART = 60,
//     TUTORIAL_CATEGORY_FLIRT = 61,
//     TUTORIAL_CATEGORY_FOLLOW = 62,
//     TUTORIAL_CATEGORY_GIGGLE = 63,
//     TUTORIAL_CATEGORY_HEROIC_STANCE = 64,
//     TUTORIAL_CATEGORY_MIDDLE_FINGER = 65,
//     TUTORIAL_CATEGORY_PELVIC_THRUST = 66,
//     TUTORIAL_CATEGORY_PICKLOCK = 67,
//     TUTORIAL_CATEGORY_PICKPOCKET = 68,
//     TUTORIAL_CATEGORY_SHIT = 69,
//     TUTORIAL_CATEGORY_SNEER = 70,
//     TUTORIAL_CATEGORY_STEAL = 71,
//     TUTORIAL_CATEGORY_THANKS = 72,
//     TUTORIAL_CATEGORY_VICTORY_PUMP = 73,
//     TUTORIAL_CATEGORY_WAIT = 74,
//     TUTORIAL_CATEGORY_COCK_A_DOODLE_DO = 75,
//     TUTORIAL_CATEGORY_CROTCH_GRAB = 76,
//     TUTORIAL_CATEGORY_KISS_MY_ASS = 77,
//     TUTORIAL_CATEGORY_FLAMENCO = 78,
//     TUTORIAL_CATEGORY_COSSACK = 79,
//     TUTORIAL_CATEGORY_AIR_GUITAR = 80,
//     TUTORIAL_CATEGORY_BALLET = 81,
//     TUTORIAL_CATEGORY_SATURDAY_NIGHT_FEVER = 82,
//     TUTORIAL_CATEGORY_TAP = 83,
//     TUTORIAL_CATEGORY_Y = 84,
//     TUTORIAL_CATEGORY_M = 85,
//     TUTORIAL_CATEGORY_C = 86,
//     TUTORIAL_CATEGORY_A = 87,
//     TUTORIAL_CATEGORY_CRIME_WEAPONOUT = 88,
//     TUTORIAL_CATEGORY_CRIME_TRESPASSING = 89,
//     TUTORIAL_CATEGORY_CRIME_VANDALISM = 90,
//     TUTORIAL_CATEGORY_CRIME_THEFT = 91,
//     TUTORIAL_CATEGORY_CRIME_ASSAULT = 92,
//     TUTORIAL_CATEGORY_CRIME_GBH = 93,
//     TUTORIAL_CATEGORY_CRIME_MURDER = 94,
//     TUTORIAL_CATEGORY_COUNT = 95,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum EEventType {
// 	EVENT_HIT_BY = 1,
// 	EVENT_TALKED_TO = 2,
// 	EVENT_EXPRESSION_PERFORMED_TO = 3,
// 	EVENT_APOLOGY_RECEIVED = 4,
// 	EVENT_ITEM_USED = 5,
// 	EVENT_DIED = 6,
// 	EVENT_TRIGGERED = 7,
// 	EVENT_KNOCKED_OUT = 8,
// 	EVENT_RECEIVED_MONEY = 9,
// 	EVENT_RECEIVED_ITEM = 10,
// 	EVENT_SHOWN_ITEM = 11,
// 	EVENT_SOLD_ITEM = 12,
// 	EVENT_KILL_LOCKED = 13,
// 	EVENT_VILLAGE_MOB_AWESOME_HERO = 14,
// 	EVENT_VILLAGE_MOB_LYNCHED_HERO = 15,
// 	EVENT_VILLAGE_HERO_WELCOMED = 16,
// 	EVENT_VILLAGE_RESPONDED_TO_HERO = 17,
// 	EVENT_HERO_HAS_BEEN_SPOKEN_TO_BY_VILLAGERS = 18,
// 	EVENT_HERO_HAS_BEEN_SPOTTED_PICKPOCKETING = 19,
// 	EVENT_HERO_HAS_BEEN_SPOTTED_PICKINGLOCK = 20,
// 	EVENT_HERO_HAS_BEEN_SPOTTED_STEALING = 21,
// 	EVENT_CREATURE_INTERACTION_TARGET = 22,
// 	EVENT_CREATURE_READY_FOR_INTERACTION = 23,
// 	EVENT_CHILD_HURT_GO_TO_MOTHER = 24,
// 	EVENT_VILLAGER_INTERACTION = 25,
// 	EVENT_PERFORMED_SPECIAL_ABILITY = 26,
// 	EVENT_HIT_BY_SPECIAL_ABILITY = 27,
// 	EVENT_OPENED_CHEST = 28,
// 	EVENT_HIT_THING = 29,
// 	EVENT_ATTACKED_WITH_MELEE_WEAPON_WITHOUT_HITTING_ANYTHING = 30,
// 	EVENT_ATTACKED_WITH_BARE_HANDS_WITHOUT_HITTING_ANYTHING = 31,
// 	EVENT_ATTACKED_WITH_FLOURISH_WITHOUT_HITTING_ANYTHING = 32,
// 	EVENT_TALKED_TO_ANYONE = 33,
// 	EVENT_KILLED_CREATURE = 34,
// 	EVENT_MORALITY_CHANGED = 35,
// 	EVENT_CUT_SCENE_ANIM_EVENT = 36,
// 	EVENT_CREATURE_CHARGING = 37,
// 	EVENT_FIRED_SHOT = 38,
// 	EVENT_SHOT_STRUCK = 39,
// 	EVENT_RECEIVED_ITEM_IN_INVENTORY = 40,
// 	EVENT_KICKED = 41,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum ESurfaceMultisampleType {
//     SURFACE_MULTISAMPLE_NONE = 0x00,
//     SURFACE_MULTISAMPLE_2 = 0x02,
//     SURFACE_MULTISAMPLE_4 = 0x04,
//     SURFACE_MULTISAMPLE_9 = 0x09,
//     SURFACE_MULTISAMPLE_3 = 0x03,
//     SURFACE_MULTISAMPLE_5 = 0x05,
//     SURFACE_MULTISAMPLE_6 = 0x06,
//     SURFACE_MULTISAMPLE_7 = 0x07,
//     SURFACE_MULTISAMPLE_8 = 0x08,
//     SURFACE_MULTISAMPLE_10 = 0x0a,
//     SURFACE_MULTISAMPLE_11 = 0x0b,
//     SURFACE_MULTISAMPLE_12 = 0x0c,
//     SURFACE_MULTISAMPLE_13 = 0x0d,
//     SURFACE_MULTISAMPLE_14 = 0x0e,
//     SURFACE_MULTISAMPLE_15 = 0x0f,
//     SURFACE_MULTISAMPLE_16 = 0x10,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum EPrimitiveType {
//     PT_NULL = 0xff,
//     PT_TRIANGLE_LIST = 0x04,
//     PT_TRIANGLE_STRIP = 0x05,
//     PT_TRIANGLE_FAN = 0x06,
//     PT_LINE_LIST = 0x02,
//     PT_LINE_STRIP = 0x03,
//     PT_POINT_LIST = 0x01,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum EShaderTarget {
//     SHADER_TARGET_PC = 0,
//     SHADER_TARGET_XBOX = 1,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum EType {
//     TYPE_DEFINE = 0,
//     TYPE_EXCLUSIVE_NAME = 1,
//     TYPE_DYNAMIC_CONST = 2,
//     TYPE_MACRO = 3,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum EVertexType {
//     VERTEX_NULL = 0x0000,
//     VERTEX_RHW_COL_TEX1 = 0x0144,
//     VERTEX_RHW_COL_SPEC_TEX1 = 0x01c4,
//     VERTEX_POS = 0x0002,
//     VERTEX_NORM_TEX1 = 0x0112,
//     VERTEX_NORM_TEX2 = 0x0312,
//     VERTEX_NORM_COL_TEX1 = 0x0152,
//     VERTEX_NORM_COL_TEX2 = 0x0352,
//     VERTEX_NORM_COL_SPEC_TEX1 = 0x01d2,
//     VERTEX_COL_TEX1 = 0x0142,
//     VERTEX_COL_TEX2 = 0x0342,
//     VERTEX_COL = 0x0042,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum EVertexStreamDataType {
//     VERTEX_STREAM_DATA_TYPE_FLOAT1 = 0,
//     VERTEX_STREAM_DATA_TYPE_FLOAT2 = 1,
//     VERTEX_STREAM_DATA_TYPE_FLOAT3 = 2,
//     VERTEX_STREAM_DATA_TYPE_FLOAT4 = 3,
//     VERTEX_STREAM_DATA_TYPE_D3DCOLOR = 4,
//     VERTEX_STREAM_DATA_TYPE_SHORT2 = 5,
//     VERTEX_STREAM_DATA_TYPE_SHORT4 = 6,
//     VERTEX_STREAM_DATA_TYPE_UVSHORT2 = 7,
//     VERTEX_STREAM_DATA_TYPE_NORMSHORT1 = 8,
//     VERTEX_STREAM_DATA_TYPE_NORMSHORT2 = 9,
//     VERTEX_STREAM_DATA_TYPE_NORMSHORT3 = 10,
//     VERTEX_STREAM_DATA_TYPE_NORMSHORT4 = 11,
//     VERTEX_STREAM_DATA_TYPE_NORMPACKED3 = 12,
//     VERTEX_STREAM_DATA_TYPE_POSPACKED3 = 13,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum EDrawMode {
//     DRAW_NONE = 0,
//     DRAW_HIERARCHY = 1,
//     DRAW_TOTALS = 2,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum EEvalMode {
//     EVAL_MIN = 0,
//     EVAL_AVERAGE = 1,
// }

// #[derive(Debug,Copy,Clone)]
// #[repr(C)]
// pub enum EInputKey {
//     KB_NULL = 0,
//     KB_ESC = 1,
//     KB_1 = 2,
//     KB_2 = 3,
//     KB_3 = 4,
//     KB_4 = 5,
//     KB_5 = 6,
//     KB_6 = 7,
//     KB_7 = 8,
//     KB_8 = 9,
//     KB_9 = 10,
//     KB_0 = 11,
//     KB_MINUS = 12,
//     KB_EQUALS = 13,
//     KB_BACKSPACE = 14,
//     KB_TAB = 15,
//     KB_Q = 16,
//     KB_W = 17,
//     KB_E = 18,
//     KB_R = 19,
//     KB_T = 20,
//     KB_Y = 21,
//     KB_U = 22,
//     KB_I = 23,
//     KB_O = 24,
//     KB_P = 25,
//     KB_LBRACKET = 26,
//     KB_RBRACKET = 27,
//     KB_RETURN = 28,
//     KB_LCONTROL = 29,
//     KB_A = 30,
//     KB_S = 31,
//     KB_D = 32,
//     KB_F = 33,
//     KB_G = 34,
//     KB_H = 35,
//     KB_J = 36,
//     KB_K = 37,
//     KB_L = 38,
//     KB_SEMICOLON = 39,
//     KB_APOSTROPHE = 40,
//     KB_HASH = 41,
//     KB_LSHIFT = 42,
//     KB_BACKSLASH = 43,
//     KB_Z = 44,
//     KB_X = 45,
//     KB_C = 46,
//     KB_V = 47,
//     KB_B = 48,
//     KB_N = 49,
//     KB_M = 50,
//     KB_COMMA = 51,
//     KB_FULLSTOP = 52,
//     KB_SLASH = 53,
//     KB_RSHIFT = 54,
//     KB_PMULTIPLY = 55,
//     KB_LALT = 56,
//     KB_SPACE = 57,
//     KB_CAPSLOCK = 58,
//     KB_F1 = 59,
//     KB_F2 = 60,
//     KB_F3 = 61,
//     KB_F4 = 62,
//     KB_F5 = 63,
//     KB_F6 = 64,
//     KB_F7 = 65,
//     KB_F8 = 66,
//     KB_F9 = 67,
//     KB_F10 = 68,
//     KB_NUMLOCK = 69,
//     KB_SCROLLLOCK = 70,
//     KB_P7 = 71,
//     KB_P8 = 72,
//     KB_P9 = 73,
//     KB_PMINUS = 74,
//     KB_P4 = 75,
//     KB_P5 = 76,
//     KB_P6 = 77,
//     KB_PPLUS = 78,
//     KB_P1 = 79,
//     KB_P2 = 80,
//     KB_P3 = 81,
//     KB_P0 = 82,
//     KB_PFULLSTOP = 83,
//     KB_F11 = 84,
//     KB_F12 = 85,
//     KB_F13 = 86,
//     KB_F14 = 87,
//     KB_F15 = 88,
//     KB_KANA = 89,
//     KB_CONVERT = 90,
//     KB_NOCONVERT = 91,
//     KB_YEN = 92,
//     KB_PEQUALS = 93,
//     KB_CIRCUMFLEX = 94,
//     KB_AT = 95,
//     KB_COLON = 96,
//     KB_UNDERLINE = 97,
//     KB_KANJI = 98,
//     KB_STOP = 99,
//     KB_AX = 100,
//     KB_UNLABELED = 101,
//     KB_PENTER = 102,
//     KB_RCONTROL = 103,
//     KB_PCOMMA = 104,
//     KB_PDIVIDE = 105,
//     KB_SYSRQ = 106,
//     KB_RALT = 107,
//     KB_HOME = 108,
//     KB_UP = 109,
//     KB_PAGEUP = 110,
//     KB_LEFT = 111,
//     KB_RIGHT = 112,
//     KB_END = 113,
//     KB_DOWN = 114,
//     KB_PAGEDOWN = 115,
//     KB_INSERT = 116,
//     KB_DELETE = 117,
//     KB_LWIN = 118,
//     KB_RWIN = 119,
//     KB_APPS = 120,
//     KB_PAUSE = 121,
//     NO_INPUT_KEYS = 122,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum EInputDeviceType {
//     INPUT_DEVICE_TYPE_NULL = 0,
//     INPUT_DEVICE_TYPE_JOYSTICK = 1,
//     INPUT_DEVICE_TYPE_KEYBOARD = 2,
//     INPUT_DEVICE_TYPE_MOUSE = 3,
// }

// #[derive(Debug)]
// #[repr(C)]
// pub enum EInputEventType {
//     IE_NULL = 0,
//     IE_KEY_PRESSED = 1,
//     IE_KEY_HELD = 2,
//     IE_KEY_RELEASED = 3,
//     IE_LMB_PRESSED = 4,
//     IE_LMB_HELD = 5,
//     IE_LMB_RELEASED = 6,
//     IE_MMB_PRESSED = 7,
//     IE_MMB_HELD = 8,
//     IE_MMB_RELEASED = 9,
//     IE_RMB_PRESSED = 10,
//     IE_RMB_HELD = 11,
//     IE_RMB_RELEASED = 12,
//     IE_MOUSE_MOVEMENT = 13,
//     IE_MOUSE_WHEEL_MOVEMENT = 14,
//     IE_CHAR_PRESSED = 15,
//     IE_CHAR_RELEASED = 16,
//     IE_JOYSTICK_POSITION = 17,
//     IE_JOYSTICK_POSITION2 = 18,
//     IE_JOYSTICK_BUTTON_PRESSED = 19,
//     IE_JOYSTICK_BUTTON_HELD = 20,
//     IE_JOYSTICK_BUTTON_RELEASED = 21,
//     IE_MB4_PRESSED = 22,
//     IE_MB5_PRESSED = 23,
//     IE_MB6_PRESSED = 24,
//     IE_MB7_PRESSED = 25,
//     IE_MB8_PRESSED = 26,
//     IE_MB4_HELD = 27,
//     IE_MB5_HELD = 28,
//     IE_MB6_HELD = 29,
//     IE_MB7_HELD = 30,
//     IE_MB8_HELD = 31,
//     IE_MB4_RELEASED = 32,
//     IE_MB5_RELEASED = 33,
//     IE_MB6_RELEASED = 34,
//     IE_MB7_RELEASED = 35,
//     IE_MB8_RELEASED = 36,
// }

// /// An unknown type with zero bytes.
// #[derive(Debug)]
// #[repr(C)]
// pub struct UnknownEmptyType;
