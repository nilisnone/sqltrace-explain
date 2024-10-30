use std::io::Write;

#[test]
#[should_panic(expected = "SOI_HOME: ttt 不是存在的目录，请重新配置")]
fn test_set_home_dir_panic_1() {
    std::env::set_var("SOI_HOME", "ttt");
    let _ = soi::config::set_home_dir();
}

#[test]
#[should_panic(expected = "SOI_HOME: /etc/dir/test 不是存在的目录，请重新配置")]
fn test_set_home_dir_panic_2() {
    std::env::set_var("SOI_HOME", "/etc/dir/test");
    println!("SOI_HOME {}", std::env::var("SOI_HOME").expect("msg"));
    let _ = soi::config::set_home_dir();
}

#[test]
#[cfg(target_os = "macos")]
fn test_set_home_dir() {
    std::env::set_var("SOI_HOME", "/var/log");
    let _ = soi::config::set_home_dir();
    assert_eq!("/private/var/log", std::env::current_dir().unwrap().as_os_str());
}

#[test]
#[should_panic]
fn test_load_config_err() {
    // 重置当前目录
    std::env::set_var("SOI_HOME", ".");
    // 设置env=error
    std::env::set_var("SOI_ENV", "error");
    let _ = soi::config::load_config();
}

#[test]
fn test_load_config() {
    // 重置当前目录
    std::env::set_var("SOI_HOME", ".");
    // 设置env=local
    std::env::set_var("SOI_ENV", "local");
    let _ = soi::config::set_home_dir();
    println!("cur dir {:?}", std::env::current_dir());

    let content = "
env: prod
scan_dir: \"/tmp/scan_dir\"
position_file: \"/tmp/test.ini\"
explain_interval: 33
database:
  default:
    driver: mysql
    dsn: test1
";
    std::fs::create_dir_all("./tests/output/config").expect("无法创建文件夹./tests/output/config");
    let mut file = std::fs::File::create("./tests/output/config/app.yaml").expect("无法创建文件./tests/output/config/app.yaml");
    file.write_all(content.as_bytes()).expect("无法写入./tests/output/config/app.yaml");

    let mut file2 = std::fs::File::create("./tests/output/config/local.yaml").expect("无法创建./tests/output/config/local.yaml");
    let content_2 = "
env: local
";
    file2.write_all(content_2.as_bytes()).expect("无法写入./output/config/local.yaml");

    std::env::set_var("SOI_HOME", "./tests/output");
    let app_config = soi::config::load_config().expect("加载config文件失败");

    // local|production.yaml 会覆盖 app.yaml 相同的值
    assert_eq!("local", app_config.env);
    assert_eq!("/tmp/scan_dir", app_config.scan_dir);
    assert_eq!("/tmp/test.ini", app_config.position_file);
    assert_eq!(33, app_config.explain_interval);
    let database: &str = "default";
    assert_eq!("mysql", app_config.database.get(database).unwrap().driver);
    assert_eq!("test1", app_config.database.get("default").unwrap().dsn);
}






