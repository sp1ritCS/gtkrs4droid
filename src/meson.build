cargo_bin  = find_program('cargo')
cargo_opt  = [ '--manifest-path', meson.project_source_root() / 'Cargo.toml' ]
cargo_opt += [ '--target-dir', meson.current_build_dir() ]

cargo_env = environment()
cargo_env.set('CARGO_HOME', meson.project_build_root() / 'cargo-home')
cargo_env.set('GRESOURCES_PATH', app_res.full_path())

if ['release', 'minsize'].contains(get_option('buildtype'))
  cargo_opt += [ '--release' ]
  cargo_output_dir = 'release'
else
  cargo_output_dir = 'debug'
endif

if host_machine.system() == 'android'
  target = '@0@-linux-android'.format(host_machine.cpu_family()) 
  cargo_opt += [ '--target', target ]
  cargo_output_dir = target / cargo_output_dir
endif

app_rust_tgt = custom_target(
  'gtkrs4droid-rust',
  build_by_default: true,
  build_always_stale: true,
  output: 'lib@0@.a'.format(meson.project_name()),
  console: true,
  install: false,
  env: cargo_env,
  depends: [ app_res ],
  command: [
    'sh', '-c', 'eval $@', '--',
    cargo_bin, 'build',
    cargo_opt, '&&', 'cp', meson.current_build_dir() / cargo_output_dir / 'lib@0@.a'.format(meson.project_name()), '@OUTPUT@',
  ]
)

app_rdep = declare_dependency(
  link_args: ['-Wl,--whole-archive', meson.current_build_dir() / 'lib@0@.a'.format(meson.project_name()), '-Wl,--no-whole-archive'],
  sources: app_rust_tgt
)

app = gnome.executable(meson.project_name(),
  dependencies: [
    glib_dep,
    gobject_dep,
    gio_dep,
    gtk_dep,
    app_rdep
  ],
  install: true
)
