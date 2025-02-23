use caps::runtime;

#[test]
fn test_ambient_supported() {
    runtime::ambient_set_supported().unwrap();
}

#[test]
fn test_thread_all_supported() {
    assert!(runtime::thread_all_supported_caps::<caps::CapsHashSet>().len() > 0);
    assert!(runtime::thread_all_supported_caps::<caps::CapsHashSet>().len() <= caps::all_caps::<caps::CapsHashSet>().len());
}

#[test]
fn test_procfs_all_supported() {
    use std::path::PathBuf;

    let p1 = runtime::procfs_all_supported_caps::<caps::CapsHashSet>(None).unwrap();
    let p2 = runtime::procfs_all_supported_caps::<caps::CapsHashSet>(Some(PathBuf::from("/proc"))).unwrap();
    let thread: caps::CapsHashSet = runtime::thread_all_supported_caps();
    let all = caps::all_caps::<caps::CapsHashSet>();

    assert!(thread.len() > 0);
    assert!(thread.len() <= all.len());
    assert_eq!(
        p1,
        p2,
        "{:?}",
        p1.symmetric_difference(&p2).collect::<Vec<_>>()
    );
    assert_eq!(
        p1,
        thread,
        "{:?}",
        p1.symmetric_difference(&thread).collect::<Vec<_>>()
    );
}
