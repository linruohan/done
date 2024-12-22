# Desktop file
message('Configuring desktop file')

desktop_file_name_entry = app_name_capitalized
if profile == 'devel'
  desktop_file_name_entry += ' (Development)'
endif

desktop_conf = configuration_data()
desktop_conf.set('APP_ID', app_id)
desktop_conf.set('APP_NAME', app_name)
desktop_conf.set('DESKTOP_FILE_NAME_ENTRY', desktop_file_name_entry)
desktop_file_in = configure_file(
  input: 'app.desktop.in.in',
  output: '@BASENAME@',
  configuration: desktop_conf,
)

desktop_file = i18n.merge_file(
  input: desktop_file_in,
  output: app_id + '.desktop',
  type: 'desktop',
  po_dir: '../po',
  install: true,
  install_dir: datadir / 'applications',
)

# Validate Desktop file
if desktop_file_validate.found()
  test(
    'validate-desktop',
    desktop_file_validate,
    args: [desktop_file],
  )
endif

# GSchema
message('Configuring settings GSchema file')

gschema_conf = configuration_data()
gschema_conf.set('APP_ID', app_id)
gschema_conf.set('APP_IDPATH', app_idpath)
gschema_conf.set('APP_NAME', app_name)
configure_file(
  input: 'app.gschema.xml.in',
  output: app_id + '.gschema.xml',
  configuration: gschema_conf,
  install: true,
  install_dir: datadir / 'glib-2.0' / 'schemas',
)

# Validate installed GSchema
if glib_compile_schemas.found()
  test(
    'validate-gschema',
    glib_compile_schemas,
    args: [
      '--dry-run', datadir / 'glib-2.0' / 'schemas',
    ],
  )
endif

# Appdata
if host_machine.system() != 'windows'
  message('Configuring appdata file')

  appdata_conf = configuration_data()
  appdata_conf.set('APP_ID', app_id)
  appdata_conf.set('APP_NAME', app_name)
  appdata_conf.set('APP_NAME_CAPITALIZED', app_name_capitalized)
  appdata_conf.set('APP_AUTHOR_NAME', app_author_name)
  appdata_conf.set('APP_WEBSITE', app_website)
  appdata_conf.set('APP_ISSUES_URL', app_issues_url)
  appdata_conf.set('APP_SUPPORT_URL', app_support_url)
  appdata_conf.set('APP_DONATE_URL', app_donate_url)
  appdata_file_in = configure_file(
    input: 'app.metainfo.xml.in.in',
    output: '@BASENAME@',
    configuration: appdata_conf,
  )

  # Currently this would fail on windows mingw with error:
  #
  # "C:/msys64/mingw64/bin/msgfmt.EXE: cannot locate ITS rules for data/app.metainfo.xml.in"
  #
  # Even though the appdata package is installed and the rule "metainfo.its" is present in
  # "/mingw64/share/gettext/its". But because we don't need the appdata file, we can circumvent this error by just not
  # processing the metainfo file, we don't need it anyway.
  appdata_file = i18n.merge_file(
    input: appdata_file_in,
    output: app_id + '.metainfo.xml',
    po_dir: '../po',
    install: true,
    install_dir: datadir / 'metainfo',
  )

  # Validate Appdata
  if appstreamcli.found()
    test(
      'validate-appdata',
      appstreamcli,
      args: ['validate', '--no-net', appdata_file],
    )
  endif
endif

# App-Icon
if profile == 'devel'
  install_data(
    'icons/scalable/apps/' + app_name + '-devel.svg',
    install_dir: iconsdir / 'hicolor' / 'scalable' / 'apps',
    rename: app_id + '.svg',
  )
else
  install_data(
    'icons/scalable/apps/' + app_name + '.svg',
    install_dir: iconsdir / 'hicolor' / 'scalable' / 'apps',
    rename: app_id + '.svg',
  )
endif

# Symbolic App-Icon
install_data(
  'icons/symbolic/apps/' + app_name + '-symbolic.svg',
  install_dir: iconsdir / 'hicolor' / 'symbolic' / 'apps',
  rename: app_id + '-symbolic.svg',
)

# MIME-Type icon
# Unfortunately this is currently not displayed by nautilus (and possibly other file managers?) when the
# Adwaita theme is used. Instead the "application-x-generic" icon gets picked.
# See: https://gitlab.gnome.org/GNOME/nautilus/-/issues/2190
install_data(
  'icons/scalable/mimetypes/' + 'application-' + app_name + '.svg',
  install_dir: iconsdir / 'hicolor' / 'scalable' / 'mimetypes',
)

# MIME-Type
message('Configuring MIME-type file')

mimetype_conf = configuration_data()
mimetype_conf.set('APP_ID', app_id)
mimetype_conf.set('APP_NAME', app_name)
mimetype_conf.set('APP_NAME_CAPITALIZED', app_name_capitalized)
configure_file(
  input: 'app.mimetype.xml.in',
  output: app_id + '.xml',
  configuration: mimetype_conf,
  install: true,
  install_dir: datadir / 'mime' / 'packages',
)

# # D-Bus service file
# message('Configuring D-Bus service file')
#
# dbus_conf = configuration_data()
# dbus_conf.set('APP_ID', app_id)
# dbus_conf.set('APP_NAME', app_name)
# configure_file(
#   input: 'app.service.in',
#   output: app_id + '.service',
#   configuration: dbus_conf,
#   install: true,
#   install_dir: datadir / 'dbus-1' / 'services',
# )

mydone_ui_gresources_icons_files = files(
  'icons/actions/checked.png',
  'icons/actions/document-edit-symbolic.svg',
  'icons/actions/empty.png',
  'icons/actions/leaf.png',
  'icons/actions/paper-plane.png',
  'icons/dev.edfloreshz.Done-symbolic.svg',
  'icons/dev.edfloreshz.Done.Devel.svg',
  'icons/dev.edfloreshz.Done.Source.svg',
  'icons/dev.edfloreshz.Done.svg',
  'icons/services/computer.png',
  'icons/services/microsoft-todo.png',
)

mydone_ui_gresources_ui_files = files(
  'ui/main_menu.ui',
  'ui/shortcuts.ui',
  'ui/style.css',
)

mydone_ui_gresources_files = [
  mydone_ui_gresources_icons_files,
  mydone_ui_gresources_ui_files,
  files(
    'resources.gresource.xml',
  ),
]

# fonts
# fonts_files = files(
#     'fonts/GrapeNuts-Regular.ttf',
#     'fonts/OpenDyslexic-Regular.ttf',
#     'fonts/TT2020Base-Regular.ttf',
#     'fonts/Virgil.ttf',
# )

# # install fonts
# install_data(fonts_files, install_dir: datadir / 'fonts' / app_name + '-fonts')