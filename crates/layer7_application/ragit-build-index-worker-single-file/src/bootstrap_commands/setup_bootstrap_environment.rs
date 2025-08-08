async fn setup_bootstrap_environment(
    context: &mut BootstrapContext,
) -> Result<(), anyhow::Error> {
    context.memory_monitor.lock().unwrap().verbose("Setting up environment...");
    let (actual_root_dir, temp_dir, index) = setup_environment(
        context.args.max_memory_gb,
        &mut context.memory_monitor.lock().unwrap(),
    ).await?;
    context.actual_root_dir = actual_root_dir;
    context.temp_dir = temp_dir;
    context.index = index;
    context.memory_monitor.lock().unwrap().verbose(&format!("Temporary directory: {:?}", context.temp_dir));
    context.memory_monitor.lock().unwrap().capture_and_log_snapshot(AFTER_SETUP_ENV);
    context.memory_monitor.lock().unwrap().verbose("Environment setup complete.");
    Ok(())
}
