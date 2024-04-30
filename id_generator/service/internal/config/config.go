package config

import "github.com/zeromicro/go-zero/zrpc"

type Config struct {
	zrpc.RpcServerConf
	IdLength int    `json:",env=ID_LENGTH,default=5"`
	Alphabet string `json:",env=ALPHABET,default=abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"`
}
