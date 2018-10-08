package main

import (
	"github.com/Noah-Huppert/control/server/config"
	"github.com/Noah-Huppert/control/server/libdb"

	"github.com/Noah-Huppert/golog"
)

func main() {
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

	logger.Debugf("OK!: %#v", db)
}
