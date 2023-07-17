import requests

url = "https://api.trafikinfo.trafikverket.se/v2/data.json"
headers = {'Content-Type':'text/xml'}

with open("post.xml") as xml:
    response = requests.post(
        url=url,
        data=xml,
        headers=headers
    )
    print(response.status_code)
    result = response.json()["RESPONSE"]["RESULT"][0]
    cameras = result["Camera"]
    print(cameras[0])
    print("--------------")
    print(f"Amount of active traffic cameras: {len(cameras)}")