// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: conformance/child-process-facilities
async fn child_mode(mode: String) {
    let output: ByteWriter;
    let errors: ByteWriter;
    let input: ByteReader;
    let received: ReadResult;
    let echoed: WriteResult;
    let mut warning: WriteResult;
    let argv: terrane_collection_support::List<NativeString>;
    let inherited: terrane_collection_support::List<EnvironmentEntry>;
    let mut injected: bool;
    let mut entry: EnvironmentEntry;
    let mut chunk: terrane_int_support::Int;
    if mode.as_str() == "stall" {
        __terrane_await(
                terrane_time_sleep_after(
                    __terrane_traced(
                            Duration::terrane_static_seconds(
                                terrane_int_support::Int::from(2_i128),
                            ),
                            0 /* terrane-site: case.trn:13:30-13:50 */,
                        )
                        .total_nanoseconds
                        .clone(),
                ),
            )
            .await;
        return ();
    }
    if mode.as_str() == "ignore-input" {
        println!("{}", String::from("done"));
        exit(make_exit_status(terrane_int_support::Int::from(7_i128)));
    }
    output = stdout();
    errors = stderr();
    if mode.as_str() == "stream" {
        input = stdin();
        received = input.read_all(terrane_int_support::Int::from(100_i128));
        echoed = output.write_all(received.data.clone());
        warning = errors.write_all(Vec::<u8>::from([119, 97, 114, 110, 105, 110, 103]));
        if received.failed || echoed.failed || warning.failed {
            exit(make_exit_status(terrane_int_support::Int::from(99_i128)));
        }
        exit(make_exit_status(terrane_int_support::Int::from(7_i128)));
    } else {
        argv = arguments();
        inherited = environment();
        injected = false;
        let __terrane_iterable_0 = inherited;
        let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(
            __terrane_iterable_0.as_slice(),
        );
        loop {
            entry = match __terrane_iterator_0.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            if entry.name.text.as_str() == "TERRANE_CHILD_TEST_VALUE"
                && entry.value.text.as_str() == "a b"
            {
                injected = true;
            }
        }
        let __terrane_iterable_1 = __terrane_raised(
            terrane_collection_support::Range::new(
                terrane_int_support::Int::from(0_i128),
                terrane_int_support::Int::from(2048_i128),
                terrane_int_support::Int::from(1_i64),
            ),
            1 /* terrane-site: case.trn:35:23-35:37 */,
        );
        let mut __terrane_iterator_1 = terrane_collection_support::Iterable::terrane_iterator(
            &__terrane_iterable_1,
        );
        loop {
            chunk = match __terrane_iterator_1.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            let _ = &chunk;
            warning = errors
                .write_all(
                    Vec::<
                        u8,
                    >::from([
                        97, 98, 99, 100, 101, 102, 103, 104, 97, 98, 99, 100, 101, 102,
                        103, 104, 97, 98, 99, 100, 101, 102, 103, 104, 97, 98, 99, 100,
                        101, 102, 103, 104, 97, 98, 99, 100, 101, 102, 103, 104, 97, 98,
                        99, 100, 101, 102, 103, 104, 97, 98, 99, 100, 101, 102, 103, 104,
                        97, 98, 99, 100, 101, 102, 103, 104,
                    ]),
                );
            if warning.failed {
                exit(make_exit_status(terrane_int_support::Int::from(99_i128)));
            }
        }
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(argv
            .length()) == terrane_int_support::Int::from(3_i128) &&__terrane_raised(argv
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            2 /* terrane-site: case.trn:39:38-39:45 */)), 2 /* terrane-site: case.trn:39:38-39:45 */).text.as_str() == "space ; $HOME *"
            &&__terrane_raised(argv
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
            3 /* terrane-site: case.trn:39:76-39:83 */)), 3 /* terrane-site: case.trn:39:76-39:83 */).text.as_str() == "" &&injected))
        );
    }
}
fn exit_is(
    code: Option<terrane_int_support::Int>,
    expected: terrane_int_support::Int,
) -> bool {
    if (&code).is_some() {
        return *match &code {
            Some(value) => value,
            _ => unreachable!("flow-proven storage refinement"),
        } == expected;
    }
    return false;
}
async fn exercise_capture(executable: String) -> Result<(), TerraneError> {
    let mut options: ProcessOptions;
    let args: terrane_collection_support::List<String>;
    let captured: ProcessCapture;
    let mut missing_failed: bool;
    let copied: ProcessCapture;
    let expired: ProcessCapture;
    let payload: Vec<u8>;
    let ignored: ProcessCapture;
    options = ProcessOptions::terrane_construct();
    {
        let __terrane_value: String = String::from(".");
        options.directory = __terrane_value;
    }
    {
        let __terrane_collection_argument_0 = environment_pair(
            native_text(String::from("TOKIO_WORKER_THREADS")),
            native_text(String::from("2")),
        );
        options.environment.append(__terrane_collection_argument_0);
    };
    {
        let __terrane_value: bool = false;
        options.inherit_environment = __terrane_value;
    }
    {
        let __terrane_collection_argument_0 = environment_pair(
            native_text(String::from("TERRANE_CHILD_TEST_VALUE")),
            native_text(String::from("a b")),
        );
        options.environment.append(__terrane_collection_argument_0);
    };
    args = terrane_collection_support::List::<
        String,
    >::new(
        vec![String::from("capture"), String::from("space ; $HOME *"), String::from("")],
    );
    captured = __terrane_traced_err(
        __terrane_await(
                run_process_async(
                    executable.clone(),
                    Some(args),
                    Some(options),
                    Vec::<u8>::from([]),
                    terrane_int_support::Int::from(30000_i128),
                    terrane_int_support::Int::from(1048576_i128),
                    terrane_int_support::Int::from(3_i128),
                ),
            )
            .await,
        4 /* terrane-site: case.trn:53:22-53:84 */,
    )?;
    println!(
        "{}", terrane_scalar_support::scalar_text(&(exit_is(captured.status.as_ref()
        .expect("required field initialized").exit_code.clone(),
        terrane_int_support::Int::from(0_i128)) &&captured.stdout == Vec:: < u8 >
        ::from([116, 114, 117, 101, 10]) &&! captured.stdout_truncated))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(captured.stderr == Vec:: < u8 >
        ::from([97, 98, 99]) &&captured.stderr_truncated &&! captured.status.as_ref()
        .expect("required field initialized").timed_out))
    );
    missing_failed = false;
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            __terrane_traced_completion!(
                run_process(String::from("./no-such-terrane-child-binary"), None, None,
                Vec::< u8 >::from([]), terrane_int_support::Int::from(30000_i128),
                terrane_int_support::Int::from(1048576_i128),
                terrane_int_support::Int::from(1048576_i128)), 5 /* terrane-site: case.trn:58:9-58:54 */
            );
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
                    && __terrane_error_0.kind
                        == TerraneErrorKind::Custom(DescriptorId(0))
                {
                    __terrane_handled_0 = true;
                    missing_failed = true;
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
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    println!("{}", terrane_scalar_support::scalar_text(&missing_failed));
    copied = __terrane_traced_err(
        run_process(
            executable.clone(),
            Some(
                terrane_collection_support::List::<
                    String,
                >::new(vec![String::from("stream")]),
            ),
            None,
            Vec::<u8>::from([105, 110, 112, 117, 116]),
            terrane_int_support::Int::from(30000_i128),
            terrane_int_support::Int::from(1048576_i128),
            terrane_int_support::Int::from(1048576_i128),
        ),
        6 /* terrane-site: case.trn:62:14-62:73 */,
    )?;
    println!(
        "{}", terrane_scalar_support::scalar_text(&(exit_is(copied.status.as_ref()
        .expect("required field initialized").exit_code.clone(),
        terrane_int_support::Int::from(7_i128)) &&copied.stdout == Vec:: < u8 >
        ::from([105, 110, 112, 117, 116]) &&copied.stderr == Vec:: < u8 > ::from([119,
        97, 114, 110, 105, 110, 103])))
    );
    expired = __terrane_traced_err(
        __terrane_await(
                run_process_async(
                    executable.clone(),
                    Some(
                        terrane_collection_support::List::<
                            String,
                        >::new(vec![String::from("stall")]),
                    ),
                    None,
                    Vec::<u8>::from([]),
                    terrane_int_support::Int::from(5_i128),
                    terrane_int_support::Int::from(1048576_i128),
                    terrane_int_support::Int::from(1048576_i128),
                ),
            )
            .await,
        7 /* terrane-site: case.trn:64:21-64:83 */,
    )?;
    println!(
        "{}", terrane_scalar_support::scalar_text(&(expired.status.as_ref()
        .expect("required field initialized").timed_out &&expired.status.as_ref()
        .expect("required field initialized").exited))
    );
    payload = vec![b'x'; 1024 * 1024];
    ignored = __terrane_traced_err(
        __terrane_await(
                run_process_async(
                    executable,
                    Some(
                        terrane_collection_support::List::<
                            String,
                        >::new(vec![String::from("ignore-input")]),
                    ),
                    None,
                    payload,
                    terrane_int_support::Int::from(30000_i128),
                    terrane_int_support::Int::from(1048576_i128),
                    terrane_int_support::Int::from(1048576_i128),
                ),
            )
            .await,
        8 /* terrane-site: case.trn:68:21-68:91 */,
    )?;
    println!(
        "{}", terrane_scalar_support::scalar_text(&(exit_is(ignored.status.as_ref()
        .expect("required field initialized").exit_code.clone(),
        terrane_int_support::Int::from(7_i128)) &&ignored.stdout == Vec:: < u8 >
        ::from([100, 111, 110, 101, 10]) &&! ignored.status.as_ref()
        .expect("required field initialized").timed_out))
    );
    Ok(())
}
async fn received_expected(
    reader: ChildOutputStream,
    expected: Vec<u8>,
) -> Result<bool, TerraneError> {
    let first: ProcessOutput;
    first = __terrane_traced_err(
        __terrane_await((&reader).read(terrane_int_support::Int::from(100_i128))).await,
        9 /* terrane-site: case.trn:72:19-72:35 */,
    )?;
    return Ok(first.data == expected);
}
async fn exercise_streaming(executable: String) -> Result<(), TerraneError> {
    let child: ChildProcess;
    let initial: ProcessObservation;
    let elapsed: ProcessObservation;
    let input_view: ChildInputStream;
    let output_view: ChildOutputStream;
    let error_view: ChildOutputStream;
    let written: terrane_int_support::Int;
    let scope: TerraneTaskScope;
    let stdout_task: TerraneScopedTask<bool>;
    let stderr_task: TerraneScopedTask<bool>;
    let stdout_result: TerraneTaskOutcome<bool>;
    let stderr_result: TerraneTaskOutcome<bool>;
    let done: ProcessObservation;
    let stdout_matched: Option<bool>;
    let stderr_matched: Option<bool>;
    let mut matched: bool;
    let stdout_end: ProcessOutput;
    let stderr_end: ProcessOutput;
    let another: ChildProcess;
    let terminated: ProcessObservation;
    child = __terrane_traced_err(
        __terrane_await(
                spawn_process(
                    executable.clone(),
                    Some(
                        terrane_collection_support::List::<
                            String,
                        >::new(vec![String::from("stream")]),
                    ),
                    None,
                ),
            )
            .await,
        10 /* terrane-site: case.trn:76:19-76:62 */,
    )?;
    initial = __terrane_traced_err(
        child.poll(),
        11 /* terrane-site: case.trn:77:15-77:26 */,
    )?;
    elapsed = __terrane_traced_err(
        __terrane_await((&child).wait(terrane_int_support::Int::from(0_i128))).await,
        12 /* terrane-site: case.trn:78:21-78:34 */,
    )?;
    println!(
        "{}", terrane_scalar_support::scalar_text(&(! initial.exited &&elapsed.timed_out
        &&! elapsed.exited))
    );
    input_view = child.input();
    output_view = child.output();
    error_view = child.errors();
    written = __terrane_traced_err(
        __terrane_await((&input_view).write(Vec::<u8>::from([97, 98, 99]))).await,
        13 /* terrane-site: case.trn:83:21-83:45 */,
    )?;
    __terrane_traced_err(
        __terrane_await((&input_view).close()).await,
        14 /* terrane-site: case.trn:84:11-84:28 */,
    )?;
    scope = TerraneTaskScope::new(None);
    stdout_task = {
        let __terrane_scope = scope.clone();
        let __terrane_cancel = __terrane_scope.cancellation();
        let __terrane_deadline = __terrane_scope.deadline;
        let __terrane_spawned_task = received_expected(
            output_view,
            Vec::<u8>::from([97, 98, 99]),
        );
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
    stderr_task = {
        let __terrane_scope = scope.clone();
        let __terrane_cancel = __terrane_scope.cancellation();
        let __terrane_deadline = __terrane_scope.deadline;
        let __terrane_spawned_task = received_expected(
            error_view,
            Vec::<u8>::from([119, 97, 114, 110, 105, 110, 103]),
        );
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
    stdout_result = __terrane_await(scope.join(stdout_task)).await;
    stderr_result = __terrane_await(scope.join(stderr_task)).await;
    done = __terrane_traced_err(
        __terrane_await((&child).wait(terrane_int_support::Int::from(30000_i128))).await,
        17 /* terrane-site: case.trn:90:18-90:29 */,
    )?;
    stdout_matched = stdout_result.value.clone();
    stderr_matched = stderr_result.value.clone();
    matched = false;
    if (&stdout_matched).is_some() {
        if (&stderr_matched).is_some() {
            matched = *match &stdout_matched {
                Some(value) => value,
                _ => unreachable!("flow-proven storage refinement"),
            }
                && *match &stderr_matched {
                    Some(value) => value,
                    _ => unreachable!("flow-proven storage refinement"),
                };
        }
    }
    stdout_end = __terrane_traced_err(
        __terrane_await((&child).read_stdout(terrane_int_support::Int::from(100_i128)))
            .await,
        18 /* terrane-site: case.trn:97:24-97:46 */,
    )?;
    stderr_end = __terrane_traced_err(
        __terrane_await((&child).read_stderr(terrane_int_support::Int::from(100_i128)))
            .await,
        19 /* terrane-site: case.trn:98:24-98:46 */,
    )?;
    println!(
        "{}", terrane_scalar_support::scalar_text(&(written ==
        terrane_int_support::Int::from(3_i128) &&matched &&stdout_end.end &&stderr_end
        .end))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(done.exited &&! done.timed_out
        &&exit_is(done.exit_code.clone(), terrane_int_support::Int::from(7_i128))))
    );
    another = __terrane_traced_err(
        __terrane_await(
                spawn_process(
                    executable,
                    Some(
                        terrane_collection_support::List::<
                            String,
                        >::new(vec![String::from("stream")]),
                    ),
                    None,
                ),
            )
            .await,
        20 /* terrane-site: case.trn:101:21-101:64 */,
    )?;
    __terrane_traced_err(
        another.terminate(),
        21 /* terrane-site: case.trn:102:5-102:23 */,
    )?;
    terminated = __terrane_traced_err(
        __terrane_await((&another).wait(terrane_int_support::Int::from(30000_i128)))
            .await,
        22 /* terrane-site: case.trn:103:24-103:37 */,
    )?;
    println!(
        "{}", terrane_scalar_support::scalar_text(&(terminated.exited &&! terminated
        .timed_out &&! exit_is(terminated.exit_code.clone(),
        terrane_int_support::Int::from(7_i128))))
    );
    Ok(())
}
fn main() {
    __terrane_run(async move {
        let argv: terrane_collection_support::List<NativeString>;
        let executable: Option<String>;
        argv = arguments();
        if terrane_int_support::Int::from(argv.length())
            > terrane_int_support::Int::from(0_i128)
        {
            __terrane_await(
                    child_mode(
                        __terrane_raised(
                                argv
                                    .get_or_error(
                                        __terrane_raised(
                                            terrane_collection_support::index_from_int(
                                                &terrane_int_support::Int::from(0_i128),
                                            ),
                                            23 /* terrane-site: case.trn:109:27-109:34 */,
                                        ),
                                    ),
                                23 /* terrane-site: case.trn:109:27-109:34 */,
                            )
                            .text
                            .clone(),
                    ),
                )
                .await;
        } else {
            executable = native_text_value(
                __terrane_traced(
                    current_executable(),
                    24 /* terrane-site: case.trn:111:42-111:61 */,
                ),
            );
            if (&executable).is_some() {
                __terrane_traced(
                    __terrane_await(
                            exercise_capture(
                                match &executable {
                                    Some(value) => value,
                                    _ => unreachable!("flow-proven storage refinement"),
                                }
                                    .clone(),
                            ),
                        )
                        .await,
                    25 /* terrane-site: case.trn:113:19-113:47 */,
                );
                __terrane_traced(
                    __terrane_await(
                            exercise_streaming(
                                match executable {
                                    Some(value) => value,
                                    _ => unreachable!("flow-proven storage refinement"),
                                },
                            ),
                        )
                        .await,
                    26 /* terrane-site: case.trn:114:19-114:49 */,
                );
            } else {
                exit(make_exit_status(terrane_int_support::Int::from(98_i128)));
            }
        }
    });
}
