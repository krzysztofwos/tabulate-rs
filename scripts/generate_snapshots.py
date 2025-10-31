#!/usr/bin/env python3
import json
from collections import namedtuple
from dataclasses import dataclass

from tabulate import tabulate

try:
    import numpy as np  # type: ignore
except ImportError:  # pragma: no cover - optional dependency
    np = None

try:
    import pandas as pd  # type: ignore
except ImportError:  # pragma: no cover - optional dependency
    pd = None


def to_json_kwargs(kwargs):
    result = {}
    for key, value in kwargs.items():
        if isinstance(value, tuple):
            result[key] = list(value)
        elif isinstance(value, np.ndarray):  # type: ignore
            result[key] = value.tolist()
        else:
            result[key] = value
    return result


CASES = {
    "plain_simple": {
        "data": [
            ["Sun", "696000", "1989100000"],
            ["Earth", "6371", "5973.6"],
            ["Moon", "1737", "73.5"],
            ["Mars", "3390", "641.85"],
        ],
        "kwargs": {"tablefmt": "plain"},
    },
    "simple_with_headers": {
        "data": [["Name", "Score"], ["Alice", 10], ["Bob", 1000]],
        "kwargs": {"headers": "firstrow", "tablefmt": "simple"},
    },
    "pipe_alignment": {
        "data": [["Name", "Score"], ["Alice", 10], ["Bob", 1000]],
        "kwargs": {
            "headers": "firstrow",
            "tablefmt": "pipe",
            "colalign": ("left", "right"),
        },
    },
    "grid_rowalign": {
        "data": [
            ["Name", "Description"],
            ["Mercury", "nearest\nplanet"],
            ["Venus", "second\nplanet"],
        ],
        "kwargs": {"headers": "firstrow", "tablefmt": "grid", "rowalign": "bottom"},
    },
    "github_multiline": {
        "data": [["Name", "Quote"], ["Alice", "Hello\nWorld"], ["Bob", "Hi"]],
        "kwargs": {"headers": "firstrow", "tablefmt": "github"},
    },
    "orgtbl_numeric": {
        "data": [["Name", "Score"], ["Alice", 10], ["Bob", 1000]],
        "kwargs": {"headers": "firstrow", "tablefmt": "orgtbl"},
    },
    "psql_numbers": {
        "data": [["planet", "radius"], ["Mercury", 2440], ["Venus", 6052]],
        "kwargs": {"headers": "firstrow", "tablefmt": "psql"},
    },
    "html_simple": {
        "data": [["Name", "Score"], ["Alice", 1], ["Bob", 2]],
        "kwargs": {"headers": "firstrow", "tablefmt": "html"},
    },
    "unsafehtml_simple": {
        "data": [["Name", "Score"], ["<b>Alice</b>", 10], ["<i>Bob</i>", 1000]],
        "kwargs": {"headers": "firstrow", "tablefmt": "unsafehtml"},
    },
    "latex_table": {
        "data": [["Planet", "Radius"], ["Mercury", 2440], ["Venus", 6052]],
        "kwargs": {"headers": "firstrow", "tablefmt": "latex"},
    },
    "latex_booktabs": {
        "data": [["Planet", "Radius"], ["Mercury", 2440], ["Venus", 6052]],
        "kwargs": {"headers": "firstrow", "tablefmt": "latex_booktabs"},
    },
    "grid_maxcolwidth": {
        "data": [
            ["Name", "Description"],
            ["Mercury", "Nearest planet to the Sun"],
            ["Venus", "Has a thick atmosphere"],
        ],
        "kwargs": {
            "headers": "firstrow",
            "tablefmt": "grid",
            "maxcolwidths": [None, 10],
        },
    },
    "grid_disable_numparse": {
        "data": [["Region", "Value"], ["EU", "42992e1"], ["US", "1000"]],
        "kwargs": {"headers": "firstrow", "tablefmt": "grid", "disable_numparse": [1]},
    },
    "pipe_rowalign_mixed": {
        "data": [
            ["Name", "Description"],
            ["Mercury", "nearest\nplanet"],
            ["Venus", "second\nplanet"],
        ],
        "kwargs": {
            "headers": "firstrow",
            "tablefmt": "pipe",
            "rowalign": ["top", "bottom"],
        },
    },
    "pipe_disable_numparse": {
        "data": [["Region", "Value"], ["EU", "42992e1"], ["US", "1000"]],
        "kwargs": {"headers": "firstrow", "tablefmt": "pipe", "disable_numparse": [1]},
    },
    "plain_showindex": {
        "data": [["Sun", "696000"], ["Earth", "6371"]],
        "kwargs": {"headers": "firstrow", "tablefmt": "plain", "showindex": "always"},
    },
    "plain_colglobal_right": {
        "data": [["Alice", 10], ["Bob", 1000]],
        "kwargs": {"tablefmt": "plain", "colalign": ("right", "right")},
    },
    "ansi_plain": {
        "data": [
            ["Name", "Value"],
            ["\u001b[31mRed\u001b[0m", 10],
            ["Plain", 5],
        ],
        "kwargs": {"headers": "firstrow", "tablefmt": "plain"},
    },
    "wide_grid": {
        "data": [["Name", "Note"], ["寿司", "おいしい"], ["カレー", "辛い"]],
        "kwargs": {"headers": "firstrow", "tablefmt": "grid"},
    },
    "plain_preserve_whitespace": {
        "data": [["Name", "Value"], ["  Alice", " 10"], ["Bob  ", "5"]],
        "kwargs": {"headers": "firstrow", "tablefmt": "plain"},
    },
    "mediawiki_basic": {
        "data": [["Name", "Score"], ["Alice", 10], ["Bob", 1000]],
        "kwargs": {"headers": "firstrow", "tablefmt": "mediawiki"},
    },
    "textile_basic": {
        "data": [["Name", "Score"], ["Alice", 10], ["Bob", 1000]],
        "kwargs": {"headers": "firstrow", "tablefmt": "textile"},
    },
}


Point = namedtuple("Point", "x y")


@dataclass
class Person:
    name: str
    age: int


CASES["namedtuple_keys_plain"] = {
    "python_data": [Point(1, 2), Point(3, 4)],
    "data_repr": [{"x": 1, "y": 2}, {"x": 3, "y": 4}],
    "kwargs": {"headers": "keys", "tablefmt": "plain"},
}

CASES["dataclass_keys_plain"] = {
    "python_data": [Person("Alice", 30), Person("Bob", 25)],
    "data_repr": [{"name": "Alice", "age": 30}, {"name": "Bob", "age": 25}],
    "kwargs": {"headers": "keys", "tablefmt": "plain"},
}

CASES["dict_iterables_mismatch_plain"] = {
    "python_data": {
        "name": ["Alice", "Bob", "Cara"],
        "age": [30, 25],
        "city": ["NYC", "Paris", "Berlin"],
    },
    "data_repr": [
        {
            "name": ["Alice", "Bob", "Cara"],
            "age": [30, 25],
            "city": ["NYC", "Paris", "Berlin"],
        }
    ],
    "kwargs": {"headers": "keys", "tablefmt": "plain"},
}


def to_json_value(value):
    if isinstance(value, (str, int, float, bool)) or value is None:
        return value
    if hasattr(value, "item"):
        return value.item()
    if isinstance(value, (list, tuple)):
        return [to_json_value(v) for v in value]
    return str(value)


if pd is not None:

    def dataframe_repr(df, *, include_index=True):
        data = [[to_json_value(v) for v in row] for row in df.to_numpy().tolist()]
        columns = [to_json_value(col) for col in df.columns.tolist()]
        index = None
        index_label = None
        if include_index:
            index_values = []
            for value in df.index.tolist():
                if isinstance(value, tuple):
                    index_values.append(str(tuple(value)))
                else:
                    index_values.append(to_json_value(value))
            index = index_values
            index_names = [name for name in (df.index.names or []) if name is not None]
            if index_names:
                if len(index_names) == 1:
                    index_label = index_names[0]
                else:
                    index_label = None
        result = {
            "__tabulate_dataframe__": True,
            "columns": columns,
            "data": data,
        }
        if index is not None:
            result["index"] = index
        if index_label is not None:
            result["index_label"] = index_label
        return result

    multi_index = pd.MultiIndex.from_product(
        [["foo", "bar"], ["one", "two"]], names=("first", "second")
    )
    multi_df = pd.DataFrame(
        {
            "A": [1, 2, 3, 4],
            "B": [5, 6, 7, 8],
        },
        index=multi_index,
    )
    CASES["dataframe_multiindex_grid"] = {
        "python_data": multi_df,
        "data_repr": [dataframe_repr(multi_df)],
        "kwargs": {"headers": "keys", "tablefmt": "grid"},
    }

    simple_df = pd.DataFrame(
        {"name": ["Alice", "Bob"], "score": [10, 20]},
        index=pd.Index(["row 1", "row 2"], name="id"),
    )
    CASES["dataframe_index_label_plain"] = {
        "python_data": simple_df,
        "data_repr": [dataframe_repr(simple_df)],
        "kwargs": {"headers": "keys", "tablefmt": "plain"},
    }

if np is not None:
    ndarray = np.array([[1, 2], [3, 4]])
    CASES["numpy_array_plain"] = {
        "python_data": ndarray,
        "data_repr": ndarray.tolist(),
        "kwargs": {"tablefmt": "plain"},
    }

    rec_array = np.array(
        [(1, "Alice"), (2, "Bob")], dtype=[("id", "i4"), ("name", "U10")]
    )
    CASES["numpy_recarray_keys_plain"] = {
        "python_data": rec_array,
        "data_repr": [
            {
                "__tabulate_numpy_recarray__": True,
                "dtype": [("id", "i4"), ("name", "U10")],
                "rows": rec_array.tolist(),
            }
        ],
        "kwargs": {"headers": "keys", "tablefmt": "plain"},
    }

EXTRA_CASES = {
    "latex_raw_basic": {
        "data": [["Name", "Score"], ["Alice", 10], ["Bob", 1000]],
        "kwargs": {"headers": "firstrow", "tablefmt": "latex_raw"},
    },
    "latex_raw_colalign": {
        "data": [["Name", "Value"], ["Alice", "42992e1"], ["Bob", "1000"]],
        "kwargs": {
            "headers": "firstrow",
            "tablefmt": "latex_raw",
            "colalign": ("right", "left"),
        },
    },
    "latex_longtable_basic": {
        "data": [["Name", "Score"], ["Alice", 10], ["Bob", 1000]],
        "kwargs": {"headers": "firstrow", "tablefmt": "latex_longtable"},
    },
    "pretty_basic": {
        "data": [["Name", "Score"], ["Alice", 10], ["Bob", 1000]],
        "kwargs": {"headers": "firstrow", "tablefmt": "pretty"},
    },
    "presto_basic": {
        "data": [["Name", "Score"], ["Alice", 10], ["Bob", 1000]],
        "kwargs": {"headers": "firstrow", "tablefmt": "presto"},
    },
    "colon_grid_alignment": {
        "data": [["Name", "Score"], ["Alice", 10], ["Bob", 1000]],
        "kwargs": {
            "headers": "firstrow",
            "tablefmt": "colon_grid",
            "colalign": ("left", "right"),
        },
    },
    "simple_outline_basic": {
        "data": [["Name", "Score"], ["Alice", 10], ["Bob", 1000]],
        "kwargs": {"headers": "firstrow", "tablefmt": "simple_outline"},
    },
    "rounded_outline_basic": {
        "data": [["Name", "Score"], ["Alice", 10], ["Bob", 1000]],
        "kwargs": {"headers": "firstrow", "tablefmt": "rounded_outline"},
    },
    "heavy_outline_basic": {
        "data": [["Name", "Score"], ["Alice", 10], ["Bob", 1000]],
        "kwargs": {"headers": "firstrow", "tablefmt": "heavy_outline"},
    },
    "mixed_outline_basic": {
        "data": [["Name", "Score"], ["Alice", 10], ["Bob", 1000]],
        "kwargs": {"headers": "firstrow", "tablefmt": "mixed_outline"},
    },
    "double_outline_basic": {
        "data": [["Name", "Score"], ["Alice", 10], ["Bob", 1000]],
        "kwargs": {"headers": "firstrow", "tablefmt": "double_outline"},
    },
    "fancy_outline_basic": {
        "data": [["Name", "Score"], ["Alice", 10], ["Bob", 1000]],
        "kwargs": {"headers": "firstrow", "tablefmt": "fancy_outline"},
    },
    "tsv_basic": {
        "data": [["Name", "Score"], ["Alice", 10], ["Bob", 1000]],
        "kwargs": {"headers": "firstrow", "tablefmt": "tsv"},
    },
    "moinmoin_basic": {
        "data": [["Name", "Score"], ["Alice", 10], ["Bob", 1000]],
        "kwargs": {"headers": "firstrow", "tablefmt": "moinmoin"},
    },
    "youtrack_basic": {
        "data": [["Name", "Score"], ["Alice", 10], ["Bob", 1000]],
        "kwargs": {"headers": "firstrow", "tablefmt": "youtrack"},
    },
    "disable_numparse_mixed": {
        "data": [["Value"], ["42992e1"], ["100"]],
        "kwargs": {"headers": "firstrow", "tablefmt": "plain", "disable_numparse": [0]},
    },
    "maxheadercolwidths_scalar": {
        "data": [["VeryLongHeader", "Short"], ["1", "2"]],
        "kwargs": {"headers": "firstrow", "tablefmt": "grid", "maxheadercolwidths": 6},
    },
}

CASES.update(EXTRA_CASES)


snapshots = {}
for name, case in CASES.items():
    python_data = case.get("python_data", case.get("data"))
    if python_data is None:
        raise ValueError(f"case '{name}' is missing python data")
    kwargs = case["kwargs"]
    output = tabulate(python_data, **kwargs)
    snapshots[name] = {
        "data": case.get("data_repr", case.get("data")),
        "kwargs": to_json_kwargs(kwargs),
        "output": output,
    }

import pathlib

pathlib.Path("tests/fixtures").mkdir(parents=True, exist_ok=True)
with open("tests/fixtures/python_snapshots.json", "w", encoding="utf-8") as f:
    json.dump(snapshots, f, indent=2, ensure_ascii=False)
