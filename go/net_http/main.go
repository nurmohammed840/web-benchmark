package main

import (
    "fmt"
    "net/http"
)

func main() {
	fmt.Println("net/http (Go), http://localhost:8080/")
    http.HandleFunc("/", HelloServer)
    http.ListenAndServe(":8080", nil)
}

func HelloServer(w http.ResponseWriter, r *http.Request) {
    fmt.Fprintf(w, "Hello, World!")
}