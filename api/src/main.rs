use std::env;

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
};
use chrono::{DateTime, Datelike, Duration, NaiveDate, TimeZone, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use rand::{Rng, distributions::Alphanumeric};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Number, Value as JsonValue, json};
use sqlx::{PgPool, Row, postgres::PgPoolOptions, postgres::PgRow};
use uuid::Uuid;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct HealthResponse {
    status: &'static str,
    version: &'static str,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RegisterRequest {
    name: String,
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PasswordResetRequest {
    email: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PasswordResetConfirm {
    token: String,
    password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct AuthResponse {
    token: String,
    user: PublicUser,
    expires_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PublicUser {
    id: String,
    name: String,
    email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar_url: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PasswordResetResponse {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    reset_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Calendar {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner_id: Option<String>,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recipient_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    published_at: Option<DateTime<Utc>>,
    doors: Vec<Door>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Door {
    day: i16,
    #[serde(skip_serializing_if = "Option::is_none")]
    opens_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    present: Option<JsonValue>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NewCalendar {
    name: String,
    #[serde(default)]
    owner_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct DoorPresentMedia {
    #[serde(rename = "type", default)]
    media_type: Option<String>,
    #[serde(default)]
    url: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    thumbnail_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct DoorPresentContent {
    title: String,
    #[serde(default)]
    message: Option<String>,
    #[serde(default)]
    media: Option<DoorPresentMedia>,
    #[serde(default)]
    tasks: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct DoorPresentPayload {
    #[serde(default)]
    id: Option<String>,
    content: DoorPresentContent,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpsertDoorRequest {
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    opens_at: Option<DateTime<Utc>>,
    #[serde(default)]
    state: Option<String>,
    #[serde(default)]
    present: Option<DoorPresentPayload>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Clone)]
struct AppState {
    db: PgPool,
    encoding_key: EncodingKey,
    token_ttl: Duration,
    argon2: Argon2<'static>,
    expose_reset_token: bool,
}

#[tokio::main]
async fn main() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("failed to connect to database");

    ensure_database_schema(&db)
        .await
        .expect("failed to ensure database schema");

    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token_ttl = env::var("JWT_TTL_SECS")
        .ok()
        .and_then(|raw| raw.parse::<i64>().ok())
        .map(Duration::seconds)
        .unwrap_or_else(|| Duration::hours(1));

    let encoding_key = EncodingKey::from_secret(jwt_secret.as_bytes());
    let argon2 = Argon2::default();
    let expose_reset_token = env::var("MODE")
        .map(|mode| mode != "production")
        .unwrap_or(true);

    let state = AppState {
        db,
        encoding_key,
        token_ttl,
        argon2,
        expose_reset_token,
    };

    let api_routes = Router::new()
        .route("/health", get(healthcheck))
        .route("/calendars", get(list_calendars).post(create_calendar))
        .route("/calendars/{id}", get(get_calendar))
        .route("/calendars/{id}/doors/{day}", put(upsert_door));

    let auth_routes = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/password/request", post(request_password_reset))
        .route("/password/reset", post(reset_password));

    let v1_routes = Router::new().merge(api_routes).nest("/auth", auth_routes);

    let app = Router::new().nest("/api/v1", v1_routes).with_state(state);

    let port = env::var("API_PORT").unwrap_or_else(|_| "8080".to_string());
    let port = port.parse::<u16>().expect("API_PORT must be a valid u16");

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port))
        .await
        .expect("failed to bind TCP listener");
    axum::serve(listener, app).await.expect("server error");
}

async fn ensure_database_schema(db: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "ALTER TABLE presents ADD COLUMN IF NOT EXISTS content JSONB NOT NULL DEFAULT '{}'::jsonb",
    )
    .execute(db)
    .await?;

    Ok(())
}

async fn healthcheck() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        version: "v1",
    })
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), StatusCode> {
    if payload.email.trim().is_empty() || payload.password.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let password_hash = hash_password(&state.argon2, &payload.password)?;

    let result = sqlx::query(
        "INSERT INTO users (email, display_name, password_hash) VALUES ($1, $2, $3) RETURNING id, email, display_name, avatar_url"
    )
    .bind(payload.email.trim())
    .bind(payload.name.trim())
    .bind(password_hash)
    .fetch_one(&state.db)
    .await;

    let row = match result {
        Ok(row) => row,
        Err(sqlx::Error::Database(db_err)) if db_err.constraint() == Some("users_email_key") => {
            return Err(StatusCode::CONFLICT);
        }
        Err(err) => return Err(log_internal_error(err)),
    };

    let user = map_user_row(&row);
    let response = issue_auth_response(&state, user)?;

    Ok((StatusCode::CREATED, Json(response)))
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    let row = sqlx::query(
        "SELECT id, email, display_name, avatar_url, password_hash FROM users WHERE email = $1",
    )
    .bind(payload.email.trim())
    .fetch_optional(&state.db)
    .await
    .map_err(log_internal_error)?;

    let row = match row {
        Some(row) => row,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let stored_hash: String = row.get("password_hash");
    verify_password(&state.argon2, &payload.password, &stored_hash)?;

    let user = map_user_row(&row);
    let response = issue_auth_response(&state, user)?;

    Ok(Json(response))
}

async fn request_password_reset(
    State(state): State<AppState>,
    Json(payload): Json<PasswordResetRequest>,
) -> Result<Json<PasswordResetResponse>, StatusCode> {
    if payload.email.trim().is_empty() {
        return Ok(Json(PasswordResetResponse {
            success: true,
            reset_token: None,
            expires_at: None,
        }));
    }

    let token = generate_reset_token();
    let expiry = Utc::now() + Duration::minutes(30);

    let updated = sqlx::query(
        "UPDATE users SET reset_token = $1, reset_token_expires_at = $2 WHERE email = $3 RETURNING id"
    )
    .bind(&token)
    .bind(expiry)
    .bind(payload.email.trim())
    .fetch_optional(&state.db)
    .await
    .map_err(log_internal_error)?;

    let (reset_token, expires_at) = if updated.is_some() && state.expose_reset_token {
        (Some(token), Some(expiry))
    } else {
        (None, None)
    };

    Ok(Json(PasswordResetResponse {
        success: true,
        reset_token,
        expires_at,
    }))
}

async fn reset_password(
    State(state): State<AppState>,
    Json(payload): Json<PasswordResetConfirm>,
) -> Result<StatusCode, StatusCode> {
    if payload.token.trim().is_empty() || payload.password.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let candidate =
        sqlx::query("SELECT id FROM users WHERE reset_token = $1 AND reset_token_expires_at > $2")
            .bind(payload.token.trim())
            .bind(Utc::now())
            .fetch_optional(&state.db)
            .await
            .map_err(log_internal_error)?;

    let user_id: i64 = match candidate {
        Some(row) => row.get("id"),
        None => return Err(StatusCode::BAD_REQUEST),
    };

    let password_hash = hash_password(&state.argon2, &payload.password)?;

    sqlx::query(
        "UPDATE users SET password_hash = $1, reset_token = NULL, reset_token_expires_at = NULL, updated_at = now() WHERE id = $2"
    )
    .bind(password_hash)
    .bind(user_id)
    .execute(&state.db)
    .await
    .map_err(log_internal_error)?;

    Ok(StatusCode::NO_CONTENT)
}

async fn list_calendars(State(state): State<AppState>) -> Result<Json<Vec<Calendar>>, StatusCode> {
    let calendar_rows = sqlx::query(
        "SELECT id, owner, name, description, recipient_name, status::text AS status, created_at, updated_at, published_at FROM calendars ORDER BY id"
    )
    .fetch_all(&state.db)
    .await
    .map_err(log_internal_error)?;

    let mut calendars = Vec::with_capacity(calendar_rows.len());
    for row in calendar_rows {
        let id: i64 = row.get("id");
        let name: String = row.get("name");
        let owner_id = row
            .try_get::<Option<i64>, _>("owner")
            .unwrap_or(None)
            .map(|value| value.to_string());
        let description = row
            .try_get::<Option<String>, _>("description")
            .unwrap_or(None);
        let recipient_name = row
            .try_get::<Option<String>, _>("recipient_name")
            .unwrap_or(None);
        let status = row.try_get::<Option<String>, _>("status").unwrap_or(None);
        let created_at = row
            .try_get::<Option<DateTime<Utc>>, _>("created_at")
            .unwrap_or(None);
        let updated_at = row
            .try_get::<Option<DateTime<Utc>>, _>("updated_at")
            .unwrap_or(None);
        let published_at = row
            .try_get::<Option<DateTime<Utc>>, _>("published_at")
            .unwrap_or(None);

        let doors = load_doors(&state.db, id).await?;
        calendars.push(Calendar {
            id: id.to_string(),
            owner_id,
            name,
            description,
            recipient_name,
            status,
            created_at,
            updated_at,
            published_at,
            doors,
        });
    }

    Ok(Json(calendars))
}

async fn get_calendar(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Calendar>, StatusCode> {
    let id = i64::try_from(id).map_err(|_| StatusCode::BAD_REQUEST)?;

    let row = sqlx::query(
        "SELECT id, owner, name, description, recipient_name, status::text AS status, created_at, updated_at, published_at FROM calendars WHERE id = $1"
    )
    .bind(id)
    .fetch_one(&state.db)
    .await
    .map_err(|err| match err {
        sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
        other => log_internal_error(other),
    })?;

    let calendar_id: i64 = row.get("id");
    let name: String = row.get("name");
    let owner_id = row
        .try_get::<Option<i64>, _>("owner")
        .unwrap_or(None)
        .map(|value| value.to_string());
    let description = row
        .try_get::<Option<String>, _>("description")
        .unwrap_or(None);
    let recipient_name = row
        .try_get::<Option<String>, _>("recipient_name")
        .unwrap_or(None);
    let status = row.try_get::<Option<String>, _>("status").unwrap_or(None);
    let created_at = row
        .try_get::<Option<DateTime<Utc>>, _>("created_at")
        .unwrap_or(None);
    let updated_at = row
        .try_get::<Option<DateTime<Utc>>, _>("updated_at")
        .unwrap_or(None);
    let published_at = row
        .try_get::<Option<DateTime<Utc>>, _>("published_at")
        .unwrap_or(None);

    let doors = load_doors(&state.db, calendar_id).await?;

    Ok(Json(Calendar {
        id: calendar_id.to_string(),
        name,
        owner_id,
        description,
        recipient_name,
        status,
        created_at,
        updated_at,
        published_at,
        doors,
    }))
}

async fn create_calendar(
    State(state): State<AppState>,
    Json(payload): Json<NewCalendar>,
) -> Result<impl IntoResponse, StatusCode> {
    let owner_numeric = payload
        .owner_id
        .as_deref()
        .and_then(|value| value.parse::<i64>().ok());

    let row = sqlx::query(
        "INSERT INTO calendars (name, owner) VALUES ($1, $2) RETURNING id, owner, name, description, recipient_name, status::text AS status, created_at, updated_at, published_at"
    )
    .bind(payload.name.trim())
    .bind(owner_numeric)
    .fetch_one(&state.db)
    .await
    .map_err(log_internal_error)?;

    let id: i64 = row.get("id");
    seed_calendar_doors(&state.db, id).await?;
    let doors = load_doors(&state.db, id).await?;
    let calendar = Calendar {
        id: id.to_string(),
        owner_id: row
            .try_get::<Option<i64>, _>("owner")
            .unwrap_or(None)
            .map(|value| value.to_string()),
        name: row.get("name"),
        description: row
            .try_get::<Option<String>, _>("description")
            .unwrap_or(None),
        recipient_name: row
            .try_get::<Option<String>, _>("recipient_name")
            .unwrap_or(None),
        status: row.try_get::<Option<String>, _>("status").unwrap_or(None),
        created_at: row
            .try_get::<Option<DateTime<Utc>>, _>("created_at")
            .unwrap_or(None),
        updated_at: row
            .try_get::<Option<DateTime<Utc>>, _>("updated_at")
            .unwrap_or(None),
        published_at: row
            .try_get::<Option<DateTime<Utc>>, _>("published_at")
            .unwrap_or(None),
        doors,
    };

    Ok((StatusCode::CREATED, Json(calendar)))
}

async fn seed_calendar_doors(pool: &PgPool, calendar_id: i64) -> Result<(), StatusCode> {
    let year = Utc::now().year();
    let mut tx = pool.begin().await.map_err(log_internal_error)?;

    for day in 1..=24 {
        let Some(date) = NaiveDate::from_ymd_opt(year, 12, day) else {
            continue;
        };
        let Some(naive_datetime) = date.and_hms_opt(6, 0, 0) else {
            continue;
        };
        let opens_at = Utc.from_utc_datetime(&naive_datetime);

        sqlx::query(
            "INSERT INTO calendar_doors (calendar_id, day, title, opens_at, state)
           VALUES ($1, $2, $3, $4, $5::calendar_door_state)
             ON CONFLICT (calendar_id, day) DO NOTHING",
        )
        .bind(calendar_id)
        .bind(day as i16)
        .bind(format!("Door {}", day))
        .bind(opens_at)
        .bind(if day == 1 { "available" } else { "locked" })
        .execute(&mut *tx)
        .await
        .map_err(log_internal_error)?;
    }

    tx.commit().await.map_err(log_internal_error)?;
    Ok(())
}

async fn load_doors(pool: &PgPool, calendar_id: i64) -> Result<Vec<Door>, StatusCode> {
    let door_rows = sqlx::query(
        "SELECT
            d.day,
            d.opens_at,
            d.state::text AS state,
            d.title,
            p.id AS present_id,
            p.calendar_id AS present_calendar_id,
            p.title AS present_title,
            p.description AS present_description,
            p.media_url AS present_media_url,
            p.link_url AS present_link_url,
            p.content AS present_content,
            p.created_at AS present_created_at,
            p.updated_at AS present_updated_at
         FROM calendar_doors d
         LEFT JOIN presents p ON p.id = d.present_id
         WHERE d.calendar_id = $1
         ORDER BY d.day",
    )
    .bind(calendar_id)
    .fetch_all(pool)
    .await
    .map_err(log_internal_error)?;

    Ok(door_rows
        .into_iter()
        .map(|row| {
            let present = build_present_json(&row, calendar_id);
            Door {
                day: row.get("day"),
                opens_at: row
                    .try_get::<Option<DateTime<Utc>>, _>("opens_at")
                    .unwrap_or(None),
                state: row.try_get::<Option<String>, _>("state").unwrap_or(None),
                present,
            }
        })
        .collect())
}

fn build_present_json(row: &PgRow, calendar_id: i64) -> Option<JsonValue> {
    let present_id: Option<i64> = row.try_get("present_id").unwrap_or(None);
    let Some(id) = present_id else {
        return None;
    };

    let door_number: i16 = row.get("day");
    let opens_at = row
        .try_get::<Option<DateTime<Utc>>, _>("opens_at")
        .unwrap_or(None);

    let calendar_id_str = row
        .try_get::<Option<i64>, _>("present_calendar_id")
        .unwrap_or(None)
        .map(|value| value.to_string())
        .unwrap_or_else(|| calendar_id.to_string());

    let mut content = row
        .try_get::<Option<JsonValue>, _>("present_content")
        .unwrap_or(None)
        .unwrap_or_else(|| json!({}));

    if !content.is_object() {
        content = json!({});
    }

    if let Some(obj) = content.as_object_mut() {
        if !obj.contains_key("title") {
            let fallback = row
                .try_get::<Option<String>, _>("present_title")
                .unwrap_or(None)
                .or_else(|| row.try_get::<Option<String>, _>("title").unwrap_or(None))
                .unwrap_or_else(|| format!("Door {}", door_number));
            obj.insert("title".to_string(), JsonValue::String(fallback));
        }

        if !obj.contains_key("message") {
            let message = row
                .try_get::<Option<String>, _>("present_description")
                .unwrap_or(None)
                .unwrap_or_default();
            obj.insert("message".to_string(), JsonValue::String(message));
        }

        if !obj.contains_key("media") {
            if let Some(url) = row
                .try_get::<Option<String>, _>("present_media_url")
                .unwrap_or(None)
            {
                let mut media = Map::new();
                media.insert("url".to_string(), JsonValue::String(url));
                if let Some(media_type) = row
                    .try_get::<Option<String>, _>("present_link_url")
                    .unwrap_or(None)
                {
                    media.insert("type".to_string(), JsonValue::String(media_type));
                }
                obj.insert("media".to_string(), JsonValue::Object(media));
            }
        }

        let requires_tasks = obj
            .get("tasks")
            .map(|value| !value.is_array())
            .unwrap_or(true);
        if requires_tasks {
            obj.insert("tasks".to_string(), JsonValue::Array(vec![]));
        }
    }

    let mut present = Map::new();
    present.insert("id".to_string(), JsonValue::String(id.to_string()));
    present.insert("calendarId".to_string(), JsonValue::String(calendar_id_str));
    present.insert(
        "doorNumber".to_string(),
        JsonValue::Number(Number::from(i64::from(door_number))),
    );
    present.insert("content".to_string(), content);

    if let Some(release) = opens_at {
        present.insert(
            "releaseDate".to_string(),
            JsonValue::String(release.to_rfc3339()),
        );
    }

    if let Some(created) = row
        .try_get::<Option<DateTime<Utc>>, _>("present_created_at")
        .unwrap_or(None)
    {
        present.insert(
            "createdAt".to_string(),
            JsonValue::String(created.to_rfc3339()),
        );
    }

    if let Some(updated) = row
        .try_get::<Option<DateTime<Utc>>, _>("present_updated_at")
        .unwrap_or(None)
    {
        present.insert(
            "updatedAt".to_string(),
            JsonValue::String(updated.to_rfc3339()),
        );
    }

    Some(JsonValue::Object(present))
}

async fn load_door(pool: &PgPool, calendar_id: i64, day: i16) -> Result<Door, StatusCode> {
    let row = sqlx::query(
        "SELECT
            d.day,
            d.opens_at,
            d.state::text AS state,
            d.title,
            p.id AS present_id,
            p.calendar_id AS present_calendar_id,
            p.title AS present_title,
            p.description AS present_description,
            p.media_url AS present_media_url,
            p.link_url AS present_link_url,
            p.content AS present_content,
            p.created_at AS present_created_at,
            p.updated_at AS present_updated_at
         FROM calendar_doors d
         LEFT JOIN presents p ON p.id = d.present_id
         WHERE d.calendar_id = $1 AND d.day = $2",
    )
    .bind(calendar_id)
    .bind(day)
    .fetch_optional(pool)
    .await
    .map_err(log_internal_error)?;

    let Some(row) = row else {
        return Err(StatusCode::NOT_FOUND);
    };

    let present = build_present_json(&row, calendar_id);
    Ok(Door {
        day: row.get("day"),
        opens_at: row
            .try_get::<Option<DateTime<Utc>>, _>("opens_at")
            .unwrap_or(None),
        state: row.try_get::<Option<String>, _>("state").unwrap_or(None),
        present,
    })
}

async fn upsert_door(
    State(state): State<AppState>,
    Path((calendar_id, day)): Path<(u64, i16)>,
    Json(payload): Json<UpsertDoorRequest>,
) -> Result<Json<Door>, StatusCode> {
    if !(1..=31).contains(&day) {
        return Err(StatusCode::BAD_REQUEST);
    }

    if let Some(ref state_value) = payload.state {
        if state_value != "locked" && state_value != "available" && state_value != "opened" {
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    let calendar_id = i64::try_from(calendar_id).map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut tx = state.db.begin().await.map_err(log_internal_error)?;

    let door_row = sqlx::query(
        "SELECT id, present_id FROM calendar_doors WHERE calendar_id = $1 AND day = $2",
    )
    .bind(calendar_id)
    .bind(day)
    .fetch_optional(&mut *tx)
    .await
    .map_err(log_internal_error)?;

    let Some(door_row) = door_row else {
        tx.rollback().await.map_err(log_internal_error)?;
        return Err(StatusCode::NOT_FOUND);
    };

    let door_id: i64 = door_row.get("id");
    let current_present_id: Option<i64> = door_row.try_get("present_id").unwrap_or(None);

    sqlx::query(
        "UPDATE calendar_doors
         SET title = COALESCE($1, title),
             opens_at = COALESCE($2, opens_at),
             state = COALESCE($3::calendar_door_state, state),
             updated_at = now()
         WHERE id = $4",
    )
    .bind(payload.title.clone())
    .bind(payload.opens_at)
    .bind(payload.state.as_deref())
    .bind(door_id)
    .execute(&mut *tx)
    .await
    .map_err(log_internal_error)?;

    match payload.present {
        Some(present_payload) => {
            let content_value = serde_json::to_value(&present_payload.content).map_err(|err| {
                eprintln!("failed to serialize present content: {err}");
                StatusCode::BAD_REQUEST
            })?;

            let media_url = present_payload
                .content
                .media
                .as_ref()
                .and_then(|media| media.url.clone());
            let media_type = present_payload
                .content
                .media
                .as_ref()
                .and_then(|media| media.media_type.clone());
            let message = present_payload.content.message.clone().unwrap_or_default();
            let title = present_payload.content.title.clone();

            let present_id = match current_present_id {
                Some(existing_id) => {
                    sqlx::query(
                        "UPDATE presents
                         SET title = $1,
                             description = $2,
                             media_url = $3,
                             link_url = $4,
                             content = $5,
                             updated_at = now()
                         WHERE id = $6",
                    )
                    .bind(&title)
                    .bind(&message)
                    .bind(media_url.clone())
                    .bind(media_type.clone())
                    .bind(content_value.clone())
                    .bind(existing_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(log_internal_error)?;

                    existing_id
                }
                None => {
                    let inserted = sqlx::query(
                        "INSERT INTO presents (calendar_id, title, description, media_url, link_url, content)
                         VALUES ($1, $2, $3, $4, $5, $6)
                         RETURNING id",
                    )
                    .bind(calendar_id)
                    .bind(&title)
                    .bind(&message)
                    .bind(media_url.clone())
                    .bind(media_type.clone())
                    .bind(content_value.clone())
                    .fetch_one(&mut *tx)
                    .await
                    .map_err(log_internal_error)?;

                    inserted.get("id")
                }
            };

            sqlx::query(
                "UPDATE calendar_doors SET present_id = $1, updated_at = now() WHERE id = $2",
            )
            .bind(present_id)
            .bind(door_id)
            .execute(&mut *tx)
            .await
            .map_err(log_internal_error)?;
        }
        None => {
            if let Some(existing_id) = current_present_id {
                sqlx::query(
                    "UPDATE calendar_doors SET present_id = NULL, updated_at = now() WHERE id = $1",
                )
                .bind(door_id)
                .execute(&mut *tx)
                .await
                .map_err(log_internal_error)?;

                sqlx::query("DELETE FROM presents WHERE id = $1")
                    .bind(existing_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(log_internal_error)?;
            }
        }
    }

    tx.commit().await.map_err(log_internal_error)?;

    let door = load_door(&state.db, calendar_id, day).await?;
    Ok(Json(door))
}

fn log_internal_error<E: std::fmt::Display>(err: E) -> StatusCode {
    eprintln!("database error: {err}");
    StatusCode::INTERNAL_SERVER_ERROR
}

#[derive(Debug)]
struct DbUser {
    id: i64,
    email: String,
    display_name: Option<String>,
    avatar_url: Option<String>,
}

fn map_user_row(row: &sqlx::postgres::PgRow) -> DbUser {
    DbUser {
        id: row.get("id"),
        email: row.get("email"),
        display_name: row
            .try_get::<Option<String>, _>("display_name")
            .unwrap_or(None),
        avatar_url: row
            .try_get::<Option<String>, _>("avatar_url")
            .unwrap_or(None),
    }
}

fn to_public_user(user: &DbUser) -> PublicUser {
    PublicUser {
        id: user.id.to_string(),
        name: user.display_name.clone().unwrap_or_else(|| "".to_string()),
        email: user.email.clone(),
        avatar_url: user.avatar_url.clone(),
    }
}

fn issue_auth_response(state: &AppState, user: DbUser) -> Result<AuthResponse, StatusCode> {
    let expires_at = Utc::now() + state.token_ttl;
    let claims = Claims {
        sub: user.id.to_string(),
        exp: expires_at.timestamp() as usize,
    };

    let token =
        encode(&Header::default(), &claims, &state.encoding_key).map_err(log_internal_error)?;

    Ok(AuthResponse {
        token,
        user: to_public_user(&user),
        expires_at,
    })
}

fn hash_password(argon2: &Argon2<'static>, password: &str) -> Result<String, StatusCode> {
    let salt = SaltString::generate(&mut OsRng);
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|err| {
            eprintln!("password hashing error: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        })
        .map(|hash| hash.to_string())
}

fn verify_password(
    argon2: &Argon2<'static>,
    password: &str,
    stored_hash: &str,
) -> Result<(), StatusCode> {
    let parsed = PasswordHash::new(stored_hash).map_err(|err| {
        eprintln!("password hash parse error: {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    argon2
        .verify_password(password.as_bytes(), &parsed)
        .map_err(|_| StatusCode::UNAUTHORIZED)
}

fn generate_reset_token() -> String {
    let random_suffix: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(20)
        .map(char::from)
        .collect();
    format!("{}-{}", Uuid::new_v4(), random_suffix)
}
