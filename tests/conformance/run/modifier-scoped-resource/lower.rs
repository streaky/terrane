// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: modifier-scoped-resource
#[derive(Clone)]
pub struct Marker {
    __terrane_lifetime: std::sync::Arc<()>,
    pub name: String,
}
impl Marker {
    pub fn terrane_construct(name: String) -> Self {
        let mut __terrane_constructed_value = Self {
            name: String::new(),
            __terrane_lifetime: std::sync::Arc::new(()),
        };
        __terrane_constructed_value.construct(name);
        __terrane_constructed_value
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    pub fn construct(&mut self, name: String) {
        {
            let __terrane_value = name;
            self.name = __terrane_value;
        }
    }
    pub fn destruct(&mut self) {
        println!(
            "{}{}", String::from("drop "), terrane_scalar_support::scalar_text(&self
            .name)
        );
    }
}
impl Drop for Marker {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) != 1 {
            return;
        }
        self.destruct();
    }
}
fn acquire(name: String, fail: bool) -> Result<Marker, TerraneError> {
    println!(
        "{}{}", String::from("acquire "), terrane_scalar_support::scalar_text(&name)
    );
    if fail {
        return Err(
            TerraneError::raised(
                TerraneErrorKind::ArithmeticOverflow,
                0 /* terrane-site: case.trn:17:9-17:34 */,
            ),
        );
    }
    return Ok(Marker::terrane_construct(name));
}
fn normal() -> Result<(), TerraneError> {
    let result: String;
    {
        let guard: Marker = __terrane_traced_err(
            acquire(String::from("outer"), false),
            1 /* terrane-site: case.trn:22:29-22:45 */,
        )?;
        result = guard.name.clone();
        {
            let guard_2: Marker = __terrane_traced_err(
                acquire(String::from("inner"), false),
                2 /* terrane-site: case.trn:24:30-24:46 */,
            )?;
            println!(
                "{}{}", String::from("inside "),
                terrane_scalar_support::scalar_text(&guard_2.name)
            );
        }
        println!(
            "{}{}", String::from("after inner "),
            terrane_scalar_support::scalar_text(&guard.name)
        );
    }
    println!(
        "{}{}", String::from("owned result "),
        terrane_scalar_support::scalar_text(&result)
    );
    Ok(())
}
fn borrowed() -> Result<(), TerraneError> {
    {
        let parent: Marker = __terrane_traced_err(
            acquire(String::from("borrow-parent"), false),
            3 /* terrane-site: case.trn:32:27-32:51 */,
        )?;
        {
            let view: &Marker = &parent;
            println!(
                "{}{}", String::from("view "), terrane_scalar_support::scalar_text(&{ let
                __terrane_snapshot = view.name.clone(); __terrane_snapshot })
            );
        }
        println!(
            "{}{}", String::from("parent "), terrane_scalar_support::scalar_text(&parent
            .name)
        );
    }
    Ok(())
}
fn early() -> Result<String, TerraneError> {
    {
        let __trn_5f6775617264: Marker = __terrane_traced_err(
            acquire(String::from("return-outer"), false),
            4 /* terrane-site: case.trn:40:27-40:50 */,
        )?;
        let _ = &__trn_5f6775617264;
        {
            let child: Marker = __terrane_traced_err(
                acquire(String::from("return-inner"), false),
                5 /* terrane-site: case.trn:41:30-41:53 */,
            )?;
            return Ok(child.name.clone());
        }
    }
}
fn failed() {
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            {
                let __trn_5f6775617264: Marker = __terrane_traced_completion!(
                    acquire(String::from("failure-outer"), false), 6 /* terrane-site: case.trn:48:31-48:55 */
                );
                let _ = &__trn_5f6775617264;
                {
                    let child: Marker = __terrane_traced_completion!(
                        acquire(String::from("failure-inner"), true),
                        7 /* terrane-site: case.trn:49:34-49:64 */
                    );
                    println!("{}", terrane_scalar_support::scalar_text(&child.name));
                }
            }
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0
                    && __terrane_error_0.kind == TerraneErrorKind::ArithmeticOverflow
                {
                    __terrane_handled_0 = true;
                    println!("{}", String::from("caught outside"));
                }
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
fn recovered() -> Result<(), TerraneError> {
    {
        let guard: Marker = __terrane_traced_err(
            acquire(String::from("catch-outer"), false),
            8 /* terrane-site: case.trn:57:26-57:48 */,
        )?;
        let __terrane_completion_1: TerraneCompletion<()> = (|| {
            let __terrane_try_1: TerraneCompletion<()> = (|| {
                {
                    let child: Marker = __terrane_traced_completion!(
                        acquire(String::from("catch-inner"), true), 9 /* terrane-site: case.trn:59:34-59:62 */
                    );
                    println!("{}", terrane_scalar_support::scalar_text(&child.name));
                }
                TerraneCompletion::Normal
            })();
            match __terrane_try_1 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_1) => {
                    let mut __terrane_handled_1 = false;
                    if !__terrane_handled_1
                        && __terrane_error_1.kind == TerraneErrorKind::ArithmeticOverflow
                    {
                        __terrane_handled_1 = true;
                        println!(
                            "{}{}", String::from("caught inside "),
                            terrane_scalar_support::scalar_text(&guard.name)
                        );
                    }
                    if !__terrane_handled_1 {
                        return TerraneCompletion::Error(__terrane_error_1);
                    }
                }
            }
            TerraneCompletion::Normal
        })();
        match __terrane_completion_1 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return Ok(value),
            TerraneCompletion::Error(error) => return Err(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        println!(
            "{}{}", String::from("still held "),
            terrane_scalar_support::scalar_text(&guard.name)
        );
    }
    Ok(())
}
fn loop_exits() -> Result<(), TerraneError> {
    let mut index: terrane_int_support::Int;
    index = terrane_int_support::Int::from(0_i128);
    while index < terrane_int_support::Int::from(2_i128) {
        index = index.clone() + terrane_int_support::Int::from(1_i128);
        {
            let guard: Marker = __terrane_traced_err(
                acquire(String::from("loop"), false),
                10 /* terrane-site: case.trn:71:30-71:45 */,
            )?;
            println!(
                "{}{}{}{}", String::from("iteration "),
                terrane_scalar_support::scalar_text(&index), String::from(" "),
                terrane_scalar_support::scalar_text(&guard.name)
            );
            if index == terrane_int_support::Int::from(1_i128) {
                continue;
            }
            break;
        }
    }
    println!("{}", String::from("after loop"));
    Ok(())
}
fn ordinary_exits() -> Result<(), TerraneError> {
    __terrane_traced_err(normal(), 11 /* terrane-site: case.trn:80:5-80:12 */)?;
    __terrane_traced_err(borrowed(), 12 /* terrane-site: case.trn:81:5-81:14 */)?;
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced_err(early(),
        13 /* terrane-site: case.trn:82:13-82:19 */) ?)
    );
    failed();
    __terrane_traced_err(recovered(), 14 /* terrane-site: case.trn:84:5-84:15 */)?;
    __terrane_traced_err(loop_exits(), 15 /* terrane-site: case.trn:85:5-85:16 */)?;
    Ok(())
}
#[derive(Clone)]
pub struct Readiness {
    pub ready: bool,
}
impl Readiness {
    pub fn terrane_construct() -> Self {
        Self { ready: false }
    }
}
async fn hold(
    observer: std::sync::Arc<TerraneSharedCell<Readiness>>,
) -> Result<(), TerraneError> {
    {
        let outer: Marker = __terrane_traced_err(
            acquire(String::from("cancel-outer"), false),
            16 /* terrane-site: case.trn:91:26-91:49 */,
        )?;
        {
            let inner: Marker = __terrane_traced_err(
                acquire(String::from("cancel-inner"), false),
                17 /* terrane-site: case.trn:92:30-92:53 */,
            )?;
            println!(
                "{}{}{}{}", String::from("holding "),
                terrane_scalar_support::scalar_text(&outer.name), String::from(" "),
                terrane_scalar_support::scalar_text(&inner.name)
            );
            {
                let __terrane_owner_0 = observer.clone();
                let __terrane_value = true;
                __terrane_owner_0
                    .lock()
                    .expect("shared reference lock poisoned")
                    .ready = __terrane_value;
            }
            {
                let _: () = __terrane_await(
                        terrane_time_sleep_after(
                            __terrane_traced_err(
                                    Duration::terrane_static_seconds(
                                        terrane_int_support::Int::from(30_i128),
                                    ),
                                    18 /* terrane-site: case.trn:95:38-95:59 */,
                                )?
                                .total_nanoseconds
                                .clone(),
                        ),
                    )
                    .await;
            }
        }
    }
    return Ok(());
}
async fn fail_when_ready(
    observer: std::sync::Arc<TerraneSharedCell<Readiness>>,
) -> Result<(), TerraneError> {
    while !{
        let __terrane_snapshot = observer
            .lock()
            .expect("shared reference lock poisoned")
            .ready;
        __terrane_snapshot
    } {
        {
            let _: () = __terrane_await(
                    terrane_time_sleep_after(
                        __terrane_traced_err(
                                Duration::terrane_static_milliseconds(
                                    terrane_int_support::Int::from(1_i128),
                                ),
                                19 /* terrane-site: case.trn:102:34-102:59 */,
                            )?
                            .total_nanoseconds
                            .clone(),
                    ),
                )
                .await;
        }
    }
    return Err(
        TerraneError::raised(
            TerraneErrorKind::ArithmeticOverflow,
            20 /* terrane-site: case.trn:103:5-103:30 */,
        ),
    );
}
fn main() {
    __terrane_run(async move {
        let observer: std::sync::Arc<TerraneSharedCell<Readiness>>;
        let scope: TerraneTaskScope;
        let pending: TerraneScopedTask<()>;
        let failing: TerraneScopedTask<()>;
        let joined: TerraneTaskOutcome<()>;
        __terrane_traced(
            ordinary_exits(),
            21 /* terrane-site: case.trn:106:5-106:20 */,
        );
        observer = std::sync::Arc::new(
            TerraneSharedCell::new(Readiness::terrane_construct()),
        );
        scope = TerraneTaskScope::new(None);
        pending = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = hold(observer.clone());
            TerraneScopedTask::spawn(async move {
                match __terrane_cancellable(
                        __terrane_spawned_task,
                        __terrane_cancel,
                        __terrane_deadline,
                    )
                    .await
                {
                    Some(Ok(value)) => TerraneTaskResult::Completed(value),
                    Some(Err(error)) => TerraneTaskResult::Failed(error),
                    None => TerraneTaskResult::Cancelled,
                }
            })
        };
        failing = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = fail_when_ready(observer.clone());
            TerraneScopedTask::spawn(async move {
                match __terrane_cancellable(
                        __terrane_spawned_task,
                        __terrane_cancel,
                        __terrane_deadline,
                    )
                    .await
                {
                    Some(Ok(value)) => TerraneTaskResult::Completed(value),
                    Some(Err(error)) => TerraneTaskResult::Failed(error),
                    None => TerraneTaskResult::Cancelled,
                }
            })
        };
        {
            let _: TerraneTaskOutcome<()> = __terrane_await(scope.join(failing)).await;
        }
        joined = __terrane_await(scope.join(pending)).await;
        println!(
            "{}{}", String::from("cancelled "),
            terrane_scalar_support::scalar_text(&joined.cancelled)
        );
    });
}
