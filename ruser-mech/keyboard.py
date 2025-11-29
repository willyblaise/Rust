import requests

BASE_URL = "http://localhost:3003/keyboards"

def get_keyboards():
    response = requests.get(BASE_URL)
    if response.status_code == 200:
        keyboards = response.json()
        keyboards = keyboards.get("data", [])
        print("\nKeyboards in DB:")
        for k in keyboards:
            print(k)
    else:
        print(f"Error fetching keyboards: {response.status_code} - {response.text}")

def post_keyboard():
    print("\nEnter keyboard details to add a new entry:")
    brand = input("Brand: ").strip()
    model = input("Model: ").strip()
    switch_type = input("Switch type: ").strip()
    key_count = input("Key count: ").strip()
    connection = input("Connection: ").strip()

    # Validate numeric input
    try:
        key_count = int(key_count)
    except ValueError:
        print("Error: Key count must be an integer.")
        return

    payload = {
        "brand": brand,
        "model": model,
        "switch_type": switch_type,
        "key_count": key_count,
        "connection": connection
    }

    response = requests.post(BASE_URL, json=payload)
    if response.status_code in (200, 201):
        print("Keyboard successfully added:")
        print(response.json())
    else:
        print(f"Error posting keyboard: {response.status_code} - {response.text}")

if __name__ == "__main__":
    while True:
        print("\n--- Keyboard API Client ---")
        print("1. List all keyboards")
        print("2. Add a new keyboard")
        print("3. Exit")
        choice = input("Choose an option: ").strip()

        if choice == "1":
            get_keyboards()
        elif choice == "2":
            post_keyboard()
        elif choice == "3":
            print("Exiting...")
            break
        else:
            print("Invalid choice, try again.")

