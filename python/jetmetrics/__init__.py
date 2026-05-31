"""jetmetrics — Arrow-native statistical metrics for ML monitoring."""

__version__ = "0.1.0"

try:
    from jetmetrics._core import *
except ImportError:
    # Not yet compiled; running from source
    pass
