// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: try-loop-completion
async fn cleanup() {
    return ();
}
#[derive(Clone)]
pub struct Worker {}
impl Worker {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub async fn run(&self) {
        let mut index: terrane_int_support::Int;
        index = terrane_int_support::Int::from(0_i128);
        while index < terrane_int_support::Int::from(4_i128) {
            let mut __terrane_finally_guard_0 = __terrane_finally_guard();
            let __terrane_maybe_completion_0: Option<TerraneCompletion<()>> = __terrane_cancel_operation(
                    &__terrane_finally_guard_0,
                    async {
                        let __terrane_try_0: TerraneCompletion<()> = async {
                            index = index.clone()
                                + terrane_int_support::Int::from(1_i128);
                            if index == terrane_int_support::Int::from(1_i128) {
                                println!("{}", String::from("loop-continue"));
                                return TerraneCompletion::Continue;
                            }
                            if index == terrane_int_support::Int::from(3_i128) {
                                println!("{}", String::from("loop-break"));
                                return TerraneCompletion::Break;
                            }
                            println!("{}", String::from("loop-body"));
                            TerraneCompletion::Normal
                        }
                            .await;
                        match __terrane_try_0 {
                            TerraneCompletion::Return(value) => {
                                return TerraneCompletion::Return(value);
                            }
                            TerraneCompletion::Break => return TerraneCompletion::Break,
                            TerraneCompletion::Continue => {
                                return TerraneCompletion::Continue;
                            }
                            TerraneCompletion::Normal => {}
                            TerraneCompletion::Error(__terrane_error_0) => {
                                let mut __terrane_handled_0 = false;
                                if !__terrane_handled_0 {
                                    return TerraneCompletion::Error(__terrane_error_0);
                                }
                            }
                        }
                        TerraneCompletion::Normal
                    },
                )
                .await;
            let __terrane_cancelled_0 = __terrane_maybe_completion_0.is_none();
            let mut __terrane_completion_0 = __terrane_maybe_completion_0
                .unwrap_or(TerraneCompletion::Normal);
            let __terrane_finally_0: TerraneCompletion<()> = async {
                __terrane_await(cleanup()).await;
                println!("{}", String::from("loop-finally"));
                TerraneCompletion::Normal
            }
                .await;
            match __terrane_finally_0 {
                TerraneCompletion::Normal => {}
                replacement => __terrane_completion_0 = replacement,
            }
            if __terrane_cancelled_0
                && matches!(&__terrane_completion_0, TerraneCompletion::Normal)
            {
                __terrane_finish_cancelled_finally(__terrane_finally_guard_0).await;
            }
            __terrane_finally_guard_0.finish();
            match __terrane_completion_0 {
                TerraneCompletion::Normal => {}
                TerraneCompletion::Return(value) => return value,
                TerraneCompletion::Error(error) => __terrane_uncaught(error),
                TerraneCompletion::Break => break,
                TerraneCompletion::Continue => continue,
            }
        }
        let mut __terrane_finally_guard_1 = __terrane_finally_guard();
        let __terrane_maybe_completion_1: Option<TerraneCompletion<()>> = __terrane_cancel_operation(
                &__terrane_finally_guard_1,
                async {
                    let __terrane_try_1: TerraneCompletion<()> = async {
                        index = terrane_int_support::Int::from(0_i128);
                        loop {
                            let mut __terrane_finally_guard_2 = __terrane_finally_guard();
                            let __terrane_maybe_completion_2: Option<
                                TerraneCompletion<()>,
                            > = __terrane_cancel_operation(
                                    &__terrane_finally_guard_2,
                                    async {
                                        let __terrane_try_2: TerraneCompletion<()> = async {
                                            println!("{}", String::from("try-in-loop"));
                                            index = index.clone()
                                                + terrane_int_support::Int::from(1_i128);
                                            if index == terrane_int_support::Int::from(1_i128) {
                                                return TerraneCompletion::Continue;
                                            }
                                            return TerraneCompletion::Break;
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
                            let __terrane_finally_2: TerraneCompletion<()> = async {
                                __terrane_await(cleanup()).await;
                                TerraneCompletion::Normal
                            }
                                .await;
                            match __terrane_finally_2 {
                                TerraneCompletion::Normal => {}
                                replacement => __terrane_completion_2 = replacement,
                            }
                            if __terrane_cancelled_2
                                && matches!(
                                    &__terrane_completion_2, TerraneCompletion::Normal
                                )
                            {
                                __terrane_finish_cancelled_finally(
                                        __terrane_finally_guard_2,
                                    )
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
                                TerraneCompletion::Break => break,
                                TerraneCompletion::Continue => continue,
                            }
                        }
                        TerraneCompletion::Normal
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
        let __terrane_finally_1: TerraneCompletion<()> = async {
            __terrane_await(cleanup()).await;
            println!("{}", String::from("outer-finally"));
            TerraneCompletion::Normal
        }
            .await;
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
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        index = terrane_int_support::Int::from(0_i128);
        while index < terrane_int_support::Int::from(3_i128) {
            index = index.clone() + terrane_int_support::Int::from(1_i128);
            let mut __terrane_finally_guard_3 = __terrane_finally_guard();
            let __terrane_maybe_completion_3: Option<TerraneCompletion<()>> = __terrane_cancel_operation(
                    &__terrane_finally_guard_3,
                    async {
                        let __terrane_try_3: TerraneCompletion<()> = async {
                            let mut __terrane_finally_guard_4 = __terrane_finally_guard();
                            let __terrane_maybe_completion_4: Option<
                                TerraneCompletion<()>,
                            > = __terrane_cancel_operation(
                                    &__terrane_finally_guard_4,
                                    async {
                                        let __terrane_try_4: TerraneCompletion<()> = async {
                                            loop {
                                                println!("{}", String::from("nested-loop"));
                                                break;
                                            }
                                            TerraneCompletion::Normal
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
                            let __terrane_finally_4: TerraneCompletion<()> = async {
                                __terrane_await(cleanup()).await;
                                println!("{}", String::from("nested-finally"));
                                TerraneCompletion::Normal
                            }
                                .await;
                            match __terrane_finally_4 {
                                TerraneCompletion::Normal => {}
                                replacement => __terrane_completion_4 = replacement,
                            }
                            if __terrane_cancelled_4
                                && matches!(
                                    &__terrane_completion_4, TerraneCompletion::Normal
                                )
                            {
                                __terrane_finish_cancelled_finally(
                                        __terrane_finally_guard_4,
                                    )
                                    .await;
                            }
                            __terrane_finally_guard_4.finish();
                            match __terrane_completion_4 {
                                TerraneCompletion::Normal => {}
                                TerraneCompletion::Return(value) => {
                                    return TerraneCompletion::Return(value);
                                }
                                TerraneCompletion::Error(error) => {
                                    return TerraneCompletion::Error(error);
                                }
                                TerraneCompletion::Break => return TerraneCompletion::Break,
                                TerraneCompletion::Continue => {
                                    return TerraneCompletion::Continue;
                                }
                            }
                            if index == terrane_int_support::Int::from(1_i128) {
                                return TerraneCompletion::Continue;
                            }
                            return TerraneCompletion::Break;
                        }
                            .await;
                        match __terrane_try_3 {
                            TerraneCompletion::Return(value) => {
                                return TerraneCompletion::Return(value);
                            }
                            TerraneCompletion::Break => return TerraneCompletion::Break,
                            TerraneCompletion::Continue => {
                                return TerraneCompletion::Continue;
                            }
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
            let __terrane_finally_3: TerraneCompletion<()> = async {
                __terrane_await(cleanup()).await;
                println!("{}", String::from("nested-outer-finally"));
                TerraneCompletion::Normal
            }
                .await;
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
                TerraneCompletion::Break => break,
                TerraneCompletion::Continue => continue,
            }
        }
    }
    pub async fn __terrane_shared_async_run(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
    ) {
        let mut index: terrane_int_support::Int;
        index = terrane_int_support::Int::from(0_i128);
        while index < terrane_int_support::Int::from(4_i128) {
            let mut __terrane_finally_guard_5 = __terrane_finally_guard();
            let __terrane_maybe_completion_5: Option<TerraneCompletion<()>> = __terrane_cancel_operation(
                    &__terrane_finally_guard_5,
                    async {
                        let __terrane_try_5: TerraneCompletion<()> = async {
                            index = index.clone()
                                + terrane_int_support::Int::from(1_i128);
                            if index == terrane_int_support::Int::from(1_i128) {
                                println!("{}", String::from("loop-continue"));
                                return TerraneCompletion::Continue;
                            }
                            if index == terrane_int_support::Int::from(3_i128) {
                                println!("{}", String::from("loop-break"));
                                return TerraneCompletion::Break;
                            }
                            println!("{}", String::from("loop-body"));
                            TerraneCompletion::Normal
                        }
                            .await;
                        match __terrane_try_5 {
                            TerraneCompletion::Return(value) => {
                                return TerraneCompletion::Return(value);
                            }
                            TerraneCompletion::Break => return TerraneCompletion::Break,
                            TerraneCompletion::Continue => {
                                return TerraneCompletion::Continue;
                            }
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
            let __terrane_finally_5: TerraneCompletion<()> = async {
                __terrane_await(cleanup()).await;
                println!("{}", String::from("loop-finally"));
                TerraneCompletion::Normal
            }
                .await;
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
                TerraneCompletion::Normal => {}
                TerraneCompletion::Return(value) => return value,
                TerraneCompletion::Error(error) => __terrane_uncaught(error),
                TerraneCompletion::Break => break,
                TerraneCompletion::Continue => continue,
            }
        }
        let mut __terrane_finally_guard_6 = __terrane_finally_guard();
        let __terrane_maybe_completion_6: Option<TerraneCompletion<()>> = __terrane_cancel_operation(
                &__terrane_finally_guard_6,
                async {
                    let __terrane_try_6: TerraneCompletion<()> = async {
                        index = terrane_int_support::Int::from(0_i128);
                        loop {
                            let mut __terrane_finally_guard_7 = __terrane_finally_guard();
                            let __terrane_maybe_completion_7: Option<
                                TerraneCompletion<()>,
                            > = __terrane_cancel_operation(
                                    &__terrane_finally_guard_7,
                                    async {
                                        let __terrane_try_7: TerraneCompletion<()> = async {
                                            println!("{}", String::from("try-in-loop"));
                                            index = index.clone()
                                                + terrane_int_support::Int::from(1_i128);
                                            if index == terrane_int_support::Int::from(1_i128) {
                                                return TerraneCompletion::Continue;
                                            }
                                            return TerraneCompletion::Break;
                                        }
                                            .await;
                                        match __terrane_try_7 {
                                            TerraneCompletion::Return(value) => {
                                                return TerraneCompletion::Return(value);
                                            }
                                            TerraneCompletion::Break => return TerraneCompletion::Break,
                                            TerraneCompletion::Continue => {
                                                return TerraneCompletion::Continue;
                                            }
                                            TerraneCompletion::Normal => {}
                                            TerraneCompletion::Error(__terrane_error_7) => {
                                                let mut __terrane_handled_7 = false;
                                                if !__terrane_handled_7 {
                                                    return TerraneCompletion::Error(__terrane_error_7);
                                                }
                                            }
                                        }
                                        TerraneCompletion::Normal
                                    },
                                )
                                .await;
                            let __terrane_cancelled_7 = __terrane_maybe_completion_7
                                .is_none();
                            let mut __terrane_completion_7 = __terrane_maybe_completion_7
                                .unwrap_or(TerraneCompletion::Normal);
                            let __terrane_finally_7: TerraneCompletion<()> = async {
                                __terrane_await(cleanup()).await;
                                TerraneCompletion::Normal
                            }
                                .await;
                            match __terrane_finally_7 {
                                TerraneCompletion::Normal => {}
                                replacement => __terrane_completion_7 = replacement,
                            }
                            if __terrane_cancelled_7
                                && matches!(
                                    &__terrane_completion_7, TerraneCompletion::Normal
                                )
                            {
                                __terrane_finish_cancelled_finally(
                                        __terrane_finally_guard_7,
                                    )
                                    .await;
                            }
                            __terrane_finally_guard_7.finish();
                            match __terrane_completion_7 {
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
                                TerraneCompletion::Break => break,
                                TerraneCompletion::Continue => continue,
                            }
                        }
                        TerraneCompletion::Normal
                    }
                        .await;
                    match __terrane_try_6 {
                        TerraneCompletion::Return(value) => {
                            return TerraneCompletion::Return(value);
                        }
                        TerraneCompletion::Break => return TerraneCompletion::Break,
                        TerraneCompletion::Continue => return TerraneCompletion::Continue,
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
        let __terrane_cancelled_6 = __terrane_maybe_completion_6.is_none();
        let mut __terrane_completion_6 = __terrane_maybe_completion_6
            .unwrap_or(TerraneCompletion::Normal);
        let __terrane_finally_6: TerraneCompletion<()> = async {
            __terrane_await(cleanup()).await;
            println!("{}", String::from("outer-finally"));
            TerraneCompletion::Normal
        }
            .await;
        match __terrane_finally_6 {
            TerraneCompletion::Normal => {}
            replacement => __terrane_completion_6 = replacement,
        }
        if __terrane_cancelled_6
            && matches!(&__terrane_completion_6, TerraneCompletion::Normal)
        {
            __terrane_finish_cancelled_finally(__terrane_finally_guard_6).await;
        }
        __terrane_finally_guard_6.finish();
        match __terrane_completion_6 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        index = terrane_int_support::Int::from(0_i128);
        while index < terrane_int_support::Int::from(3_i128) {
            index = index.clone() + terrane_int_support::Int::from(1_i128);
            let mut __terrane_finally_guard_8 = __terrane_finally_guard();
            let __terrane_maybe_completion_8: Option<TerraneCompletion<()>> = __terrane_cancel_operation(
                    &__terrane_finally_guard_8,
                    async {
                        let __terrane_try_8: TerraneCompletion<()> = async {
                            let mut __terrane_finally_guard_9 = __terrane_finally_guard();
                            let __terrane_maybe_completion_9: Option<
                                TerraneCompletion<()>,
                            > = __terrane_cancel_operation(
                                    &__terrane_finally_guard_9,
                                    async {
                                        let __terrane_try_9: TerraneCompletion<()> = async {
                                            loop {
                                                println!("{}", String::from("nested-loop"));
                                                break;
                                            }
                                            TerraneCompletion::Normal
                                        }
                                            .await;
                                        match __terrane_try_9 {
                                            TerraneCompletion::Return(value) => {
                                                return TerraneCompletion::Return(value);
                                            }
                                            TerraneCompletion::Break => return TerraneCompletion::Break,
                                            TerraneCompletion::Continue => {
                                                return TerraneCompletion::Continue;
                                            }
                                            TerraneCompletion::Normal => {}
                                            TerraneCompletion::Error(__terrane_error_9) => {
                                                let mut __terrane_handled_9 = false;
                                                if !__terrane_handled_9 {
                                                    return TerraneCompletion::Error(__terrane_error_9);
                                                }
                                            }
                                        }
                                        TerraneCompletion::Normal
                                    },
                                )
                                .await;
                            let __terrane_cancelled_9 = __terrane_maybe_completion_9
                                .is_none();
                            let mut __terrane_completion_9 = __terrane_maybe_completion_9
                                .unwrap_or(TerraneCompletion::Normal);
                            let __terrane_finally_9: TerraneCompletion<()> = async {
                                __terrane_await(cleanup()).await;
                                println!("{}", String::from("nested-finally"));
                                TerraneCompletion::Normal
                            }
                                .await;
                            match __terrane_finally_9 {
                                TerraneCompletion::Normal => {}
                                replacement => __terrane_completion_9 = replacement,
                            }
                            if __terrane_cancelled_9
                                && matches!(
                                    &__terrane_completion_9, TerraneCompletion::Normal
                                )
                            {
                                __terrane_finish_cancelled_finally(
                                        __terrane_finally_guard_9,
                                    )
                                    .await;
                            }
                            __terrane_finally_guard_9.finish();
                            match __terrane_completion_9 {
                                TerraneCompletion::Normal => {}
                                TerraneCompletion::Return(value) => {
                                    return TerraneCompletion::Return(value);
                                }
                                TerraneCompletion::Error(error) => {
                                    return TerraneCompletion::Error(error);
                                }
                                TerraneCompletion::Break => return TerraneCompletion::Break,
                                TerraneCompletion::Continue => {
                                    return TerraneCompletion::Continue;
                                }
                            }
                            if index == terrane_int_support::Int::from(1_i128) {
                                return TerraneCompletion::Continue;
                            }
                            return TerraneCompletion::Break;
                        }
                            .await;
                        match __terrane_try_8 {
                            TerraneCompletion::Return(value) => {
                                return TerraneCompletion::Return(value);
                            }
                            TerraneCompletion::Break => return TerraneCompletion::Break,
                            TerraneCompletion::Continue => {
                                return TerraneCompletion::Continue;
                            }
                            TerraneCompletion::Normal => {}
                            TerraneCompletion::Error(__terrane_error_8) => {
                                let mut __terrane_handled_8 = false;
                                if !__terrane_handled_8 {
                                    return TerraneCompletion::Error(__terrane_error_8);
                                }
                            }
                        }
                        TerraneCompletion::Normal
                    },
                )
                .await;
            let __terrane_cancelled_8 = __terrane_maybe_completion_8.is_none();
            let mut __terrane_completion_8 = __terrane_maybe_completion_8
                .unwrap_or(TerraneCompletion::Normal);
            let __terrane_finally_8: TerraneCompletion<()> = async {
                __terrane_await(cleanup()).await;
                println!("{}", String::from("nested-outer-finally"));
                TerraneCompletion::Normal
            }
                .await;
            match __terrane_finally_8 {
                TerraneCompletion::Normal => {}
                replacement => __terrane_completion_8 = replacement,
            }
            if __terrane_cancelled_8
                && matches!(&__terrane_completion_8, TerraneCompletion::Normal)
            {
                __terrane_finish_cancelled_finally(__terrane_finally_guard_8).await;
            }
            __terrane_finally_guard_8.finish();
            match __terrane_completion_8 {
                TerraneCompletion::Normal => {
                    __terrane_generated_defect("non-fallthrough try completed normally")
                }
                TerraneCompletion::Return(value) => return value,
                TerraneCompletion::Error(error) => __terrane_uncaught(error),
                TerraneCompletion::Break => break,
                TerraneCompletion::Continue => continue,
            }
        }
    }
    pub async fn caught(&self) -> terrane_int_support::Int {
        let mut index: terrane_int_support::Int;
        index = terrane_int_support::Int::from(0_i128);
        while index < terrane_int_support::Int::from(3_i128) {
            index = index.clone() + terrane_int_support::Int::from(1_i128);
            let mut __terrane_finally_guard_10 = __terrane_finally_guard();
            let __terrane_maybe_completion_10: Option<
                TerraneCompletion<terrane_int_support::Int>,
            > = __terrane_cancel_operation(
                    &__terrane_finally_guard_10,
                    async {
                        let __terrane_try_10: TerraneCompletion<
                            terrane_int_support::Int,
                        > = async {
                            return TerraneCompletion::Error(
                                TerraneError::raised(
                                    TerraneErrorKind::CoercionError,
                                    0 /* terrane-site: case.trn:63:17-63:37 */,
                                ),
                            );
                        }
                            .await;
                        match __terrane_try_10 {
                            TerraneCompletion::Return(value) => {
                                return TerraneCompletion::Return(value);
                            }
                            TerraneCompletion::Break => return TerraneCompletion::Break,
                            TerraneCompletion::Continue => {
                                return TerraneCompletion::Continue;
                            }
                            TerraneCompletion::Normal => {}
                            TerraneCompletion::Error(__terrane_error_10) => {
                                let mut __terrane_handled_10 = false;
                                if !__terrane_handled_10
                                    && __terrane_error_10.kind
                                        == TerraneErrorKind::CoercionError
                                {
                                    __terrane_handled_10 = true;
                                    if index == terrane_int_support::Int::from(1_i128) {
                                        return TerraneCompletion::Continue;
                                    }
                                    return TerraneCompletion::Break;
                                }
                                if !__terrane_handled_10 {
                                    return TerraneCompletion::Error(__terrane_error_10);
                                }
                            }
                        }
                        TerraneCompletion::Normal
                    },
                )
                .await;
            let __terrane_cancelled_10 = __terrane_maybe_completion_10.is_none();
            let mut __terrane_completion_10 = __terrane_maybe_completion_10
                .unwrap_or(TerraneCompletion::Normal);
            let __terrane_finally_10: TerraneCompletion<terrane_int_support::Int> = async {
                __terrane_await(cleanup()).await;
                println!("{}", String::from("catch-finally"));
                TerraneCompletion::Normal
            }
                .await;
            match __terrane_finally_10 {
                TerraneCompletion::Normal => {}
                replacement => __terrane_completion_10 = replacement,
            }
            if __terrane_cancelled_10
                && matches!(&__terrane_completion_10, TerraneCompletion::Normal)
            {
                __terrane_finish_cancelled_finally(__terrane_finally_guard_10).await;
            }
            __terrane_finally_guard_10.finish();
            match __terrane_completion_10 {
                TerraneCompletion::Normal => {
                    __terrane_generated_defect("non-fallthrough try completed normally")
                }
                TerraneCompletion::Return(value) => return value,
                TerraneCompletion::Error(error) => __terrane_uncaught(error),
                TerraneCompletion::Break => break,
                TerraneCompletion::Continue => continue,
            }
        }
        return terrane_int_support::Int::from(42_i128);
    }
    pub async fn __terrane_shared_async_caught(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
    ) -> terrane_int_support::Int {
        let mut index: terrane_int_support::Int;
        index = terrane_int_support::Int::from(0_i128);
        while index < terrane_int_support::Int::from(3_i128) {
            index = index.clone() + terrane_int_support::Int::from(1_i128);
            let mut __terrane_finally_guard_11 = __terrane_finally_guard();
            let __terrane_maybe_completion_11: Option<
                TerraneCompletion<terrane_int_support::Int>,
            > = __terrane_cancel_operation(
                    &__terrane_finally_guard_11,
                    async {
                        let __terrane_try_11: TerraneCompletion<
                            terrane_int_support::Int,
                        > = async {
                            return TerraneCompletion::Error(
                                TerraneError::raised(
                                    TerraneErrorKind::CoercionError,
                                    0 /* terrane-site: case.trn:63:17-63:37 */,
                                ),
                            );
                        }
                            .await;
                        match __terrane_try_11 {
                            TerraneCompletion::Return(value) => {
                                return TerraneCompletion::Return(value);
                            }
                            TerraneCompletion::Break => return TerraneCompletion::Break,
                            TerraneCompletion::Continue => {
                                return TerraneCompletion::Continue;
                            }
                            TerraneCompletion::Normal => {}
                            TerraneCompletion::Error(__terrane_error_11) => {
                                let mut __terrane_handled_11 = false;
                                if !__terrane_handled_11
                                    && __terrane_error_11.kind
                                        == TerraneErrorKind::CoercionError
                                {
                                    __terrane_handled_11 = true;
                                    if index == terrane_int_support::Int::from(1_i128) {
                                        return TerraneCompletion::Continue;
                                    }
                                    return TerraneCompletion::Break;
                                }
                                if !__terrane_handled_11 {
                                    return TerraneCompletion::Error(__terrane_error_11);
                                }
                            }
                        }
                        TerraneCompletion::Normal
                    },
                )
                .await;
            let __terrane_cancelled_11 = __terrane_maybe_completion_11.is_none();
            let mut __terrane_completion_11 = __terrane_maybe_completion_11
                .unwrap_or(TerraneCompletion::Normal);
            let __terrane_finally_11: TerraneCompletion<terrane_int_support::Int> = async {
                __terrane_await(cleanup()).await;
                println!("{}", String::from("catch-finally"));
                TerraneCompletion::Normal
            }
                .await;
            match __terrane_finally_11 {
                TerraneCompletion::Normal => {}
                replacement => __terrane_completion_11 = replacement,
            }
            if __terrane_cancelled_11
                && matches!(&__terrane_completion_11, TerraneCompletion::Normal)
            {
                __terrane_finish_cancelled_finally(__terrane_finally_guard_11).await;
            }
            __terrane_finally_guard_11.finish();
            match __terrane_completion_11 {
                TerraneCompletion::Normal => {
                    __terrane_generated_defect("non-fallthrough try completed normally")
                }
                TerraneCompletion::Return(value) => return value,
                TerraneCompletion::Error(error) => __terrane_uncaught(error),
                TerraneCompletion::Break => break,
                TerraneCompletion::Continue => continue,
            }
        }
        return terrane_int_support::Int::from(42_i128);
    }
    pub async fn returning(&self) -> terrane_int_support::Int {
        let mut __terrane_finally_guard_12 = __terrane_finally_guard();
        let __terrane_maybe_completion_12: Option<
            TerraneCompletion<terrane_int_support::Int>,
        > = __terrane_cancel_operation(
                &__terrane_finally_guard_12,
                async {
                    let __terrane_try_12: TerraneCompletion<terrane_int_support::Int> = async {
                        loop {
                            let mut __terrane_finally_guard_13 = __terrane_finally_guard();
                            let __terrane_maybe_completion_13: Option<
                                TerraneCompletion<terrane_int_support::Int>,
                            > = __terrane_cancel_operation(
                                    &__terrane_finally_guard_13,
                                    async {
                                        let __terrane_try_13: TerraneCompletion<
                                            terrane_int_support::Int,
                                        > = async {
                                            return TerraneCompletion::Return(
                                                terrane_int_support::Int::from(7_i128),
                                            );
                                        }
                                            .await;
                                        match __terrane_try_13 {
                                            TerraneCompletion::Return(value) => {
                                                return TerraneCompletion::Return(value);
                                            }
                                            TerraneCompletion::Break => return TerraneCompletion::Break,
                                            TerraneCompletion::Continue => {
                                                return TerraneCompletion::Continue;
                                            }
                                            TerraneCompletion::Normal => {}
                                            TerraneCompletion::Error(__terrane_error_13) => {
                                                let mut __terrane_handled_13 = false;
                                                if !__terrane_handled_13 {
                                                    return TerraneCompletion::Error(__terrane_error_13);
                                                }
                                            }
                                        }
                                        TerraneCompletion::Normal
                                    },
                                )
                                .await;
                            let __terrane_cancelled_13 = __terrane_maybe_completion_13
                                .is_none();
                            let mut __terrane_completion_13 = __terrane_maybe_completion_13
                                .unwrap_or(TerraneCompletion::Normal);
                            let __terrane_finally_13: TerraneCompletion<
                                terrane_int_support::Int,
                            > = async {
                                __terrane_await(cleanup()).await;
                                println!("{}", String::from("return-inner"));
                                TerraneCompletion::Normal
                            }
                                .await;
                            match __terrane_finally_13 {
                                TerraneCompletion::Normal => {}
                                replacement => __terrane_completion_13 = replacement,
                            }
                            if __terrane_cancelled_13
                                && matches!(
                                    &__terrane_completion_13, TerraneCompletion::Normal
                                )
                            {
                                __terrane_finish_cancelled_finally(
                                        __terrane_finally_guard_13,
                                    )
                                    .await;
                            }
                            __terrane_finally_guard_13.finish();
                            match __terrane_completion_13 {
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
                                TerraneCompletion::Break => break,
                                TerraneCompletion::Continue => continue,
                            }
                        }
                        TerraneCompletion::Normal
                    }
                        .await;
                    match __terrane_try_12 {
                        TerraneCompletion::Return(value) => {
                            return TerraneCompletion::Return(value);
                        }
                        TerraneCompletion::Break => return TerraneCompletion::Break,
                        TerraneCompletion::Continue => return TerraneCompletion::Continue,
                        TerraneCompletion::Normal => {}
                        TerraneCompletion::Error(__terrane_error_12) => {
                            let mut __terrane_handled_12 = false;
                            if !__terrane_handled_12 {
                                return TerraneCompletion::Error(__terrane_error_12);
                            }
                        }
                    }
                    TerraneCompletion::Normal
                },
            )
            .await;
        let __terrane_cancelled_12 = __terrane_maybe_completion_12.is_none();
        let mut __terrane_completion_12 = __terrane_maybe_completion_12
            .unwrap_or(TerraneCompletion::Normal);
        let __terrane_finally_12: TerraneCompletion<terrane_int_support::Int> = async {
            __terrane_await(cleanup()).await;
            println!("{}", String::from("return-outer"));
            TerraneCompletion::Normal
        }
            .await;
        match __terrane_finally_12 {
            TerraneCompletion::Normal => {}
            replacement => __terrane_completion_12 = replacement,
        }
        if __terrane_cancelled_12
            && matches!(&__terrane_completion_12, TerraneCompletion::Normal)
        {
            __terrane_finish_cancelled_finally(__terrane_finally_guard_12).await;
        }
        __terrane_finally_guard_12.finish();
        match __terrane_completion_12 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        return terrane_int_support::Int::from(0_i128);
    }
    pub async fn __terrane_shared_async_returning(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
    ) -> terrane_int_support::Int {
        let mut __terrane_finally_guard_14 = __terrane_finally_guard();
        let __terrane_maybe_completion_14: Option<
            TerraneCompletion<terrane_int_support::Int>,
        > = __terrane_cancel_operation(
                &__terrane_finally_guard_14,
                async {
                    let __terrane_try_14: TerraneCompletion<terrane_int_support::Int> = async {
                        loop {
                            let mut __terrane_finally_guard_15 = __terrane_finally_guard();
                            let __terrane_maybe_completion_15: Option<
                                TerraneCompletion<terrane_int_support::Int>,
                            > = __terrane_cancel_operation(
                                    &__terrane_finally_guard_15,
                                    async {
                                        let __terrane_try_15: TerraneCompletion<
                                            terrane_int_support::Int,
                                        > = async {
                                            return TerraneCompletion::Return(
                                                terrane_int_support::Int::from(7_i128),
                                            );
                                        }
                                            .await;
                                        match __terrane_try_15 {
                                            TerraneCompletion::Return(value) => {
                                                return TerraneCompletion::Return(value);
                                            }
                                            TerraneCompletion::Break => return TerraneCompletion::Break,
                                            TerraneCompletion::Continue => {
                                                return TerraneCompletion::Continue;
                                            }
                                            TerraneCompletion::Normal => {}
                                            TerraneCompletion::Error(__terrane_error_15) => {
                                                let mut __terrane_handled_15 = false;
                                                if !__terrane_handled_15 {
                                                    return TerraneCompletion::Error(__terrane_error_15);
                                                }
                                            }
                                        }
                                        TerraneCompletion::Normal
                                    },
                                )
                                .await;
                            let __terrane_cancelled_15 = __terrane_maybe_completion_15
                                .is_none();
                            let mut __terrane_completion_15 = __terrane_maybe_completion_15
                                .unwrap_or(TerraneCompletion::Normal);
                            let __terrane_finally_15: TerraneCompletion<
                                terrane_int_support::Int,
                            > = async {
                                __terrane_await(cleanup()).await;
                                println!("{}", String::from("return-inner"));
                                TerraneCompletion::Normal
                            }
                                .await;
                            match __terrane_finally_15 {
                                TerraneCompletion::Normal => {}
                                replacement => __terrane_completion_15 = replacement,
                            }
                            if __terrane_cancelled_15
                                && matches!(
                                    &__terrane_completion_15, TerraneCompletion::Normal
                                )
                            {
                                __terrane_finish_cancelled_finally(
                                        __terrane_finally_guard_15,
                                    )
                                    .await;
                            }
                            __terrane_finally_guard_15.finish();
                            match __terrane_completion_15 {
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
                                TerraneCompletion::Break => break,
                                TerraneCompletion::Continue => continue,
                            }
                        }
                        TerraneCompletion::Normal
                    }
                        .await;
                    match __terrane_try_14 {
                        TerraneCompletion::Return(value) => {
                            return TerraneCompletion::Return(value);
                        }
                        TerraneCompletion::Break => return TerraneCompletion::Break,
                        TerraneCompletion::Continue => return TerraneCompletion::Continue,
                        TerraneCompletion::Normal => {}
                        TerraneCompletion::Error(__terrane_error_14) => {
                            let mut __terrane_handled_14 = false;
                            if !__terrane_handled_14 {
                                return TerraneCompletion::Error(__terrane_error_14);
                            }
                        }
                    }
                    TerraneCompletion::Normal
                },
            )
            .await;
        let __terrane_cancelled_14 = __terrane_maybe_completion_14.is_none();
        let mut __terrane_completion_14 = __terrane_maybe_completion_14
            .unwrap_or(TerraneCompletion::Normal);
        let __terrane_finally_14: TerraneCompletion<terrane_int_support::Int> = async {
            __terrane_await(cleanup()).await;
            println!("{}", String::from("return-outer"));
            TerraneCompletion::Normal
        }
            .await;
        match __terrane_finally_14 {
            TerraneCompletion::Normal => {}
            replacement => __terrane_completion_14 = replacement,
        }
        if __terrane_cancelled_14
            && matches!(&__terrane_completion_14, TerraneCompletion::Normal)
        {
            __terrane_finish_cancelled_finally(__terrane_finally_guard_14).await;
        }
        __terrane_finally_guard_14.finish();
        match __terrane_completion_14 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        return terrane_int_support::Int::from(0_i128);
    }
}
fn main() {
    __terrane_run(async move {
        let item: Worker;
        item = Worker::terrane_construct();
        __terrane_await((&item).run()).await;
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await((&item).caught()).
            await)
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await((&item)
            .returning()). await)
        );
    });
}
