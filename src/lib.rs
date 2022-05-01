#[cfg(feature = "enabled")]
mod tools;

#[cfg(feature = "enabled")]
pub use tools::*;

pub const DEFAULT_CAPACITY: usize = 17;

#[cfg(test)]
#[cfg(feature = "test")]
mod tests {
    use super::*;

    #[cfg(feature = "sc-test")]
    mod sc_tests {
        use super::{Map, SCHashMap};
        #[test]
        fn get_test_0() {
            let map: SCHashMap<&str, &str> =
                vec![("4", "four"), ("69", "four-twenty"), ("2", "two")].into();
            assert_eq!(map.get("69"), Some("four-twenty"));
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
            assert_eq!(map.get("69"), Some("four-twenty"));
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
        fn entries() {
            let mut data = vec![
                ("4", "four"),
                ("69", "sixty-nine"),
                ("2", "two"),
                ("69", "four-twenty"),
            ];

            let map: SCHashMap<&str, &str> = data.clone().into();

            let mut result: Vec<(&str, &str)> = map
                .entries()
                .into_iter()
                .map(|e| (e.key(), e.value()))
                .collect();

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

            let mut result: Vec<&str> = map.entries().into_iter().map(|e| e.key()).collect();

            //  Remove the replaced value
            data.remove(1);

            let mut keys = data.into_iter().map(|e| e.0).collect::<Vec<&str>>();

            keys.sort();
            result.sort();

            assert_eq!(keys, result)
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

            let mut result: Vec<&str> = map.entries().into_iter().map(|e| e.value()).collect();

            //  Remove the replaced value
            data.remove(1);

            let mut values = data.into_iter().map(|e| e.1).collect::<Vec<&str>>();

            values.sort();
            result.sort();

            assert_eq!(values, result)
        }
    }

    #[cfg(feature = "open-addressing-test")]
    mod oa_tests {
        #[cfg(feature = "linear-probing-tests")]
        mod lp_tests {
            use super::{LPHashMap, Map};
            #[test]
            fn get_test_0() {
                let map: LPHashMap<&str, &str> =
                    vec![("4", "four"), ("69", "four-twenty"), ("2", "two")].into();
                assert_eq!(map.get("69"), Some("four-twenty"));
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
                assert_eq!(map.get("69"), Some("four-twenty"));
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
            fn entries() {
                let mut data = vec![
                    ("4", "four"),
                    ("69", "sixty-nine"),
                    ("2", "two"),
                    ("69", "four-twenty"),
                ];

                let map: LPHashMap<&str, &str> = data.clone().into();

                let mut result: Vec<(&str, &str)> = map
                    .entries()
                    .into_iter()
                    .map(|e| (e.key(), e.value()))
                    .collect();

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

                let mut result: Vec<&str> = map.entries().into_iter().map(|e| e.key()).collect();

                //  Remove the replaced value
                data.remove(1);

                let mut keys = data.into_iter().map(|e| e.0).collect::<Vec<&str>>();

                keys.sort();
                result.sort();

                assert_eq!(keys, result)
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

                let mut result: Vec<&str> = map.entries().into_iter().map(|e| e.value()).collect();

                //  Remove the replaced value
                data.remove(1);

                let mut values = data.into_iter().map(|e| e.1).collect::<Vec<&str>>();

                values.sort();
                result.sort();

                assert_eq!(values, result)
            }
        }

        #[cfg(feature = "double-hashing-tests")]
        mod dh_tests {
            use super::{DHHashMap, Map};
            #[test]
            fn get_test_0() {
                let map: DHHashMap<&str, &str> =
                    vec![("4", "four"), ("69", "four-twenty"), ("2", "two")].into();
                assert_eq!(map.get("69"), Some("four-twenty"));
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
                assert_eq!(map.get("69"), Some("four-twenty"));
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
            fn entries() {
                let mut data = vec![
                    ("4", "four"),
                    ("69", "sixty-nine"),
                    ("2", "two"),
                    ("69", "four-twenty"),
                ];

                let map: DHHashMap<&str, &str> = data.clone().into();

                let mut result: Vec<(&str, &str)> = map
                    .entries()
                    .into_iter()
                    .map(|e| (e.key(), e.value()))
                    .collect();

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

                let mut result: Vec<&str> = map.entries().into_iter().map(|e| e.key()).collect();

                //  Remove the replaced value
                data.remove(1);

                let mut keys = data.into_iter().map(|e| e.0).collect::<Vec<&str>>();

                keys.sort();
                result.sort();

                assert_eq!(keys, result)
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

                let mut result: Vec<&str> = map.entries().into_iter().map(|e| e.value()).collect();

                //  Remove the replaced value
                data.remove(1);

                let mut values = data.into_iter().map(|e| e.1).collect::<Vec<&str>>();

                values.sort();
                result.sort();

                assert_eq!(values, result)
            }
        }

        #[cfg(feature = "quadratic-probing-tests")]
        mod qp_tests {
            use super::{Map, QPHashMap};
            #[test]
            fn get_test_0() {
                let map: QPHashMap<&str, &str> =
                    vec![("4", "four"), ("69", "four-twenty"), ("2", "two")].into();
                assert_eq!(map.get("69"), Some("four-twenty"));
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
                assert_eq!(map.get("69"), Some("four-twenty"));
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
            fn entries() {
                let mut data = vec![
                    ("4", "four"),
                    ("69", "sixty-nine"),
                    ("2", "two"),
                    ("69", "four-twenty"),
                ];

                let map: QPHashMap<&str, &str> = data.clone().into();

                let mut result: Vec<(&str, &str)> = map
                    .entries()
                    .into_iter()
                    .map(|e| (e.key(), e.value()))
                    .collect();

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

                let mut result: Vec<&str> = map.entries().into_iter().map(|e| e.key()).collect();

                //  Remove the replaced value
                data.remove(1);

                let mut keys = data.into_iter().map(|e| e.0).collect::<Vec<&str>>();

                keys.sort();
                result.sort();

                assert_eq!(keys, result)
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

                let mut result: Vec<&str> = map.entries().into_iter().map(|e| e.value()).collect();

                //  Remove the replaced value
                data.remove(1);

                let mut values = data.into_iter().map(|e| e.1).collect::<Vec<&str>>();

                values.sort();
                result.sort();

                assert_eq!(values, result)
            }
        }
    }
}
