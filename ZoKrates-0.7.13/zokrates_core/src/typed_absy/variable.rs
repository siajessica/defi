use crate::typed_absy::types::{DeclarationConstant, GStructType, UBitwidth};
use crate::typed_absy::types::{GType, SpecializationError};
use crate::typed_absy::Identifier;
use crate::typed_absy::UExpression;
use crate::typed_absy::{TryFrom, TryInto};
use std::fmt;

#[derive(Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct GVariable<'ast, S> {
    pub id: Identifier<'ast>,
    pub _type: GType<S>,
}

pub type DeclarationVariable<'ast, T> = GVariable<'ast, DeclarationConstant<'ast, T>>;
pub type ConcreteVariable<'ast> = GVariable<'ast, u32>;
pub type Variable<'ast, T> = GVariable<'ast, UExpression<'ast, T>>;

impl<'ast, T> TryFrom<Variable<'ast, T>> for ConcreteVariable<'ast> {
    type Error = SpecializationError;

    fn try_from(v: Variable<'ast, T>) -> Result<Self, Self::Error> {
        let _type = v._type.try_into()?;

        Ok(Self { _type, id: v.id })
    }
}

impl<'ast, T> From<ConcreteVariable<'ast>> for Variable<'ast, T> {
    fn from(v: ConcreteVariable<'ast>) -> Self {
        let _type = v._type.into();

        Self { _type, id: v.id }
    }
}

pub fn try_from_g_variable<T: TryInto<U>, U>(
    v: GVariable<T>,
) -> Result<GVariable<U>, SpecializationError> {
    let _type = crate::typed_absy::types::try_from_g_type(v._type)?;

    Ok(GVariable { _type, id: v.id })
}

impl<'ast, S: Clone> GVariable<'ast, S> {
    pub fn field_element<I: Into<Identifier<'ast>>>(id: I) -> Self {
        Self::with_id_and_type(id, GType::FieldElement)
    }

    pub fn boolean<I: Into<Identifier<'ast>>>(id: I) -> Self {
        Self::with_id_and_type(id, GType::Boolean)
    }

    pub fn uint<I: Into<Identifier<'ast>>, W: Into<UBitwidth>>(id: I, bitwidth: W) -> Self {
        Self::with_id_and_type(id, GType::uint(bitwidth))
    }

    #[cfg(test)]
    pub fn field_array<I: Into<Identifier<'ast>>>(id: I, size: S) -> Self {
        Self::array(id, GType::FieldElement, size)
    }

    pub fn array<I: Into<Identifier<'ast>>, U: Into<S>>(id: I, ty: GType<S>, size: U) -> Self {
        Self::with_id_and_type(id, GType::array((ty, size.into())))
    }

    pub fn struc<I: Into<Identifier<'ast>>>(id: I, ty: GStructType<S>) -> Self {
        Self::with_id_and_type(id, GType::Struct(ty))
    }

    pub fn with_id_and_type<I: Into<Identifier<'ast>>>(id: I, _type: GType<S>) -> Self {
        GVariable {
            id: id.into(),
            _type,
        }
    }

    pub fn get_type(&self) -> GType<S> {
        self._type.clone()
    }
}

impl<'ast, S: fmt::Display> fmt::Display for GVariable<'ast, S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self._type, self.id,)
    }
}

impl<'ast, S: fmt::Debug> fmt::Debug for GVariable<'ast, S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Variable(type: {:?}, id: {:?})", self._type, self.id,)
    }
}
