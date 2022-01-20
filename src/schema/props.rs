macro_rules! props {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(crate::schema::props::props!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { crate::schema::props::props!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let _cap = crate::schema::props::props!(@count $($key),*);
            let mut _map = ::std::collections::HashMap::<String, crate::schema::properties::Property>::with_capacity(_cap);
            let _rows: usize = 0;
            $(
                let _ = _map.insert($key.to_string(), crate::schema::properties::Property::new($value.0, $value.1, _rows));
                let _rows: usize = _rows + 1;
            )*
            _map
        }
    };
}

pub(crate) use props;
