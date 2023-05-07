package gotokenizer

import (
	_ "embed"
	"os"
	"path/filepath"

	"github.com/wailovet/easycgo"
)

//go:embed c_tokenizers.dll
var dllFile []byte

func Install() {
	tmpDir := os.TempDir()
	os.WriteFile(filepath.Join(tmpDir, "c_tokenizers.dll"), dllFile, 0666)
	_tokenizers = easycgo.MustLoad(filepath.Join(tmpDir, "c_tokenizers.dll"))
	C_tokenizer_from_str = _tokenizers.MustFind("C_tokenizer_from_str")
	C_tokenizer_from_file = _tokenizers.MustFind("C_tokenizer_from_file")
	C_tokenizer_encode_ids = _tokenizers.MustFind("C_tokenizer_encode_ids")
	C_tokenizer_decode = _tokenizers.MustFind("C_tokenizer_decode")
}
