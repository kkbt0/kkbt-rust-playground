import json
import requests

# pocketbase 迁移
file = open("nav.json", "r")
data = json.load(file)
file.close()

url = "https://api.ftls.xyz/api/collections/nav/records"
for iter in data["items"]:
    data = {
        "title": iter["title"],
        "icon": iter["icon"],
        "description": iter["description"],
        "url": iter["url"],
    }
    response = requests.post(url=url,json=data)


