## run the scripts

> **Note**  
> the following table must be read as follows:
> - an `x` means *it works*
> - a `?` means *no data available*
>
> `.rsh` scripts must be run as `rsh .../foo.rsh`  
> `.sh` scripts must be run as `./.../foo.sh`  
> `.ps1` scripts must be run as `powershell .../foo.ps1`
>
> let's say a script is called `foo`
> - an `x` in the *`./scripts`* column means *`foo` can be run from `./scripts`*
> - an `x` in the *root* column means *`foo` can be run from the root of `rshshell`*
> - an `x` in the *anywhere* column means *`foo` can be run from anywhere!*

| script                  | `./scripts/` | root | anywhere |
| ----------------------- | ------------ | ---- | -------- |
| `build-all-maclin.sh`   | x            | x    | x        |
| `build-all-windows.cmd` | ?            | x    | ?        |
| `build-all.rsh`          | x            | x    | x        |
| `coverage-local.rsh`     | x            | x    | x        |
| `coverage-local.sh`     | x            | x    | x        |
| `install-all.ps1`       | ?            | x    | ?        |
| `install-all.sh`        | x            | x    | x        |
| `register-plugins.rsh`   | x            | x    | x        |
| `uninstall-all.sh`      | x            | x    | x        |
