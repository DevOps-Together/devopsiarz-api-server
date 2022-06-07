**Uruchomienie:**

export FLASK_APP=api.py

export FLASK_ENV=development

flask run


**Wyświetlanie danych:**

curl -i localhost:5000/wiadomosc

**Dodawanie danych:**

curl -i localhost:5000/wiadomosc -X POST -H 'Content-Type: application/json' -d '{"name":"Ala nie ma kota"}'
