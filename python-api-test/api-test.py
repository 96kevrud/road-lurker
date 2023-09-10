import sys
import requests

api_key = "YOUR_API_KEY"
url = "https://api.trafikinfo.trafikverket.se/v2/data.json"
headers = {'Content-Type':'text/xml'}

with open("post.xml") as xml:
    response = requests.post(
        url=url,
        data=xml,
        headers=headers
    )
    if str(responseCode := response.status_code) == "2":
        print("Success code:",responseCode)
    else:
        print("Request failed code:",responseCode)
        sys.exit("Did not get a 2xx response")
    result = response.json()["RESPONSE"]["RESULT"][0]
    cameras = result["Camera"]
    print(cameras[0])
    print("--------------")
    print(f"Amount of active traffic cameras: {len(cameras)}")