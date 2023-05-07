package gotokenizer

import "github.com/wailovet/easycgo"

var _tokenizers *easycgo.EasyCgo
var (
	BufferSize             = 10240
	C_tokenizer_from_str   *easycgo.EasyCgoProc
	C_tokenizer_from_file  *easycgo.EasyCgoProc
	C_tokenizer_encode_ids *easycgo.EasyCgoProc
	C_tokenizer_decode     *easycgo.EasyCgoProc
)

type Tokenizer struct {
	ctx easycgo.ValueInf
}

func NewTokenizerFromStr(tokenizerJson string) *Tokenizer {
	ctx := C_tokenizer_from_str.Call(tokenizerJson)
	if ctx.IsNil() {
		return nil
	}
	return &Tokenizer{
		ctx: ctx,
	}
}

func NewTokenizerFromFile(filename string) *Tokenizer {
	ctx := C_tokenizer_from_file.Call(filename)
	if ctx.IsNil() {
		return nil
	}
	return &Tokenizer{
		ctx: ctx,
	}
}

func (t *Tokenizer) EncodeIds(text string, addSpecialTokens bool) []uint32 {
	output := make([]uint32, BufferSize)
	s := C_tokenizer_encode_ids.Call(t.ctx, text, addSpecialTokens, output, BufferSize)
	return output[:s.ToInt()]
}

func (t *Tokenizer) Decode(ids []uint32, skipSpecialTokens bool) string {
	ret := C_tokenizer_decode.Call(t.ctx, ids, len(ids), skipSpecialTokens)
	return ret.ToString()
}
