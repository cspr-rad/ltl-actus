import Lake
open System Lake DSL

package «ltl-model» where
  -- moreLinkArgs := #["-L."]
  -- add package configuration options here
  precompileModules := true

lean_lib «LtlModel» where
  -- add library configuration options here

target time.o pkg : FilePath := do
  let oFile := pkg.buildDir / "csrc" / "time.o"
  let srcJob ← inputFile <| pkg.dir / "csrc" / "time.c"
  let weakArgs := #["-I", (← getLeanIncludeDir).toString]
  buildO "time.c" oFile srcJob weakArgs #["-fPIC"] "gcc" getLeanTrace

extern_lib libleantime pkg := do
  let name := nameToStaticLib "leantime"
  let timeO ← fetch <| pkg.target ``time.o
  buildStaticLib (pkg.nativeLibDir / name) #[timeO]

@[default_target]
lean_exe «ltl-model» where
  root := `Main
