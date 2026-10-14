// Adapted from https://github.com/hanabi1224/Programming-Language-Benchmarks/blob/71e9eb58cdbd6b3cb8568b1a135e0d75209ca318/bench/algorithm/http-server/2.go
// Changes: bind an OS-assigned port and set backlog 1024 before starting clients, post to /api, and close response bodies.
package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"net"
	"net/http"
	"os"
	"strconv"
	"syscall"
	"time"

	"github.com/valyala/fasthttp"
)

var client = http.Client{Timeout: 10 * time.Second}

func requestHandler(ctx *fasthttp.RequestCtx) {
	var payload Payload
	if json.Unmarshal(ctx.Request.Body(), &payload) == nil {
		fmt.Fprintf(ctx, "%d", payload.Value)
	}
}

func runServer(listener net.Listener) {
	if err := fasthttp.Serve(listener, requestHandler); err != nil {
		panic(err)
	}
}

func send(api string, value int, ch chan<- int) {
	ret := -1
	payloadBytes, _ := json.Marshal(Payload{Value: value})
	for {
		resp, err := client.Post(api, "", bytes.NewReader(payloadBytes))
		if err != nil {
			continue
		}
		if resp.StatusCode == http.StatusOK {
			_ = json.NewDecoder(resp.Body).Decode(&ret)
			_ = resp.Body.Close()
			ch <- ret
			return
		}
		_ = resp.Body.Close()
	}
}

func main() {
	n := 10
	if len(os.Args) > 1 {
		parsed, err := strconv.Atoi(os.Args[1])
		if err != nil || parsed < 0 {
			panic("size must be a non-negative integer")
		}
		n = parsed
	}
	listener, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		panic(err)
	}
	raw, err := listener.(*net.TCPListener).SyscallConn()
	if err != nil {
		panic(err)
	}
	// On Unix, listen on an existing listener updates its backlog. Set it
	// before starting the server or clients, retaining net.Listen's setup.
	var listenErr error
	if err := raw.Control(func(fd uintptr) {
		listenErr = syscall.Listen(int(fd), 1024)
	}); err != nil {
		panic(err)
	}
	if listenErr != nil {
		panic(listenErr)
	}
	port := listener.Addr().(*net.TCPAddr).Port
	go runServer(listener)
	api := fmt.Sprintf("http://localhost:%d/api", port)
	ch := make(chan int, n)
	for i := 1; i <= n; i++ {
		go send(api, i, ch)
	}
	sum := 0
	for i := 1; i <= n; i++ {
		sum += <-ch
	}
	fmt.Println(sum)
}

type Payload struct {
	Value int `json:"value"`
}
