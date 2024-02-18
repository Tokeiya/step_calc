use syntax::binary_operation::Operation;
use syntax::number_value::NumberValue;
pub enum RpnToken {
	Number(NumberValue),
	Operator(Operation),
}
