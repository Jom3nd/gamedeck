use sea_orm::{
    DatabaseConnection,
    ActiveModelTrait,
    ActiveValue::Set,
    ColumnTrait,
    EntityTrait,
    QueryFilter,
};
use crate::entities::{game,genre,game_genre};
use uuid::Uuid;
use chrono::Utc;
use crate::{
    dto::game::{
        create_game_dto::CreateGameDto,
        game_response_dto::GameResponseDto,
        update_game_dto::UpdateGameDto,
    },
    errors::api_erro::ApiError,
};

pub struct GameService;

impl GameService{

    pub async fn create(
        db: &DatabaseConnection,
        dto: CreateGameDto,
    ) -> Result<GameResponseDto, ApiError> {

        //Validação
        if dto.title.trim().is_empty() {
            return Err(ApiError::BadRequest("Título é obrigatório".to_string()));
        }
        if dto.genres.is_empty() {
            return Err(ApiError::BadRequest("É necessário informar pelo menos um gênero".into()))
        }

        for genre_id in &dto.genres {

            let genre_exists = genre::Entity::find_by_id(*genre_id)
            .one(db)
            .await?;

            if genre_exists.is_none() {
                return Err(ApiError::NotFound("Um dos gêneros informados não foi encontrado".into()));
            }
        }


        //Consulta ao banco

        let exists = game::Entity::find()
        .filter(game::Column::Title.eq(&dto.title))
        .one(db)
        .await?;

        if exists.is_some() {
            return Err(ApiError::Conflict("Jogo já cadastrado".into()));
        }

        let game = game::ActiveModel {
            id: Set(Uuid::new_v4()),
            title: Set(dto.title),
            description: Set(dto.description),
            developer: Set(dto.developer),
            price: Set(dto.price),
            release_date: Set(dto.release_date),
            created_at: Set(Utc::now()),
            game_url: Set(dto.game_url),
            ..Default::default()
        };

        let game = game.insert(db).await?;

        //Criar relacionamentos com os gêneros

        for genre_id in &dto.genres {
            let relation = game_genre::ActiveModel {
                game_id: Set(game.id),
                genre_id: Set(*genre_id),
            };
            relation.insert(db).await?;
        }
        
        Ok(GameResponseDto {
            id: game.id,
            title: game.title,
            description: game.description,
            developer: game.developer,
            price: game.price,
            release_date: game.release_date,
            game_url: game.game_url,
            genres: dto.genres,
    })


    }

}