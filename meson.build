project('screen-test', 'rust',
  version: '0.2',
  default_options: ['buildtype=debug', 'warning_level=2']
)

prefix = get_option('prefix')
bindir = prefix / get_option('bindir')

cargo = find_program('cargo-1.89', 'cargo', dirs: ['/usr/bin'])

cargo_build = custom_target('screen-test',
  output: meson.project_name(),
  input: 'Cargo.toml',
  command: [cargo, 'build', '--manifest-path', '@INPUT@', '--target-dir', meson.project_build_root(), '--release', '&&', 'cp', 'release' / meson.project_name(), '@OUTPUT@'],
  install: true,
  install_dir: get_option('bindir')
)
