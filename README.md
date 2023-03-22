# 🌰 conker

**Con**textual Doc**ker** Compose

```diff
 $ cd app-name
-$ docker context update app-name-staging --docker host=ssh://username@hostname
-$ docker --context app-name-staging compose -p app-name-staging -f docker-compose.yml -f docker-compose.staging.yml ps
+$ git pull
+$ conker staging ps
```

Conker uses a list of environments and Docker Engine URLs from a text file in your project's root to transparently run your Docker Compose commands on 
the right engine with the right compose configuration.

## Set up

1. Install: `TODO`
2. Create a Conkerfile in your project directory: `conker init`
3. Add at least one environment to `Conkerfile`

## Run

`conker [environment] [docker compose command]`

e.g.

`conker production ps`

`conker staging run --rm web bash`

`conker qa1 up -d`

The first argument (environment) must exist in `Conkerfile` (see below), and the remaining arguments are appended to the contextualised docker compose 
command that will be executed on the remote engine.

STDIN, STDOUT and STDERR are attached to the spawned docker process.

## Conkerfile

`Conkerfile` is just a list of environment names and their corresponding Docker Engine URLs. `conker` will always create or update the relevant Docker 
context according to this list prior to executing your command, so you should check this file into source control.

#### Example
```
# Conkerfile
production ssh://user@host
staging tcp://host:2375
qa1 tcp://host:2375
```

Lines starting with `#` are ignored as comments. Otherwise, each line must split by whitespace into exactly two components:

1. Environment name
2. Docker Engine URL, which is passed to 
[docker context as `--docker host=$`](https://docs.docker.com/engine/reference/commandline/context_create/#examples), e.g.:
    - `ssh://user@host` for any account that can run commands on the remote docker engine; there's nothing else to set up, but it's slow and can upset
firewalls
    - `tcp://host:port` if the remote Docker Engine is accessible this way; this is faster and more reliable, but trickier to secure

## Hmm?

Docker comes out of the box with everything needed to run multiple, namespaced compose stacks on single remote hosts. You don't need complicated cluster 
management shenanigans like Swarm or Kubernetes; at a minimum, all you need is SSH access to a host running a vanilla Docker Engine, so it's an extremely 
cheap way to deploy applications that don't need five 9s of uptime or scaling to the moon.

You can do exactly the same thing without conker, it just looks like this:

```bash
cd app-name

docker context \ # first set up the context
  create \ # unless you did this before, then it's update
  app-name-staging \ # always an arbitrary variation of application and environment
  --docker host=ssh://user@host # kinda-weird format I always forget
  
docker \ # alright, time to run my command!
  --context app-name-staging \ # there's that naming convention again
  compose \ # cool, we're on the right host, now we can drop in to compose
  -p app-name-staging \ # namespace this compose stack... is that an echo?
  -f docker-compose.yml \ # base configuration is literally always the same, but we digress
  -f docker-compose.staging.yml \ # a new naming convention, hurrah
  ps # or `up -d` or `run --rm web bash` or whatever... finally, whatever it was we came for
```

This inevitably results in 1) a wonky bash script and 2) hopes that we can keep track of how to reach our application instances once they're deployed.

Conker aims to solve both problems:

- Transparently synchronise Docker contexts from source control
- Hide all the boilerplate needed to run docker compose commands in those remote contexts
