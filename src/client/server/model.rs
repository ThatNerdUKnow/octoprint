use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize)]
pub struct CurrentUser{
    /// The id of the current user. Unset if guest.
    pub name: String,
    /// The effective list of permissions assigned to the user
    pub permissions: Vec<PermissionRecord>,
    /// The list of groups assigned to the user
    pub groups: Vec<PermissionRecord>
}

#[derive(Serialize,Deserialize)]
pub struct PermissionRecord{
    /// The permission's identifier
    pub key: String,
    /// The permission's name
    pub name: String,
    /// Whether the permission should be considered dangerous due to a 
    /// high responsibility
    pub dangerous: bool,
    /// List of group identifiers for which this permission is enabled by default
    pub default_groups: Vec<String>,
    /// Human readable description of the permission
    pub description: String,
    /// Needs assigned to the permission
    pub needs: Needs
}

#[derive(Serialize,Deserialize)]
pub struct GroupRecord{
    /// The group's identifier
    pub key: String,
    /// The group's name
    pub name: String,
    /// A human readable description of the group
    pub description: String,
    /// The list of permissions assigned to the group (node: this does not include implicit permissions inherited from subgroups)
    pub permissions: Vec<PermissionRecord>,
    /// Subgroups assigned to the group
    pub subgroups: Vec<GroupRecord>,
    /// Effective needs of the group
    pub needs: Needs,
    /// Is this a default group?
    pub default: bool,
    /// Is this group removable?
    pub removable: bool,
    /// Can this group be modified?
    pub changeable: bool,
    /// Can this group be assigned to users or other groups?
    pub toggleable: bool
}

#[derive(Serialize,Deserialize)]
pub struct Needs{
    /// List of `role` needs
    pub role: Vec<String>,
    /// List of `group` needs
    pub group: Vec<String>
}