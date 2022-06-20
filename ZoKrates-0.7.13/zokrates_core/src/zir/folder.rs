// Generic walk through ZIR. Not mutating in place

use crate::zir::types::UBitwidth;
use crate::zir::*;
use zokrates_field::Field;

pub trait Folder<'ast, T: Field>: Sized {
    fn fold_program(&mut self, p: ZirProgram<'ast, T>) -> ZirProgram<'ast, T> {
        fold_program(self, p)
    }

    fn fold_function(&mut self, f: ZirFunction<'ast, T>) -> ZirFunction<'ast, T> {
        fold_function(self, f)
    }

    fn fold_parameter(&mut self, p: Parameter<'ast>) -> Parameter<'ast> {
        Parameter {
            id: self.fold_variable(p.id),
            ..p
        }
    }

    fn fold_name(&mut self, n: Identifier<'ast>) -> Identifier<'ast> {
        n
    }

    fn fold_variable(&mut self, v: Variable<'ast>) -> Variable<'ast> {
        Variable {
            id: self.fold_name(v.id),
            ..v
        }
    }

    fn fold_assignee(&mut self, a: ZirAssignee<'ast>) -> ZirAssignee<'ast> {
        self.fold_variable(a)
    }

    fn fold_statement(&mut self, s: ZirStatement<'ast, T>) -> Vec<ZirStatement<'ast, T>> {
        fold_statement(self, s)
    }

    fn fold_expression(&mut self, e: ZirExpression<'ast, T>) -> ZirExpression<'ast, T> {
        match e {
            ZirExpression::FieldElement(e) => self.fold_field_expression(e).into(),
            ZirExpression::Boolean(e) => self.fold_boolean_expression(e).into(),
            ZirExpression::Uint(e) => self.fold_uint_expression(e).into(),
        }
    }

    fn fold_expression_list(
        &mut self,
        es: ZirExpressionList<'ast, T>,
    ) -> ZirExpressionList<'ast, T> {
        match es {
            ZirExpressionList::EmbedCall(embed, generics, arguments) => {
                ZirExpressionList::EmbedCall(
                    embed,
                    generics,
                    arguments
                        .into_iter()
                        .map(|a| self.fold_expression(a))
                        .collect(),
                )
            }
        }
    }

    fn fold_field_expression(
        &mut self,
        e: FieldElementExpression<'ast, T>,
    ) -> FieldElementExpression<'ast, T> {
        fold_field_expression(self, e)
    }
    fn fold_boolean_expression(
        &mut self,
        e: BooleanExpression<'ast, T>,
    ) -> BooleanExpression<'ast, T> {
        fold_boolean_expression(self, e)
    }
    fn fold_uint_expression(&mut self, e: UExpression<'ast, T>) -> UExpression<'ast, T> {
        fold_uint_expression(self, e)
    }

    fn fold_uint_expression_inner(
        &mut self,
        bitwidth: UBitwidth,
        e: UExpressionInner<'ast, T>,
    ) -> UExpressionInner<'ast, T> {
        fold_uint_expression_inner(self, bitwidth, e)
    }
}

pub fn fold_statement<'ast, T: Field, F: Folder<'ast, T>>(
    f: &mut F,
    s: ZirStatement<'ast, T>,
) -> Vec<ZirStatement<'ast, T>> {
    let res = match s {
        ZirStatement::Return(expressions) => ZirStatement::Return(
            expressions
                .into_iter()
                .map(|e| f.fold_expression(e))
                .collect(),
        ),
        ZirStatement::Definition(a, e) => {
            ZirStatement::Definition(f.fold_assignee(a), f.fold_expression(e))
        }
        ZirStatement::IfElse(condition, consequence, alternative) => ZirStatement::IfElse(
            f.fold_boolean_expression(condition),
            consequence
                .into_iter()
                .flat_map(|e| f.fold_statement(e))
                .collect(),
            alternative
                .into_iter()
                .flat_map(|e| f.fold_statement(e))
                .collect(),
        ),
        ZirStatement::Assertion(e, error) => {
            ZirStatement::Assertion(f.fold_boolean_expression(e), error)
        }
        ZirStatement::MultipleDefinition(variables, elist) => ZirStatement::MultipleDefinition(
            variables.into_iter().map(|v| f.fold_variable(v)).collect(),
            f.fold_expression_list(elist),
        ),
    };
    vec![res]
}

pub fn fold_field_expression<'ast, T: Field, F: Folder<'ast, T>>(
    f: &mut F,
    e: FieldElementExpression<'ast, T>,
) -> FieldElementExpression<'ast, T> {
    match e {
        FieldElementExpression::Number(n) => FieldElementExpression::Number(n),
        FieldElementExpression::Identifier(id) => {
            FieldElementExpression::Identifier(f.fold_name(id))
        }
        FieldElementExpression::Select(a, box i) => FieldElementExpression::Select(
            a.into_iter().map(|a| f.fold_field_expression(a)).collect(),
            box f.fold_uint_expression(i),
        ),
        FieldElementExpression::Add(box e1, box e2) => {
            let e1 = f.fold_field_expression(e1);
            let e2 = f.fold_field_expression(e2);
            FieldElementExpression::Add(box e1, box e2)
        }
        FieldElementExpression::Sub(box e1, box e2) => {
            let e1 = f.fold_field_expression(e1);
            let e2 = f.fold_field_expression(e2);
            FieldElementExpression::Sub(box e1, box e2)
        }
        FieldElementExpression::Mult(box e1, box e2) => {
            let e1 = f.fold_field_expression(e1);
            let e2 = f.fold_field_expression(e2);
            FieldElementExpression::Mult(box e1, box e2)
        }
        FieldElementExpression::Div(box e1, box e2) => {
            let e1 = f.fold_field_expression(e1);
            let e2 = f.fold_field_expression(e2);
            FieldElementExpression::Div(box e1, box e2)
        }
        FieldElementExpression::Pow(box e1, box e2) => {
            let e1 = f.fold_field_expression(e1);
            let e2 = f.fold_uint_expression(e2);
            FieldElementExpression::Pow(box e1, box e2)
        }
        FieldElementExpression::IfElse(box cond, box cons, box alt) => {
            let cond = f.fold_boolean_expression(cond);
            let cons = f.fold_field_expression(cons);
            let alt = f.fold_field_expression(alt);
            FieldElementExpression::IfElse(box cond, box cons, box alt)
        }
    }
}

pub fn fold_boolean_expression<'ast, T: Field, F: Folder<'ast, T>>(
    f: &mut F,
    e: BooleanExpression<'ast, T>,
) -> BooleanExpression<'ast, T> {
    match e {
        BooleanExpression::Value(v) => BooleanExpression::Value(v),
        BooleanExpression::Identifier(id) => BooleanExpression::Identifier(f.fold_name(id)),
        BooleanExpression::Select(a, box i) => BooleanExpression::Select(
            a.into_iter()
                .map(|a| f.fold_boolean_expression(a))
                .collect(),
            box f.fold_uint_expression(i),
        ),
        BooleanExpression::FieldEq(box e1, box e2) => {
            let e1 = f.fold_field_expression(e1);
            let e2 = f.fold_field_expression(e2);
            BooleanExpression::FieldEq(box e1, box e2)
        }
        BooleanExpression::BoolEq(box e1, box e2) => {
            let e1 = f.fold_boolean_expression(e1);
            let e2 = f.fold_boolean_expression(e2);
            BooleanExpression::BoolEq(box e1, box e2)
        }
        BooleanExpression::UintEq(box e1, box e2) => {
            let e1 = f.fold_uint_expression(e1);
            let e2 = f.fold_uint_expression(e2);
            BooleanExpression::UintEq(box e1, box e2)
        }
        BooleanExpression::FieldLt(box e1, box e2) => {
            let e1 = f.fold_field_expression(e1);
            let e2 = f.fold_field_expression(e2);
            BooleanExpression::FieldLt(box e1, box e2)
        }
        BooleanExpression::FieldLe(box e1, box e2) => {
            let e1 = f.fold_field_expression(e1);
            let e2 = f.fold_field_expression(e2);
            BooleanExpression::FieldLe(box e1, box e2)
        }
        BooleanExpression::FieldGt(box e1, box e2) => {
            let e1 = f.fold_field_expression(e1);
            let e2 = f.fold_field_expression(e2);
            BooleanExpression::FieldGt(box e1, box e2)
        }
        BooleanExpression::FieldGe(box e1, box e2) => {
            let e1 = f.fold_field_expression(e1);
            let e2 = f.fold_field_expression(e2);
            BooleanExpression::FieldGe(box e1, box e2)
        }
        BooleanExpression::UintLt(box e1, box e2) => {
            let e1 = f.fold_uint_expression(e1);
            let e2 = f.fold_uint_expression(e2);
            BooleanExpression::UintLt(box e1, box e2)
        }
        BooleanExpression::UintLe(box e1, box e2) => {
            let e1 = f.fold_uint_expression(e1);
            let e2 = f.fold_uint_expression(e2);
            BooleanExpression::UintLe(box e1, box e2)
        }
        BooleanExpression::UintGt(box e1, box e2) => {
            let e1 = f.fold_uint_expression(e1);
            let e2 = f.fold_uint_expression(e2);
            BooleanExpression::UintGt(box e1, box e2)
        }
        BooleanExpression::UintGe(box e1, box e2) => {
            let e1 = f.fold_uint_expression(e1);
            let e2 = f.fold_uint_expression(e2);
            BooleanExpression::UintGe(box e1, box e2)
        }
        BooleanExpression::Or(box e1, box e2) => {
            let e1 = f.fold_boolean_expression(e1);
            let e2 = f.fold_boolean_expression(e2);
            BooleanExpression::Or(box e1, box e2)
        }
        BooleanExpression::And(box e1, box e2) => {
            let e1 = f.fold_boolean_expression(e1);
            let e2 = f.fold_boolean_expression(e2);
            BooleanExpression::And(box e1, box e2)
        }
        BooleanExpression::Not(box e) => {
            let e = f.fold_boolean_expression(e);
            BooleanExpression::Not(box e)
        }
        BooleanExpression::IfElse(box cond, box cons, box alt) => {
            let cond = f.fold_boolean_expression(cond);
            let cons = f.fold_boolean_expression(cons);
            let alt = f.fold_boolean_expression(alt);
            BooleanExpression::IfElse(box cond, box cons, box alt)
        }
    }
}

pub fn fold_uint_expression<'ast, T: Field, F: Folder<'ast, T>>(
    f: &mut F,
    e: UExpression<'ast, T>,
) -> UExpression<'ast, T> {
    UExpression {
        inner: f.fold_uint_expression_inner(e.bitwidth, e.inner),
        ..e
    }
}

pub fn fold_uint_expression_inner<'ast, T: Field, F: Folder<'ast, T>>(
    f: &mut F,
    _: UBitwidth,
    e: UExpressionInner<'ast, T>,
) -> UExpressionInner<'ast, T> {
    match e {
        UExpressionInner::Value(v) => UExpressionInner::Value(v),
        UExpressionInner::Identifier(id) => UExpressionInner::Identifier(f.fold_name(id)),
        UExpressionInner::Select(a, box i) => UExpressionInner::Select(
            a.into_iter().map(|a| f.fold_uint_expression(a)).collect(),
            box f.fold_uint_expression(i),
        ),
        UExpressionInner::Add(box left, box right) => {
            let left = f.fold_uint_expression(left);
            let right = f.fold_uint_expression(right);

            UExpressionInner::Add(box left, box right)
        }
        UExpressionInner::Sub(box left, box right) => {
            let left = f.fold_uint_expression(left);
            let right = f.fold_uint_expression(right);

            UExpressionInner::Sub(box left, box right)
        }
        UExpressionInner::Mult(box left, box right) => {
            let left = f.fold_uint_expression(left);
            let right = f.fold_uint_expression(right);

            UExpressionInner::Mult(box left, box right)
        }
        UExpressionInner::Div(box left, box right) => {
            let left = f.fold_uint_expression(left);
            let right = f.fold_uint_expression(right);

            UExpressionInner::Div(box left, box right)
        }
        UExpressionInner::Rem(box left, box right) => {
            let left = f.fold_uint_expression(left);
            let right = f.fold_uint_expression(right);

            UExpressionInner::Rem(box left, box right)
        }
        UExpressionInner::Xor(box left, box right) => {
            let left = f.fold_uint_expression(left);
            let right = f.fold_uint_expression(right);

            UExpressionInner::Xor(box left, box right)
        }
        UExpressionInner::And(box left, box right) => {
            let left = f.fold_uint_expression(left);
            let right = f.fold_uint_expression(right);

            UExpressionInner::And(box left, box right)
        }
        UExpressionInner::Or(box left, box right) => {
            let left = f.fold_uint_expression(left);
            let right = f.fold_uint_expression(right);

            UExpressionInner::Or(box left, box right)
        }
        UExpressionInner::LeftShift(box e, by) => {
            let e = f.fold_uint_expression(e);

            UExpressionInner::LeftShift(box e, by)
        }
        UExpressionInner::RightShift(box e, by) => {
            let e = f.fold_uint_expression(e);

            UExpressionInner::RightShift(box e, by)
        }
        UExpressionInner::Not(box e) => {
            let e = f.fold_uint_expression(e);

            UExpressionInner::Not(box e)
        }
        UExpressionInner::IfElse(box cond, box cons, box alt) => {
            let cond = f.fold_boolean_expression(cond);
            let cons = f.fold_uint_expression(cons);
            let alt = f.fold_uint_expression(alt);
            UExpressionInner::IfElse(box cond, box cons, box alt)
        }
    }
}

pub fn fold_function<'ast, T: Field, F: Folder<'ast, T>>(
    f: &mut F,
    fun: ZirFunction<'ast, T>,
) -> ZirFunction<'ast, T> {
    ZirFunction {
        arguments: fun
            .arguments
            .into_iter()
            .map(|a| f.fold_parameter(a))
            .collect(),
        statements: fun
            .statements
            .into_iter()
            .flat_map(|s| f.fold_statement(s))
            .collect(),
        ..fun
    }
}

pub fn fold_program<'ast, T: Field, F: Folder<'ast, T>>(
    f: &mut F,
    p: ZirProgram<'ast, T>,
) -> ZirProgram<'ast, T> {
    ZirProgram {
        main: f.fold_function(p.main),
    }
}
