// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: conformance/child-process-policy
async fn handle_termination() -> Result<(), TerraneError> {
    let signal: ProcessSignal;
    let mut subscription: ProcessSignalSubscription;
    let output: ByteWriter;
    let ready: WriteResult;
    let ready_flush: StreamOperationResult;
    let event: ProcessSignalEvent;
    let written: WriteResult;
    let delivered: StreamOperationResult;
    signal = ProcessSignal::terrane_static_terminate();
    subscription = __terrane_traced_err(
        process_signals(
            terrane_collection_support::Set::<ProcessSignal>::new(vec![signal]),
        ),
        0 /* terrane-site: case.trn:13:20-13:50 */,
    )?;
    output = stdout();
    ready = output.write_all(Vec::<u8>::from([82]));
    ready_flush = output.flush();
    if ready.failed || ready_flush.failed {
        exit(make_exit_status(terrane_int_support::Int::from(98_i128)));
    }
    event = __terrane_traced_err(
        __terrane_await((&mut subscription).next()).await,
        1 /* terrane-site: case.trn:19:19-19:37 */,
    )?;
    if event.signal.name.as_str() == "terminate" {
        written = output.write_all(Vec::<u8>::from([84]));
        delivered = output.flush();
        if !written.failed && !delivered.failed {
            exit(make_exit_status(terrane_int_support::Int::from(23_i128)));
        }
    }
    exit(make_exit_status(terrane_int_support::Int::from(99_i128)));
    Ok(())
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
async fn exercise_signals(executable: String) -> Result<(), TerraneError> {
    let mut options: ProcessOptions;
    let child: ChildProcess;
    let ready: ProcessOutput;
    let graceful: ProcessObservation;
    let delivered: ProcessOutput;
    let mut rejected: bool;
    let mut error: Option<TerraneError> = None;
    let forced_child: ChildProcess;
    let forced_ready: ProcessOutput;
    let forced: ProcessObservation;
    let output: ProcessOutput;
    options = ProcessOptions::terrane_construct();
    {
        let __terrane_value: bool = true;
        options.new_process_group = __terrane_value;
    }
    child = __terrane_traced_err(
        __terrane_await(
                spawn_process(
                    executable.clone(),
                    Some(
                        terrane_collection_support::List::<
                            String,
                        >::new(vec![String::from("handle")]),
                    ),
                    Some(options.clone()),
                ),
            )
            .await,
        2 /* terrane-site: case.trn:35:19-35:71 */,
    )?;
    ready = __terrane_traced_err(
        __terrane_await((&child).read_stdout(terrane_int_support::Int::from(1_i128)))
            .await,
        3 /* terrane-site: case.trn:36:19-36:39 */,
    )?;
    println!(
        "{}", terrane_scalar_support::scalar_text(&(ready.data == Vec:: < u8 >
        ::from([82])))
    );
    __terrane_traced_err(
        __terrane_await((&child).send_signal(ProcessSignal::terrane_static_terminate()))
            .await,
        4 /* terrane-site: case.trn:38:11-38:58 */,
    )?;
    graceful = __terrane_traced_err(
        __terrane_await((&child).wait(terrane_int_support::Int::from(3000_i128))).await,
        5 /* terrane-site: case.trn:39:22-39:38 */,
    )?;
    println!(
        "{}", terrane_scalar_support::scalar_text(&(graceful.exited &&! graceful
        .timed_out &&exit_is(graceful.exit_code.clone(),
        terrane_int_support::Int::from(23_i128))))
    );
    if graceful.exited {
        delivered = __terrane_traced_err(
            __terrane_await((&child).read_stdout(terrane_int_support::Int::from(1_i128)))
                .await,
            6 /* terrane-site: case.trn:42:27-42:47 */,
        )?;
        println!(
            "{}", terrane_scalar_support::scalar_text(&(delivered.data == Vec:: < u8 >
            ::from([84])))
        );
        rejected = false;
        let __terrane_completion_0: TerraneCompletion<()> = async {
            let __terrane_try_0: TerraneCompletion<()> = async {
                __terrane_traced_completion!(
                    __terrane_await((&child)
                    .send_signal(ProcessSignal::terrane_static_terminate())). await,
                    7 /* terrane-site: case.trn:46:19-46:66 */
                );
                TerraneCompletion::Normal
            }
                .await;
            match __terrane_try_0 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
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
                        let _ = error.insert(__terrane_error_0.clone());
                        rejected = {
                            let __terrane_error_text_0 = error
                                .as_ref()
                                .expect("flow-proven available binding")
                                .message();
                            __terrane_error_text_0 == "child process has exited"
                        };
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
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return Ok(value),
            TerraneCompletion::Error(error) => return Err(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        println!("{}", terrane_scalar_support::scalar_text(&rejected));
    }
    forced_child = __terrane_traced_err(
        __terrane_await(
                spawn_process(
                    executable,
                    Some(
                        terrane_collection_support::List::<
                            String,
                        >::new(vec![String::from("handle")]),
                    ),
                    Some(options),
                ),
            )
            .await,
        8 /* terrane-site: case.trn:50:26-50:78 */,
    )?;
    forced_ready = __terrane_traced_err(
        __terrane_await(
                (&forced_child).read_stdout(terrane_int_support::Int::from(1_i128)),
            )
            .await,
        9 /* terrane-site: case.trn:51:26-51:53 */,
    )?;
    __terrane_traced_err(
        forced_child.terminate(),
        10 /* terrane-site: case.trn:52:5-52:28 */,
    )?;
    forced = __terrane_traced_err(
        __terrane_await((&forced_child).wait(terrane_int_support::Int::from(3000_i128)))
            .await,
        11 /* terrane-site: case.trn:53:20-53:43 */,
    )?;
    println!(
        "{}", terrane_scalar_support::scalar_text(&(forced_ready.data == Vec:: < u8 >
        ::from([82]) &&forced.exited &&! forced.timed_out &&(&forced.exit_code)
        .is_none()))
    );
    if forced.exited {
        output = __terrane_traced_err(
            __terrane_await(
                    (&forced_child).read_stdout(terrane_int_support::Int::from(1_i128)),
                )
                .await,
            12 /* terrane-site: case.trn:56:24-56:51 */,
        )?;
        println!(
            "{}", terrane_scalar_support::scalar_text(&(output.end &&output.data == Vec::
            < u8 > ::from([])))
        );
    }
    Ok(())
}
async fn exercise_groups(executable: String) -> Result<(), TerraneError> {
    let parent_group: terrane_int_support::Int;
    let group_line: String;
    let expected: Vec<u8>;
    let inherited: ProcessCapture;
    let mut options: ProcessOptions;
    let isolated: ProcessCapture;
    let spawned: ChildProcess;
    let spawned_status: ProcessObservation;
    let spawned_output: ProcessOutput;
    parent_group = current_process_group_id();
    group_line = {
        let mut __terrane_text = String::with_capacity(1);
        terrane_scalar_support::ScalarDisplay::write_scalar(
            &terrane_scalar_support::scalar_text(&parent_group),
            &mut __terrane_text,
        );
        __terrane_text.push_str("\n");
        __terrane_text
    };
    expected = {
        let __terrane_string_argument_0 = terrane_string_support::Encoding::Utf8;
        terrane_string_support::encode(&group_line, __terrane_string_argument_0)
    };
    inherited = __terrane_traced_err(
        run_process(
            executable.clone(),
            Some(
                terrane_collection_support::List::<
                    String,
                >::new(vec![String::from("group-id")]),
            ),
            None,
            Vec::<u8>::from([]),
            terrane_int_support::Int::from(30000_i128),
            terrane_int_support::Int::from(1048576_i128),
            terrane_int_support::Int::from(1048576_i128),
        ),
        13 /* terrane-site: case.trn:63:17-63:60 */,
    )?;
    println!(
        "{}", terrane_scalar_support::scalar_text(&(exit_is(inherited.status.as_ref()
        .expect("required field initialized").exit_code.clone(),
        terrane_int_support::Int::from(0_i128)) &&inherited.stdout == expected))
    );
    options = ProcessOptions::terrane_construct();
    {
        let __terrane_value: bool = true;
        options.new_process_group = __terrane_value;
    }
    isolated = __terrane_traced_err(
        __terrane_await(
                run_process_async(
                    executable.clone(),
                    Some(
                        terrane_collection_support::List::<
                            String,
                        >::new(vec![String::from("group-leader")]),
                    ),
                    Some(options.clone()),
                    Vec::<u8>::from([]),
                    terrane_int_support::Int::from(30000_i128),
                    terrane_int_support::Int::from(1048576_i128),
                    terrane_int_support::Int::from(1048576_i128),
                ),
            )
            .await,
        14 /* terrane-site: case.trn:67:22-67:84 */,
    )?;
    println!(
        "{}", terrane_scalar_support::scalar_text(&(exit_is(isolated.status.as_ref()
        .expect("required field initialized").exit_code.clone(),
        terrane_int_support::Int::from(0_i128)) &&isolated.stdout == Vec:: < u8 >
        ::from([116, 114, 117, 101, 10])))
    );
    spawned = __terrane_traced_err(
        __terrane_await(
                spawn_process(
                    executable,
                    Some(
                        terrane_collection_support::List::<
                            String,
                        >::new(vec![String::from("group-leader")]),
                    ),
                    Some(options),
                ),
            )
            .await,
        15 /* terrane-site: case.trn:69:21-69:79 */,
    )?;
    spawned_status = __terrane_traced_err(
        __terrane_await((&spawned).wait(terrane_int_support::Int::from(3000_i128)))
            .await,
        16 /* terrane-site: case.trn:70:28-70:46 */,
    )?;
    if spawned_status.exited {
        spawned_output = __terrane_traced_err(
            __terrane_await(
                    (&spawned).read_stdout(terrane_int_support::Int::from(100_i128)),
                )
                .await,
            17 /* terrane-site: case.trn:72:32-72:56 */,
        )?;
        println!(
            "{}", terrane_scalar_support::scalar_text(&(exit_is(spawned_status.exit_code
            .clone(), terrane_int_support::Int::from(0_i128)) &&spawned_output.data ==
            Vec:: < u8 > ::from([116, 114, 117, 101, 10])))
        );
    }
    Ok(())
}
fn main() {
    __terrane_run(async move {
        let argv: terrane_collection_support::List<NativeString>;
        let mode: String;
        let executable: Option<String>;
        argv = arguments();
        if terrane_int_support::Int::from(argv.length())
            > terrane_int_support::Int::from(0_i128)
        {
            mode = __terrane_raised(
                    argv
                        .get_or_error(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(
                                    &terrane_int_support::Int::from(0_i128),
                                ),
                                18 /* terrane-site: case.trn:78:16-78:23 */,
                            ),
                        ),
                    18 /* terrane-site: case.trn:78:16-78:23 */,
                )
                .text
                .clone();
            if mode.as_str() == "group-id" {
                println!(
                    "{}",
                    terrane_scalar_support::scalar_text(&current_process_group_id())
                );
            } else if mode.as_str() == "group-leader" {
                println!(
                    "{}",
                    terrane_scalar_support::scalar_text(&(current_process_group_id() ==
                    current_process_id()))
                );
            } else {
                let __terrane_completion_1: TerraneCompletion<()> = async {
                    let __terrane_try_1: TerraneCompletion<()> = async {
                        __terrane_traced_completion!(
                            __terrane_await(handle_termination()). await,
                            19 /* terrane-site: case.trn:85:23-85:42 */
                        );
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
                            if !__terrane_handled_1
                                && __terrane_error_1.kind
                                    == TerraneErrorKind::Custom(DescriptorId(1))
                            {
                                __terrane_handled_1 = true;
                                exit(
                                    make_exit_status(terrane_int_support::Int::from(97_i128)),
                                );
                            }
                            if !__terrane_handled_1 {
                                return TerraneCompletion::Error(__terrane_error_1);
                            }
                        }
                    }
                    TerraneCompletion::Normal
                }
                    .await;
                match __terrane_completion_1 {
                    TerraneCompletion::Normal => {}
                    TerraneCompletion::Return(value) => return value,
                    TerraneCompletion::Error(error) => __terrane_uncaught(error),
                    TerraneCompletion::Break | TerraneCompletion::Continue => {
                        __terrane_generated_defect("loop control escaped a non-loop try")
                    }
                }
            }
        } else {
            executable = native_text_value(
                __terrane_traced(
                    current_executable(),
                    20 /* terrane-site: case.trn:89:42-89:61 */,
                ),
            );
            if (&executable).is_some() {
                __terrane_traced(
                    __terrane_await(
                            exercise_groups(
                                match &executable {
                                    Some(value) => value,
                                    _ => unreachable!("flow-proven storage refinement"),
                                }
                                    .clone(),
                            ),
                        )
                        .await,
                    21 /* terrane-site: case.trn:91:19-91:46 */,
                );
                __terrane_traced(
                    __terrane_await(
                            exercise_signals(
                                match executable {
                                    Some(value) => value,
                                    _ => unreachable!("flow-proven storage refinement"),
                                },
                            ),
                        )
                        .await,
                    22 /* terrane-site: case.trn:92:19-92:47 */,
                );
            } else {
                exit(make_exit_status(terrane_int_support::Int::from(95_i128)));
            }
        }
    });
}
