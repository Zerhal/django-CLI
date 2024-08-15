use std::collections::BTreeMap;

pub fn get_compatible_django_version(python_version: &str) -> Option<&'static str> {
    // Mapping de la version de Django aux versions de Python compatibles
    let compatibility: BTreeMap<&str, Vec<&str>> = BTreeMap::from([
        ("5.0", vec!["3.10", "3.11", "3.12"]),
        ("4.2", vec!["3.8", "3.9", "3.10", "3.11", "3.12"]),
        ("4.1", vec!["3.8", "3.9", "3.10", "3.11"]),
        ("4.0", vec!["3.8", "3.9", "3.10"]),
        ("3.2", vec!["3.6", "3.7", "3.8", "3.9", "3.10"]),
    ]);

    // Extraire seulement la version majeure et mineure
    let normalized_version = python_version
        .split('.')
        .take(2)
        .collect::<Vec<&str>>()
        .join(".");

    // Convertir la version normalisée en `&str` pour la comparaison
    compatibility
        .iter()
        .rev()
        .find(|(_, python_versions)| python_versions.contains(&normalized_version.as_str()))
        .map(|(django_version, _)| *django_version)
}
