use diesel::{expression::AsExpression, Expression};

diesel_infix_operator!(TextSerach, "&@");
diesel_infix_operator!(RegexMatch, "~");

pub trait ExpressionMethods: Expression + Sized {
    fn text_search<T: AsExpression<Self::SqlType>>(
        self,
        other: T,
    ) -> TextSerach<Self, T::Expression> {
        TextSerach::new(self, other.as_expression())
    }

    fn regex_match<T: AsExpression<Self::SqlType>>(
        self,
        other: T,
    ) -> RegexMatch<Self, T::Expression> {
        RegexMatch::new(self, other.as_expression())
    }
}

impl<T: Expression> ExpressionMethods for T {}

sql_function! {
    fn length(a: diesel::sql_types::Text) -> diesel::sql_types::Integer;
}
