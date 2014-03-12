#!/usr/local/bin/python
import ruff
import ruffx.utility
import ruffx.command
import ruffx.typescript
import ruffx.sass
import ruffx.npm


# Paths
base = ruff.path(__file__, ".")


## Common libraries
def watch(build_only):

  # Clean
  build = ruff.build()
  build.run("rustc", "--test", "main.rs")
  build.run("main")

  target = ruff.target(timeout=10)
  target.pattern('.*\.rs', base, recurse=True)

  ruff.bind(target, build)
  ruff.run(build=build_only)


if __name__ == '__main__':
  ruffx.command.register('watch', lambda: watch(False))
  ruffx.command.register('rebuild', lambda: watch(True))
  ruffx.command.register('default', 'rebuild')
  ruffx.command.run()
