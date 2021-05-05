use diesel::{expression::AsExpression, sql_types::Text, Expression};

diesel_infix_operator!(TextSerach, "&@");

pub trait ExpressionMethods: Expression + Sized {
    fn text_search<T: AsExpression<Self::SqlType>>(
        self,
        other: T,
    ) -> TextSerach<Self, T::Expression> {
        TextSerach::new(self, other.as_expression())
    }
}

impl<T: Expression<SqlType = Text>> ExpressionMethods for T {}
