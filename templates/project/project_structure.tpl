project_name: "{{ name }}"
project_type: "{{ project_type | default(value="Not specified") }}"
package_manager: "{{ package_manager | default(value="Not specified") }}"
auth_system: "{{ auth_system | default(value="None") }}"
database:
  type: "{{ database.db_type | default(value="SQLite") }}"
  name: "{{ database.name | default(value="db") }}"
  user: "{{ database.user | default(value="user") }}"
  password: "{{ database.password | default(value="password") }}"
  host: "{{ database.host | default(value="localhost") }}"
  port: "{{ database.port | default(value="N/A") }}"

frontend_framework: "{{ frontend_framework | default(value="None") }}"

additional_options:
  {% if additional_options | length > 0 %}
  {% for option in additional_options %}
  - "{{ option | default(value="No additional options") }}"
  {% endfor %}
  {% else %}
  - "No additional options"
  {% endif %}
