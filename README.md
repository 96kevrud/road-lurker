# Road Lurker
### About
This project aims to provide a map over all swedish traffic flow cameras. The idea is to show all the area covered by the cameras, and the ability to view images from said cameras. 

### Open questions
- [x] How to use openstreetmap, or some other alternative?
> See useful links #4
- [x] Which language should this project use?
> Javascript
- [x] Trafikverkets api has camera info and images for Sweden. How to use?
> Api seems easy to use, see section *Using trafikverkets api* below.

- [x] How many cameras does Trafikverket have? **1549** active cameras
>
- [ ] How to map Trafikverkets cameras to background map?
> leafletjs allows us to place markers at cordinates,
> but we still need to figure out how to use Trafikverkets camera info API.
> *Trafikverket may use a difrent scheme for cordinates then leafletjs*

### Using trafikverkets api
The api wants an xml style post request, see python-api-test for an example.
All camera images seems to have 'HasFullSizePhoto' attribute which is nice.
About half of all cameras has an overhead sketch of camera location and rotation, might be useful for double checking our camera rotations later on. 
Exmaple:
 * https://api.trafikinfo.trafikverket.se/v2/Images/TrafficFlowCamera_39635330.Jpeg
 * https://api.trafikinfo.trafikverket.se/v2/Images/TrafficFlowCamera_39635330.Jpeg?type=sketch
 * https://api.trafikinfo.trafikverket.se/v2/Images/TrafficFlowCamera_39635330.Jpeg?type=fullsize


### Preparations
Place your trafikverket apikey in a `.apikey` file next to the DockerFile

### Useful links
1. https://api.trafikinfo.trafikverket.se/
2. https://www.openstreetmap.org
3. https://api.trafikinfo.trafikverket.se/v1.3/Images/TrafficFlowCamera_39635266.Jpeg?type=fullsize&maxage=15
4. https://leafletjs.com/examples/quick-start/
5. http://www.webbkameror.se
