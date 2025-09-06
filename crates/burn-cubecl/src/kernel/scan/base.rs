use crate::{CubeRuntime, tensor::CubeTensor};
use cubecl::CubeElement;

pub fn associative_scan<R: CubeRuntime, In: CubeElement, Out: CubeElement>()
-> Result<CubeTensor<R>, cubecl::scan::ScanError> {
    todo!()
}
