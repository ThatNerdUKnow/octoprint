use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize)]
pub struct CurrentUser{
    /// The id of the current user. Unset if guest.
    name: String,
    /// The effective list of permissions assigned to the user
    permissions: Vec<PermissionRecord>,
    /// The list of groups assigned to the user
    groups: Vec<PermissionRecord>
}

#[derive(Serialize,Deserialize)]
pub struct PermissionRecord{
    /// The permission's identifier
    key: String,
    /// The permission's name
    name: String,
    /// Whether the permission should be considered dangerous due to a 
    /// high responsibility
    dangerous: bool,
    /// List of group identifiers for which this permission is enabled by default
    default_groups: Vec<String>,
    /// Human readable description of the permission
    description: String,
    /// Needs assigned to the permission
    needs: Needs
}

#[derive(Serialize,Deserialize)]
pub struct GroupRecord{
    /// The group's identifier
    key: String,
    /// The group's name
    name: String,
    /// A human readable description of the group
    description: String,
    /// The list of permissions assigned to the group (node: this does not include implicit permissions inherited from subgroups)
    permissions: Vec<PermissionRecord>,
    /// Subgroups assigned to the group
    subgroups: Vec<GroupRecord>,
    /// Effective needs of the group
    needs: Needs,
    /// Is this a default group?
    default: bool,
    /// Is this group removable?
    removable: bool,
    /// Can this group be modified?
    changeable: bool,
    /// Can this group be assigned to users or other groups?
    toggleable: bool
}

#[derive(Serialize,Deserialize)]
pub struct Needs{
    /// List of `role` needs
    role: Vec<String>,
    /// List of `group` needs
    group: Vec<String>
}