use super::{
    model::{Clip, GetClip, NewClip, UpdateClip},
    DataError, DatabasePool,
};

type Result<T> = std::result::Result<T, DataError>;

pub async fn get_clip<M: Into<GetClip>>(model: M, pool: &DatabasePool) -> Result<Clip> {
    let model: GetClip = model.into();
    let shortcode = model.shortcode.as_str();
    Ok(sqlx::query_as!(
        Clip,
        r#"SELECT * FROM clips
            WHERE shortcode = ?"#,
        shortcode
    )
    .fetch_one(pool)
    .await?)
}

pub async fn new_clip<M: Into<NewClip>>(model: M, pool: &DatabasePool) -> Result<Clip> {
    let model: NewClip = model.into();
    sqlx::query!(
        r#"INSERT INTO clips (clip_id, content, expires, hits, password, posted, shortcode, title) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
        model.clip_id,
        model.content,
        model.expires,
        0,
        model.password,
        model.posted,
        model.shortcode,
        model.title,
    ).execute(pool).await?;

    get_clip(model.shortcode, pool).await
}

pub async fn update_clip<M: Into<UpdateClip>>(model: M, pool: &DatabasePool) -> Result<Clip> {
    let model: UpdateClip = model.into();
    sqlx::query!(
        r#"UPDATE clips SET
            content = ?,
            expires = ?,
            hits = ?,
            password = ?,
            title = ?
           WHERE shortcode = ?"#,
        model.content,
        model.expires,
        model.hits,
        model.password,
        model.title,
        model.shortcode,
    )
    .execute(pool)
    .await?;

    get_clip(model.shortcode, pool).await
}
