var map = L.map('map').setView([51.505, -0.09], 13);

L.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
    maxZoom: 19,
    attribution: '&copy; <a href="http://www.openstreetmap.org/copyright">OpenStreetMap</a>'
}).addTo(map);

const EARTH_R = 6378000
function toRadians(angle){return angle * (180/Math.PI);}
function calculateNewLat(lat,addMeters){
    return lat + (addMeters/EARTH_R) * (180/Math.PI);
}
function calculateNewLong(lat,long,addMeters){
    return long + (addMeters/EARTH_R) * (180/Math.PI) / Math.cos(lat*Math.PI/180);
}

function markCamera(lat,long,ang,link){
    function calculateCoordinates(lat,long,ang){
        ang=toRadians(ang)
        const RANGE = 500;
        const CONE_ANGLE = toRadians(70)
        var coneSide = Math.abs(RANGE/Math.cos(CONE_ANGLE/2))
        var gamma = toRadians(90)-(CONE_ANGLE/2)-ang
        var p1xdx = Math.sin(gamma)*coneSide
        var p1ydy = Math.cos(gamma)*coneSide
        console.log(p1xdx)
        var p1x = calculateNewLat(lat,p1xdx)
        var p1y = calculateNewLong(lat,long,p1ydy)
        var zulu = gamma-(CONE_ANGLE/2)
        var p2xdx = Math.sin(zulu)*coneSide
        var p2ydy = Math.cos(zulu)*coneSide
        var p2x = calculateNewLat(lat,p2xdx)
        var p2y = calculateNewLong(lat,long,p2ydy)
        return [[p1x,p1y],[p2x,p2y]]
    }
    var cords=calculateCoordinates(lat,long,ang)
    L.polygon([
        [lat, long],
        [cords[0][0], cords[0][1]],
        [cords[1][0], cords[1][1]]
    ]).addTo(map);
    var marker = L.marker([lat, long]).addTo(map);
    marker.bindPopup("<a href="+link+"><img src="+link+"></a>");
}

async function run(){
    const response = await fetch("data/cameras.json")
    const data = await response.json()

    for(let i=0 ; data.cameras.length; i++){
        let cam = data.cameras[i];
        markCamera(
            cam.latitude,
            cam.longitude,
            cam.direction,
            cam.url
        )
    }
}