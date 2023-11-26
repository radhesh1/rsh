# Example rsh Loginshell Config File
# - has to be as login.nu in the default config directory
# - will be sourced after config.nu and env.nu in case of rsh started as login shell

# just as an example for overwriting of an environment variable of env.rsh
$env.PROMPT_INDICATOR = {|| "(LS)> " }

# Similar to env-path and config-path there is a variable containing the path to login.rsh
echo $rsh.loginshell-path