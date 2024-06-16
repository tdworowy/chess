import requests

if __name__ == "__main__":
    response = requests.get("http://localhost:8080/healthcheck")
    print(response.text)

    response = requests.post(
        "http://localhost:8080/make_move",
        headers={"Content-Type": "application/json"},
        data={
            "player": "Black",
            "board_state": [
                {id: "1_1", "content": ["Black", "Pawn"]},
                {id: "1_2", "content": ["White", "Dame"]},
            ],
        },
    )
    print(response.status_code)
    print(response.text)
