#!/usr/bin/env sh

#!/bin/bash

RUNS=10  # Nombre de répétitions pour moyenner

echo "=== Benchmark make vs rustake ==="
echo ""

# --- Premier build (cold) ---
echo ">> Build initial (sans cache)"

make clean > /dev/null 2>&1
rustake clean > /dev/null 2>&1

echo -n "make  (cold): "
{ time make > /dev/null 2>&1 ; } 2>&1 | grep real

echo -n "rustake (cold): "
{ time rustake > /dev/null 2>&1 ; } 2>&1 | grep real

# --- Build sans changement (warm) ---
echo ""
echo ">> Re-build sans modification (warm cache)"

echo -n "make  (warm): "
{ time make > /dev/null 2>&1 ; } 2>&1 | grep real

echo -n "rustake (warm): "
{ time rustake > /dev/null 2>&1 ; } 2>&1 | grep real

# --- Après modification d'un seul fichier ---
echo ""
echo ">> Re-build après modification de math_utils.c"

touch src/math_utils.c  # make utilise le timestamp
# Pour rustake : modifier réellement le fichier
echo "// change" >> src/math_utils.c

echo -n "make  (partial): "
{ time make > /dev/null 2>&1 ; } 2>&1 | grep real

echo -n "rustake (partial): "
{ time rustake > /dev/null 2>&1 ; } 2>&1 | grep real
