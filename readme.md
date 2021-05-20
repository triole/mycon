# Mycon

<!--- mdtoc: toc begin -->

1.	[Synopsis](#synopsis)
2.	[Config](#config)
3.	[Examples](#examples)<!--- mdtoc: toc end -->

## Synopsis

Mycon is a fast external ip retrieval tool. It fires multiple parallel requests and prints the first valid answer.

## Config

There is a configuration file `config/mycon.yaml`. It contains several web services that provide the external ip. These are the ones that `mycon` calls.

## Examples

Simplest use looks like this

```
$ mycon
Reponse from: http://ident.me
Duration    : 0,102 sec
External IP : 111.22.111.22
```

Run `mycon -h` for help.
