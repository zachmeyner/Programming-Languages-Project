package main

import (
	"fmt"
	"log"
	"net/http"

	"github.com/gorilla/mux"
)

func main() {
	router := mux.NewRouter().StrictSlash(true)

	router.HandleFunc("/", index).Methods("GET")
	router.HandleFunc("/{number}", response).Methods("GET")
	log.Fatal(http.ListenAndServe(":8000", router))
}

func index(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintln(w, "Hi mom :)")
}

func response(w http.ResponseWriter, r *http.Request) {
	params := mux.Vars(r)
	who := params["number"]
	fmt.Fprintln(w, who)
}
