# Python
__pycache__/
*.py[cod]
*$py.class

# Django
*.log
local_settings.py
db.sqlite3
db.sqlite3-journal
media

# Environnement virtuel
{{ venv_name | default(value="Venv") }}/
ENV/

# IDE
.vscode/
.idea/

# Système d'exploitation
.DS_Store
Thumbs.db

# Fichiers de configuration
*.env
*.env.*

# Fichiers de déploiement
*.pyc
*.pyo
*.pyd
.Python
build/
develop-eggs/
dist/
downloads/
eggs/
.eggs/
lib/
lib64/
parts/
sdist/
var/
wheels/
share/python-wheels/
*.egg-info/
.installed.cfg
*.egg
MANIFEST