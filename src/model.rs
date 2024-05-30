//! Simplistic Model Layer
//! (with mock-store layer)

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// region:          ───── Ticket Types

#[derive(Debug, Clone, Serialize)]
pub struct Ticket {
  pub id: u64,
  pub title: String,
}

#[derive(Deserialize)]
pub struct TicketInput {
  pub title: String,
}

// endregion:       ───── Ticket Types

// region:          ───── Model Controller

#[derive(Clone)]
pub struct ModelController {
  ticket_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
  pub async fn new() -> Result<Self> {
    Ok(Self {
      ticket_store: Arc::default(),
    })
  }
}

// CRUD Implementation
impl ModelController {
  pub async fn create_ticket(&self, TicketInput { title }: TicketInput) -> Result<Ticket> {
    let mut store = self.ticket_store.lock().unwrap();

    let id = store.len() as u64;
    let ticket = Ticket { id, title };

    store.push(Some(ticket.clone()));

    Ok(ticket)
  }

  pub async fn find_many(&self) -> Result<Vec<Ticket>> {
    let store = self.ticket_store.lock().unwrap();
    let tickets = store.iter().filter_map(|ticket| ticket.clone()).collect();

    Ok(tickets)
  }

  pub async fn delete_ticket(&self, id: u64) -> Result<Ticket> {
    let mut store = self.ticket_store.lock().unwrap();
    let ticket = store.get_mut(id as usize).and_then(|ticket| ticket.take());

    ticket.ok_or(Error::ResourceNotFound { id })
  }
}

// endregion:       ───── Model Controller
