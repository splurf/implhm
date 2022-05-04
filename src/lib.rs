#[cfg(feature = "enabled")]
mod tools;

#[cfg(feature = "enabled")]
pub use tools::*;

#[cfg(test)]
#[cfg(feature = "test")]
mod tests {

    #[cfg(feature = "separate-chaining-test")]
    mod separate_chaining_test {
        use crate::{Map, MapMut, MapUtil, SCHashMap};

        #[test]
        fn get_test_0() {
            let map: SCHashMap<&str, &str> =
                vec![("4", "four"), ("69", "four-twenty"), ("2", "two")].into();
            assert_eq!(map.get("69"), Some(&"four-twenty"));
        }

        #[test]
        fn get_test_1() {
            let map: SCHashMap<&str, &str> =
                vec![("4", "four"), ("69", "four-twenty"), ("2", "two")].into();
            assert_eq!(map.get("0"), None);
        }

        #[test]
        fn remove_test_0() {
            let mut map: SCHashMap<&str, &str> = vec![
                ("4", "four"),
                ("69", "sixty-nine"),
                ("2", "two"),
                ("69", "four-twenty"),
            ]
            .into();
            map.remove("69");
            assert_eq!(map.len(), 2);
        }

        #[test]
        fn insert_test_0() {
            let map: SCHashMap<&str, &str> = vec![
                ("4", "four"),
                ("69", "sixty-nine"),
                ("2", "two"),
                ("69", "four-twenty"),
            ]
            .into();
            assert_eq!(map.len(), 3);
        }

        #[test]
        fn insert_test_1() {
            let map: SCHashMap<&str, &str> = vec![
                ("4", "four"),
                ("69", "sixty-nine"),
                ("2", "two"),
                ("69", "four-twenty"),
            ]
            .into();
            assert_eq!(map.get("69"), Some(&"four-twenty"));
        }

        #[test]
        fn remove_test_1() {
            let mut map: SCHashMap<&str, &str> = vec![
                ("4", "four"),
                ("69", "sixty-nine"),
                ("2", "two"),
                ("69", "four-twenty"),
            ]
            .into();
            assert_eq!(map.remove("69"), Some("four-twenty"));
        }

        #[test]
        #[ignore = "not yet implemented"]
        fn entries() {
            todo!()
        }

        #[test]
        #[ignore = "not yet implemented"]
        fn keys() {
            todo!()
        }

        #[test]
        #[ignore = "not yet implemented"]
        fn values() {
            todo!()
        }
    }

    #[cfg(feature = "open-addressing-test")]
    mod open_addressing_tests {
        #[cfg(feature = "linear-probing-test")]
        mod linear_probing_test {
            use crate::{LPHashMap, Map, MapMut, MapUtil};

            #[test]
            fn get_test_0() {
                let map: LPHashMap<&str, &str> =
                    vec![("4", "four"), ("69", "four-twenty"), ("2", "two")].into();
                assert_eq!(map.get("69"), Some(&"four-twenty"));
            }

            #[test]
            fn get_test_1() {
                let map: LPHashMap<&str, &str> =
                    vec![("4", "four"), ("69", "four-twenty"), ("2", "two")].into();
                assert_eq!(map.get("0"), None);
            }

            #[test]
            fn remove_test_0() {
                let mut map: LPHashMap<&str, &str> = vec![
                    ("4", "four"),
                    ("69", "sixty-nine"),
                    ("2", "two"),
                    ("69", "four-twenty"),
                ]
                .into();
                map.remove("69");
                assert_eq!(map.len(), 2);
            }

            #[test]
            fn insert_test_0() {
                let map: LPHashMap<&str, &str> = vec![
                    ("4", "four"),
                    ("69", "sixty-nine"),
                    ("2", "two"),
                    ("69", "four-twenty"),
                ]
                .into();
                assert_eq!(map.len(), 3);
            }

            #[test]
            fn insert_test_1() {
                let map: LPHashMap<&str, &str> = vec![
                    ("4", "four"),
                    ("69", "sixty-nine"),
                    ("2", "two"),
                    ("69", "four-twenty"),
                ]
                .into();
                assert_eq!(map.get("69"), Some(&"four-twenty"));
            }

            #[test]
            fn remove_test_1() {
                let mut map: LPHashMap<&str, &str> = vec![
                    ("4", "four"),
                    ("69", "sixty-nine"),
                    ("2", "two"),
                    ("69", "four-twenty"),
                ]
                .into();
                assert_eq!(map.remove("69"), Some("four-twenty"));
            }

            #[test]
            #[ignore = "not yet implemented"]
            fn entries() {
                todo!()
            }

            #[test]
            #[ignore = "not yet implemented"]
            fn keys() {
                todo!()
            }

            #[test]
            #[ignore = "not yet implemented"]
            fn values() {
                todo!()
            }
        }

        #[cfg(feature = "double-hashing-test")]
        mod double_hashing_test {
            use crate::{DHHashMap, Map, MapMut, MapUtil};

            #[test]
            fn get_test_0() {
                let map: DHHashMap<&str, &str> =
                    vec![("4", "four"), ("69", "four-twenty"), ("2", "two")].into();
                assert_eq!(map.get("69"), Some(&"four-twenty"));
            }

            #[test]
            fn get_test_1() {
                let map: DHHashMap<&str, &str> =
                    vec![("4", "four"), ("69", "four-twenty"), ("2", "two")].into();
                assert_eq!(map.get("0"), None);
            }

            #[test]
            fn remove_test_0() {
                let mut map: DHHashMap<&str, &str> = vec![
                    ("4", "four"),
                    ("69", "sixty-nine"),
                    ("2", "two"),
                    ("69", "four-twenty"),
                ]
                .into();
                map.remove("69");
                assert_eq!(map.len(), 2);
            }

            #[test]
            fn insert_test_0() {
                let map: DHHashMap<&str, &str> = vec![
                    ("4", "four"),
                    ("69", "sixty-nine"),
                    ("2", "two"),
                    ("69", "four-twenty"),
                ]
                .into();
                assert_eq!(map.len(), 3);
            }

            #[test]
            fn insert_test_1() {
                let map: DHHashMap<&str, &str> = vec![
                    ("4", "four"),
                    ("69", "sixty-nine"),
                    ("2", "two"),
                    ("69", "four-twenty"),
                ]
                .into();
                assert_eq!(map.get("69"), Some(&"four-twenty"));
            }

            #[test]
            fn remove_test_1() {
                let mut map: DHHashMap<&str, &str> = vec![
                    ("4", "four"),
                    ("69", "sixty-nine"),
                    ("2", "two"),
                    ("69", "four-twenty"),
                ]
                .into();
                assert_eq!(map.remove("69"), Some("four-twenty"));
            }

            #[test]
            #[ignore = "not yet implemented"]
            fn entries() {
                todo!()
            }

            #[test]
            #[ignore = "not yet implemented"]
            fn keys() {
                todo!()
            }

            #[test]
            #[ignore = "not yet implemented"]
            fn values() {
                todo!()
            }
        }

        #[cfg(feature = "quadratic-probing-test")]
        mod quadratic_probing_test {
            use crate::{Map, MapMut, MapUtil, QPHashMap};

            #[test]
            fn get_test_0() {
                let map: QPHashMap<&str, &str> =
                    vec![("4", "four"), ("69", "four-twenty"), ("2", "two")].into();
                assert_eq!(map.get("69"), Some(&"four-twenty"));
            }

            #[test]
            fn get_test_1() {
                let map: QPHashMap<&str, &str> =
                    vec![("4", "four"), ("69", "four-twenty"), ("2", "two")].into();
                assert_eq!(map.get("0"), None);
            }

            #[test]
            fn remove_test_0() {
                let mut map: QPHashMap<&str, &str> = vec![
                    ("4", "four"),
                    ("69", "sixty-nine"),
                    ("2", "two"),
                    ("69", "four-twenty"),
                ]
                .into();
                map.remove("69");
                assert_eq!(map.len(), 2);
            }

            #[test]
            fn insert_test_0() {
                let map: QPHashMap<&str, &str> = vec![
                    ("4", "four"),
                    ("69", "sixty-nine"),
                    ("2", "two"),
                    ("69", "four-twenty"),
                ]
                .into();
                assert_eq!(map.len(), 3);
            }

            #[test]
            fn insert_test_1() {
                let map: QPHashMap<&str, &str> = vec![
                    ("4", "four"),
                    ("69", "sixty-nine"),
                    ("2", "two"),
                    ("69", "four-twenty"),
                ]
                .into();
                assert_eq!(map.get("69"), Some(&"four-twenty"));
            }

            #[test]
            fn remove_test_1() {
                let mut map: QPHashMap<&str, &str> = vec![
                    ("4", "four"),
                    ("69", "sixty-nine"),
                    ("2", "two"),
                    ("69", "four-twenty"),
                ]
                .into();
                assert_eq!(map.remove("69"), Some("four-twenty"));
            }

            #[test]
            #[ignore = "not yet implemented"]
            fn entries() {
                todo!()
            }

            #[test]
            #[ignore = "not yet implemented"]
            fn keys() {
                todo!()
            }

            #[test]
            #[ignore = "not yet implemented"]
            fn values() {
                todo!()
            }
        }
    }
}
