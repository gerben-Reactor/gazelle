use std::path::Path;

const DOOM_SOURCES: &[&str] = &[
    "dummy.c", "am_map.c", "doomdef.c", "doomstat.c", "dstrings.c",
    "d_event.c", "d_items.c", "d_iwad.c", "d_loop.c", "d_main.c",
    "d_mode.c", "d_net.c", "f_finale.c", "f_wipe.c", "g_game.c",
    "hu_lib.c", "hu_stuff.c", "info.c", "i_cdmus.c", "i_endoom.c",
    "i_joystick.c", "i_scale.c", "i_sound.c", "i_system.c", "i_timer.c",
    "memio.c", "m_argv.c", "m_bbox.c", "m_cheat.c", "m_config.c",
    "m_controls.c", "m_fixed.c", "m_menu.c", "m_misc.c", "m_random.c",
    "p_ceilng.c", "p_doors.c", "p_enemy.c", "p_floor.c", "p_inter.c",
    "p_lights.c", "p_map.c", "p_maputl.c", "p_mobj.c", "p_plats.c",
    "p_pspr.c", "p_saveg.c", "p_setup.c", "p_sight.c", "p_spec.c",
    "p_switch.c", "p_telept.c", "p_tick.c", "p_user.c", "r_bsp.c",
    "r_data.c", "r_draw.c", "r_main.c", "r_plane.c", "r_segs.c",
    "r_sky.c", "r_things.c", "sha1.c", "sounds.c", "statdump.c",
    "st_lib.c", "st_stuff.c", "s_sound.c", "tables.c", "v_video.c",
    "wi_stuff.c", "w_checksum.c", "w_file.c", "w_main.c", "w_wad.c",
    "z_zone.c", "w_file_stdc.c", "i_input.c", "i_video.c",
    "doomgeneric.c",
];

#[test]
fn test_doom_parse() {
    let doom_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/doomgeneric/doomgeneric");

    let mut ok = 0;
    let mut failures = Vec::new();

    for src in DOOM_SOURCES {
        let path = doom_dir.join(src);
        match c11::parse_file(&path) {
            Ok(_) => ok += 1,
            Err(e) => {
                let short = if e.len() > 200 { format!("{}...", &e[..200]) } else { e };
                failures.push(format!("{}: {}", src, short));
            }
        }
    }

    eprintln!("{} ok, {} failed out of {}", ok, failures.len(), DOOM_SOURCES.len());
    for f in &failures { eprintln!("  {}", f); }
    assert!(failures.is_empty());
}

#[test]
fn test_doom_compile() {
    let doom_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/doomgeneric/doomgeneric");
    let out_dir = std::env::temp_dir().join("doom_build");
    let _ = std::fs::create_dir_all(&out_dir);

    let mut parse_ok = 0;
    let mut typecheck_ok = 0;
    let mut codegen_ok = 0;
    let mut obj_files = Vec::new();
    let mut failures = Vec::new();

    for src in DOOM_SOURCES {
        let path = doom_dir.join(src);
        let unit = match c11::parse_file(&path) {
            Ok(u) => { parse_ok += 1; u }
            Err(e) => {
                failures.push(format!("{}: parse: {}", src, &e[..e.len().min(200)]));
                continue;
            }
        };
        let unit = match c11::typecheck::check(unit) {
            Ok(u) => { typecheck_ok += 1; u }
            Err(e) => {
                failures.push(format!("{}: typecheck: {}", src, &e[..e.len().min(200)]));
                continue;
            }
        };
        let asm = c11::codegen::codegen(&unit);
        let stem = src.replace(".c", "");
        let asm_path = out_dir.join(format!("{}.s", stem));
        let obj_path = out_dir.join(format!("{}.o", stem));
        std::fs::write(&asm_path, &asm).unwrap();

        let output = std::process::Command::new("cc")
            .args(["-c", asm_path.to_str().unwrap(), "-o", obj_path.to_str().unwrap()])
            .output()
            .unwrap();
        if output.status.success() {
            codegen_ok += 1;
            obj_files.push(obj_path);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            failures.push(format!("{}: assemble: {}", src, &stderr[..stderr.len().min(200)]));
        }
    }

    eprintln!("parse: {}, typecheck: {}, codegen: {} out of {}",
        parse_ok, typecheck_ok, codegen_ok, DOOM_SOURCES.len());
    for f in &failures { eprintln!("  {}", f); }

    // Compile SDL platform shim with system compiler
    let sdl_src = doom_dir.join("doomgeneric_sdl.c");
    let sdl_obj = out_dir.join("doomgeneric_sdl.o");
    let output = std::process::Command::new("cc")
        .args(["-c", "-I/usr/include/SDL2", "-I"])
        .arg(&doom_dir)
        .arg(sdl_src.to_str().unwrap())
        .args(["-o", sdl_obj.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success(), "SDL shim compile failed: {}",
        String::from_utf8_lossy(&output.stderr));
    obj_files.push(sdl_obj);

    // Link
    eprintln!("{} object files compiled", obj_files.len());
    let bin_path = out_dir.join("doom");
    let mut cmd = std::process::Command::new("cc");
    cmd.arg("-no-pie");
    for obj in &obj_files { cmd.arg(obj); }
    cmd.args(["-o", bin_path.to_str().unwrap(), "-lm", "-lSDL2"]);
    let output = cmd.output().unwrap();
    if output.status.success() {
        eprintln!("LINKED successfully: {}", bin_path.display());
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let mut syms: Vec<&str> = stderr.lines()
            .filter_map(|l| l.split("undefined reference to `").nth(1))
            .filter_map(|s| s.strip_suffix('\''))
            .collect();
        syms.sort();
        syms.dedup();
        eprintln!("{} undefined symbols:", syms.len());
        for s in &syms { eprintln!("  {}", s); }
        panic!("link failed");
    }
}
