#!/usr/bin/env bash
set -ex

[[ $(uname) = Linux ]] || exit 1
[[ $USER = root ]] || exit 1

adduser morgan --gecos "" --disabled-password --quiet
adduser morgan sudo
adduser morgan adm
echo "morgan ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers
id morgan

[[ -r /morgan-id_ecdsa ]] || exit 1
[[ -r /morgan-id_ecdsa.pub ]] || exit 1

sudo -u morgan bash -c "
  mkdir -p /home/morgan/.ssh/
  cd /home/morgan/.ssh/
  cp /morgan-id_ecdsa.pub authorized_keys
  umask 377
  cp /morgan-id_ecdsa id_ecdsa
  echo \"
    Host *
    BatchMode yes
    IdentityFile ~/.ssh/id_ecdsa
    StrictHostKeyChecking no
  \" > config
"

