package main

import (
	"context"

	"github.com/Noah-Huppert/control/server/api"
	"github.com/Noah-Huppert/control/server/config"
	"github.com/Noah-Huppert/control/server/libdb"

	"github.com/Noah-Huppert/golog"
)

func main() {
	// Context
	ctx := context.Background()

	// Logger
	logger := golog.NewStdLogger("control")

	// Load configuration
	cfg, err := config.NewConfig()
	if err != nil {
		logger.Fatalf("error loading configuration: %s", err.Error())
	}

	// Database
	db, err := libdb.ConnectX(cfg.DBConfig)
	if err != nil {
		logger.Fatalf("error connecting to database: %s", err.Error())
	}

	logger.Debugf("db: %#v", db)

	// API server
	server := api.NewServer(*cfg, logger, db)

	logger.Debugf("starting api server on :%d", cfg.Port)
	err = api.StartServer(ctx, server)

	if err != nil {
		logger.Fatalf("error running api server: %s", err.Error())
	}

	logger.Debug("Done")
}
