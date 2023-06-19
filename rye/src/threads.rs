use std::thread::scope;

/// Run f1 and f2 in parallel
///
/// Propagate the first error from f1 and f2, if any.
/// If f1 or f2 panic, the panic is propagated.
pub fn parallel_call<E: Send>(
    is_parallel: bool,
    f1: impl FnOnce() -> Result<(), E> + Send,
    f2: impl FnOnce() -> Result<(), E> + Send,
) -> Result<(), E> {
    if !is_parallel {
        f1().and_then(move |_| f2())
    } else {
        scope(move |scope| -> Result<(), E> {
            let handle1 = scope.spawn(f1);
            let handle2 = scope.spawn(f2);

            match (handle1.join(), handle2.join()) {
                (Ok(res1), Ok(res2)) => res1.and(res2),
                (Err(e), _) | (_, Err(e)) => std::panic::resume_unwind(e),
            }
        })
    }
}
