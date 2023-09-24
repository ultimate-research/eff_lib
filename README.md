# eff_lib

eff_lib is a repository for libraries and tools written in Rust for working with EFF files from Super Smash Bros. Ultimate.

## eff_lib_json

A command-line program for creating and editing EFF files using JSON. Drag and drop an EFF file onto the executable to create a JSON file. Drag and drop a properly structured JSON file onto the executable to create an EFF file. JSON files are text files, so they can be viewed and edited in any text editor.

Sample output from an EFF file:

```json
{
  "effect_handles": [
    {
      "flags": {
        "bytes": [
          0,
          0,
          0,
          0
        ]
      },
      "emitter_set_handle": 1,
      "effect_model_entry_handle": 0,
      "effect_group_element_start": 0,
      "effect_group_element_count": 0
    }
  ],
  "effect_group_elements": [],
  "effect_model_entries": [],
  "effect_handle_names": [
    "STG_CARTBOARD_SMOKE"
  ],
  "effect_model_names": [],
  "parent_joint_names": []
}
```

### Usage

The latest executable for Windows is available in the [Releases](https://github.com/ultimate-research/eff_lib/releases/latest).

`eff_lib_json <input> [output] [ptcl]`<br>
`eff_lib_json ef_mario.eff ef_mario.json ef_mario.ptcl`<br>
`eff_lib_json ef_mario.json ef_mario.eff ef_mario.ptcl`<br>

## eff_data_json

A command-line program for creating and editing EFF files using JSON. Drag and drop an EFF file onto the executable to create a JSON file. Drag and drop a properly structured JSON file onto the executable to create an EFF file. JSON files are text files, so they can be viewed and edited in any text editor.

Sample output from an EFF file:

```json
{
  "effect_handles": [
    {
      "name": "STG_CARTBOARD_SMOKE",
      "flags": {
        "unk_01": false,
        "unk_02": false,
        "unk_03": false,
        "unk_04": false,
        "unk_05": false,
        "unk_06": false,
        "unk_07": false,
        "unk_09": false,
        "unk_10": false,
        "unk_13": false,
        "unk_14": false,
        "unk_15": false,
        "unk_16": false,
        "unk_17": false,
        "hit_effect": false,
        "unk_20": false,
        "unk_21": false,
        "unk_23": false,
        "update_always": false,
        "unk_25": false,
        "unk_26": false,
        "unk_29": false,
        "unk_30": false,
        "unk_31": false,
        "unk_32": false,
      },
      "emitter_set_handle": 1,
      "effect_model_name": "",
      "effect_group": []
    }
  ],
  "effect_model_entries": []
}
```

### Usage

The latest executable for Windows is available in the [Releases](https://github.com/ultimate-research/eff_lib/releases/latest).

`eff_data_json <input> [output] [ptcl]`<br>
`eff_data_json ef_mario.eff ef_mario.json ef_mario.ptcl`<br>
`eff_data_json ef_mario.json ef_mario.eff ef_mario.ptcl`<br>
