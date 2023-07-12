# Road Lurker
### About
This project aims to provide a map over all swedish traffic flow cameras. The idea is to show all the area covered by the cameras, and the ability to view images from said cameras. 

### Open questions
- [x] How to use openstreetmap, or some other alternative?
> See useful links #4
- [x] Which language should this project use?
> Javascript
- [ ] Trafikverkets api has camera info and images for Sweden. How to use?
> We have figured out and tested using the camera images,
> but we still need to figure out and test using camera info api
>> For use of image see *getting-started-leafletjs*
- [ ] How many cameras does Trafikverket have?
> We might be able to figure this out via webbkameror.se
> (see useful links #5), **but that might not be accurate**.
- [ ] How to map Trafikverkets cameras to background map?
> leafletjs allows us to place markers at cordinates,
> but we still need to figure out how to use Trafikverkets camera info API.
> *Trafikverket may use a difrent scheme for cordinates then leafletjs*

### Useful links
1. https://api.trafikinfo.trafikverket.se/
2. https://www.openstreetmap.org
3. https://api.trafikinfo.trafikverket.se/v1.3/Images/TrafficFlowCamera_39635266.Jpeg?type=fullsize&maxage=15
4. https://leafletjs.com/examples/quick-start/
5. http://www.webbkameror.se
