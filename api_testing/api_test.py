import requests

if __name__ == "__main__":
    response = requests.get("http://localhost:8080/healthcheck")
    print(response.text)

    response = requests.get("http://localhost:8080/get_example")
    print(response.text)

    data = {"player":"Black","board_state":{"1_1":{"pawn_color":"Black","pawn_type":"Pawn"}}}
    response = requests.post(
        "http://localhost:8080/make_move",
        headers={"Content-Type": "application/json"},
        json=data,
    )
    print(response.status_code)
    print(response.text)

# TODO rewrite as pytest tests