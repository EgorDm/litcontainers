use crate::ops::*;
use std::ops::{Add, Sub, Mul, Div, Rem};
use crate::{InplaceMap, StorageSize, Element};

operation_scalar_binary_op!(
    AddScalar | AddAssignScalar => Add: add,
	SubScalar | SubAssignScalar => Sub: sub,
	MulScalar | MulAssignScalar => Mul: mul,
	DivScalar | DivAssignScalar => Div: div,
	RemScalar | RemAssignScalar => Rem: rem,
);

operation_scalar_binary_rev_op!(
	SubScalarRev | SubAssignScalarRev => Sub: sub,
	DivScalarRev | DivAssignScalarRev => Div: div,
	RemScalarRev | RemAssignScalarRev => Rem: rem,
);

pub trait ArithmeticScalarOps: IntoOperation + Sized
{
	operation_group_scalar_binary!(
		AddScalar: add_scalar | AddAssignScalar: add_assign_scalar => Add,
		SubScalar: sub_scalar | SubAssignScalar: sub_assign_scalar => Sub,
		MulScalar: mul_scalar | MulAssignScalar: mul_assign_scalar => Mul,
		DivScalar: div_scalar | DivAssignScalar: div_assign_scalar => Div,
		RemScalar: rem_scalar | RemAssignScalar: rem_assign_scalar => Rem,
	);

	operation_group_scalar_binary_rev!(
		SubScalarRev: sub_scalar_rev | SubAssignScalarRev: sub_assign_scalar_rev => Sub,
		DivScalarRev: div_scalar_rev | DivAssignScalarRev: div_assign_scalar_rev => Div,
		RemScalarRev: rem_scalar_rev | RemAssignScalarRev: rem_assign_scalar_rev => Rem,
	);
}

impl<O: IntoOperation> ArithmeticScalarOps for O {}



