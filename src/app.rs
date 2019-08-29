use clap::{crate_description, crate_name, crate_authors, crate_version, App, AppSettings, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
  App::new(crate_name!())
    .version(crate_version!())
    .author(crate_authors!("\n"))
    .about(crate_description!())
    .global_setting(AppSettings::ColoredHelp)
    .global_setting(AppSettings::ColorAuto)
    .global_setting(AppSettings::UnifiedHelpMessage)
    .global_setting(AppSettings::InferSubcommands)
    .global_setting(AppSettings::VersionlessSubcommands)
    .global_setting(AppSettings::DontCollapseArgsInUsage)
    .setting(AppSettings::SubcommandRequiredElseHelp)
    .arg(
      Arg::with_name("profile")
        .short("p")
        .long("profile")
        .value_name("AWS_PROFILE")
        .help("AWS profile to run commands against")
        .takes_value(true)
        .global(true)
    )
    .subcommand(
      SubCommand::with_name("ssm")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
          SubCommand::with_name("search")
            .about("Search for SSM parameters")
            .arg(
              Arg::with_name("input")
                .index(1) 
            )
        )
    )
    .subcommand(
      SubCommand::with_name("s3")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
          SubCommand::with_name("delete")
            .setting(AppSettings::ArgRequiredElseHelp)
            .arg(
              Arg::with_name("input")
                .index(1)
            )
            .arg(
              Arg::with_name("force")
                .short("f")
                .long("force")
            )
        )
    )
}
