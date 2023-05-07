package main

import (
	"log"

	"github.com/wailovet/gotokenizer-bin/gotokenizer"
)

func main() {
	gotokenizer.Install()

	tokenizer := gotokenizer.NewTokenizerFromFile("tokenizer.json")

	if tokenizer == nil {
		return
	}

	outputRet := tokenizer.EncodeIds("hello", true)
	log.Println(outputRet)

	deret := tokenizer.Decode(outputRet, true)

	log.Println(deret)
}
