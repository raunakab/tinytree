#[path = "../common/mod.rs"]
mod common;

use slotmap::DefaultKey;

#[test]
fn test_remove_random_key() {
    let trees = common::fixtures::all();

    let key = DefaultKey::default();

    for mut tree in trees {
        assert!(!tree.contains(key));
        assert!(tree.remove(key, None).is_none());
    }
}

#[test]
fn test_remove_root() {
    let trees = common::fixtures::all_non_empty();

    for mut tree in trees {
        assert!(!tree.is_empty());

        let root_key = tree.root_key().unwrap();
        assert!(tree.remove(root_key, None).is_some());

        assert!(tree.is_empty());
    }
}

#[test]
fn test_remove_leaf() {
    let trees = common::fixtures::all_depth_2_or_greater();

    for mut tree in trees {
        let key = common::utils::get_some_leaf_key(&tree);
        let parent_key = tree.get(key).unwrap().parent_key.unwrap();

        let length = tree.len();
        let parent_child_keys = tree.get(parent_key).unwrap().child_keys;
        let parent_child_keys_length = parent_child_keys.len();
        assert!(tree.contains(parent_key));
        assert!(parent_child_keys.contains(&key));

        tree.remove(key, Some(0)).unwrap();

        let new_length = tree.len();
        let parent_child_keys = tree.get(parent_key).unwrap().child_keys;
        let new_parent_child_keys_length = parent_child_keys.len();
        assert!(!tree.contains(key));
        assert!(!parent_child_keys.contains(&key));

        assert_eq!(new_parent_child_keys_length, parent_child_keys_length - 1);
        assert_eq!(new_length, length - 1);

        assert!(tree.contains(parent_key));
        assert!(!tree.contains(key));
    }
}

#[test]
fn test_remove_child_non_leaf() {
    todo!()
}
