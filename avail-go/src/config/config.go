package config

import (
    "encoding/json"
    "flag"
    "fmt"
    "io"
    "log"
    "os"
)

type Config struct {
    Seed        string `json:"seed"`
    ApiURL      string `json:"api_url"`
    Size        int    `json:"size"`
    AppID       int    `json:"app_id"`
    Dest        string `json:"dest"`
    Amount      uint64 `json:"amount"`
    SessionKeys string `json:"session_keys"`
}

func (c *Config) GetConfig(configFileName string) error {
    jsonFile, err := os.Open(configFileName)
    if err != nil {
        return err
    }
    defer jsonFile.Close()

    byteValue, err := io.ReadAll(jsonFile)
    if err != nil {
        return err
    }

    err = json.Unmarshal(byteValue, c)
    if err != nil {
        return err
    }

    return nil
}

func LoadConfig() (*Config, error) {
    var configJSON string
    flag.StringVar(&configJSON, "config", "", "config json file")
    flag.Parse()

    if configJSON == "" {
        log.Println("No config file provided. Exiting...")
        os.Exit(0)
    }

    var config Config
    err := config.GetConfig(configJSON)
    if err != nil {
        return nil, fmt.Errorf("cannot get config: %v", err)
    }

    return &config, nil
}
