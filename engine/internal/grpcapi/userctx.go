package grpcapi

import (
	"context"

	"google.golang.org/grpc"
	"google.golang.org/grpc/metadata"
)

// UserContext carries user identity extracted from gRPC metadata.
type UserContext struct {
	UserID string
	Email  string
}

type userContextKey struct{}

// UserFromContext extracts the UserContext from a context, if present.
func UserFromContext(ctx context.Context) *UserContext {
	if uc, ok := ctx.Value(userContextKey{}).(*UserContext); ok {
		return uc
	}
	return nil
}

// UserContextInterceptor extracts x-user-id and x-user-email from gRPC
// metadata and stores them in the context for downstream handlers.
func UserContextInterceptor() grpc.UnaryServerInterceptor {
	return func(
		ctx context.Context,
		req interface{},
		info *grpc.UnaryServerInfo,
		handler grpc.UnaryHandler,
	) (interface{}, error) {
		md, ok := metadata.FromIncomingContext(ctx)
		if ok {
			uc := &UserContext{}
			if vals := md.Get("x-user-id"); len(vals) > 0 {
				uc.UserID = vals[0]
			}
			if vals := md.Get("x-user-email"); len(vals) > 0 {
				uc.Email = vals[0]
			}
			if uc.UserID != "" {
				ctx = context.WithValue(ctx, userContextKey{}, uc)
			}
		}
		return handler(ctx, req)
	}
}

// UserContextStreamInterceptor is the streaming equivalent.
func UserContextStreamInterceptor() grpc.StreamServerInterceptor {
	return func(
		srv interface{},
		ss grpc.ServerStream,
		info *grpc.StreamServerInfo,
		handler grpc.StreamHandler,
	) error {
		ctx := ss.Context()
		md, ok := metadata.FromIncomingContext(ctx)
		if ok {
			uc := &UserContext{}
			if vals := md.Get("x-user-id"); len(vals) > 0 {
				uc.UserID = vals[0]
			}
			if vals := md.Get("x-user-email"); len(vals) > 0 {
				uc.Email = vals[0]
			}
			if uc.UserID != "" {
				// For streaming, we wrap the stream with an augmented context
				ss = &wrappedServerStream{
					ServerStream: ss,
					ctx:          context.WithValue(ctx, userContextKey{}, uc),
				}
			}
		}
		return handler(srv, ss)
	}
}

// wrappedServerStream wraps grpc.ServerStream to override Context().
type wrappedServerStream struct {
	grpc.ServerStream
	ctx context.Context
}

func (w *wrappedServerStream) Context() context.Context {
	return w.ctx
}
