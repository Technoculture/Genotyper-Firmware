from dynaconf import Dynaconf  # type: ignore

settings = Dynaconf(
    envvar_prefix="GERO",
    # `envvar_prefix` = export envvars with `export DYNACONF_FOO=bar`.
    settings_files=['settings.toml', '.secrets.toml'],
    # `settings_files` = Load these files in the order.
)
