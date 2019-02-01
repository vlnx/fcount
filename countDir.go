package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"os"
)

func listFolder(folderDir string) []os.FileInfo {
	files, err := ioutil.ReadDir(folderDir)
	if err != nil {
		log.Output(0, err.Error())
	}
	return files
}

func countDir(dir string, count *int64) {
	files := listFolder(dir)
	for _, i := range files {
		if i.IsDir() {
			countDir(dir+i.Name()+"/", count)
		} else {
			*count++
		}
	}
}

func main() {
	var dir string = os.Args[1]
	if dir[len(dir)-1] != '/' {
		dir = dir + "/"
	}
	fmt.Println("Scanning directory:", dir)

	var count int64 = 0
	countDir(dir, &count)
	fmt.Printf("Count: %d", count)
}
