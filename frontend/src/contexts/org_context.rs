use std::rc::Rc;
use chrono::NaiveDate;
use uuid::Uuid;
use yew::prelude::*;
use shared_core::models::Organization;

pub type OrgContextHandle = UseReducerHandle<OrgState>;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct OrgState {
    pub id: Uuid,
    pub name: String,
    pub strict_audit_mode: bool,
    pub locked_until: Option<NaiveDate>,
}

pub enum OrgAction {
    SetOrg(OrgState),
    UpdateLockDate(Option<NaiveDate>),
}

impl Reducible for OrgState {
    type Action = OrgAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            OrgAction::SetOrg(new_state) => Rc::new(new_state),
            OrgAction::UpdateLockDate(new_date) => {
                let mut current = (*self).clone();
                current.locked_until = new_date;
                Rc::new(current)
            }
        }
    }
}

#[hook]
pub fn use_org_context() -> OrgContextHandle {
    use_context::<OrgContextHandle>().unwrap()
}
