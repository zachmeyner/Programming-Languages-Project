// I am writing this one after rust, so all my thoughts on what I am doing here
// is based entirely on my only experience making web servers only in rust.
// Everything I say will be a comparison to rust

package main

// Imports are very nice in go. This is a hard comparison vs Rust
import (
	"flag"
	"log"
	"net/http"
	"strconv"

	"io/ioutil"

	"github.com/aymerick/raymond"
	"github.com/gorilla/mux"
)

// I enjoy that serialization for structs are declared right in the struct,
// and you can define the json serialization of the struct to be different than the variable name
type Media struct {
	Pic      bool   `json:"pic"`
	Location string `json:"location"`
	Date     string `json:"date"`
	Id       uint32 `json:"id"`
	Desc     string `json:"desc"`
}

// Substitute for database cause again, not setting that up for this.
var MYLO_MEDIA = []Media{
	{
		Pic:      true,
		Location: "mylo/01.jpeg",
		Date:     "March 2, 2019",
		Id:       0,
		Desc:     "I thought he looked really funny in this. One of my favorite Mylo photos of all time.",
	},
	{
		Pic:      true,
		Location: "mylo/02.jpeg",
		Date:     "March 1, 2019",
		Id:       1,
		Desc:     "I don't know much about this one. I thought it was cute the way he laId against my leg.",
	},
	{
		Pic:      true,
		Location: "mylo/03.jpg",
		Date:     "September 3, 2018",
		Id:       2,
		Desc:     "Mylo really likes sleeping near the shoes during weekdays. I do not know why.",
	},
	{
		Pic:      true,
		Location: "mylo/04.jpeg",
		Date:     "July 8, 2018",
		Id:       3,
		Desc:     "He hates the basket, but he decIded on this day to make peace with the basked.",
	},
	{
		Pic:      true,
		Location: "mylo/05.jpg",
		Date:     "July 21, 2018",
		Id:       4,
		Desc:     "Another time I thought he looked really cute laying up against my leg.",
	},
	{
		Pic:      true,
		Location: "mylo/06.jpg",
		Date:     "June 7, 2018",
		Id:       5,
		Desc:     "He enjoys laying in the during weekdays when no one is home. THe basked was in his way but he made it work.",
	},
	{
		Pic:      true,
		Location: "mylo/07.jpg",
		Date:     "July 7, 2021",
		Id:       6,
		Desc:     "Mylo really likes sleeping near the shoes during weekdays. I do not know why.",
	},
	{
		Pic:      true,
		Location: "mylo/08.jpg",
		Date:     "May 21, 2018",
		Id:       7,
		Desc:     "He was feeling a little sick around this time and would come upstairs to where I used to my computer to stay near me.",
	},
	{
		Pic:      true,
		Location: "mylo/09.jpg",
		Date:     "Mar 14, 2018",
		Id:       8,
		Desc:     "We have to keep blankets and pillow on the chair when he's alone cause he likes to tear up the chair. He's taken advantage of that to his comfort here.",
	},
	{
		Pic:      true,
		Location: "mylo/10.jpg",
		Date:     "August 3, 2021",
		Id:       9,
		Desc:     "He likes to lay in the sunlight during weekdays when no one is home. He does not care if anything is in his way.",
	},
	{
		Pic:      true,
		Location: "mylo/11.jpg",
		Date:     "July 27, 2021",
		Id:       10,
		Desc:     "He was sleeping next to the socks.",
	},
	{
		Pic:      true,
		Location: "mylo/12.jpg",
		Date:     "June 26, 2021",
		Id:       11,
		Desc:     "He is very sleepy during the day. He has the whole night to sleep and always ends up like this.",
	},
	{
		Pic:      true,
		Location: "mylo/13.jpg",
		Date:     "April 27, 2021",
		Id:       12,
		Desc:     "Once again leveraging the pillow and blanket on chair.",
	},
	{
		Pic:      true,
		Location: "mylo/15.jpg",
		Date:     "June 29, 2019",
		Id:       13,
		Desc:     "He likes to sleep on shoes.",
	},
	{
		Pic:      true,
		Location: "mylo/16.jpg",
		Date:     "May 15, 2019",
		Id:       14,
		Desc:     "He's very cute while yawning.",
	},
	{
		Pic:      true,
		Location: "mylo/17.jpg",
		Date:     "May 15, 2019",
		Id:       15,
		Desc:     "He does not often use his bed, but when he does he makes sure to stare at you to let you know that he could be laying down with you istead.",
	},
	{
		Pic:      true,
		Location: "mylo/18.jpg",
		Date:     "April 14, 2019",
		Id:       16,
		Desc:     "He likes to sleep on peoples chests.",
	},
	{
		Pic:      true,
		Location: "mylo/19.jpg",
		Date:     "April 8, 2019",
		Id:       17,
		Desc:     "There was no sunlight for him to lay in on this day, so he just slept in the bed.",
	},
	{
		Pic:      true,
		Location: "mylo/20.jpg",
		Date:     "February 26, 2021",
		Id:       18,
		Desc:     "He looks very comfy as he lays with all these blanks around him.",
	},
	{
		Pic:      false,
		Location: "mylo/01.mp4",
		Date:     "4/30/2018",
		Id:       19,
		Desc:     "He would always get very excited when I got home from school. He was often much more excited than this when I would get home.",
	},
	{
		Pic:      false,
		Location: "mylo/02.mp4",
		Date:     "4/12/2019",
		Id:       20,
		Desc:     "He makes funny sounds when he likes the way that you are petting him.",
	},
}

// For a web server in go you run it out of main.
// In main you are declaring what you locations you're serving, what you're serving there,
// and what port it should all be on.
func main() {
	router := mux.NewRouter().StrictSlash(true)

	port := flag.String("p", "8000", "port to serve on")
	mediaDir := flag.String("m", "../../../media", "media static files to serve")
	stylesDir := flag.String("s", "../templates/", "media static files to serve")

	fs1 := http.FileServer(http.Dir(*mediaDir))
	fs2 := http.FileServer(http.Dir(*stylesDir))

	// To initialize the fileserver and HTTP request handler is a bit more tricky than it is in rust
	router.HandleFunc("/", index).Methods("GET")
	router.HandleFunc("/dispmedia/{id}", media_page).Methods("GET")
	router.PathPrefix("/photos/").Handler(http.StripPrefix("/photos", fs1))
	router.PathPrefix("/styles/").Handler(http.StripPrefix("/styles", fs2))

	log.Printf("Serving %s and %s on HTTP port %s\n", *mediaDir, *stylesDir, *port)

	// Starting the server is way easier than in rust for sure though.
	log.Fatal(http.ListenAndServe(":"+*port, router))
}

// http functions are very different from rust.
// In rust you return from the function what you are returning to the user,
// but here the response and request are both in the function header.
func index(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusOK)
	w.Header().Set("Content-Type", "text/html; charset=utf-8")

	tpl, err := ioutil.ReadFile("../templates/index.html.hbs")
	if err != nil {
		panic("Oh no 1")
	}

	result, err := raymond.Render(string(tpl), MYLO_MEDIA)
	if err != nil {
		panic("Oh no 2")
	}
	w.Write([]byte(result))
}

func media_page(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	id, ok := vars["id"]

	if !ok {
		w.Write([]byte("You forgot the image number"))
	}

	args, err := strconv.Atoi(id)

	w.WriteHeader(http.StatusOK)
	w.Header().Set("Content-Type", "text/html; charset=utf-8")

	tpl, err := ioutil.ReadFile("../templates/picture.html.hbs")
	if err != nil {
		panic("Oh no 1")
	}

	result, err := raymond.Render(string(tpl), MYLO_MEDIA[args])
	if err != nil {
		panic("Oh no 2")
	}
	w.Write([]byte(result))
}
