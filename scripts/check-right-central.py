"""Validate the configuration actually emitted by Cargo build.rs."""
import pathlib, tomllib, re, struct, hashlib, json, subprocess
root = pathlib.Path(__file__).resolve().parents[1]
base = tomllib.loads((root / "keyboard.toml").read_text())
configs = []
for file in (root / "target/thumbv7em-none-eabihf/release/build").glob("rmk-keyboard-madula-*/out/madula-*.toml"):
    cfg = tomllib.loads(file.read_text())
    if cfg["keyboard"]["product_name"] == "Madula Right RMK":
        configs.append(cfg)
assert configs, "Build the right-central set first"
assert any(c["host"].get("rynk_enabled") for c in configs)
for cfg in configs:
    central, left, madula = cfg["split"]["central"], *cfg["split"]["peripheral"]
    assert central["matrix"] == base["split"]["peripheral"][1]["matrix"]
    assert left["matrix"] == base["split"]["peripheral"][0]["matrix"]
    assert madula["matrix"] == base["split"]["central"]["matrix"]
    assert central["input_device"] == base["split"]["peripheral"][1]["input_device"]
    assert left["input_device"] == base["split"]["peripheral"][0]["input_device"]
    assert (central["row_offset"],central["col_offset"]) == (0,7)
    assert (left["row_offset"],left["col_offset"]) == (0,0)
    assert (madula["row_offset"],madula["col_offset"]) == (4,0)
    for layer, old in zip(cfg["keymap"]["layer"], base["keymap"]["layer"], strict=True):
        assert layer["keys"] == old["keys"]
        assert layer["encoders"] == list(reversed(old["encoders"]))
    assert re.findall(r"\(\d+,\d+\)",cfg["layout"]["map"]) == re.findall(r"\(\d+,\d+\)",base["layout"]["map"])
    assert cfg["layout"]["shapes"] == base["layout"]["shapes"]
    assert cfg["layout"]["variant"] == base["layout"]["variant"]
    assert cfg["storage"]["start_addr"] == 0xD4000
    assert cfg["storage"]["start_addr"] + 4096*cfg["storage"]["num_sectors"] == 0xE0000
out = root / "dist/right-central"
variants = ["right-central-left","right-central-tb-rynk","right-central-madula-tb"]
for variant in variants:
    data = (out / (variant+".uf2")).read_bytes()
    assert len(data)%512 == 0
    for offset in range(0,len(data),512):
        w = struct.unpack_from("<8I",data,offset)
        assert w[0]==0x0a324655 and w[1]==0x9e5d5157 and w[7]==0xada52840
        assert w[3]==0x1000+offset//2 and w[4]==256
        assert w[5]==offset//512 and w[6]==len(data)//512
        assert w[3]+w[4]<=0xD4000, "Firmware must not overwrite either topology's settings"
manifest = {
 "topology":"right central / left peripheral 0 / Madula TB peripheral 1",
 "commit":subprocess.check_output(["git","rev-parse","HEAD"],cwd=root,text=True).strip(),
 "working_tree_dirty":bool(subprocess.check_output(["git","status","--porcelain"],cwd=root,text=True).strip()),
 "hardware_tested":False,
 "storage_start":"0xD4000",
 "variants":{v:{ext:hashlib.sha256((out/(v+"."+ext)).read_bytes()).hexdigest()
              for ext in ["uf2","elf"]} for v in variants}
}
(out/"manifest.json").write_text(json.dumps(manifest,indent=2)+"\n")
(out/"SHA256SUMS").write_text("".join(hashlib.sha256(f.read_bytes()).hexdigest()+"  "+f.name+"\n"
    for f in sorted(out.iterdir()) if f.suffix in [".uf2",".elf",".json"]))
print("Generated topology, all layer mappings, encoder order and 3 UF2 address ranges verified.")
