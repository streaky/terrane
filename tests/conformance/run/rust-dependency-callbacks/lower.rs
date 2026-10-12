// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn add_two(value: terrane_int_support::Int) -> terrane_int_support::Int {
    return value.clone() + terrane_int_support::Int::from(2_i128);
}
fn keep_string(value: String) -> String {
    return value;
}
async fn keep_string_async(value: String) -> String {
    return value;
}
async fn stay_pending(value: terrane_int_support::Int) -> terrane_int_support::Int {
    let result: terrane_int_support::Int;
    result = __terrane_traced(
        __terrane_await({
                let __terrane_future = pending_value(value.clone());
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        0 /* terrane-site: src/main.trn:20:24-20:44 */,
                    )
                }
            })
            .await,
        0 /* terrane-site: src/main.trn:20:24-20:44 */,
    );
    return result.clone();
}
async fn run_retained() -> terrane_int_support::Int {
    let __terrane_completion_0: TerraneCompletion<terrane_int_support::Int> = async {
        let __terrane_try_0: TerraneCompletion<terrane_int_support::Int> = async {
            return TerraneCompletion::Return(
                __terrane_traced_completion!(
                    __terrane_await({ let __terrane_future =
                    invoke_retained(std::sync::Arc::new(move | argument_0 :
                    terrane_int_support::Int | -> std::pin::Pin < Box < dyn Future <
                    Output = _ > + Send >> { Box::pin(stay_pending(argument_0)) }));
                    async move { __terrane_raised_err(__terrane_future. await,
                    1 /* terrane-site: src/main.trn:25:22-25:51 */) } }). await,
                    1 /* terrane-site: src/main.trn:25:22-25:51 */
                ),
            );
        }
            .await;
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0
                    && __terrane_error_0.kind
                        == TerraneErrorKind::Custom(DescriptorId(1))
                {
                    __terrane_handled_0 = true;
                    return TerraneCompletion::Return(
                        terrane_int_support::Int::from(0_i128),
                    );
                }
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    }
        .await;
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {
            __terrane_generated_defect("non-fallthrough try completed normally")
        }
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
fn render(label: String, enabled: bool) -> String {
    if enabled {
        return terrane_string_support::upper(&label);
    }
    return label;
}
#[derive(Clone)]
pub struct ProjectedMessage {}
impl ProjectedMessage {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn render(&self, label: String) -> String {
        return terrane_string_support::upper(&label);
    }
}
impl RenderableProtocol for ProjectedMessage {
    fn clone_box(&self) -> Box<dyn RenderableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn RenderableProtocol> {
        Box::new(self.clone())
    }
    fn render(&self, label_: String) -> String {
        ProjectedMessage::render(&*self, label_)
    }
    fn decorated(&self, label_: String) -> String {
        || -> Result<String, crate::TerraneForeignError> {
            let __terrane_default = <ProjectedMessage as terrane_render_witness::Renderable>::decorated(
                &*self,
                label_,
            );
            Ok(__terrane_default)
        }()
            .unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn borrowed(&self, label_: String) -> String {
        || -> Result<String, crate::TerraneForeignError> {
            let __terrane_default = <ProjectedMessage as terrane_render_witness::Renderable>::borrowed(
                &*self,
                &label_,
            );
            Ok(__terrane_default)
        }()
            .unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn parsed(&self, text: String) -> Result<terrane_int_support::Int, TerraneError> {
        || -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
            let __terrane_default = match <ProjectedMessage as terrane_render_witness::Renderable>::parsed(
                &*self,
                text,
            ) {
                Ok(value) => value,
                Err(error) => {
                    return Err(
                        crate::TerraneForeignError(
                            crate::TerraneError::custom_raised(
                                crate::TERRANE_DEPENDENCY_ERROR,
                                format!(
                                    "Rust dependency `terrane_render_witness::Renderable` member `parsed` failed: {error}"
                                ),
                                crate::TERRANE_NO_SITE,
                            ),
                        ),
                    );
                }
            };
            Ok(terrane_int_support::Int::from(i128::from(__terrane_default)))
        }()
            .map_err(|error| error.raised(crate::TERRANE_NO_SITE))
    }
}
impl From<ProjectedMessage> for Renderable {
    fn from(value: ProjectedMessage) -> Self {
        Self(Box::new(value))
    }
}
impl terrane_render_witness::Renderable for ProjectedMessage {
    fn render(&self, label_: String) -> String {
        let __terrane_boundary: Result<String, crate::TerraneForeignError> = (|| {
            let __terrane_value = <ProjectedMessage>::render(&*self, label_);
            Ok(__terrane_value)
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
#[derive(Clone)]
pub struct ProjectedCounter {
    pub total: terrane_int_support::Int,
}
impl ProjectedCounter {
    pub fn terrane_construct() -> Self {
        Self {
            total: terrane_int_support::Int::from(0_i128),
        }
    }
    pub fn adjust(
        &mut self,
        delta: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        self.total = self.total.clone() + delta.clone();
        return self.total.clone();
    }
    pub fn current(&self) -> terrane_int_support::Int {
        return self.total.clone();
    }
}
impl AdjustableProtocol for ProjectedCounter {
    fn clone_box(&self) -> Box<dyn AdjustableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn AdjustableProtocol> {
        Box::new(self.clone())
    }
    fn adjust(&mut self, delta: terrane_int_support::Int) -> terrane_int_support::Int {
        ProjectedCounter::adjust(&mut *self, delta)
    }
    fn current(&self) -> terrane_int_support::Int {
        ProjectedCounter::current(&*self)
    }
}
impl From<ProjectedCounter> for Adjustable {
    fn from(value: ProjectedCounter) -> Self {
        Self(Box::new(value))
    }
}
impl terrane_callback_witness::Adjustable for ProjectedCounter {
    fn adjust(&mut self, delta: i64) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <ProjectedCounter>::adjust(
                &mut *self,
                terrane_int_support::Int::from(i128::from(delta)),
            );
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn current(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <ProjectedCounter>::current(&*self);
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
#[derive(Clone)]
pub struct ProjectedLocalCounter {
    pub total: terrane_int_support::Int,
}
impl ProjectedLocalCounter {
    pub fn terrane_construct() -> Self {
        Self {
            total: terrane_int_support::Int::from(0_i128),
        }
    }
    pub fn adjust(
        &mut self,
        delta: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        self.total = self.total.clone() + delta.clone();
        return self.total.clone();
    }
    pub fn current(&self) -> terrane_int_support::Int {
        return self.total.clone();
    }
}
impl LocalAdjustableProtocol for ProjectedLocalCounter {
    fn clone_box(&self) -> Box<dyn LocalAdjustableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn LocalAdjustableProtocol> {
        Box::new(self.clone())
    }
    fn adjust(&mut self, delta: terrane_int_support::Int) -> terrane_int_support::Int {
        ProjectedLocalCounter::adjust(&mut *self, delta)
    }
    fn current(&self) -> terrane_int_support::Int {
        ProjectedLocalCounter::current(&*self)
    }
}
impl From<ProjectedLocalCounter> for LocalAdjustable {
    fn from(value: ProjectedLocalCounter) -> Self {
        Self(Box::new(value))
    }
}
impl terrane_callback_witness::LocalAdjustable for ProjectedLocalCounter {
    fn adjust(&mut self, delta: i64) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <ProjectedLocalCounter>::adjust(
                &mut *self,
                terrane_int_support::Int::from(i128::from(delta)),
            );
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn current(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <ProjectedLocalCounter>::current(&*self);
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
pub struct DropAwareValue {
    pub token: DropToken,
}
impl DropAwareValue {
    pub fn terrane_construct() -> Self {
        Self {
            token: __terrane_raised(
                drop_token(),
                2 /* terrane-site: src/main.trn:60:24-60:40 */,
            ),
        }
    }
    pub fn label(&self) -> String {
        return String::from("drop-aware");
    }
    pub fn destruct(&mut self) {
        println!("{}", String::from("drop-aware-destruct"));
    }
}
impl terrane_callback_witness::DropAware for DropAwareValue {
    fn label(&self) -> String {
        let __terrane_boundary: Result<String, crate::TerraneForeignError> = (|| {
            let __terrane_value = <DropAwareValue>::label(&*self);
            Ok(__terrane_value)
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
impl Drop for DropAwareValue {
    fn drop(&mut self) {
        self.destruct();
    }
}
#[derive(Clone)]
pub struct ProjectedAsyncEntry {
    __terrane_lifetime: std::sync::Arc<()>,
}
impl ProjectedAsyncEntry {
    pub fn terrane_construct() -> Self {
        Self {
            __terrane_lifetime: std::sync::Arc::new(()),
        }
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    pub async fn shared(&self) -> terrane_int_support::Int {
        let mut __terrane_finally_guard_1 = __terrane_finally_guard();
        let __terrane_maybe_completion_1: Option<
            TerraneCompletion<terrane_int_support::Int>,
        > = __terrane_cancel_operation(
                &__terrane_finally_guard_1,
                async {
                    let __terrane_try_1: TerraneCompletion<terrane_int_support::Int> = async {
                        let mut __terrane_finally_guard_2 = __terrane_finally_guard();
                        let __terrane_maybe_completion_2: Option<
                            TerraneCompletion<terrane_int_support::Int>,
                        > = __terrane_cancel_operation(
                                &__terrane_finally_guard_2,
                                async {
                                    let __terrane_try_2: TerraneCompletion<
                                        terrane_int_support::Int,
                                    > = async {
                                        return TerraneCompletion::Return(
                                            __terrane_traced_completion!(
                                                __terrane_await({ let __terrane_future =
                                                pending_value(terrane_int_support::Int::from(0_i128)); async
                                                move { __terrane_raised_err(__terrane_future. await,
                                                3 /* terrane-site: src/main.trn:72:30-72:46 */) } }).
                                                await, 3 /* terrane-site: src/main.trn:72:30-72:46 */
                                            ),
                                        );
                                    }
                                        .await;
                                    match __terrane_try_2 {
                                        TerraneCompletion::Return(value) => {
                                            return TerraneCompletion::Return(value);
                                        }
                                        TerraneCompletion::Break => return TerraneCompletion::Break,
                                        TerraneCompletion::Continue => {
                                            return TerraneCompletion::Continue;
                                        }
                                        TerraneCompletion::Normal => {}
                                        TerraneCompletion::Error(__terrane_error_2) => {
                                            let mut __terrane_handled_2 = false;
                                            if !__terrane_handled_2 {
                                                return TerraneCompletion::Error(__terrane_error_2);
                                            }
                                        }
                                    }
                                    TerraneCompletion::Normal
                                },
                            )
                            .await;
                        let __terrane_cancelled_2 = __terrane_maybe_completion_2
                            .is_none();
                        let mut __terrane_completion_2 = __terrane_maybe_completion_2
                            .unwrap_or(TerraneCompletion::Normal);
                        let __terrane_finally_2: TerraneCompletion<
                            terrane_int_support::Int,
                        > = (|| {
                            __terrane_raised_completion!(
                                record_async_entry_cleanup(), 4 /* terrane-site: src/main.trn:74:17-74:44 */
                            );
                            TerraneCompletion::Normal
                        })();
                        match __terrane_finally_2 {
                            TerraneCompletion::Normal => {}
                            replacement => __terrane_completion_2 = replacement,
                        }
                        if __terrane_cancelled_2
                            && matches!(
                                &__terrane_completion_2, TerraneCompletion::Normal
                            )
                        {
                            __terrane_finish_cancelled_finally(__terrane_finally_guard_2)
                                .await;
                        }
                        __terrane_finally_guard_2.finish();
                        match __terrane_completion_2 {
                            TerraneCompletion::Normal => {
                                __terrane_generated_defect(
                                    "non-fallthrough try completed normally",
                                )
                            }
                            TerraneCompletion::Return(value) => {
                                return TerraneCompletion::Return(value);
                            }
                            TerraneCompletion::Error(error) => {
                                return TerraneCompletion::Error(error);
                            }
                            TerraneCompletion::Break | TerraneCompletion::Continue => {
                                __terrane_generated_defect(
                                    "loop control escaped a non-loop try",
                                )
                            }
                        }
                    }
                        .await;
                    match __terrane_try_1 {
                        TerraneCompletion::Return(value) => {
                            return TerraneCompletion::Return(value);
                        }
                        TerraneCompletion::Break => return TerraneCompletion::Break,
                        TerraneCompletion::Continue => return TerraneCompletion::Continue,
                        TerraneCompletion::Normal => {}
                        TerraneCompletion::Error(__terrane_error_1) => {
                            let mut __terrane_handled_1 = false;
                            if !__terrane_handled_1 {
                                return TerraneCompletion::Error(__terrane_error_1);
                            }
                        }
                    }
                    TerraneCompletion::Normal
                },
            )
            .await;
        let __terrane_cancelled_1 = __terrane_maybe_completion_1.is_none();
        let mut __terrane_completion_1 = __terrane_maybe_completion_1
            .unwrap_or(TerraneCompletion::Normal);
        let __terrane_finally_1: TerraneCompletion<terrane_int_support::Int> = (|| {
            __terrane_raised_completion!(
                record_async_entry_cleanup(), 5 /* terrane-site: src/main.trn:76:13-76:40 */
            );
            TerraneCompletion::Normal
        })();
        match __terrane_finally_1 {
            TerraneCompletion::Normal => {}
            replacement => __terrane_completion_1 = replacement,
        }
        if __terrane_cancelled_1
            && matches!(&__terrane_completion_1, TerraneCompletion::Normal)
        {
            __terrane_finish_cancelled_finally(__terrane_finally_guard_1).await;
        }
        __terrane_finally_guard_1.finish();
        match __terrane_completion_1 {
            TerraneCompletion::Normal => {
                __terrane_generated_defect("non-fallthrough try completed normally")
            }
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
    }
    pub async fn mutable(&mut self) -> terrane_int_support::Int {
        let mut __terrane_finally_guard_3 = __terrane_finally_guard();
        let __terrane_maybe_completion_3: Option<
            TerraneCompletion<terrane_int_support::Int>,
        > = __terrane_cancel_operation(
                &__terrane_finally_guard_3,
                async {
                    let __terrane_try_3: TerraneCompletion<terrane_int_support::Int> = async {
                        let mut __terrane_finally_guard_4 = __terrane_finally_guard();
                        let __terrane_maybe_completion_4: Option<
                            TerraneCompletion<terrane_int_support::Int>,
                        > = __terrane_cancel_operation(
                                &__terrane_finally_guard_4,
                                async {
                                    let __terrane_try_4: TerraneCompletion<
                                        terrane_int_support::Int,
                                    > = async {
                                        return TerraneCompletion::Return(
                                            __terrane_traced_completion!(
                                                __terrane_await({ let __terrane_future =
                                                pending_value(terrane_int_support::Int::from(0_i128)); async
                                                move { __terrane_raised_err(__terrane_future. await,
                                                6 /* terrane-site: src/main.trn:81:30-81:46 */) } }).
                                                await, 6 /* terrane-site: src/main.trn:81:30-81:46 */
                                            ),
                                        );
                                    }
                                        .await;
                                    match __terrane_try_4 {
                                        TerraneCompletion::Return(value) => {
                                            return TerraneCompletion::Return(value);
                                        }
                                        TerraneCompletion::Break => return TerraneCompletion::Break,
                                        TerraneCompletion::Continue => {
                                            return TerraneCompletion::Continue;
                                        }
                                        TerraneCompletion::Normal => {}
                                        TerraneCompletion::Error(__terrane_error_4) => {
                                            let mut __terrane_handled_4 = false;
                                            if !__terrane_handled_4 {
                                                return TerraneCompletion::Error(__terrane_error_4);
                                            }
                                        }
                                    }
                                    TerraneCompletion::Normal
                                },
                            )
                            .await;
                        let __terrane_cancelled_4 = __terrane_maybe_completion_4
                            .is_none();
                        let mut __terrane_completion_4 = __terrane_maybe_completion_4
                            .unwrap_or(TerraneCompletion::Normal);
                        let __terrane_finally_4: TerraneCompletion<
                            terrane_int_support::Int,
                        > = (|| {
                            __terrane_raised_completion!(
                                record_async_entry_cleanup(), 7 /* terrane-site: src/main.trn:83:17-83:44 */
                            );
                            TerraneCompletion::Normal
                        })();
                        match __terrane_finally_4 {
                            TerraneCompletion::Normal => {}
                            replacement => __terrane_completion_4 = replacement,
                        }
                        if __terrane_cancelled_4
                            && matches!(
                                &__terrane_completion_4, TerraneCompletion::Normal
                            )
                        {
                            __terrane_finish_cancelled_finally(__terrane_finally_guard_4)
                                .await;
                        }
                        __terrane_finally_guard_4.finish();
                        match __terrane_completion_4 {
                            TerraneCompletion::Normal => {
                                __terrane_generated_defect(
                                    "non-fallthrough try completed normally",
                                )
                            }
                            TerraneCompletion::Return(value) => {
                                return TerraneCompletion::Return(value);
                            }
                            TerraneCompletion::Error(error) => {
                                return TerraneCompletion::Error(error);
                            }
                            TerraneCompletion::Break | TerraneCompletion::Continue => {
                                __terrane_generated_defect(
                                    "loop control escaped a non-loop try",
                                )
                            }
                        }
                    }
                        .await;
                    match __terrane_try_3 {
                        TerraneCompletion::Return(value) => {
                            return TerraneCompletion::Return(value);
                        }
                        TerraneCompletion::Break => return TerraneCompletion::Break,
                        TerraneCompletion::Continue => return TerraneCompletion::Continue,
                        TerraneCompletion::Normal => {}
                        TerraneCompletion::Error(__terrane_error_3) => {
                            let mut __terrane_handled_3 = false;
                            if !__terrane_handled_3 {
                                return TerraneCompletion::Error(__terrane_error_3);
                            }
                        }
                    }
                    TerraneCompletion::Normal
                },
            )
            .await;
        let __terrane_cancelled_3 = __terrane_maybe_completion_3.is_none();
        let mut __terrane_completion_3 = __terrane_maybe_completion_3
            .unwrap_or(TerraneCompletion::Normal);
        let __terrane_finally_3: TerraneCompletion<terrane_int_support::Int> = (|| {
            __terrane_raised_completion!(
                record_async_entry_cleanup(), 8 /* terrane-site: src/main.trn:85:13-85:40 */
            );
            TerraneCompletion::Normal
        })();
        match __terrane_finally_3 {
            TerraneCompletion::Normal => {}
            replacement => __terrane_completion_3 = replacement,
        }
        if __terrane_cancelled_3
            && matches!(&__terrane_completion_3, TerraneCompletion::Normal)
        {
            __terrane_finish_cancelled_finally(__terrane_finally_guard_3).await;
        }
        __terrane_finally_guard_3.finish();
        match __terrane_completion_3 {
            TerraneCompletion::Normal => {
                __terrane_generated_defect("non-fallthrough try completed normally")
            }
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
    }
    pub async fn consuming(self) -> terrane_int_support::Int {
        let mut __terrane_finally_guard_5 = __terrane_finally_guard();
        let __terrane_maybe_completion_5: Option<
            TerraneCompletion<terrane_int_support::Int>,
        > = __terrane_cancel_operation(
                &__terrane_finally_guard_5,
                async {
                    let __terrane_try_5: TerraneCompletion<terrane_int_support::Int> = async {
                        let mut __terrane_finally_guard_6 = __terrane_finally_guard();
                        let __terrane_maybe_completion_6: Option<
                            TerraneCompletion<terrane_int_support::Int>,
                        > = __terrane_cancel_operation(
                                &__terrane_finally_guard_6,
                                async {
                                    let __terrane_try_6: TerraneCompletion<
                                        terrane_int_support::Int,
                                    > = async {
                                        return TerraneCompletion::Return(
                                            __terrane_traced_completion!(
                                                __terrane_await({ let __terrane_future =
                                                pending_value(terrane_int_support::Int::from(0_i128)); async
                                                move { __terrane_raised_err(__terrane_future. await,
                                                9 /* terrane-site: src/main.trn:90:30-90:46 */) } }).
                                                await, 9 /* terrane-site: src/main.trn:90:30-90:46 */
                                            ),
                                        );
                                    }
                                        .await;
                                    match __terrane_try_6 {
                                        TerraneCompletion::Return(value) => {
                                            return TerraneCompletion::Return(value);
                                        }
                                        TerraneCompletion::Break => return TerraneCompletion::Break,
                                        TerraneCompletion::Continue => {
                                            return TerraneCompletion::Continue;
                                        }
                                        TerraneCompletion::Normal => {}
                                        TerraneCompletion::Error(__terrane_error_6) => {
                                            let mut __terrane_handled_6 = false;
                                            if !__terrane_handled_6 {
                                                return TerraneCompletion::Error(__terrane_error_6);
                                            }
                                        }
                                    }
                                    TerraneCompletion::Normal
                                },
                            )
                            .await;
                        let __terrane_cancelled_6 = __terrane_maybe_completion_6
                            .is_none();
                        let mut __terrane_completion_6 = __terrane_maybe_completion_6
                            .unwrap_or(TerraneCompletion::Normal);
                        let __terrane_finally_6: TerraneCompletion<
                            terrane_int_support::Int,
                        > = (|| {
                            __terrane_raised_completion!(
                                record_async_entry_cleanup(), 10 /* terrane-site: src/main.trn:92:17-92:44 */
                            );
                            TerraneCompletion::Normal
                        })();
                        match __terrane_finally_6 {
                            TerraneCompletion::Normal => {}
                            replacement => __terrane_completion_6 = replacement,
                        }
                        if __terrane_cancelled_6
                            && matches!(
                                &__terrane_completion_6, TerraneCompletion::Normal
                            )
                        {
                            __terrane_finish_cancelled_finally(__terrane_finally_guard_6)
                                .await;
                        }
                        __terrane_finally_guard_6.finish();
                        match __terrane_completion_6 {
                            TerraneCompletion::Normal => {
                                __terrane_generated_defect(
                                    "non-fallthrough try completed normally",
                                )
                            }
                            TerraneCompletion::Return(value) => {
                                return TerraneCompletion::Return(value);
                            }
                            TerraneCompletion::Error(error) => {
                                return TerraneCompletion::Error(error);
                            }
                            TerraneCompletion::Break | TerraneCompletion::Continue => {
                                __terrane_generated_defect(
                                    "loop control escaped a non-loop try",
                                )
                            }
                        }
                    }
                        .await;
                    match __terrane_try_5 {
                        TerraneCompletion::Return(value) => {
                            return TerraneCompletion::Return(value);
                        }
                        TerraneCompletion::Break => return TerraneCompletion::Break,
                        TerraneCompletion::Continue => return TerraneCompletion::Continue,
                        TerraneCompletion::Normal => {}
                        TerraneCompletion::Error(__terrane_error_5) => {
                            let mut __terrane_handled_5 = false;
                            if !__terrane_handled_5 {
                                return TerraneCompletion::Error(__terrane_error_5);
                            }
                        }
                    }
                    TerraneCompletion::Normal
                },
            )
            .await;
        let __terrane_cancelled_5 = __terrane_maybe_completion_5.is_none();
        let mut __terrane_completion_5 = __terrane_maybe_completion_5
            .unwrap_or(TerraneCompletion::Normal);
        let __terrane_finally_5: TerraneCompletion<terrane_int_support::Int> = (|| {
            __terrane_raised_completion!(
                record_async_entry_cleanup(), 11 /* terrane-site: src/main.trn:94:13-94:40 */
            );
            TerraneCompletion::Normal
        })();
        match __terrane_finally_5 {
            TerraneCompletion::Normal => {}
            replacement => __terrane_completion_5 = replacement,
        }
        if __terrane_cancelled_5
            && matches!(&__terrane_completion_5, TerraneCompletion::Normal)
        {
            __terrane_finish_cancelled_finally(__terrane_finally_guard_5).await;
        }
        __terrane_finally_guard_5.finish();
        match __terrane_completion_5 {
            TerraneCompletion::Normal => {
                __terrane_generated_defect("non-fallthrough try completed normally")
            }
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
    }
    pub fn destruct(&mut self) {
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(async_entry_cleanup_count(),
            12 /* terrane-site: src/main.trn:97:17-97:43 */))
        );
    }
}
impl AsyncEntryProtocol for ProjectedAsyncEntry {
    fn clone_box(&self) -> Box<dyn AsyncEntryProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn AsyncEntryProtocol> {
        Box::new(self.terrane_separate())
    }
    fn shared(
        &self,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = terrane_int_support::Int> + Send + '_>,
    > {
        Box::pin(async move { ProjectedAsyncEntry::shared(&*self).await })
    }
    fn mutable(
        &mut self,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = terrane_int_support::Int> + Send + '_>,
    > {
        Box::pin(async move { ProjectedAsyncEntry::mutable(&mut *self).await })
    }
    fn consuming(
        self: Box<Self>,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = terrane_int_support::Int> + Send + 'static>,
    > {
        Box::pin(async move { ProjectedAsyncEntry::consuming(*self).await })
    }
}
impl From<ProjectedAsyncEntry> for AsyncEntry {
    fn from(value: ProjectedAsyncEntry) -> Self {
        Self(Box::new(value))
    }
}
impl terrane_callback_witness::AsyncEntry for ProjectedAsyncEntry {
    async fn shared(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = async {
            let __terrane_value = {
                let __terrane_receiver = self.clone();
                __terrane_projected_async_entry(async move {
                        <ProjectedAsyncEntry>::shared(&__terrane_receiver).await
                    })
                    .await
            };
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        }
            .await;
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
    async fn mutable(&mut self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = async {
            let __terrane_value = {
                let mut __terrane_receiver = self.clone();
                let (__terrane_output, __terrane_receiver) = __terrane_projected_async_entry(async move {
                        let __terrane_output = <ProjectedAsyncEntry>::mutable(
                                &mut __terrane_receiver,
                            )
                            .await;
                        (__terrane_output, __terrane_receiver)
                    })
                    .await;
                *self = __terrane_receiver;
                __terrane_output
            };
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        }
            .await;
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
    async fn consuming(self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = async {
            let __terrane_value = __terrane_projected_async_entry(async move {
                    <ProjectedAsyncEntry>::consuming(self).await
                })
                .await;
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        }
            .await;
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
impl Drop for ProjectedAsyncEntry {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) != 1 {
            return;
        }
        self.destruct();
    }
}
#[derive(Clone)]
pub struct ProjectedCounterHolder {
    pub inner: ProjectedCounter,
}
impl ProjectedCounterHolder {
    pub fn terrane_construct() -> Self {
        Self {
            inner: ProjectedCounter::terrane_construct(),
        }
    }
}
fn main() {
    __terrane_run(async move {
        let changed: String;
        let offset: i64;
        let add_offset: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> std::pin::Pin<
                    Box<dyn Future<Output = terrane_int_support::Int> + Send>,
                > + Send + Sync,
        >;
        let concurrent: terrane_int_support::Int;
        let callback_registry: Registrar;
        let retained_scope: TerraneTaskScope;
        let retained_child: TerraneScopedTask<terrane_int_support::Int>;
        let active: bool;
        let retained_outcome: TerraneTaskOutcome<terrane_int_support::Int>;
        let message: ProjectedMessage;
        let interface_message: Renderable;
        let impl_message: ProjectedMessage;
        let mut counter: ProjectedCounter;
        let mut holder: ProjectedCounterHolder;
        let mut retained_counter: AdjustableOwner;
        let boxed_counter: ProjectedCounter;
        let mut boxed_owner: AdjustableOwner;
        let erased_boxed: Adjustable;
        let mut erased_owner: AdjustableOwner;
        let mut local_owner: LocalAdjustableOwner;
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(apply_shared(terrane_int_support::Int::from(40_i128),
            std::sync::Arc::new(add_two)), 13 /* terrane-site: src/main.trn:103:13-103:38 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(apply_mutable(terrane_int_support::Int::from(10_i128),
            TerraneMutableCallable::new(move | (argument_0,) :
            (terrane_int_support::Int,) | add_two(argument_0))), 14 /* terrane-site: src/main.trn:104:13-104:39 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(apply_once(String::from("HELLO"),
            TerraneConsumingCallable::new(move | (argument_0,) : (String,) |
            keep_string(argument_0))), 15 /* terrane-site: src/main.trn:105:13-105:45 */))
        );
        changed = __terrane_traced(
            __terrane_await({
                    let __terrane_future = apply_async(
                        String::from("hello"),
                        std::sync::Arc::new(move |
                            argument_0: String,
                        | -> std::pin::Pin<Box<dyn Future<Output = _> + Send>> {
                            Box::pin(keep_string_async(argument_0))
                        }),
                    );
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            16 /* terrane-site: src/main.trn:106:28-106:67 */,
                        )
                    }
                })
                .await,
            16 /* terrane-site: src/main.trn:106:28-106:67 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&changed));
        offset = 10;
        add_offset = {
            let offset = offset.clone();
            std::sync::Arc::new(move |
                value: terrane_int_support::Int,
            | -> std::pin::Pin<
                Box<dyn Future<Output = terrane_int_support::Int> + Send>,
            > {
                let offset = offset.clone();
                Box::pin(async move {
                    return value.clone()
                        + terrane_int_support::Int::from(offset as i128);
                })
            })
        };
        concurrent = __terrane_traced(
            __terrane_await({
                    let __terrane_future = apply_async_concurrently(
                        terrane_int_support::Int::from(20_i128),
                        add_offset.clone(),
                    );
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            17 /* terrane-site: src/main.trn:111:28-111:68 */,
                        )
                    }
                })
                .await,
            17 /* terrane-site: src/main.trn:111:28-111:68 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&concurrent));
        callback_registry = __terrane_raised(
            registrar(),
            18 /* terrane-site: src/main.trn:113:25-113:35 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
            (&callback_registry).apply(match | | -> Result < _, crate
            ::TerraneForeignError > { Ok(terrane_int_support::coerce:: < i64 >
            (&terrane_int_support::Int::from(5_i128)).map_err(| error | crate
            ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
            ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
            std::panic::panic_any(error) }, match | | -> Result < _, crate
            ::TerraneForeignError > { Ok({ let callback = std::sync::Arc::new(add_two)
            .clone(); move | callback_argument_0 : i64 | { match | | -> Result < _, crate
            ::TerraneForeignError > { let callback_value =
            callback(terrane_int_support::Int::from(i128::from(callback_argument_0)));
            Ok(terrane_int_support::coerce:: < i64 > (&callback_value).map_err(| error |
            crate ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
            ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
            std::panic::panic_any(error.0) } } }) } () { Ok(value) => value, Err(error)
            => std::panic::panic_any(error) }))) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "terrane_callback_witness",
            "terrane_callback_witness::Registrar::apply")) }, 19 /* terrane-site: src/main.trn:114:13-114:48 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(dispatch(String::from("dissimilar"),
            true, std::sync::Arc::new(render)), 20 /* terrane-site: src/main.trn:116:13-116:49 */))
        );
        retained_scope = TerraneTaskScope::new(None);
        retained_child = {
            let __terrane_scope = retained_scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            TerraneScopedTask::spawn(async move {
                match __terrane_cancellable(
                        std::sync::Arc::new(move || -> std::pin::Pin<
                            Box<dyn Future<Output = _> + Send>,
                        > { Box::pin(run_retained()) })(),
                        __terrane_cancel,
                        __terrane_deadline,
                    )
                    .await
                {
                    Some(value) => TerraneTaskResult::Completed(value),
                    None => TerraneTaskResult::Cancelled,
                }
            })
        };
        active = __terrane_traced(
            __terrane_await({
                    let __terrane_future = wait_until_retained_invocation_active();
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            21 /* terrane-site: src/main.trn:119:25-119:63 */,
                        )
                    }
                })
                .await,
            21 /* terrane-site: src/main.trn:119:25-119:63 */,
        );
        retained_scope.cancel();
        retained_outcome = __terrane_await(retained_scope.join(retained_child)).await;
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&active),
            terrane_scalar_support::scalar_text(&retained_outcome.cancelled),
            terrane_scalar_support::scalar_text(&__terrane_raised(active_retained_invocations(),
            22 /* terrane-site: src/main.trn:122:49-122:77 */))
        );
        message = ProjectedMessage::terrane_construct();
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(render_value(&message,
            String::from("interface")), 23 /* terrane-site: src/main.trn:124:13-124:47 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(render_decorated(&message,
            String::from("default")), 24 /* terrane-site: src/main.trn:125:13-125:49 */))
        );
        interface_message = <Renderable>::from(message);
        println!(
            "{}", terrane_scalar_support::scalar_text(&(&interface_message)
            .decorated(String::from("provided")))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&(&interface_message)
            .borrowed(String::from("borrowed")))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced((&interface_message)
            .parsed(String::from("7")), 25 /* terrane-site: src/main.trn:129:13-129:42 */))
        );
        impl_message = ProjectedMessage::terrane_construct();
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(render_impl(&impl_message,
            String::from("opaque")), 26 /* terrane-site: src/main.trn:131:13-131:48 */))
        );
        counter = ProjectedCounter::terrane_construct();
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(adjust_value(&mut
            counter, terrane_int_support::Int::from(7_i128)), 27 /* terrane-site: src/main.trn:133:13-133:37 */))
        );
        holder = ProjectedCounterHolder::terrane_construct();
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(adjust_value(&mut
            holder.inner, terrane_int_support::Int::from(9_i128)), 28 /* terrane-site: src/main.trn:135:13-135:42 */))
        );
        retained_counter = __terrane_raised(
            retain_adjustable(counter),
            29 /* terrane-site: src/main.trn:136:24-136:50 */,
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | retained_counter
            .adjust(match | | -> Result < _, crate ::TerraneForeignError > {
            Ok(terrane_int_support::coerce:: < i64 >
            (&terrane_int_support::Int::from(5_i128)).map_err(| error | crate
            ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
            ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
            std::panic::panic_any(error) }))) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "terrane_callback_witness",
            "terrane_callback_witness::AdjustableOwner::adjust")) },
            30 /* terrane-site: src/main.trn:137:13-137:39 */)),
            terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&retained_counter)
            .current())) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "terrane_callback_witness",
            "terrane_callback_witness::AdjustableOwner::current")) },
            31 /* terrane-site: src/main.trn:137:43-137:68 */))
        );
        boxed_counter = ProjectedCounter::terrane_construct();
        boxed_owner = __terrane_raised(
            retain_boxed_adjustable(boxed_counter),
            32 /* terrane-site: src/main.trn:139:19-139:57 */,
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | boxed_owner
            .adjust(match | | -> Result < _, crate ::TerraneForeignError > {
            Ok(terrane_int_support::coerce:: < i64 >
            (&terrane_int_support::Int::from(11_i128)).map_err(| error | crate
            ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
            ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
            std::panic::panic_any(error) }))) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "terrane_callback_witness",
            "terrane_callback_witness::AdjustableOwner::adjust")) },
            33 /* terrane-site: src/main.trn:140:13-140:35 */)),
            terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&boxed_owner)
            .current())) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "terrane_callback_witness",
            "terrane_callback_witness::AdjustableOwner::current")) },
            34 /* terrane-site: src/main.trn:140:39-140:59 */))
        );
        erased_boxed = <Adjustable>::from(ProjectedCounter::terrane_construct());
        erased_owner = __terrane_raised(
            retain_boxed_adjustable(erased_boxed),
            35 /* terrane-site: src/main.trn:142:20-142:57 */,
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | erased_owner
            .adjust(match | | -> Result < _, crate ::TerraneForeignError > {
            Ok(terrane_int_support::coerce:: < i64 >
            (&terrane_int_support::Int::from(4_i128)).map_err(| error | crate
            ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
            ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
            std::panic::panic_any(error) }))) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "terrane_callback_witness",
            "terrane_callback_witness::AdjustableOwner::adjust")) },
            36 /* terrane-site: src/main.trn:143:13-143:35 */)),
            terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&erased_owner)
            .current())) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "terrane_callback_witness",
            "terrane_callback_witness::AdjustableOwner::current")) },
            37 /* terrane-site: src/main.trn:143:39-143:60 */))
        );
        local_owner = __terrane_raised(
            retain_local_adjustable(ProjectedLocalCounter::terrane_construct()),
            38 /* terrane-site: src/main.trn:144:19-144:79 */,
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | local_owner
            .adjust(match | | -> Result < _, crate ::TerraneForeignError > {
            Ok(terrane_int_support::coerce:: < i64 >
            (&terrane_int_support::Int::from(3_i128)).map_err(| error | crate
            ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
            ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
            std::panic::panic_any(error) }))) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "terrane_callback_witness",
            "terrane_callback_witness::LocalAdjustableOwner::adjust")) },
            39 /* terrane-site: src/main.trn:145:13-145:34 */)),
            terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&local_owner)
            .current())) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "terrane_callback_witness",
            "terrane_callback_witness::LocalAdjustableOwner::current")) },
            40 /* terrane-site: src/main.trn:145:38-145:58 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(consume_drop(DropAwareValue::terrane_construct()),
            41 /* terrane-site: src/main.trn:146:13-146:55 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await({
            let __terrane_future =
            cancel_async_entry(ProjectedAsyncEntry::terrane_construct()); async move {
            __terrane_raised_err(__terrane_future. await, 42 /* terrane-site: src/main.trn:147:19-147:72 */) } }). await, 42 /* terrane-site: src/main.trn:147:19-147:72 */))
        );
    });
}
