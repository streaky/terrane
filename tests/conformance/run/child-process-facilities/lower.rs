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
                            0 /* terrane-site: case.trn:12:30-12:50 */,
                        )
                        .total_nanoseconds
                        .clone(),
                ),
            )
            .await;
        return ();
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
            1 /* terrane-site: case.trn:31:23-31:37 */,
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
            2 /* terrane-site: case.trn:35:38-35:45 */)), 2 /* terrane-site: case.trn:35:38-35:45 */).text.as_str() == "space ; $HOME *"
            &&__terrane_raised(argv
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
            3 /* terrane-site: case.trn:35:76-35:83 */)), 3 /* terrane-site: case.trn:35:76-35:83 */).text.as_str() == "" &&injected))
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
        4 /* terrane-site: case.trn:49:22-49:84 */,
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
                terrane_int_support::Int::from(1048576_i128)), 5 /* terrane-site: case.trn:54:9-54:54 */
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
        6 /* terrane-site: case.trn:58:14-58:73 */,
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
                    executable,
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
        7 /* terrane-site: case.trn:60:21-60:83 */,
    )?;
    println!(
        "{}", terrane_scalar_support::scalar_text(&(expired.status.as_ref()
        .expect("required field initialized").timed_out &&expired.status.as_ref()
        .expect("required field initialized").exited))
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
        8 /* terrane-site: case.trn:64:19-64:35 */,
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
        9 /* terrane-site: case.trn:68:19-68:62 */,
    )?;
    initial = __terrane_traced_err(
        child.poll(),
        10 /* terrane-site: case.trn:69:15-69:26 */,
    )?;
    elapsed = __terrane_traced_err(
        __terrane_await((&child).wait(terrane_int_support::Int::from(0_i128))).await,
        11 /* terrane-site: case.trn:70:21-70:34 */,
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
        12 /* terrane-site: case.trn:75:21-75:45 */,
    )?;
    __terrane_traced_err(
        __terrane_await((&input_view).close()).await,
        13 /* terrane-site: case.trn:76:11-76:28 */,
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
        16 /* terrane-site: case.trn:82:18-82:29 */,
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
        17 /* terrane-site: case.trn:89:24-89:46 */,
    )?;
    stderr_end = __terrane_traced_err(
        __terrane_await((&child).read_stderr(terrane_int_support::Int::from(100_i128)))
            .await,
        18 /* terrane-site: case.trn:90:24-90:46 */,
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
        19 /* terrane-site: case.trn:93:21-93:64 */,
    )?;
    __terrane_traced_err(
        another.terminate(),
        20 /* terrane-site: case.trn:94:5-94:23 */,
    )?;
    terminated = __terrane_traced_err(
        __terrane_await((&another).wait(terrane_int_support::Int::from(30000_i128)))
            .await,
        21 /* terrane-site: case.trn:95:24-95:37 */,
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
        let executable: String;
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
                                            22 /* terrane-site: case.trn:101:27-101:34 */,
                                        ),
                                    ),
                                22 /* terrane-site: case.trn:101:27-101:34 */,
                            )
                            .text
                            .clone(),
                    ),
                )
                .await;
        } else {
            executable = std::env::current_exe()
                .expect("fixture executable")
                .into_os_string()
                .into_string()
                .expect("Unicode fixture path");
            __terrane_traced(
                __terrane_await(exercise_capture(executable.clone())).await,
                23 /* terrane-site: case.trn:105:15-105:43 */,
            );
            __terrane_traced(
                __terrane_await(exercise_streaming(executable)).await,
                24 /* terrane-site: case.trn:106:15-106:45 */,
            );
        }
    });
}
