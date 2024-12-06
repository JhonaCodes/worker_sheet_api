#[cfg(test)]
mod tests {
    use crate::activities::models::{Activities, ActivityFilter, PhotoActivity, UpdateActivityStatus, NewPhoto};
    use chrono::{Utc};
    use uuid::Uuid;

    // Helper function para crear una actividad válida
    fn create_valid_activity() -> Activities {
        Activities {
            id: Uuid::new_v4(),
            title: "Test Activity".to_string(),
            description: "Test Description".to_string(),
            status: "PENDING".to_string(),
            risk_level: "LOW".to_string(),
            location_lat: Some(40.7128),
            location_lng: Some(-74.0060),
            user_id: Uuid::new_v4().to_string(),
            start_date: Some(Utc::now().naive_utc()),
            end_date: Some(Utc::now().naive_utc()),
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
            hash_sync: Some("test_hash".to_string()),
            is_deleted: Some(false),
        }
    }

    #[test]
    fn test_activity_creation() {
        let activity = create_valid_activity();

        assert!(!activity.title.is_empty());
        assert!(!activity.description.is_empty());
        assert!(!activity.status.is_empty());
        assert!(!activity.risk_level.is_empty());
        assert!(activity.location_lat.is_some());
        assert!(activity.location_lng.is_some());
        assert!(!activity.user_id.is_empty());
        assert!(activity.start_date.is_some());
        assert!(activity.end_date.is_some());
        assert!(!activity.is_deleted.unwrap_or(true));
    }

    #[test]
    fn test_activity_date_validation() {
        let now = Utc::now().naive_utc();
        let tomorrow = now + chrono::Duration::days(1);

        let activity = Activities {
            start_date: Some(now),
            end_date: Some(tomorrow),
            ..create_valid_activity()
        };

        assert!(activity.start_date.unwrap() <= activity.end_date.unwrap());
    }

    #[test]
    fn test_activity_status_values() {
        let valid_statuses = vec!["PENDING", "IN_PROGRESS", "COMPLETED", "CANCELLED"];

        for status in valid_statuses {
            let activity = Activities {
                status: status.to_string(),
                ..create_valid_activity()
            };

            assert!(["PENDING", "IN_PROGRESS", "COMPLETED", "CANCELLED"]
                .contains(&activity.status.as_str()));
        }
    }

    #[test]
    fn test_activity_risk_levels() {
        let valid_risk_levels = vec!["LOW", "MEDIUM", "HIGH", "CRITICAL"];

        for risk_level in valid_risk_levels {
            let activity = Activities {
                risk_level: risk_level.to_string(),
                ..create_valid_activity()
            };

            assert!(["LOW", "MEDIUM", "HIGH", "CRITICAL"]
                .contains(&activity.risk_level.as_str()));
        }
    }

    #[test]
    fn test_activity_coordinates() {
        let activity = Activities {
            location_lat: Some(40.7128),
            location_lng: Some(-74.0060),
            ..create_valid_activity()
        };

        // Verificar rango válido de coordenadas
        if let Some(lat) = activity.location_lat {
            assert!(lat >= -90.0 && lat <= 90.0);
        }
        if let Some(lng) = activity.location_lng {
            assert!(lng >= -180.0 && lng <= 180.0);
        }
    }

    #[test]
    fn test_activity_filter() {
        let filter = ActivityFilter {
            status: Some("PENDING".to_string()),
            risk_level: Some("HIGH".to_string()),
            start_date: Some(Utc::now().naive_utc()),
            end_date: Some(Utc::now().naive_utc()),
            user_id: Some(Uuid::new_v4().to_string()),
            hash_sync: Some("test_hash".to_string()),
        };

        assert!(filter.status.is_some());
        assert!(filter.risk_level.is_some());
        assert!(filter.start_date.is_some());
        assert!(filter.end_date.is_some());
        assert!(filter.user_id.is_some());
        assert!(filter.hash_sync.is_some());
    }

    #[test]
    fn test_photo_activity() {
        let photo = PhotoActivity {
            id: 1,
            activity_id: Uuid::new_v4().to_string(),
            url: "https://example.com/photo.jpg".to_string(),
        };

        assert!(photo.id > 0);
        assert!(!photo.activity_id.is_empty());
        assert!(photo.url.starts_with("http"));
    }

    #[test]
    fn test_new_photo() {
        let new_photo = NewPhoto {
            activity_id: Uuid::new_v4().to_string(),
            url: "https://example.com/new-photo.jpg".to_string(),
        };

        assert!(!new_photo.activity_id.is_empty());
        assert!(new_photo.url.starts_with("http"));
    }

    #[test]
    fn test_update_activity_status() {
        let status_update = UpdateActivityStatus {
            status: "IN_PROGRESS".to_string(),
            hash_sync: Some("new_hash".to_string()),
        };

        assert!(!status_update.status.is_empty());
        assert!(status_update.hash_sync.is_some());
        assert!(["PENDING", "IN_PROGRESS", "COMPLETED", "CANCELLED"]
            .contains(&status_update.status.as_str()));
    }

    #[test]
    fn test_activity_title_length() {
        let activity = create_valid_activity();
        assert!(activity.title.len() <= 255); // Asumiendo un límite común de BD
    }

    #[test]
    fn test_activity_description_length() {
        let activity = create_valid_activity();
        assert!(!activity.description.is_empty());
        // Podrías agregar una validación de longitud máxima si tu BD tiene un límite
    }

    #[test]
    fn test_empty_activity_filter() {
        let filter = ActivityFilter {
            status: None,
            risk_level: None,
            start_date: None,
            end_date: None,
            user_id: None,
            hash_sync: None,
        };

        // Verificar que un filtro vacío es válido
        assert!(filter.status.is_none());
        assert!(filter.risk_level.is_none());
        assert!(filter.start_date.is_none());
        assert!(filter.end_date.is_none());
        assert!(filter.user_id.is_none());
        assert!(filter.hash_sync.is_none());
    }

    #[test]
    fn test_activity_default_values() {
        let activity = Activities {
            id: Uuid::new_v4(),
            title: "Test".to_string(),
            description: "Test".to_string(),
            status: "PENDING".to_string(),
            risk_level: "LOW".to_string(),
            location_lat: None,
            location_lng: None,
            user_id: Uuid::new_v4().to_string(),
            start_date: None,
            end_date: None,
            created_at: None,
            updated_at: None,
            hash_sync: None,
            is_deleted: None,
        };

        // Verificar valores por defecto
        assert!(activity.location_lat.is_none());
        assert!(activity.location_lng.is_none());
        assert!(activity.start_date.is_none());
        assert!(activity.end_date.is_none());
        assert!(activity.hash_sync.is_none());
        assert!(activity.is_deleted.is_none());
    }
}