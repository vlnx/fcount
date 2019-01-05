package main

import (
	"fmt"
	"io/ioutil"
	"bufio"
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
	scanner := bufio.NewScanner(os.Stdin)
	fmt.Println("Enter directory:")
	scanner.Scan()
	dir = scanner.Text()
	fmt.Println("Scanning directory:", dir)

	var count int64 = 0
	countDir(dir, &count)
	log.Output(0, fmt.Sprintf("Count: %d", count))
}
