import logging
from genotyper.util import logging_config

logging_config.getConsoleLogger()

log = logging.getLogger("Genotyper")

log.info("Initializing the Library")
# print(f"Libray running in {settings.name} mode.")  # from settings.toml
# ok: float = settings.ok  # type: ignore

# log.info(ok)
