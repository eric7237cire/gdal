use gdal_sys::{GDALGetDataTypeName, GDALDataType};
use crate::errors::Result as GdalResult;
use crate::utils::{_string, _last_null_pointer_err};

pub fn get_type_name(gdal_type: GDALDataType::Type) -> GdalResult<String> {
	let c_res = unsafe { GDALGetDataTypeName(gdal_type)};
	if c_res.is_null() {
		Err(_last_null_pointer_err("GDALGetDescription"))?;
	}
	Ok(_string(c_res))
}