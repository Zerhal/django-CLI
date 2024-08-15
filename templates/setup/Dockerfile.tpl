FROM python:{{python_version}}-slim

WORKDIR /app

COPY . /app

RUN python -m venv /opt/venv && \
    /opt/venv/bin/pip install --upgrade pip && \
    /opt/venv/bin/pip install -r requirements.txt

CMD ["/opt/venv/bin/python", "manage.py", "runserver", "0.0.0.0:8000"]