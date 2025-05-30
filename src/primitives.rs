//! Implements [`MigrateUp`] for Rust primitives and [`std`] library types.

use std::{
    collections::HashMap,
    hash::{BuildHasher, Hash},
    path::PathBuf,
};

use crate::traits::MigrateUp;

/// Implements [`MigrateUp`] for types that are not versioned, usually because they are not within
/// `rustdoc_types`.
///
/// For all unversioned types, `Self::Up = Self`.
macro_rules! impl_unversioned_migrations {
    {
        $($primitive:path),*
    } => {
        $(
            impl $crate::traits::MigrateUp for $primitive {
                type Up = Self;

                fn migrate_up(self) -> Self::Up {
                    self
                }
            }
        )*
    };
}

// Only implement `MigrateUp` for types that are used by `rustdoc_types`, to avoid increasing
// compile times even further.
impl_unversioned_migrations! {
    bool,
    String,
    u32,
    usize,
    PathBuf
}

/// Implements [`MigrateUp`] for a tuple whose contents also implement [`MigrateUp`].
///
/// This macro is designed to work with `variadics_please::all_tuples_enumerated!`, but currently
/// doesn't to improve compile-times.
macro_rules! impl_tuple_migrations {
    (
        $(($n:tt, $t:ident)),*
    ) => {
        impl<$($t: MigrateUp),*> MigrateUp for ($($t,)*) {
            type Up = ($($t::Up,)*);

            fn migrate_up(self) -> Self::Up {
                (
                    $(self.$n.migrate_up(),)*
                )
            }
        }
    };
}

// Only implement `MigrateUp` for the tuple sizes that need it.
impl_tuple_migrations!((0, T0), (1, T1));

impl<T: MigrateUp> MigrateUp for Option<T> {
    type Up = Option<T::Up>;

    fn migrate_up(self) -> Self::Up {
        self.map(MigrateUp::migrate_up)
    }
}

impl<T: MigrateUp> MigrateUp for Box<T> {
    type Up = Box<T::Up>;

    fn migrate_up(self) -> Self::Up {
        Box::new((*self).migrate_up())
    }
}

impl<T: MigrateUp> MigrateUp for Vec<T> {
    type Up = Vec<T::Up>;

    fn migrate_up(self) -> Self::Up {
        self.into_iter().map(MigrateUp::migrate_up).collect()
    }
}

impl<K: MigrateUp, V: MigrateUp, S: BuildHasher + Default> MigrateUp for HashMap<K, V, S>
where
    K::Up: Hash + Eq,
{
    type Up = HashMap<K::Up, V::Up, S>;

    fn migrate_up(self) -> Self::Up {
        self.into_iter()
            .map(|(k, v)| (k.migrate_up(), v.migrate_up()))
            .collect()
    }
}
