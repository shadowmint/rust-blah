#!/usr/local/bin/python
import ruff
import ruffx.command


# Paths
path_examples = ruff.path(__file__, "src")


## Common libraries
def build():

  # Examples
  ebuild = ruff.build()
  ebuild.run("cargo", "test")

  etarget = ruff.target(timeout=10)
  etarget.pattern('.*\.rs$', path_examples, recurse=True)

  ruff.bind(etarget, ebuild)

  # Goooo~
  ruff.run(build=False)


if __name__ == '__main__':
  ruffx.command.register('rebuild', build)
  ruffx.command.register('default', 'rebuild')
  ruffx.command.run()
