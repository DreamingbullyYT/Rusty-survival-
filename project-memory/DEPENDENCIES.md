# Dependency Map

## Stare curentă

Necompletat până la importul și auditul sursei upstream.

## Reguli

- Orice dependență trebuie să aibă motiv și direcție documentate.
- Evităm ciclurile între sisteme.
- Dependențele către implementări concrete se justifică; preferăm interfețe interne existente când acestea sunt potrivite.
- Nu duplicăm date, cache-uri sau mecanisme de sincronizare fără o decizie explicită.

## Înregistrare

Pentru fiecare muchie: `consumer -> provider`, motiv, tipul contractului, riscuri, teste și excepții.
