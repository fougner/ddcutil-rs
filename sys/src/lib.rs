#![doc(html_root_url = "http://arcnmx.github.io/ddcutil-rs/")]
#![allow(non_camel_case_types, non_snake_case)]

extern crate libc;

use std::fmt;
use std::slice::from_raw_parts;
use libc::{c_char, c_int, c_void};

#[link(name = "ddcutil")]
extern {
    pub fn ddca_ddcutil_version() -> DDCA_Ddcutil_Version_Spec;
    pub fn ddca_ddcutil_version_string() -> *const c_char;

    pub fn ddca_rc_name(status_code: DDCA_Status) -> *mut c_char;
    pub fn ddca_rc_desc(status_code: DDCA_Status) -> *mut c_char;

    pub fn ddca_mccs_version_id_name(version_id: DDCA_MCCS_Version_Id) -> *mut c_char;
    pub fn ddca_mccs_version_id_desc(version_id: DDCA_MCCS_Version_Id) -> *mut c_char;

    pub fn ddca_max_max_tries() -> c_int;
    pub fn ddca_get_max_tries(retry_type: DDCA_Retry_Type) -> c_int;
    pub fn ddca_set_max_tries(retry_type: DDCA_Retry_Type, max_tries: c_int) -> DDCA_Status;

    pub fn ddca_enable_verify(onoff: bool);
    pub fn ddca_is_verify_enabled() -> bool;

    pub fn ddca_get_output_level() -> DDCA_Output_Level;
    pub fn ddca_set_output_level(newval: DDCA_Output_Level);
    pub fn ddca_enable_report_ddc_errors(onoff: bool);
    pub fn ddca_is_report_ddc_errors_enabled() -> bool;

    pub fn ddca_get_display_info_list2(
        include_invalid_displays: bool,
        dlist_loc: *mut *mut DDCA_Display_Info_List,
    ) -> DDCA_Status;

    pub fn ddca_free_display_info_list(dlist: *mut DDCA_Display_Info_List);
    pub fn ddca_report_display_info(dinfo: *mut DDCA_Display_Info, depth: c_int);
    pub fn ddca_report_display_info_list(dlist: *mut DDCA_Display_Info_List, depth: c_int);
    pub fn ddca_report_active_displays(depth: c_int) -> c_int;

    pub fn ddca_open_display2(
        ddca_dref: DDCA_Display_Ref,
        p_ddca_dh: *mut DDCA_Display_Handle,
    ) -> DDCA_Status;
    pub fn ddca_close_display(ddca_dh: DDCA_Display_Handle) -> DDCA_Status;

    pub fn ddca_get_capabilities_string(
        ddca_dh: DDCA_Display_Handle,
        p_caps: *mut *mut c_char,
    ) -> DDCA_Status;

    pub fn ddca_parse_capabilities_string(
        capabilities_string: *mut c_char,
        p_parsed_capabilities: *mut *mut DDCA_Capabilities,
    ) -> DDCA_Status;

    pub fn ddca_free_parsed_capabilities(
        pcaps: *mut DDCA_Capabilities,
    );

    pub fn ddca_report_parsed_capabilities(
        pcaps: *mut DDCA_Capabilities,
        depth: c_int,
    );

    pub fn ddca_get_feature_metadata_by_vspec(
        feature_code: DDCA_Vcp_Feature_Code,
        vspec: DDCA_MCCS_Version_Spec,
        create_default_if_not_found: bool,
        meta_loc: *mut *mut DDCA_Feature_Metadata,
    ) -> DDCA_Status;

    pub fn ddca_get_feature_metadata_by_dref(
        feature_code: DDCA_Vcp_Feature_Code,
        ddca_dref: DDCA_Display_Ref,
        create_default_if_not_found: bool,
        meta_loc: *mut *mut DDCA_Feature_Metadata,
    ) -> DDCA_Status;

    pub fn ddca_get_feature_metadata_by_dh(
        feature_code: DDCA_Vcp_Feature_Code,
        ddca_dh: DDCA_Display_Handle,
        create_default_if_not_found: bool,
        meta_loc: *mut *mut DDCA_Feature_Metadata,
    ) -> DDCA_Status;

    pub fn ddca_get_feature_name(feature_code: DDCA_Vcp_Feature_Code) -> *mut c_char;

    pub fn ddca_get_simple_sl_value_table(
        feature_code: DDCA_Vcp_Feature_Code,
        mccs_version_id: DDCA_MCCS_Version_Id,
        p_value_table: *mut DDCA_Feature_Value_Table,
    ) -> DDCA_Status;

    pub fn ddca_get_simple_nc_feature_value_name(
        ddca_dh: DDCA_Display_Handle,
        feature_code: DDCA_Vcp_Feature_Code,
        feature_value: u8,
        p_feature_name: *mut *mut c_char,
    ) -> DDCA_Status;

    pub fn ddca_free_feature_info(info: *mut DDCA_Version_Feature_Info) -> DDCA_Status;
    pub fn ddca_free_feature_metadata(metadata: *mut DDCA_Feature_Metadata);

    pub fn ddca_get_mccs_version(
        ddca_dh: DDCA_Display_Handle,
        pspec: *mut DDCA_MCCS_Version_Spec,
    ) -> DDCA_Status;

    pub fn ddca_get_mccs_version_id(
        ddca_dh: DDCA_Display_Handle,
        p_id: *mut DDCA_MCCS_Version_Id,
    ) -> DDCA_Status;

    /// do not free
    pub fn ddca_get_edid_by_display_ref(
        ddca_dref: DDCA_Display_Ref,
        pbytes: *mut *mut u8,
    ) -> DDCA_Status;

    pub fn ddca_free_table_vcp_value(
        table_value_response: *mut DDCA_Table_Value,
    );

    pub fn ddca_get_any_vcp_value_using_explicit_type(
        ddca_dh: DDCA_Display_Handle,
        feature_code: DDCA_Vcp_Feature_Code,
        value_type: DDCA_Vcp_Value_Type,
        valrec_loc: *mut *mut DDCA_Any_Vcp_Value,
    ) -> DDCA_Status;

    pub fn ddca_get_any_vcp_value_using_implicit_type(
        ddca_dh: DDCA_Display_Handle,
        feature_code: DDCA_Vcp_Feature_Code,
        valrec_loc: *mut *mut DDCA_Any_Vcp_Value,
    ) -> DDCA_Status;

    pub fn ddca_set_table_vcp_value(
        ddca_dh: DDCA_Display_Handle,
        feature_code: DDCA_Vcp_Feature_Code,
        new_value: *mut DDCA_Table_Vcp_Value,
    ) -> DDCA_Status;

    pub fn ddca_set_any_vcp_value(
        ddca_dh: DDCA_Display_Handle,
        feature_code: DDCA_Vcp_Feature_Code,
        new_value: *mut DDCA_Any_Vcp_Value,
    ) -> DDCA_Status;

    pub fn ddca_free_any_vcp_value(valrec: *mut DDCA_Any_Vcp_Value);

    pub fn ddca_set_raw_vcp_value(
        ddca_dh: DDCA_Display_Handle,
        feature_code: DDCA_Vcp_Feature_Code,
        hi_byte: u8,
        lo_byte: u8,
    ) -> DDCA_Status;
}

pub type DDCA_Status = c_int;

pub type DDCA_Retry_Type = c_int;
pub const DDCA_WRITE_ONLY_TRIES: DDCA_Retry_Type = 0;
pub const DDCA_WRITE_READ_TRIES: DDCA_Retry_Type = 1;
pub const DDCA_MULTI_PART_TRIES: DDCA_Retry_Type = 2;

pub type DDCA_Output_Level = c_int;
pub const DDCA_OL_TERSE: DDCA_Output_Level = 0x04;
pub const DDCA_OL_NORMAL: DDCA_Output_Level = 0x08;
pub const DDCA_OL_VERBOSE: DDCA_Output_Level = 0x10;


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDCA_Table_Vcp_Value {
    pub bytect: u16,
    pub bytes: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DDCA_Ddcutil_Version_Spec {
    pub major: u8,
    pub minor: u8,
    pub micro: u8,
}

pub type DDCA_Display_Identifier = *mut c_void;

pub type DDCA_Display_Ref = *mut c_void;

pub type DDCA_Display_Handle = *mut c_void;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DDCA_Adlno {
    pub iAdapterIndex: c_int,
    pub iDisplayIndex: c_int,
}

pub type DDCA_IO_Mode = c_int;
pub const DDCA_IO_DEVI2C: DDCA_IO_Mode = 0;
pub const DDCA_IO_ADL: DDCA_IO_Mode = 1;
pub const DDCA_IO_USB: DDCA_IO_Mode = 2;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DDCA_IO_Path {
    pub io_mode: DDCA_IO_Mode,
    // union { i2c_busno: c_int, adlno: DDCA_Adlno: adlno, hiddev_devno: c_int }
    pub _union: DDCA_Adlno,
}

impl DDCA_IO_Path {
    pub fn i2c_busno(&self) -> c_int {
        self._union.iAdapterIndex
    }

    pub fn hiddev_devno(&self) -> c_int {
        self._union.iAdapterIndex
    }

    pub fn adlno(&self) -> &DDCA_Adlno {
        &self._union
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDCA_Display_Info {
    pub marker: [c_char; 4],
    pub dispno: c_int,
    pub path: DDCA_IO_Path,
    pub usb_bus: c_int,
    pub usb_device: c_int,
    pub mfg_id: *const c_char,
    pub model_name: *const c_char,
    pub sn: *const c_char,
    pub edid_bytes: *const u8,
    pub dref: DDCA_Display_Ref,
}

impl DDCA_Display_Info {
    pub fn edid_bytes(&self) -> &[u8] {
        unsafe {
            from_raw_parts(self.edid_bytes, 0x80)
        }
    }
}

#[repr(C)]
pub struct DDCA_Display_Info_List {
    pub ct: c_int,
    pub info: __IncompleteArrayField<DDCA_Display_Info>,
}

#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}

impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}

impl DDCA_Display_Info_List {
    pub fn info(&self) -> &[DDCA_Display_Info] {
        unsafe {
            from_raw_parts(self.info.as_ptr(), self.ct as usize)
        }
    }
}

impl fmt::Debug for DDCA_Display_Info_List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.info(), f)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DDCA_MCCS_Version_Spec {
    pub major: u8,
    pub minor: u8,
}

pub type DDCA_MCCS_Version_Id = c_int;
pub const DDCA_VNONE: DDCA_MCCS_Version_Id = 0;
pub const DDCA_V10: DDCA_MCCS_Version_Id = 1;
pub const DDCA_V20: DDCA_MCCS_Version_Id = 2;
pub const DDCA_V21: DDCA_MCCS_Version_Id = 4;
pub const DDCA_V30: DDCA_MCCS_Version_Id = 8;
pub const DDCA_V22: DDCA_MCCS_Version_Id = 16;
pub const DDCA_VANY: DDCA_MCCS_Version_Id = DDCA_VNONE;
pub const DDCA_VUNK: DDCA_MCCS_Version_Id = DDCA_VNONE;

pub const DDCA_VSPEC_V10: DDCA_MCCS_Version_Spec       = DDCA_MCCS_Version_Spec { major: 1, minor: 0 }; ///< MCCS version 1.0
pub const DDCA_VSPEC_V20: DDCA_MCCS_Version_Spec       = DDCA_MCCS_Version_Spec { major: 2, minor: 0 }; ///< MCCS version 2.0
pub const DDCA_VSPEC_V21: DDCA_MCCS_Version_Spec       = DDCA_MCCS_Version_Spec { major: 2, minor: 1 }; ///< MCCS version 2.1
pub const DDCA_VSPEC_V30: DDCA_MCCS_Version_Spec       = DDCA_MCCS_Version_Spec { major: 3, minor: 0 }; ///< MCCS version 3.0
pub const DDCA_VSPEC_V22: DDCA_MCCS_Version_Spec       = DDCA_MCCS_Version_Spec { major: 2, minor: 2 }; ///< MCCS version 2.2
pub const DDCA_VSPEC_UNKNOWN: DDCA_MCCS_Version_Spec   = DDCA_MCCS_Version_Spec { major: 0, minor: 0 }; ///< value for monitor has been queried unsuccessfully
pub const DDCA_VSPEC_ANY: DDCA_MCCS_Version_Spec       = DDCA_MCCS_Version_Spec { major: 0, minor: 0 };   ///< used as query specifier
pub const DDCA_VSPEC_UNQUERIED: DDCA_MCCS_Version_Spec = DDCA_MCCS_Version_Spec { major: 0xff, minor: 0xff }; ///< indicates version not queried

pub type DDCA_Vcp_Feature_Code = u8;


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDCA_Feature_Metadata {
    pub marker: [::std::os::raw::c_char; 4usize],
    pub feature_code: DDCA_Vcp_Feature_Code,
    pub vcp_version: DDCA_MCCS_Version_Spec,
    pub feature_flags: DDCA_Feature_Flags,
    pub sl_values: *mut DDCA_Feature_Value_Entry,
    pub latest_sl_values: *mut DDCA_Feature_Value_Entry,
    pub feature_name: *mut ::std::os::raw::c_char,
    pub feature_desc: *mut ::std::os::raw::c_char,
}

impl DDCA_Feature_Metadata {
    pub fn sl_values_len(&self) -> usize {
        if self.feature_flags & DDCA_SIMPLE_NC != 0 {
            let mut ptr = self.sl_values;
            let mut len = 0;
            unsafe {
                while (*ptr).value_code != 0 || !(*ptr).value_name.is_null() {
                    ptr = ptr.offset(1);
                    len += 1;
                }
            }

            len
        } else {
            0
        }
    }

    pub fn sl_values(&self) -> &[DDCA_Feature_Value_Entry] {
        let len = self.sl_values_len();
        if len == 0 {
            &[]
        } else {
            unsafe {
                from_raw_parts(self.sl_values as *const _, len)
            }
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// an entry of {0, NULL} terminates the list
pub struct DDCA_Feature_Value_Entry {
    pub value_code: u8,
    pub value_name: *mut c_char,
}

pub type DDCA_Feature_Value_Table = *mut DDCA_Feature_Value_Entry;

pub type DDCA_Version_Feature_Flags = u16;
pub const DDCA_RO: DDCA_Version_Feature_Flags = 0x0400;
pub const DDCA_WO: DDCA_Version_Feature_Flags = 0x0200;
pub const DDCA_RW: DDCA_Version_Feature_Flags = 0x0100;
pub const DDCA_READABLE: DDCA_Version_Feature_Flags = DDCA_RO | DDCA_RW;
pub const DDCA_WRITABLE: DDCA_Version_Feature_Flags = DDCA_WO | DDCA_RW;
pub const DDCA_STD_CONT: DDCA_Version_Feature_Flags = 0x0080;
pub const DDCA_COMPLEX_CONT: DDCA_Version_Feature_Flags = 0x0040;
pub const DDCA_SIMPLE_NC: DDCA_Version_Feature_Flags = 0x0020;
pub const DDCA_COMPLEX_NC: DDCA_Version_Feature_Flags = 0x0010;
pub const DDCA_WO_NC: DDCA_Version_Feature_Flags = 0x0008;
pub const DDCA_NORMAL_TABLE: DDCA_Version_Feature_Flags = 0x0004;
pub const DDCA_WO_TABLE: DDCA_Version_Feature_Flags = 0x0002;
pub const DDCA_CONT: DDCA_Version_Feature_Flags = DDCA_STD_CONT|DDCA_COMPLEX_CONT;
pub const DDCA_NC: DDCA_Version_Feature_Flags = DDCA_SIMPLE_NC|DDCA_COMPLEX_NC|DDCA_WO_NC;
pub const DDCA_NON_TABLE: DDCA_Version_Feature_Flags = DDCA_CONT | DDCA_NC;
pub const DDCA_TABLE: DDCA_Version_Feature_Flags = DDCA_NORMAL_TABLE | DDCA_WO_TABLE;
pub const DDCA_KNOWN: DDCA_Version_Feature_Flags = DDCA_CONT | DDCA_NC | DDCA_TABLE;
pub const DDCA_DEPRECATED: DDCA_Version_Feature_Flags = 0x0001;

pub type DDCA_Global_Feature_Flags = u16;
pub const DDCA_SYNTHETIC: DDCA_Global_Feature_Flags = 0x8000;

/// union (DDCA_Version_Feature_Flags, DDCA_Global_Feature_Flags)
pub type DDCA_Feature_Flags = u16;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DDCA_Version_Feature_Info {
    pub marker: [c_char; 4],
    pub feature_code: DDCA_Vcp_Feature_Code,
    pub vspec: DDCA_MCCS_Version_Spec,
    pub version_id: DDCA_MCCS_Version_Id,
    pub desc: *mut c_char,
    /// valid when DDCA_SIMPLE_NC set
    pub sl_values: DDCA_Feature_Value_Table,
    pub feature_name: *mut c_char,
    pub feature_flags: DDCA_Feature_Flags,
}

impl DDCA_Version_Feature_Info {
    pub fn sl_values_len(&self) -> usize {
        if self.feature_flags & DDCA_SIMPLE_NC != 0 {
            let mut ptr = self.sl_values;
            let mut len = 0;
            unsafe {
                while (*ptr).value_code != 0 || !(*ptr).value_name.is_null() {
                    ptr = ptr.offset(1);
                    len += 1;
                }
            }

            len
        } else {
            0
        }
    }

    pub fn sl_values(&self) -> &[DDCA_Feature_Value_Entry] {
        let len = self.sl_values_len();
        if len == 0 {
            &[]
        } else {
            unsafe {
                from_raw_parts(self.sl_values as *const _, len)
            }
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DDCA_Cap_Vcp {
    pub marker: [c_char; 4],
    pub feature_code: DDCA_Vcp_Feature_Code,
    pub value_ct: c_int,
    pub values: *mut u8,
}

impl DDCA_Cap_Vcp {
    pub fn values(&self) -> &[u8] {
        unsafe {
            from_raw_parts(self.values as *const _, self.value_ct as usize)
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DDCA_Capabilities {
    pub marker: [c_char; 4],
    pub unparsed_string: *mut c_char,
    pub version_spec: DDCA_MCCS_Version_Spec,
    pub vcp_code_ct: c_int,
    pub vcp_codes: *mut DDCA_Cap_Vcp,
}

impl DDCA_Capabilities {
    pub fn vcp_codes(&self) -> &[DDCA_Cap_Vcp] {
        unsafe {
            from_raw_parts(self.vcp_codes as *const _, self.vcp_code_ct as usize)
        }
    }
}

pub type DDCA_Vcp_Value_Type = c_int;
pub const DDCA_NON_TABLE_VCP_VALUE: DDCA_Vcp_Value_Type = 1;
pub const DDCA_TABLE_VCP_VALUE: DDCA_Vcp_Value_Type = 2;

pub type DDCA_Vcp_Value_Type_Parm = c_int;
pub const DDCA_UNSET_VCP_VALUE_TYPE_PARM: DDCA_Vcp_Value_Type_Parm = 0;
pub const DDCA_NON_TABLE_VCP_VALUE_PARM: DDCA_Vcp_Value_Type_Parm = 1;
pub const DDCA_TABLE_VCP_VALUE_PARM: DDCA_Vcp_Value_Type_Parm = 2;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DDCA_Non_Table_Value {
    pub mh: u8,
    pub ml: u8,
    pub sh: u8,
    pub sl: u8,
}

impl DDCA_Non_Table_Value {
    pub fn value(&self) -> u16 {
        ((self.sh as u16) << 8) | self.sl as u16
    }

    pub fn maximum(&self) -> u16 {
        ((self.mh as u16) << 8) | self.ml as u16
    }
}

#[repr(C)]
pub struct DDCA_Table_Value {
    pub bytect: u16,
    pub bytes: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _DDCA_Table_Value {
    pub bytes: *mut u8,
    pub bytect: u16,
}

impl _DDCA_Table_Value {
    pub fn bytes(&self) -> &[u8] {
        unsafe {
            from_raw_parts(self.bytes as *const _, self.bytect as usize)
        }
    }
}

impl fmt::Debug for _DDCA_Table_Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.bytes(), f)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DDCA_Any_Vcp_Value {
    pub opcode: DDCA_Vcp_Feature_Code,
    pub value_type: DDCA_Vcp_Value_Type,
    // union { _DDCA_Table_Value, DDCA_Non_Table_Value }
    pub _val_union: _DDCA_Table_Value,
}

impl DDCA_Any_Vcp_Value {
    pub unsafe fn c_nc(&self) -> &DDCA_Non_Table_Value {
        ::std::mem::transmute(self.t())
    }

    pub unsafe fn t(&self) -> &_DDCA_Table_Value {
        &self._val_union
    }
}
