# config.rs
message('Configuring info.rs file')

global_conf = configuration_data()
global_conf.set_quoted('PKGDATADIR', pkgdatadir)
global_conf.set_quoted('VERSION', version + version_suffix)
global_conf.set_quoted('APP_NAME', app_name)
global_conf.set_quoted('APP_NAME_CAPITALIZED', app_name_capitalized)
global_conf.set_quoted('APP_ID', app_id)
global_conf.set_quoted('APP_IDPATH', app_idpath)
global_conf.set_quoted('APP_VERSION', version)
global_conf.set_quoted('APP_VERSION_SUFFIX', version_suffix)
global_conf.set_quoted('APP_AUTHOR_NAME', app_author_name)
global_conf.set_quoted('APP_AUTHORS', app_authors)
global_conf.set_quoted('APP_WEBSITE', app_website)
global_conf.set_quoted('APP_ISSUES_URL', app_issues_url)
global_conf.set_quoted('APP_SUPPORT_URL', app_support_url)
global_conf.set_quoted('APP_DONATE_URL', app_donate_url)
global_conf.set_quoted('PROFILE', profile)
global_conf.set_quoted('GETTEXT_PACKAGE', gettext_package)
global_conf.set_quoted('DATADIR', datadir)
global_conf.set_quoted('LIBDIR', libdir)
global_conf.set_quoted('LOCALEDIR', localedir)

configure_file(
    input: 'info.rs.in',
    output: 'info.rs',
    configuration: global_conf,
)
# Copy the config.rs output to the source directory
run_command(
    'cp',
    meson.project_build_root() / 'src' /  'info.rs',
    meson.project_source_root() / 'src' / 'app' / 'config' / 'info.rs',
    check: true,
)

mydone_ui_sources = files(
    'app/components/about_dialog.rs',
    'app/components/content.rs',
    'app/components/delete.rs',
    'app/components/list_dialog.rs',
    'app/components/list_sidebar.rs',
    'app/components/mod.rs',
    'app/components/preferences.rs',
    'app/components/services.rs',
    'app/components/task_input.rs',
    'app/components/welcome.rs',
    'app/config/actions.rs',
    'app/config/appearance.rs',
    'app/config/gettext.rs',
    'app/config/info.rs',
    'app/config/info.rs.in',
    'app/config/localization.rs',
    'app/config/mod.rs',
    'app/config/preferences.rs',
    'app/config/resources.rs',
    'app/config/settings.rs',
    'app/config/setup.rs',
    'app/factories/mod.rs',
    'app/factories/service.rs',
    'app/factories/sub_tasks.rs',
    'app/factories/sub_tasks/factory.rs',
    'app/factories/sub_tasks/messages.rs',
    'app/factories/sub_tasks/model.rs',
    'app/factories/task.rs',
    'app/factories/task_list.rs',
    'app/models/mod.rs',
    'app/models/sidebar_list.rs',
    'app.rs',
    'main.rs',
)