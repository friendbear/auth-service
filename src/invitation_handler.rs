use actix_web::{error::BlockingError, web, App, HttpResponse};
use diesel::{prelude::*, PgConnection};
use serde::{Deserialize, Serialize};

// use crate::email_seervice::send_invitation;