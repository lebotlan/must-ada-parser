# pytest-based test for the generator script.
# It runs generate_project.py in a temporary directory and checks the file count.

import subprocess
import sys
import os
import tempfile
from pathlib import Path

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
GEN = os.path.join(ROOT, "generate_project.py")

def test_generate_small_set(tmp_path):
    outdir = tmp_path / "out"
    outdir.mkdir()
    # Run generator to create 5 files of ~20 lines
    proc = subprocess.run([sys.executable, GEN, "--out", str(outdir), "--num-files", "5", "--lines-per-file", "20"], capture_output=True, text=True)
    assert proc.returncode == 0
    files = list(outdir.glob("mixed_file_*.txt"))
    assert len(files) == 5
    # Each file should be non-empty
    for f in files:
        assert f.stat().st_size > 0