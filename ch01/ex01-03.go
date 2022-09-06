package main

import (
	"fmt"
	"time"
	"os"
	"strings"
)

func main() {
	start := time.Now()
	echo1()
	fmt.Println("%.2fs elapsed\n", time.Since(start).Seconds())
	
	start = time.Now()
	echo3()
	fmt.Println("%.2fs elapsed\n", time.Since(start).Seconds())
}

func echo1() {
	s, sep := "", ""
	for _, arg := range os.Args {
		s += arg + sep
		sep = " "
	}
	fmt.Println(s)
}

func echo3() {
	fmt.Println(strings.Join(os.Args, " "))
}
