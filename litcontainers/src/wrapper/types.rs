use crate::{Wrapper, VecStorageCM, VecStorageRM, U1};

/// Container storing scalar values in a col major order
pub type ContainerCM<T, R, C> = Wrapper<T, VecStorageCM<T, R, C>>;
/// Container storing scalar values as a column vector
pub type ColVec<T, R> = ContainerRM<T, R, U1>;
/// Container storing scalar values in a row major order
pub type ContainerRM<T, R, C> = Wrapper<T, VecStorageRM<T, R, C>>;
/// Container storing scalar values as a row vector
pub type RowVec<T, C> = ContainerRM<T, U1, C>;