#[cfg(feature = "enabled")]
mod tools;

#[cfg(feature = "enabled")]
pub use tools::*;

#[cfg(test)]
#[cfg(feature = "test")]
mod tests {

    #[cfg(feature = "separate-chaining-test")]
    mod separate_chaining_test {
        use crate::{
            map::{IntoMapUtil, Map, MapMut, MapSize},
            util::SCHashMap,
        };

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
        fn iter() {
            let mut data = vec![
                ("4", "four"),
                ("69", "sixty-nine"),
                ("2", "two"),
                ("69", "four-twenty"),
            ];

            let map: SCHashMap<&str, &str> = data.clone().into();
            let mut result: Vec<(&str, &str)> = map.into_iter().collect();

            //  Remove the replaced value
            data.remove(1);

            data.sort();
            result.sort();

            assert_eq!(data, result)
        }

        #[test]
        fn keys() {
            let mut data = vec![
                ("4", "four"),
                ("69", "sixty-nine"),
                ("2", "two"),
                ("69", "four-twenty"),
            ];

            let map: SCHashMap<&str, &str> = data.clone().into();
            let mut result: Vec<&str> = map.into_keys().collect();

            //  Remove the replaced value
            data.remove(1);
            let mut data: Vec<&str> = data.into_iter().map(|(k, _)| k).collect();

            data.sort();
            result.sort();

            assert_eq!(data, result)
        }

        #[test]
        fn values() {
            let mut data = vec![
                ("4", "four"),
                ("69", "sixty-nine"),
                ("2", "two"),
                ("69", "four-twenty"),
            ];

            let map: SCHashMap<&str, &str> = data.clone().into();
            let mut result: Vec<&str> = map.into_values().collect();

            //  Remove the replaced value
            data.remove(1);
            let mut data: Vec<&str> = data.into_iter().map(|(_, v)| v).collect();

            data.sort();
            result.sort();

            assert_eq!(data, result)
        }
    }

    #[cfg(feature = "open-addressing-test")]
    mod open_addressing_tests {
        #[cfg(feature = "linear-probing-test")]
        mod linear_probing_test {
            use crate::{
                map::{IntoMapUtil, Map, MapMut, MapSize},
                util::LPHashMap,
            };

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
            fn iter() {
                let mut data = vec![
                    ("4", "four"),
                    ("69", "sixty-nine"),
                    ("2", "two"),
                    ("69", "four-twenty"),
                ];

                let map: LPHashMap<&str, &str> = data.clone().into();
                let mut result: Vec<(&str, &str)> = map.into_iter().collect();

                //  Remove the replaced value
                data.remove(1);

                data.sort();
                result.sort();

                assert_eq!(data, result)
            }

            #[test]
            fn keys() {
                let mut data = vec![
                    ("4", "four"),
                    ("69", "sixty-nine"),
                    ("2", "two"),
                    ("69", "four-twenty"),
                ];

                let map: LPHashMap<&str, &str> = data.clone().into();
                let mut result: Vec<&str> = map.into_keys().collect();

                //  Remove the replaced value
                data.remove(1);
                let mut data: Vec<&str> = data.into_iter().map(|(k, _)| k).collect();

                data.sort();
                result.sort();

                assert_eq!(data, result)
            }

            #[test]
            fn values() {
                let mut data = vec![
                    ("4", "four"),
                    ("69", "sixty-nine"),
                    ("2", "two"),
                    ("69", "four-twenty"),
                ];

                let map: LPHashMap<&str, &str> = data.clone().into();
                let mut result: Vec<&str> = map.into_values().collect();

                //  Remove the replaced value
                data.remove(1);
                let mut data: Vec<&str> = data.into_iter().map(|(_, v)| v).collect();

                data.sort();
                result.sort();

                assert_eq!(data, result)
            }
        }

        #[cfg(feature = "double-hashing-test")]
        mod double_hashing_test {
            use crate::{
                map::{IntoMapUtil, Map, MapMut, MapSize},
                util::DHHashMap,
            };

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
            fn iter() {
                let mut data = vec![
                    ("4", "four"),
                    ("69", "sixty-nine"),
                    ("2", "two"),
                    ("69", "four-twenty"),
                ];

                let map: DHHashMap<&str, &str> = data.clone().into();
                let mut result: Vec<(&str, &str)> = map.into_iter().collect();

                //  Remove the replaced value
                data.remove(1);

                data.sort();
                result.sort();

                assert_eq!(data, result)
            }

            #[test]
            fn keys() {
                let mut data = vec![
                    ("4", "four"),
                    ("69", "sixty-nine"),
                    ("2", "two"),
                    ("69", "four-twenty"),
                ];

                let map: DHHashMap<&str, &str> = data.clone().into();
                let mut result: Vec<&str> = map.into_keys().collect();

                //  Remove the replaced value
                data.remove(1);
                let mut data: Vec<&str> = data.into_iter().map(|(k, _)| k).collect();

                data.sort();
                result.sort();

                assert_eq!(data, result)
            }

            #[test]
            fn values() {
                let mut data = vec![
                    ("4", "four"),
                    ("69", "sixty-nine"),
                    ("2", "two"),
                    ("69", "four-twenty"),
                ];

                let map: DHHashMap<&str, &str> = data.clone().into();
                let mut result: Vec<&str> = map.into_values().collect();

                //  Remove the replaced value
                data.remove(1);
                let mut data: Vec<&str> = data.into_iter().map(|(_, v)| v).collect();

                data.sort();
                result.sort();

                assert_eq!(data, result)
            }
        }

        #[cfg(feature = "quadratic-probing-test")]
        mod quadratic_probing_test {
            use crate::{
                map::{IntoMapUtil, Map, MapMut, MapSize},
                util::QPHashMap,
            };

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
            fn iter() {
                let mut data = vec![
                    ("4", "four"),
                    ("69", "sixty-nine"),
                    ("2", "two"),
                    ("69", "four-twenty"),
                ];

                let map: QPHashMap<&str, &str> = data.clone().into();
                let mut result: Vec<(&str, &str)> = map.into_iter().collect();

                //  Remove the replaced value
                data.remove(1);

                data.sort();
                result.sort();

                assert_eq!(data, result)
            }

            #[test]
            fn keys() {
                let mut data = vec![
                    ("4", "four"),
                    ("69", "sixty-nine"),
                    ("2", "two"),
                    ("69", "four-twenty"),
                ];

                let map: QPHashMap<&str, &str> = data.clone().into();
                let mut result: Vec<&str> = map.into_keys().collect();

                //  Remove the replaced value
                data.remove(1);
                let mut data: Vec<&str> = data.into_iter().map(|(k, _)| k).collect();

                data.sort();
                result.sort();

                assert_eq!(data, result)
            }

            #[test]
            fn values() {
                let mut data = vec![
                    ("4", "four"),
                    ("69", "sixty-nine"),
                    ("2", "two"),
                    ("69", "four-twenty"),
                ];

                let map: QPHashMap<&str, &str> = data.clone().into();
                let mut result: Vec<&str> = map.into_values().collect();

                //  Remove the replaced value
                data.remove(1);
                let mut data: Vec<&str> = data.into_iter().map(|(_, v)| v).collect();

                data.sort();
                result.sort();

                assert_eq!(data, result)
            }
        }
    }
}
