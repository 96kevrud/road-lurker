import requests

api_key = "YOUR_API_KEY"

url = "https://api.trafikinfo.trafikverket.se/v2/data.json"
headers = {'Content-Type':'text/xml'}

with open("post.xml") as xml_file:
    xml_string = xml_file.read()
    xml_string = xml_string.replace("APIKEY", api_key)

    response = requests.post(
        url=url,
        data=xml_string,
        headers=headers
    )

    print(response.status_code)
    result = response.json()["RESPONSE"]["RESULT"][0]
    cameras = result["Camera"]
    print(cameras[0])
    print("--------------")
    print(f"Amount of active traffic cameras: {len(cameras)}")