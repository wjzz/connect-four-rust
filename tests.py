import pytest

import subprocess
import re

def test_hello():
    assert 1 + 1 == 2

# pub const ROWS: usize = 4;
# pub const COLS: usize = 6;

def replace_file(rows, cols):
    with open("src/types.rs", 'r+') as f:
        text = f.read()
        text = re.sub(r'pub const ROWS: usize = (\d+)', f"pub const ROWS: usize = {rows}", text)
        text = re.sub(r'pub const COLS: usize = (\d+)', f"pub const COLS: usize = {cols}", text)
        # print(text)
        f.seek(0)
        f.write(text)
        f.truncate()

def connect_four(rows, cols, expected_result, nodes):
    replace_file(rows, cols)

    cmd = subprocess.run(["make", "quiet"], capture_output=True)

    stdout = cmd.stdout.decode("utf-8").splitlines()
    results = [line for line in stdout if line.startswith("result")]
    assert len(results) == 1
    _rows, _cols, result, _nodes = [int(val) for val in results[0].split(",")[1:]]

    assert _rows == rows
    assert _cols == cols
    assert result == expected_result
    assert nodes == _nodes

DRAW = 0
WHITE_WIN = -10
BLACK_WIN = 10

@pytest.mark.parametrize("rows, cols, expected, nodes",
    [
        (4, 5, DRAW, 272_297),
        (5, 4, DRAW, 40_738),
        (5, 5, DRAW, 3_266_637),
        (4, 6, WHITE_WIN, 6_404_566),
        (5, 6, DRAW, 110_361_453),
        (6, 5, DRAW, 43_279_283),
    ]
)
def test_connect_four(rows, cols, expected, nodes):
    connect_four(rows, cols, expected, nodes)
