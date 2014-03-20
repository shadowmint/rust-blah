#!/usr/local/bin/python
import ruff
import ruffx.command


# Paths
path_examples = ruff.path(__file__, "src")
path_base = ruff.path(__file__, ".")
path_n = ruff.path(__file__, "libs", "rust-n")


## Common libraries
def build(build_only, now_only):

  nbuild = ruff.build()
  nbuild.notice("libn")
  nbuild.chdir(path_n)
  nbuild.run("rustc", "--crate-type=lib", "n.rs", "--out-dir", "..")

  ntarget = ruff.target(timeout=10)
  ntarget.pattern('.*\.rs$', path_n, recurse=True)

  # Examples
  ebuild = ruff.build()
  ebuild.depend(nbuild)
  ebuild.notice("examples")
  ebuild.chdir(path_base)
  ebuild.run("rm", "-f", "main")
  ebuild.chdir(path_examples)
  if now_only:
    ebuild.run("rustc", "--test", "now.rs", "-o", "../main", "-L", "../libs")
  else:
    ebuild.run("rustc", "--test", "main.rs", "-o", "../main", "-L", "../libs")
  ebuild.chdir(path_base)
  ebuild.run("./main")

  etarget = ruff.target(timeout=10)
  etarget.pattern('.*\.rs$', path_examples, recurse=True)

  ruff.bind(etarget, ebuild)

  # Goooo~
  ruff.run(build=build_only)


if __name__ == '__main__':
  ruffx.command.register('watch', lambda: build(False, True))
  ruffx.command.register('rebuild', lambda: build(True, False))
  ruffx.command.register('now', lambda: build(True, True))
  ruffx.command.register('default', 'rebuild')
  ruffx.command.run()
