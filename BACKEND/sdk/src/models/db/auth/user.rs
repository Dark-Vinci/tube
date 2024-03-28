use sea_orm::{entity::prelude::*, ActiveValue};
use serde::{Deserialize, Serialize};
use tonic::async_trait;

use crate::helpers::db_util::compute_password_hash;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users", schema_name = "public")]
pub struct Model {
    #[sea_orm(
        primary_key,
        column_type = "Uuid",
        column_name = "id",
        auto_increment = false
    )]
    pub id: Uuid,

    #[sea_orm(column_type = "Text", nullable)]
    pub first_name: String,

    #[sea_orm(column_type = "Text", nullable)]
    pub last_name: String,

    #[sea_orm(column_type = "DateTime")]
    pub date_of_birth: DateTime,

    #[sea_orm(
        column_type = "Boolean",
        column_name = "is_active",
        default_value = false
    )]
    pub is_active: bool,

    #[sea_orm(column_name = "nick_name")]
    pub nick_name: String,

    #[sea_orm(
        column_type = "DateTime",
        column_name = "created_at",
        default_value = "CURRENT_TIMESTAMP"
    )]
    pub created_at: DateTime,

    #[sea_orm(
        column_type = "DateTime",
        column_name = "updated_at",
        default_value = "CURRENT_TIMESTAMP"
    )]
    pub updated_at: DateTime,

    #[sea_orm(column_type = "DateTime", column_name = "description", nullable)]
    pub deleted_at: Option<DateTime>,

    #[sea_orm(column_name = "password")]
    pub password: String,

    #[sea_orm(column_name = "email", unique, indexed)]
    pub email: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::channel::Entity")]
    Channel,

    #[sea_orm(has_many = "super::session::Entity")]
    Session,

    #[sea_orm(has_many = "super::short::Entity")]
    Short,
}

impl Related<super::channel::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Channel.def()
    }
}

impl Related<super::session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Session.def()
    }
}

impl Related<super::short::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Short.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        // is changed and is new
        let is_changed = self.get(Column::Password).is_unchanged();

        if insert || is_changed {
            let password = self.password.into_value().clone().unwrap().to_string();

            let hash = compute_password_hash(password);

            if let Err(err) = hash {
                return Err(DbErr::Custom(err.to_string()));
            }

            self.password = ActiveValue::Set(hash.unwrap());
        }

        Ok(self)
    }
}

// // Package main defines the entry point for goose service
// package main

// import (
// 	"bytes"
// 	"database/sql"
// 	"flag"
// 	"fmt"
// 	"io"
// 	"os"
// 	"strings"

// 	"github.com/joho/godotenv"
// 	_ "github.com/lib/pq"
// 	"github.com/pressly/goose"
// 	"github.com/rs/zerolog"

// 	"github.com/ovalfi/api-service/sdk/model/env"
// 	"github.com/ovalfi/api-service/terminal"
// )

// var (
// 	flags = flag.NewFlagSet("goose", flag.ExitOnError)
// )

// const upCommand = "up"

// func main() {
// 	logger := zerolog.New(os.Stderr).With().Timestamp().Logger()
// 	gooseLogger := logger.With().Str("ser_name", "goose").Logger()
// 	_ = flags.Parse(os.Args[1:])
// 	args := flags.Args()

// 	if len(args) < 2 {
// 		flags.Usage()
// 		return
// 	}

// 	service := args[0]
// 	command := args[1]

// 	files, err := os.ReadDir("../../")
// 	if err != nil {
// 		gooseLogger.Fatal().Err(err)
// 	}

// 	// each folder represents a database(name) attached to respective service
// 	var dirName []string
// 	for _, file := range files {
// 		if file.IsDir() {
// 			dirName = append(dirName, file.Name())
// 		}
// 	}

// 	// confirm module in args is in supported module
// 	if _, found := findService(dirName, service); !found {
// 		gooseLogger.Fatal().Err(terminal.ErrServiceNotFound).Msgf("error: service not found : %s", service)
// 	}

// 	// Load the environment variables of the service
// 	err = godotenv.Load("../../terminal.env") // use `dev.env` for local environment
// 	if err != nil {
// 		gooseLogger.Fatal().Err(err).Msgf(fmt.Sprintf("cannot load environment %v: %v", command, err))
// 		return
// 	}

// 	connection := fmt.Sprintf("postgres://%s:%s@%s:%s/%s?sslmode=disable",
// 		os.Getenv(env.PgUser),
// 		os.Getenv(env.PgPassword),
// 		os.Getenv(env.PgHost),
// 		os.Getenv(env.PgExternalPort),
// 		service)

// 	db, err := sql.Open("postgres", connection)
// 	if err != nil {
// 		gooseLogger.Fatal().Err(err).Msgf(fmt.Sprintf("goose %v: %v", command, err))
// 		return
// 	}

// 	var arguments []string
// 	for _, val := range args[2:] {
// 		if len(val) > 0 {
// 			arguments = append(arguments, val)
// 		}
// 	}

// 	gooseLogger.Info().Msgf(fmt.Sprintf("service(%s)::: running goose %s %v : args=%d",
// 		service, command, arguments, len(arguments)))

// 	moduleMigrationDirectory := fmt.Sprintf("../../%s/migration", service)
// 	if err = goose.Run(command, db, moduleMigrationDirectory, arguments...); err != nil {
// 		gooseLogger.Fatal().Err(err).Msgf(fmt.Sprintf("goose %v: %v", command, err))
// 	}

// 	// we want to grant required permissions and privileges after every up - run
// 	if command == upCommand {
// 		res := runUpMigrationHook(db, os.Getenv(env.PgUser))
// 		if res != nil {
// 			gooseLogger.Err(err).Msg("runUpMigrationHook failed")
// 			return
// 		}
// 		gooseLogger.Info().Msg("runUpMigrationHook ran successfully")
// 	}
// }

// func findService(slice []string, service string) (int, bool) {
// 	for i, item := range slice {
// 		if item == service {
// 			return i, true
// 		}
// 	}
// 	return -1, false
// }

// func upMigrationHookScript(dbUser string) string {
// 	a := fmt.Sprintf("GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO %s;", dbUser)
// 	b := fmt.Sprintf("GRANT USAGE, SELECT ON SEQUENCE goose_db_version_id_seq TO %s;", dbUser)
// 	resp := fmt.Sprintf("%s\n%s", a, b)
// 	return resp
// }

// func runUpMigrationHook(db *sql.DB, dbUser string) error {
// 	ddd := upMigrationHookScript(dbUser)

// 	buf := bytes.NewBuffer(nil)
// 	r := strings.NewReader(ddd)
// 	_, err := io.Copy(buf, r)
// 	if err != nil {
// 		return fmt.Errorf("failed to read SQL script: %v", err)
// 	}

// 	// execute script
// 	_, err = db.Exec(buf.String())
// 	if err != nil {
// 		return fmt.Errorf("failed to execute SQL script: %v", err)
// 	}

// 	return nil
// }
