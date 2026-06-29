pub fn recall(
        &self,
        params: &RecallParams,
    ) -> Result<Vec<Entity>, Box<dyn std::error::Error>> {
        let conn = self.conn()?;
        let mut entities = self.recall_query(&conn, params)?;

        if params.include_confidence {
            for entity in &mut entities {
                entity.confidence = Some(self.calculate_confidence(entity));
            }
        }

        if !params.skip_side_effects {
            let now = now_ms();
            let ids: Vec<String> = entities.iter().map(|e| e.id.clone()).collect();
            self.apply_recall_side_effects(&conn, &ids, now)?;
        }

        Ok(entities)
    }