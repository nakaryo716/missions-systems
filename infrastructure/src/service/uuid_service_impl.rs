use domain::service::uuid_service::UUIDService;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UUIDServiceImpl;

// 計算時間は75μs程(最適化していない状態)
impl UUIDService for UUIDServiceImpl {
    fn generate(&self) -> String {
        Uuid::new_v4().to_string()
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use domain::service::uuid_service::UUIDService;

    use super::UUIDServiceImpl;

    #[test]
    fn test_gen_each_uuid() {
        let service = UUIDServiceImpl;
        let uuid1 = service.generate();
        let uuid2 = service.generate();
        assert_ne!(uuid1, uuid2)
    }

    // UUID一意性をテストする
    #[test]
    fn test_multiple_uuids_are_unique() {
        let uuids: Vec<_> = (0..100).map(|_| UUIDServiceImpl.generate()).collect();
        let unique_uuids: HashSet<_> = uuids.iter().cloned().collect();
        assert_eq!(uuids.len(), unique_uuids.len());
    }
}
