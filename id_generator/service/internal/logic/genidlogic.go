package logic

import (
	"context"
	gonanoid "github.com/matoous/go-nanoid/v2"
	"github.com/zeromicro/go-zero/core/logx"
	"id_generator/service/id_generator"
	"id_generator/service/internal/svc"
)

type GenIdLogic struct {
	ctx    context.Context
	svcCtx *svc.ServiceContext
	logx.Logger
}

func NewGenIdLogic(ctx context.Context, svcCtx *svc.ServiceContext) *GenIdLogic {
	return &GenIdLogic{
		ctx:    ctx,
		svcCtx: svcCtx,
		Logger: logx.WithContext(ctx),
	}
}

var alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789="

func (l *GenIdLogic) GenId(in *id_generator.IdGenRequest) (*id_generator.IdGenResponse, error) {
	if id, err := gonanoid.Generate(alphabet, 5); err != nil {
		return nil, err
	} else {
		return &id_generator.IdGenResponse{
			Id: id,
		}, nil
	}
}
