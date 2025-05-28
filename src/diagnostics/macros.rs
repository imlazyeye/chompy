/// Provides the boilerplate for implementing [super::Diag] on a struct with
/// [super::Severity::Error].
#[macro_export]
macro_rules! define_error {
    // changed from `$ty:ty` to `$name` + optional generics
    ($name:ident $(< $($gen:ident : $bound:path),* >)? {
        $build_fn:item
        $location_fn:item
    }) => {
        $crate::define_diag!(
            $crate::diagnostics::Severity::Error
                => $name $(< $($gen : $bound),* >)? {
                    $build_fn
                    $location_fn
                }
        );
    };
}

/// Provides the boilerplate for implementing [super::Diag] on a struct with
/// [super::Severity::Warning].
#[macro_export]
macro_rules! define_warning {
    // mirror the change in define_error
    ($name:ident $(< $($gen:ident : $bound:path),* >)? {
        $build_fn:item
        $location_fn:item
    }) => {
        $crate::define_diag!(
            $crate::diagnostics::Severity::Warning
                => $name $(< $($gen : $bound),* >)? {
                    $build_fn
                    $location_fn
                }
        );
    };
}

/// Provides the boilerplate for implementing [super::Diag] on a struct with
/// [super::Severity::Bug].
#[macro_export]
macro_rules! define_bug {
    // same pattern, and corrected to Severity::Bug
    ($name:ident $(< $($gen:ident : $bound:path),* >)? {
        $build_fn:item
        $location_fn:item
    }) => {
        $crate::define_diag!(
            $crate::diagnostics::Severity::Bug
                => $name $(< $($gen : $bound),* >)? {
                    $build_fn
                    $location_fn
                }
        );
    };
}

/// Core macro: now accepts optional generics and applies to Diag, Located, Debug
#[macro_export]
macro_rules! define_diag {
    (
        $severity:expr
        =>
        $name:ident $(< $($gen:ident : $bound:path),* >)?
        {
            $build_fn:item
            $location_fn:item
        }
    ) => {

        // impl Diag for $name<...> if generics provided
        impl$(<$($gen : $bound),*>)? $crate::diagnostics::Diag for $name$(<$($gen),*>)? {
            fn severity(&self) -> $crate::diagnostics::Severity {
                $severity
            }

            $build_fn
        }

        // impl Located for $name<...>
        impl$(<$($gen : $bound),*>)? $crate::utils::Located for $name$(<$($gen),*>)? {
            $location_fn
        }

        // impl Debug for $name<...>
        impl$(<$($gen : $bound),*>)? std::fmt::Debug for $name$(<$($gen),*>)? {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let diag = <$name$(<$($gen),*>)? as $crate::diagnostics::Diag>::build(
                    &self,
                    $crate::diagnostics::Builder::new(
                        <$name$(<$($gen),*>)? as $crate::diagnostics::Diag>::severity(&self)
                    ),
                );

                let mut joined = String::new();
                for (i, lbl) in diag.labels.iter().enumerate() {
                    if i != 0 {
                        joined.push_str(", ");
                    }
                    joined.push_str(&lbl.message);
                }

                f.pad(&joined)
            }
        }
    };
}
