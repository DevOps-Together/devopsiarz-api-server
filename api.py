from flask import Flask, request, jsonify

app = Flask(__name__)

wiadomosci = [
    {"id": 1, "name": "Ala ma kota"},
    {"id": 2, "name": "Kot ma Ale"},
    {"id": 3, "name": "Kot ma kota"},
]

def _find_next_id():
    return max(wiadomosc["id"] for wiadomosc in wiadomosci) + 1

@app.get("/wiadomosc")
def get_wiadomosc():
    return jsonify(wiadomosci)

@app.post("/wiadomosc")
def add_wiadomosc():
    if request.is_json:
        wiadomosc = request.get_json()
        wiadomosc["id"] = _find_next_id()
        wiadomosci.append(wiadomosc)
        return wiadomosc, 201
    return {"error": "Request must be JSON"}, 415
