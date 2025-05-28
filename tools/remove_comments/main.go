package main

import (
	"log"
	"os"
	"path/filepath"
	"regexp"
	"strings"
)

func removeComments(input string) string {
	// Remove block comments
	re1 := regexp.MustCompilePOSIX(`(\n|^)[[:blank:]]*/\*(\n|.)*?\*/`)
	// Remove line comments
	re2 := regexp.MustCompilePOSIX(`(\n|^)[[:blank:]]*//+.*`)

	result := re1.ReplaceAllString(input, "\n")
	result = re2.ReplaceAllString(result, "\n")
	return result
}

func fmt(dir string) {
	contents, err := os.ReadDir(dir)
	if err != nil {
		log.Fatal(err)
	}
	for _, file := range contents {
		fpath := filepath.Join(dir, file.Name())
		if file.IsDir() {
			fmt(fpath)
			continue
		} else if strings.TrimRight(file.Name(), ".rs") == file.Name() {
			continue
		}
		content, err := os.ReadFile(fpath)
		if err != nil {
			log.Fatal(err)
		}
		cleaned := removeComments(string(content))
		f, err := os.CreateTemp(".", "file-XXXXXXXX.rs")
		if err != nil {
			log.Fatal(err)
		}
		os.WriteFile(f.Name(), []byte(cleaned), 0644)
		os.Rename(f.Name(), fpath)
	}

}

func main() {
	fmt("./src")
}
