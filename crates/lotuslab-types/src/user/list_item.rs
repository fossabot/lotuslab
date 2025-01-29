// SPDX-FileCopyrightText: 2025 Jesse Aubin <jesseb34r@jesseaubin.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    CardCoreDetail, CardCoreId, CardPrintingDetail, CardPrintingId, ListId, ListItemId, Patch,
    RepoError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListItemSummary {
    pub id: ListItemId,
    pub list_id: ListId,
    pub card_core_id: CardCoreId,
    pub selected_printing: Option<CardPrintingId>,
    // TODO selected side
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListItemDetail {
    pub id: ListItemId,
    pub list_id: ListId,
    pub card_card_id: CardCoreId,
    pub selected_printing: Option<CardPrintingId>,
    // TODO add relevant details
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub card_core_details: CardCoreDetail,
    pub card_printing_details: CardPrintingDetail,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewListItem {
    pub list_id: ListId,
    pub card_core_id: CardCoreId,
    pub selected_printing: Option<CardPrintingId>,
    pub quantity: Option<i32>,
    pub notes: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateListItem {
    #[serde(default)]
    pub selected_printing: Patch<CardPrintingId>,
    #[serde(default)]
    pub quantity: Patch<i32>,
    #[serde(default)]
    pub notes: Patch<String>,
}

#[async_trait::async_trait]
pub trait ListItemReadRepo: Send + Sync {
    async fn get_list_item_summary(&self, id: &ListItemId) -> Result<ListItemSummary, RepoError>;
    async fn get_list_item_detail(&self, id: &ListItemId) -> Result<ListItemDetail, RepoError>;
    async fn list_items_for_list(
        &self,
        list_id: &ListId,
    ) -> Result<Vec<ListItemSummary>, RepoError>;
}

#[async_trait::async_trait]
pub trait ListItemWriteRepo: Send + Sync {
    async fn create_list_item(&self, new: NewListItem) -> Result<ListItemDetail, RepoError>;
    async fn update_list_item(
        &self,
        id: &ListItemId,
        patch: UpdateListItem,
    ) -> Result<ListItemDetail, RepoError>;
    async fn delete_list_item(&self, id: &ListItemId) -> Result<(), RepoError>;
}
