from gero.config import settings
from gero.util import logging_config

log = logging_config.getConsoleLogger()

log.info("Initializing the Library")
# print(f"Libray running in {settings.name} mode.")  # from settings.toml
# ok: float = settings.ok  # type: ignore

# log.info(ok)
