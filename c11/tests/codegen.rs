use std::path::Path;
use std::process::Command;

/// Tests that pass the full pipeline: parse → typecheck → codegen → assemble → run.
/// Exit code must be 0 and stdout must match the .expected file.
const PASSING: &[&str] = &[
    "00001.c", "00002.c", "00003.c", "00004.c", "00005.c",
    "00006.c", "00007.c", "00008.c", "00009.c", "00010.c",
    "00011.c", "00012.c", "00013.c", "00014.c", "00015.c",
    "00016.c", "00017.c", "00018.c", "00019.c", "00020.c",
    "00021.c", "00022.c", "00023.c", "00024.c", "00025.c",
    "00026.c", "00027.c", "00028.c", "00029.c", "00030.c",
    "00031.c", "00032.c", "00033.c", "00034.c", "00035.c",
    "00036.c", "00037.c", "00038.c", "00039.c", "00041.c",
    "00042.c", "00043.c", "00047.c", "00051.c", "00052.c",
    "00056.c", "00057.c", "00060.c", "00061.c", "00062.c",
    "00063.c", "00064.c", "00065.c", "00066.c", "00067.c",
    "00068.c", "00069.c", "00070.c", "00071.c", "00072.c",
    "00073.c", "00074.c", "00075.c", "00076.c", "00079.c",
    "00080.c", "00081.c", "00082.c", "00083.c", "00084.c",
    "00085.c", "00086.c", "00090.c", "00094.c", "00097.c",
    "00098.c", "00099.c", "00100.c", "00101.c", "00102.c",
    "00103.c", "00105.c", "00106.c", "00108.c", "00109.c",
    "00110.c", "00111.c", "00112.c", "00113.c", "00114.c",
    "00115.c", "00116.c", "00119.c", "00121.c", "00122.c",
    "00123.c", "00125.c", "00126.c", "00127.c", "00128.c",
    "00131.c", "00133.c", "00134.c", "00135.c", "00139.c",
    "00140.c", "00141.c", "00142.c", "00144.c", "00145.c",
    "00146.c", "00152.c", "00153.c", "00154.c", "00155.c",
    "00156.c", "00157.c", "00159.c", "00160.c", "00161.c",
    "00162.c", "00163.c", "00164.c", "00165.c", "00166.c",
    "00167.c", "00168.c", "00169.c", "00171.c", "00172.c",
    "00173.c", "00176.c", "00178.c", "00181.c", "00183.c",
    "00184.c", "00186.c", "00188.c", "00190.c", "00191.c",
    "00192.c", "00194.c", "00195.c", "00199.c", "00200.c",
    "00201.c", "00202.c", "00203.c", "00207.c", "00208.c",
    "00209.c", "00210.c", "00215.c",
];

fn compile_and_run(c_path: &Path) -> Result<(i32, String), String> {
    let unit = c11::parse_file(c_path).map_err(|e| format!("parse: {}", e))?;
    let unit = c11::typecheck::check(unit).map_err(|e| format!("typecheck: {}", e))?;
    let asm = c11::codegen::codegen(&unit);

    let stem = c_path.file_stem().unwrap().to_str().unwrap();
    let asm_path = std::env::temp_dir().join(format!("c11test_{}.s", stem));
    let bin_path = std::env::temp_dir().join(format!("c11test_{}", stem));

    std::fs::write(&asm_path, &asm).map_err(|e| format!("write asm: {}", e))?;

    let status = Command::new("cc")
        .args([
            asm_path.to_str().unwrap(),
            "-o", bin_path.to_str().unwrap(),
            "-no-pie", "-lm",
        ])
        .output()
        .map_err(|e| format!("cc: {}", e))?;
    if !status.status.success() {
        return Err(format!("cc failed: {}", String::from_utf8_lossy(&status.stderr)));
    }

    let output = Command::new(&bin_path)
        .output()
        .map_err(|e| format!("run: {}", e))?;

    let _ = std::fs::remove_file(&asm_path);
    let _ = std::fs::remove_file(&bin_path);

    let code = output.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok((code, stdout))
}

#[test]
fn test_codegen() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/c-testsuite/tests/single-exec");

    let mut passed = 0;
    let mut unexpected_failures = Vec::new();
    let mut unexpected_passes = Vec::new();

    let entries: Vec<_> = std::fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "c"))
        .collect();

    for entry in &entries {
        let name = entry.file_name();
        let name = name.to_str().unwrap();
        let expected_pass = PASSING.contains(&name);

        let expected_output = {
            let exp_path = entry.path().with_extension("c.expected");
            std::fs::read_to_string(&exp_path).unwrap_or_default()
        };

        match compile_and_run(&entry.path()) {
            Ok((code, stdout)) if code == 0 && stdout == expected_output => {
                passed += 1;
                if !expected_pass {
                    unexpected_passes.push(name.to_string());
                }
            }
            Ok((code, stdout)) => {
                if expected_pass {
                    unexpected_failures.push(format!(
                        "{}: exit={}, stdout match={}",
                        name, code, stdout == expected_output,
                    ));
                }
            }
            Err(e) => {
                if expected_pass {
                    unexpected_failures.push(format!("{}: {}", name, e));
                }
            }
        }
    }

    eprintln!("{} passed, {} expected to pass, {} total",
        passed, PASSING.len(), entries.len());

    for e in &unexpected_failures {
        eprintln!("UNEXPECTED FAIL: {}", e);
    }
    for p in &unexpected_passes {
        eprintln!("UNEXPECTED PASS (add to PASSING): {}", p);
    }

    assert!(unexpected_failures.is_empty(), "{} unexpected failures", unexpected_failures.len());
    assert!(unexpected_passes.is_empty(), "{} unexpected passes", unexpected_passes.len());
}
