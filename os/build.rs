use std::fs::{File, read_dir};
use std::io::{Result, Write};

fn main() {
    println!("cargo:rerun-if-changed=../user/src/");
    println!("cargo:rerun-if-changed={}", TARGET_PATH);
    insert_app_data().unwrap();
}

static TARGET_PATH: &str = "../user/target/riscv64gc-unknown-none-elf/release/";

fn insert_app_data() -> Result<()> {
    let mut f = File::create("src/link_app.S")?;

    // 收集所有用户程序
    let mut apps: Vec<_> = read_dir("../user/src/bin")?
        .map(|e| {
            e.unwrap()
                .path()
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .into_owned()
        })
        .collect();

    apps.sort();

    
    writeln!(
        f,
        r#"
    .align 3
    .section .data
    .global _num_app
_num_app:
    .quad {}"#,
        apps.len()
    )?;

    // app 起始地址表
    for i in 0..apps.len() {
        writeln!(f, "    .quad app_{}_start", i)?;
    }

    // 最后一个 app 的 end
    writeln!(f, "    .quad app_{}_end", apps.len() - 1)?;

    // ✅ 关键2：真正的 app 数据放到 .app（避免污染 .data）
    for (i, app) in apps.iter().enumerate() {
        println!("app_{}: {}", i, app);

        writeln!(
            f,
            r#"
    .section .data
    .align 3
    .global app_{0}_start
    .global app_{0}_end
app_{0}_start:
    .incbin "{2}{1}.bin"
app_{0}_end:
"#,
            i, app, TARGET_PATH
        )?;
    }

    Ok(())
}