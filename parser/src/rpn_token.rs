use syntax::number_value::NumberValue;
use syntax::binary_operation::Operation;
pub enum RpnToken{
	Number(NumberValue),
	Operator(Operation)
}

