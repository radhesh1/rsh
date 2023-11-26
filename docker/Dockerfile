# Git: git version 2.30.2
# /etc/os-release: Alpine Linux v3.16
# Kernel: Linux ca3abedc4fb1 5.17.15-76051715-generic #202206141358~1655919116~22.04~1db9e34 SMP PREEMPT Wed Jun 22 19 x86_64 Linux
# Build cmd: docker build --no-cache . -t rsh-latest
# Other tags: rsh/alpine-rsh:latest, rsh
FROM alpine

LABEL maintainer=rsh

RUN echo '/usr/bin/rsh' >> /etc/shells \
    && adduser -D -s /usr/bin/rsh rsh \
    && mkdir -p /home/rsh/.config/rsh/ \
    && wget -q https://raw.githubusercontent.com/radhesh1/rsh/main/crates/rsh-utils/src/sample_config/default_config.rsh -O /home/rsh/.config/rsh/config.rsh \
    && wget -q https://raw.githubusercontent.com/radhesh1/rsh/main/crates/rsh-utils/src/sample_config/default_env.rsh -O /home/rsh/.config/rsh/env.rsh \
    && cd /tmp \
    && wget -qO - https://api.github.com/repos/rsh/rsh/releases/latest \
    | grep browser_download_url \
    | grep musl.tar.gz \
    | cut -f4 -d '"' \
    | xargs -I{} wget {} \
    && tar -xzf rsh* \
    && cd rsh*-musl \
    && mv rsh* /usr/bin \
    && chmod +x /usr/bin/rsh \
    && chown -R rsh:rsh /home/rsh/.config/rsh \
    && ls /usr/bin/rsh_plugin* \
    | xargs -I{} su -c 'register {}' rsh \
    && rm -rf /tmp/*

USER rsh

WORKDIR /home/rsh

ENTRYPOINT ["nu"]