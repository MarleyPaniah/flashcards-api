use crate::api::v1::decks::cards::models::{
    db::Card,
    domain::{CardAuditData, CardData},
    requests::UpsertCardRequestDto,
    responses::CardResponseDto,
};

impl From<UpsertCardRequestDto> for CardData {
    fn from(dto: UpsertCardRequestDto) -> Self {
        CardData {
            title: dto.title,
            content_front: dto.content_front,
            content_back: dto.content_back,
            difficulty: dto.difficulty,
            position: dto.position.unwrap_or(0),
            metadata: dto.metadata,
        }
    }
}

impl From<Card> for CardResponseDto {
    fn from(card: Card) -> Self {
        CardResponseDto {
            id: card.id,
            deck_id: card.deck_id,
            title: card.title,
            position: card.position,
            content_front: card.content_front,
            content_back: card.content_back,
            difficulty: card.difficulty,
            metadata: card.metadata,
            audit_data: CardAuditData {
                is_deleted: card.is_deleted,
                created_by: card.created_by,
                updated_by: card.updated_by,
                deleted_by: card.deleted_by,
                created_at: card.created_at,
                updated_at: card.updated_at,
                deleted_at: card.deleted_at,
            },
        }
    }
}
