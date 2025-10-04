#!/bin/bash

# Gehe in das Hauptverzeichnis, wo deine Assignments geklont sind
ASSIGNMENTS_ROOT_DIR="." # Beispiel: ~/Dokumente/Classroom-Assignments

cd "$ASSIGNMENTS_ROOT_DIR" || { echo "Verzeichnis nicht gefunden: $ASSIGNMENTS_ROOT_DIR"; exit 1; }

echo "Starte Pull-Vorgang in allen Assignment-Verzeichnissen unter: $PWD"

# Durchlaufe alle Unterverzeichnisse
for repo_dir in */ ; do
    # Überprüfe, ob es ein gültiges Git-Repository ist
    if [ -d "$repo_dir/.git" ]; then
        echo "========================================"
        echo "Bearbeite Repository: $repo_dir"
        
        # Wechsle in das Repository-Verzeichnis
        cd "$repo_dir"
        
        # Führe git pull aus, um die neuesten Änderungen zu fetchen und zu mergen
        # '-q' für quiet (leise), damit nur Fehler ausgegeben werden (optional)
        git fetch origin
        
        # Gehe zurück zum Wurzelverzeichnis
        cd ..
    else
        echo "Überspringe $repo_dir – kein Git-Repository gefunden."
    fi
done

echo "========================================"
echo "Alle Repositories wurden aktualisiert."
