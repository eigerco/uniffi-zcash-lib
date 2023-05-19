## See https://devpi.net/docs/devpi/devpi/stable/+d/quickstart-pypimirror.html
FROM python:3-alpine3.9

RUN pip install --upgrade pip
RUN apk add build-base libffi-dev supervisor && \
    pip install -U devpi-server

RUN adduser -D pypi

USER pypi
WORKDIR /home/pypi

RUN devpi-init --root-passwd admin
RUN devpi-gen-config

EXPOSE 3141

CMD ["supervisord", "-n", "-c", "gen-config/supervisord.conf"]

