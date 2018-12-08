# conf-form #

A CLI tool to overwrite configuration files using profiles.

The configuration file is parsed using a simplified parser for [zeroc
configuration](https://doc.zeroc.com/ice/latest/properties-and-configuration).

Only keys in the configuration file are overwritten, if the profile contains
a key not present in the configuration file it's ignored.

## Examples ##

Given the following configuration file:
```
❯ cat conf.txt
Author.Name = eriol
# This comment will be stripped out
Author.Like = rust, python
```

and the following profile `p1.yaml`:

```yaml
# Yes, I like all of them.
Author.Like: rust, python, c++
The.cake.is: a lie # this will be ignored since not in the configuration file
```

`conf-form` will produce:

```
❯ conf-form --config conf.txt --profile p1.yaml
Author.Name = eriol
Author.Like = rust, python, c++
```

Warnings for keys only present in profiles can be showed using `-w` flag.

```
❯ conf-form -w --config conf.txt --profile p1.yaml
Warning: The.cake.is key is not present in config file.
Author.Name = eriol
Author.Like = rust, python, c++
```
