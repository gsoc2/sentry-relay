{
    script_task(input):: {
        # Strip comments from shell script (i.e. "# shellcheck shell=bash")
        # NOTE: An alternative is to prepend a "#" which would maintain the comments in
        # the GoCD UI.
        script: std.join("\n", [x for x in std.split(input,  "\n") if !std.startsWith(x, "#") && x != ""]),
    },
}
