import requests

if __name__ == "__main__":
    response = requests.get("http://localhost:8080/healthcheck")
    print(response.text)

    response = requests.get("http://localhost:8080/get_example")
    print(response.text)

    data = {"player": "Black", "board_stat": {"1_1": ["Black", "Pawn"]}}
    response = requests.post(
        "http://localhost:8080/make_move",
        headers={"Content-Type": "application/json"},
        data=data,
    )
    print(response.status_code)
    print(response.text) # Json deserialize error: expected value at line 1 column 1
