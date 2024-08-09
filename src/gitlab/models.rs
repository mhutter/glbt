use leptos::{view, IntoView, View};
use serde::Deserialize;

use crate::components::{Badge, Class};

/// The type used as IDs
pub type ID = i32;

/// A Merge REquest
///
/// See: https://docs.gitlab.com/ee/api/merge_requests.html
#[derive(Clone, PartialEq, Eq, Deserialize)]
pub struct MergeRequest {
    pub iid: ID,
    pub id: ID,
    pub project_id: ID,
    pub title: String,
    pub references: References,
    pub sha: String,
    pub web_url: String,
    #[serde(rename = "detailed_merge_status")]
    pub status: DetailedMergeStatus,
    pub closed_at: Option<String>,
    pub merged_at: Option<String>,
}

impl MergeRequest {
    /// Determine if a merge request can be merged
    pub fn can_merge(&self) -> bool {
        self.status == DetailedMergeStatus::Mergeable
    }

    /// Determine if a merge request can be closed
    pub fn can_close(&self) -> bool {
        self.status != DetailedMergeStatus::NotOpen
    }

    pub fn can_reopen(&self) -> bool {
        self.closed_at.is_some() && self.merged_at.is_none()
    }
}

#[derive(Clone, PartialEq, Eq, Deserialize)]
pub struct References {
    pub full: String,
}

#[derive(Deserialize)]
pub struct User {
    pub username: String,
}
/// The detailed status of a Merge Request
///
/// See: https://docs.gitlab.com/ee/api/merge_requests.html#merge-status
#[derive(Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DetailedMergeStatus {
    ApprovalsSyncing,
    BlockedStatus,
    Checking,
    CiMustPass,
    CiStillRunning,
    Conflict,
    DiscussionsNotResolved,
    DraftStatus,
    ExternalStatusChecks,
    JiraAssociationMissing,
    Mergeable,
    NeedRebase,
    NotApproved,
    NotOpen,
    RequestedChanges,
    Unchecked,
}

impl DetailedMergeStatus {
    pub const fn as_str(&self) -> &'static str {
        match *self {
            Self::ApprovalsSyncing => "approvals syncing",
            Self::BlockedStatus => "blocked",
            Self::Checking => "checking",
            Self::CiMustPass => "CI must pass",
            Self::CiStillRunning => "CI still running",
            Self::Conflict => "conflict",
            Self::DiscussionsNotResolved => "discussions not resolved",
            Self::DraftStatus => "draft",
            Self::ExternalStatusChecks => "external checks",
            Self::JiraAssociationMissing => "jira association missing",
            Self::Mergeable => "mergeable",
            Self::NeedRebase => "need rebase",
            Self::NotApproved => "not approved",
            Self::NotOpen => "not open",
            Self::RequestedChanges => "changes requested",
            Self::Unchecked => "unchecked",
        }
    }

    pub const fn description(&self) -> &'static str {
        match *self {
            Self::ApprovalsSyncing => "The merge request’s approvals are syncing.",
            Self::BlockedStatus => "Blocked by another merge request.",
            Self::Checking => "Git is testing if a valid merge is possible.",
            Self::CiMustPass => "A CI/CD pipeline must succeed before merge.",
            Self::CiStillRunning => "A CI/CD pipeline is still running.",
            Self::Conflict => "Conflicts exist between the source and target branches.",
            Self::DiscussionsNotResolved => "All discussions must be resolved before merge.",
            Self::DraftStatus => "Can’t merge because the merge request is a draft.",
            Self::ExternalStatusChecks => "All status checks must pass before merge.",
            Self::JiraAssociationMissing => "The title or description must reference a Jira issue.",
            Self::Mergeable => "The branch can merge cleanly into the target branch.",
            Self::NeedRebase => "The merge request must be rebased.",
            Self::NotApproved => "Approval is required before merge.",
            Self::NotOpen => "The merge request must be open before merge.",
            Self::RequestedChanges => "The merge request has reviewers who have requested changes.",
            Self::Unchecked => "Git has not yet tested if a valid merge is possible. ",
        }
    }

    pub const fn class(&self) -> Class {
        match *self {
            Self::ApprovalsSyncing => Class::Warning,
            Self::BlockedStatus => Class::Danger,
            Self::Checking => Class::Warning,
            Self::CiMustPass => Class::Danger,
            Self::CiStillRunning => Class::Warning,
            Self::Conflict => Class::Danger,
            Self::DiscussionsNotResolved => Class::Danger,
            Self::DraftStatus => Class::Danger,
            Self::ExternalStatusChecks => Class::Danger,
            Self::JiraAssociationMissing => Class::Warning,
            Self::Mergeable => Class::Success,
            Self::NeedRebase => Class::Danger,
            Self::NotApproved => Class::Danger,
            Self::NotOpen => Class::Danger,
            Self::RequestedChanges => Class::Danger,
            Self::Unchecked => Class::Warning,
        }
    }
}

impl IntoView for DetailedMergeStatus {
    fn into_view(self) -> View {
        view! { <Badge class=self.class() label=self.as_str()/> }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Pipeline {
    pub id: i32,
    pub sha: String,
    #[serde(rename = "ref")]
    pub references: String,
    pub status: PipelineStatus,
}

/// The status of a Pipeline
///
/// See: https://docs.gitlab.com/ee/api/pipelines.html#list-project-pipelines
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PipelineStatus {
    Created,
    WaitingForResource,
    Preparing,
    Pending,
    Running,
    Success,
    Failed,
    Canceled,
    Skipped,
    Manual,
    Scheduled,
}

impl PipelineStatus {
    pub const fn class(&self) -> Class {
        match *self {
            Self::Created => Class::Warning,
            Self::WaitingForResource => Class::Warning,
            Self::Preparing => Class::Warning,
            Self::Pending => Class::Warning,
            Self::Running => Class::Warning,
            Self::Success => Class::Success,
            Self::Failed => Class::Danger,
            Self::Canceled => Class::Warning,
            Self::Skipped => Class::Warning,
            Self::Manual => Class::Info,
            Self::Scheduled => Class::Info,
        }
    }

    pub const fn as_str(&self) -> &'static str {
        match *self {
            Self::Created => "created",
            Self::WaitingForResource => "waiting for resource",
            Self::Preparing => "preparing",
            Self::Pending => "pending",
            Self::Running => "running",
            Self::Success => "success",
            Self::Failed => "failed",
            Self::Canceled => "cancelled",
            Self::Skipped => "skipped",
            Self::Manual => "manual",
            Self::Scheduled => "scheduled",
        }
    }
}

impl IntoView for PipelineStatus {
    fn into_view(self) -> View {
        view! { <Badge class=self.class() label=self.as_str()/> }
    }
}
