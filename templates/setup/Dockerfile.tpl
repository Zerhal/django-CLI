# Dockerfile

FROM python:{{ python_version | default(value="3.9") }}-slim

# Définir le répertoire de travail dans le conteneur
WORKDIR /app

# Copier les fichiers de dépendances en premier pour tirer parti du cache Docker
COPY requirements.txt .

# Installer les dépendances dans un environnement virtuel
RUN python -m venv /opt/venv && \
    /opt/venv/bin/pip install --upgrade pip && \
    /opt/venv/bin/pip install -r requirements.txt

# Copier tout le reste du code
COPY . .

# Exposer le port (optionnel)
EXPOSE 8000

# Définir la commande de démarrage
CMD ["/opt/venv/bin/python", "manage.py", "runserver", "0.0.0.0:8000"]
