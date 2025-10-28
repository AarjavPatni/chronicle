# Chapter 1 — Let’s Go

## Overview

This chapter introduces Go and its strengths for building distributed systems, motivating the use of Go for reliability, concurrency, and simplicity.
The reader builds a minimal **JSON-over-HTTP commit log service**, serving as the foundation for later chapters.

---

## 1. Why Go?

Go is highlighted as a pragmatic, fast, and elegant language that:

* Compiles quickly and runs efficiently (unlike interpreted languages).
* Handles concurrency well.
* Runs close to the hardware but with modern features.
* Excludes unnecessary complexity (like inheritance or generics at the time).

> “If Java is the cleaver and Bash the paring knife, then Go is the katana.”

Go’s impact is most visible in **distributed systems** such as Docker, Kubernetes, Etcd, and Prometheus—all written in Go.

---

## 2. JSON over HTTP in Distributed Systems

### Why JSON/HTTP?

* Universally supported and human-readable.
* Ideal for prototyping and external/public APIs.
* Many production systems (e.g., Elasticsearch, Etcd) expose JSON/HTTP interfaces.

### Why in Go?

Go’s standard library (`net/http`, `encoding/json`) simplifies building web APIs without external dependencies.

---

## 3. Setting Up the Project

### Create module

```bash
$ mkdir proglog
$ cd proglog
$ go mod init github.com/travisjeffery/proglog
```

---

## 4. Build a Commit Log Prototype

### `internal/server/log.go`

```go
package server

import (
    "fmt"
    "sync"
)

type Log struct {
    mu      sync.Mutex
    records []Record
}

func NewLog() *Log {
    return &Log{}
}

func (c *Log) Append(record Record) (uint64, error) {
    c.mu.Lock()
    defer c.mu.Unlock()
    record.Offset = uint64(len(c.records))
    c.records = append(c.records, record)
    return record.Offset, nil
}

func (c *Log) Read(offset uint64) (Record, error) {
    c.mu.Lock()
    defer c.mu.Unlock()
    if offset >= uint64(len(c.records)) {
        return Record{}, ErrOffsetNotFound
    }
    return c.records[offset], nil
}

type Record struct {
    Value  []byte `json:"value"`
    Offset uint64 `json:"offset"`
}

var ErrOffsetNotFound = fmt.Errorf("offset not found")
```

---

## 5. Build a JSON-over-HTTP Server

### `internal/server/http.go`

```go
package server

import (
    "encoding/json"
    "net/http"
    "github.com/gorilla/mux"
)

func NewHTTPServer(addr string) *http.Server {
    httpsrv := newHTTPServer()
    r := mux.NewRouter()
    r.HandleFunc("/", httpsrv.handleProduce).Methods("POST")
    r.HandleFunc("/", httpsrv.handleConsume).Methods("GET")
    return &http.Server{
        Addr:    addr,
        Handler: r,
    }
}

type httpServer struct {
    Log *Log
}

func newHTTPServer() *httpServer {
    return &httpServer{Log: NewLog()}
}

type ProduceRequest struct {
    Record Record `json:"record"`
}
type ProduceResponse struct {
    Offset uint64 `json:"offset"`
}
type ConsumeRequest struct {
    Offset uint64 `json:"offset"`
}
type ConsumeResponse struct {
    Record Record `json:"record"`
}
```

### Handlers

```go
func (s *httpServer) handleProduce(w http.ResponseWriter, r *http.Request) {
    var req ProduceRequest
    err := json.NewDecoder(r.Body).Decode(&req)
    if err != nil {
        http.Error(w, err.Error(), http.StatusBadRequest)
        return
    }
    off, err := s.Log.Append(req.Record)
    if err != nil {
        http.Error(w, err.Error(), http.StatusInternalServerError)
        return
    }
    res := ProduceResponse{Offset: off}
    err = json.NewEncoder(w).Encode(res)
    if err != nil {
        http.Error(w, err.Error(), http.StatusInternalServerError)
    }
}

func (s *httpServer) handleConsume(w http.ResponseWriter, r *http.Request) {
    var req ConsumeRequest
    err := json.NewDecoder(r.Body).Decode(&req)
    if err != nil {
        http.Error(w, err.Error(), http.StatusBadRequest)
        return
    }
    record, err := s.Log.Read(req.Offset)
    if err == ErrOffsetNotFound {
        http.Error(w, err.Error(), http.StatusNotFound)
        return
    }
    if err != nil {
        http.Error(w, err.Error(), http.StatusInternalServerError)
        return
    }
    res := ConsumeResponse{Record: record}
    err = json.NewEncoder(w).Encode(res)
    if err != nil {
        http.Error(w, err.Error(), http.StatusInternalServerError)
    }
}
```

---

## 6. Running the Server

### `cmd/server/main.go`

```go
package main

import (
    "log"
    "github.com/travisjeffery/proglog/internal/server"
)

func main() {
    srv := server.NewHTTPServer(":8080")
    log.Fatal(srv.ListenAndServe())
}
```

Run it:

```bash
$ go run cmd/server/main.go
```

---

## 7. Testing the API

### Append records

```bash
$ curl -X POST localhost:8080 -d '{"record": {"value": "TGV0J3MgR28gIzEK"}}'
$ curl -X POST localhost:8080 -d '{"record": {"value": "TGV0J3MgR28gIzIK"}}'
$ curl -X POST localhost:8080 -d '{"record": {"value": "TGV0J3MgR28gIzMK"}}'
```

*(Base64 encodes “Let’s Go #1”, “#2”, “#3”)*

### Read records

```bash
$ curl -X GET localhost:8080 -d '{"offset": 0}'
$ curl -X GET localhost:8080 -d '{"offset": 1}'
$ curl -X GET localhost:8080 -d '{"offset": 2}'
```

---

## 8. Key Learnings

* Built a **simple JSON/HTTP service** that stores records in an in-memory log.
* Understood **commit logs** as append-only structures indexed by offset.
* Learned to:

  * Implement HTTP handlers in Go.
  * Marshal/unmarshal JSON efficiently.
  * Build APIs using Gorilla Mux.
* Prepared for transitioning to **Protocol Buffers** and **gRPC** in the next chapter.

---

**Next Chapter:** *Structure Data with Protocol Buffers* — introduces Protobuf for schema definition, code generation, and gRPC readiness.

---

**Source:** *Distributed Services with Go*, Chapter 1 “Let’s Go” 

