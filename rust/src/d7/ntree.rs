/// https://developerlife.com/2022/02/24/rust-non-binary-tree/
///

type NodeDataRef<T> = Arc<NodeData<T>>;
type WeakNodeNodeRef<T> = Weak<NodeData<T>>;

/// Parent relationship is one of non-ownership.
/// This is not a `RwLock<NodeDataRef<T>>` which would cause memory leak.
type Parent<T> = RwLock<WeakNodeNodeRef<T>>;

/// Children relationship is one of ownership.
type Children<T> = RwLock<Vec<Child<T>>>;
type Child<T> = NodeDataRef<T>;

/// This struct holds underlying data. It shouldn't be created directly, instead use:
/// [`Node`](struct@Node).
///
/// ```text
/// NodeData
///  | | |
///  | | +- value: T ---------------------------------------+
///  | |                                                    |
///  | |                                        Simple onwership of value
///  | |
///  | +-- parent: RwLock<WeakNodeNodeRef<T>> --------+
///  |                                                |
///  |                     This describes a non-ownership relationship.
///  |                     When a node is dropped, its parent will not be dropped.
///  |
///  +---- children: RwLock<Vec<Child<T>>> ---+
///                                           |
///                 This describes an ownership relationship.
///                 When a node is dropped its children will be dropped as well.
/// ```
pub struct NodeData<T>
where
  T: Display,
{
  value: T,
  parent: Parent<T>,
  children: Children<T>,
}

/// This struct is used to own a [`NodeData`] inside an [`Arc`]. The [`Arc`]
/// can be shared, so that it can have multiple owners. It does not have
/// getter methods for [`NodeData`]'s properties, instead it implements the
/// `Deref` trait to allow it to be used as a [`NodeData`].
///
/// # Shared ownership
///
/// After an instance of this struct is created and it's internal reference is
/// cloned (and given to another) dropping this instance will not drop the cloned
/// internal reference.
///
/// ```text
/// Node { arc_ref: Arc<NodeData> }
///    ▲                 ▲
///    │                 │
///    │      This atomic ref owns the
///    │      `NodeData` & is shared
///    │
///    1. Has methods to manipulate nodes and their children.
///
///    2. When it is dropped, if there are other `Arc`s (shared via
///       `get_copy_of_internal_arc()`) pointing to the same underlying
///       `NodeData`, then the `NodeData` will not be dropped.
///
///    3. This struct is necessary in order for `add_child_and_update_its_parent`
///       to work. Some pointers need to be swapped between 2 nodes for this work
///       (and one of these pointers is a weak one). It is not possible to do this
///       using two `NodeData` objects, without wrapping them in `Arc`s.
/// ```

#[derive(Debug)]
pub struct Node<T: Display> {
  arc_ref: NodeDataRef<T>,
}

impl<T> Deref for Node<T>
where
  T: Display,
{
  type Target = NodeData<T>;

  fn deref(&self) -> &Self::Target {
    &self.arc_ref
  }
}

impl<T> Node<T>
where
  T: Display,
{
  pub fn new(value: T) -> Node<T> {
    let new_node = NodeData {
      value,
      parent: RwLock::new(Weak::new()),
      children: RwLock::new(Vec::new()),
    };
    let arc_ref = Arc::new(new_node);
    Node { arc_ref }
  }

  pub fn get_copy_of_internal_arc(self: &Self) -> NodeDataRef<T> {
    Arc::clone(&self.arc_ref)
  }

  pub fn create_and_add_child(
    self: &Self,
    value: T,
  ) -> NodeDataRef<T> {
    let new_child = Node::new(value);
    self.add_child_and_update_its_parent(&new_child);
    new_child.get_copy_of_internal_arc()
  }

  /// 🔏 Write locks used.
  pub fn add_child_and_update_its_parent(
    self: &Self,
    child: &Node<T>,
  ) {
    {
      let mut my_children = self.arc_ref.children.write().unwrap();
      my_children.push(child.get_copy_of_internal_arc());
    } // `my_children` guard dropped.

    {
      let mut childs_parent = child.arc_ref.parent.write().unwrap();
      *childs_parent = Arc::downgrade(&self.get_copy_of_internal_arc());
    } // `my_parent` guard dropped.
  }

  pub fn has_parent(self: &Self) -> bool {
    self.get_parent().is_some()
  }

  /// 🔒 Read lock used.
  pub fn get_parent(self: &Self) -> Option<NodeDataRef<T>> {
    let my_parent_weak = self.arc_ref.parent.read().unwrap();
    if let Some(my_parent_arc_ref) = my_parent_weak.upgrade() {
      Some(my_parent_arc_ref)
    } else {
      None
    }
  }
}

trait HasId {
    type Id;
    fn id(&self) -> &Self::Id;
  }

struct Node {
id: i32,
payload: String,
children: Vec<i32>,
}

impl HasId for Node {
type Id = i32;

fn id(&self) -> &Self::Id {
    &self.id
}
}

impl HasId for i32 {
type Id = i32;

fn id(&self) -> &Self::Id {
    self
}
}