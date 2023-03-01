use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;

use machine::{machine, transitions};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use parking_lot::RwLock;
use thiserror::Error;

use crate::amount::Amount;
use massa_time::MassaTime;

// const
// TODO: to constants.rs
const VERSIONING_THRESHOLD_TRANSITION_ACCEPTED: &str = "75.0";

// TODO: add more items here
/// Versioning component enum
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum VersioningComponent {
    Address,
    Block,
    VM,
}

/// Version info per component
#[derive(Clone, Debug)]
pub struct VersioningInfo {
    /// brief description of the versioning
    pub name: String,
    /// version
    pub version: u32,
    /// Component concerned by this versioning (e.g. a new Block version)
    pub component: VersioningComponent,
    /// a timestamp at which the version gains its meaning (e.g. accepted in block header)
    pub start: MassaTime,
    /// a timestamp at which the deployment is considered failed (timeout > start)
    pub timeout: MassaTime,
}

// Need Ord / PartialOrd so it is properly sorted in BTreeMap

impl Ord for VersioningInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.start, &self.timeout).cmp(&(other.start, &other.timeout))
    }
}

impl PartialOrd for VersioningInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for VersioningInfo {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.version == other.version
            && self.component == other.component
            && self.start == other.start
            && self.timeout == other.timeout
    }
}

impl Eq for VersioningInfo {}

// Need to impl this manually otherwise clippy is angry :-P
impl Hash for VersioningInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.version.hash(state);
        self.component.hash(state);
        self.start.hash(state);
        self.timeout.hash(state);
    }
}

machine!(
    /// State machine for a Versioning component that tracks the deployment state
    #[allow(missing_docs)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    enum VersioningState {
        /// Initial state
        Defined,
        /// Past start
        Started { threshold: Amount },
        /// Wait for some time before going to active (to let user the time to upgrade)
        LockedIn,
        /// After LockedIn, deployment is considered successful
        Active,
        /// Past the timeout, if LockedIn is not reach
        Failed,
    }
);

impl Default for VersioningState {
    fn default() -> Self {
        Self::Defined(Defined {})
    }
}

#[allow(missing_docs)]
#[derive(IntoPrimitive, Debug, Clone, Eq, PartialEq, TryFromPrimitive, PartialOrd, Ord)]
#[repr(u32)]
pub enum VersioningStateTypeId {
    Error = 0,
    Defined = 1,
    Started = 2,
    LockedIn = 3,
    Active = 4,
    Failed = 5,
}

impl From<&VersioningState> for VersioningStateTypeId {
    fn from(value: &VersioningState) -> Self {
        match value {
            VersioningState::Error => VersioningStateTypeId::Error,
            VersioningState::Defined(_) => VersioningStateTypeId::Defined,
            VersioningState::Started(_) => VersioningStateTypeId::Started,
            VersioningState::LockedIn(_) => VersioningStateTypeId::LockedIn,
            VersioningState::Active(_) => VersioningStateTypeId::Active,
            VersioningState::Failed(_) => VersioningStateTypeId::Failed,
        }
    }
}

/// A message to update the `VersioningState`
#[derive(Clone, Debug)]
pub struct Advance {
    /// from VersioningInfo.start
    pub start_timestamp: MassaTime,
    /// from VersioningInfo.timeout
    pub timeout: MassaTime,
    /// % of past blocks with this version
    pub threshold: Amount,
    /// Current time (timestamp)
    pub now: MassaTime,
}

impl Default for Advance {
    fn default() -> Self {
        Self {
            start_timestamp: MassaTime::from(0),
            timeout: MassaTime::from(0),
            threshold: Default::default(),
            now: MassaTime::from(0),
        }
    }
}

// Need Ord / PartialOrd so it is properly sorted in BTreeMap

impl Ord for Advance {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.now).cmp(&other.now)
    }
}

impl PartialOrd for Advance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Advance {
    fn eq(&self, other: &Self) -> bool {
        self.start_timestamp == other.start_timestamp
            && self.timeout == other.timeout
            && self.threshold == other.threshold
            && self.now == other.now
    }
}

impl Eq for Advance {}

transitions!(VersioningState,
    [
        (Defined, Advance) => [Defined, Started, Failed],
        (Started, Advance) => [Started, LockedIn, Failed],
        (LockedIn, Advance) => [LockedIn, Active],
        (Active, Advance) => Active,
        (Failed, Advance) => Failed
    ]
);

impl Defined {
    /// Update state from state Defined
    pub fn on_advance(self, input: Advance) -> VersioningState {
        println!("[From defined], advance now: {}", input.now);
        match input.now {
            n if n > input.timeout => VersioningState::failed(),
            n if n > input.start_timestamp => VersioningState::started(Amount::zero()),
            _ => VersioningState::Defined(Defined {}),
        }
    }
}

impl Started {
    /// Update state from state Started
    pub fn on_advance(self, input: Advance) -> VersioningState {
        /* println!(
            "[From started], advance now: {} - threshold: {:?}",
            input.now, input.threshold
        ); */
        if input.now > input.timeout {
            return VersioningState::failed();
        }

        // Safe to unwrap as we assume the constant is well defined & ok for Amount::from_str
        if input.threshold >= Amount::from_str(VERSIONING_THRESHOLD_TRANSITION_ACCEPTED).unwrap() {
            VersioningState::locked_in()
        } else {
            VersioningState::started(input.threshold)
        }
    }
}

impl LockedIn {
    /// Update state from state LockedIn ...
    pub fn on_advance(self, input: Advance) -> VersioningState {
        if input.now > input.timeout {
            VersioningState::active()
        } else {
            VersioningState::locked_in()
        }
    }
}

impl Active {
    /// Update state (will always stay in state Active)
    pub fn on_advance(self, _input: Advance) -> Active {
        Active {}
    }
}

impl Failed {
    /// Update state (will always stay in state Failed)
    pub fn on_advance(self, _input: Advance) -> Failed {
        Failed {}
    }
}

/// Wrapper of VersioningState (in order to keep state history)
#[derive(Debug, Clone, PartialEq)]
struct VersioningStateHistory {
    state: VersioningState,
    history: BTreeMap<Advance, VersioningStateTypeId>,
}

impl VersioningStateHistory {
    /// Create
    pub fn new(defined: MassaTime) -> Self {
        let state: VersioningState = Default::default();
        let state_id = VersioningStateTypeId::from(&state);
        let mut advance = Advance::default();
        advance.now = defined;
        let history = BTreeMap::from([(advance, state_id)]);
        Self {
            state: Default::default(),
            history,
        }
    }

    /// Advance state
    pub fn on_advance(&mut self, input: &Advance) {
        let now = input.now;
        // Check that input.now is not after last item in history
        // We don't want to go backward
        let is_forward = self
            .history
            .last_key_value()
            .map(|(adv, _)| adv.now < now)
            .unwrap_or(false);

        if is_forward {
            // machines crate (for state machine) does not support passing ref :-/
            let state = self.state.on_advance(input.clone());
            println!("From state: {:?} to {:?}", self.state, state);
            // Update history as well
            if state != self.state {
                let state_id = VersioningStateTypeId::from(&state);
                self.history.insert(input.clone(), state_id);
                self.state = state;
            }
        }
    }

    /// Given a corresponding VersioningInfo, check if state is coherent
    /// it is coherent
    ///   if state can be at this position (e.g. can it be at state "Started" according to given time range)
    ///   if history is coherent with current state
    /// Return false for state == VersioningState::Error
    pub fn is_coherent_with(&self, versioning_info: &VersioningInfo) -> bool {
        // Always return false for state Error or if history is empty
        if matches!(&self.state, &VersioningState::Error) || self.history.is_empty() {
            return false;
        }

        // safe to unwrap (already tested if empty or not)
        let (initial_ts, initial_state_id) = self.history.first_key_value().unwrap();
        if *initial_state_id != VersioningStateTypeId::Defined {
            // self.history does not start with Defined -> (always) false
            return false;
        }

        // Build a new VersionStateHistory from initial state, replay the whole history
        // but with given versioning info) then compare
        println!(
            "Start replay'ing history, history len: {:?}",
            self.history.len()
        );
        let mut vsh = VersioningStateHistory::new(initial_ts.now);
        let mut threshold = Amount::zero();
        let mut advance_msg = Advance {
            start_timestamp: versioning_info.start,
            timeout: versioning_info.timeout,
            threshold,
            now: initial_ts.now,
        };

        for (adv, state) in self.history.iter().skip(1) {
            println!("[Replay] adv: {:?} - state: {:?}", adv, state);
            /*
            threshold = match state {
                VersioningState::Started(Started { threshold }) => threshold.clone(),
                _ => Amount::zero(),
            };
            */
            advance_msg.now = adv.now;
            advance_msg.threshold = adv.threshold;
            println!("[Replay] advance with msg: {:?}", advance_msg);
            vsh.on_advance(&advance_msg);
        }

        println!("built vsh: {:?}", vsh);
        println!("self: {:?}", self);
        // XXX: is there always full eq? Can we have slight variation here?
        let mut res = vsh == *self;
        res
    }

    /// Query state at given timestamp
    pub fn state_at(
        &self,
        ts: MassaTime,
        start: MassaTime,
        timeout: MassaTime,
    ) -> Result<VersioningStateTypeId, StateAtError> {
        if self.history.is_empty() {
            return Err(StateAtError::EmptyHistory);
        }

        // Optim: this avoids iterating over history (cheap to retrieve first item)
        let first = self.history.first_key_value().unwrap(); // safe to unwrap
        if ts < first.0.now {
            // Before initial state
            return Err(StateAtError::BeforeInitialState(first.1.clone(), ts));
        }

        // At this point, we are >= the first state in history
        let mut lower_bound = None;
        let mut higher_bound = None;
        let mut is_after_last = false;

        // Optim: this avoids iterating over history (cheap to retrieve first item)
        let last = self.history.last_key_value().unwrap(); // safe to unwrap
        if ts > last.0.now {
            lower_bound = Some(last);
            is_after_last = true;
        }

        if !is_after_last {
            for (adv, state_id) in self.history.iter() {
                if adv.now <= ts {
                    lower_bound = Some((adv, state_id));
                }
                if adv.now >= ts && higher_bound.is_none() {
                    higher_bound = Some((adv, state_id));
                    break;
                }
            }
        }

        println!("Lower bound: {lower_bound:?}");
        println!("Higher bound: {higher_bound:?}");

        match (lower_bound, higher_bound) {
            (Some((_adv_1, st_id_1)), Some((_adv_2, _st_id_2))) => {
                println!("Between 2 states in history");
                // Between 2 states (e.g. between Defined && Started) -> return Defined
                Ok(st_id_1.clone())
            }
            (Some((adv, st_id)), None) => {
                // After the last state in history -> need to advance the state and return
                println!(
                    "After last state in history, start: {:?}, now: {:?}",
                    start, ts
                );

                let threshold_for_transition =
                    Amount::from_str(VERSIONING_THRESHOLD_TRANSITION_ACCEPTED).unwrap();
                // Note: Please update this if VersioningState transitions change as it might not hold true
                if *st_id == VersioningStateTypeId::Started
                    && adv.threshold < threshold_for_transition
                    && ts < adv.timeout
                {
                    return Err(StateAtError::Unpredictable);
                } else {
                    let msg = Advance {
                        start_timestamp: start,
                        timeout,
                        threshold: adv.threshold,
                        now: ts,
                    };
                    // Return the resulting state after advance
                    let state = self.state.on_advance(msg);
                    Ok(VersioningStateTypeId::from(&state))
                }
            }
            _ => {
                // 1. Before the first state in history: already covered
                // 2. None, None: already covered - empty history
                Err(StateAtError::EmptyHistory)
            }
        }
    }
}

/// Error returned by VersioningStateHistory::state_at
#[allow(missing_docs)]
#[derive(Error, Debug, PartialEq)]
pub enum StateAtError {
    #[error("Initial state ({0:?}) only defined after timestamp: {1}")]
    BeforeInitialState(VersioningStateTypeId, MassaTime),
    #[error("Empty history, should never happen")]
    EmptyHistory,
    #[error("Cannot predict in the future (~ threshold not reach yet)")]
    Unpredictable,
}

// Store

/// Database for all versioning info
#[derive(Debug, Clone)]
pub struct VersioningStore(pub Arc<RwLock<VersioningStoreRaw>>);

impl Default for VersioningStore {
    fn default() -> Self {
        Self(Arc::new(RwLock::new(Default::default())))
    }
}

impl VersioningStore {
    /// Retrieve the current "global" version to set in block header
    pub fn get_version_current(&self) -> u32 {
        let lock = self.0.read();
        let store = lock.deref();
        // Current version == last active
        store
            .0
            .iter()
            .rev()
            .find_map(|(k, v)| (v.state == VersioningState::active()).then_some(k.version))
            .unwrap_or(0)
    }

    /// Retrieve the "global" version number to announce in block header
    /// return 0 is there is nothing to announce
    pub fn get_version_to_announce(&self) -> u32 {
        let lock = self.0.read();
        let store = lock.deref();
        // Announce the latest versioning info in Started / LockedIn state
        // Defined == Not yet ready to announce
        // Active == current version
        store
            .0
            .iter()
            .rev()
            .find_map(|(k, v)| {
                matches!(
                    &v.state,
                    &VersioningState::Started(_) | &VersioningState::LockedIn(_)
                )
                .then_some(k.version)
            })
            .unwrap_or(0)
    }
}

/// Store of all versioning info
#[derive(Debug, Clone, Default)]
pub struct VersioningStoreRaw(BTreeMap<VersioningInfo, VersioningStateHistory>);

impl VersioningStoreRaw {
    /// Merge our store with another (usually after a bootstrap where we received another store)
    /// 'at' is the time at which 'store_raw' has been received / retrieved (e.g. bootstrap)
    fn merge_with(&mut self, store_raw: &VersioningStoreRaw, at: MassaTime) {
        // iter over items in given store:
        // -> 2 cases: VersioningInfo is already in self store ->
        //             VersioningInfo is not in self.store -> add it (check uniqueness / time ranges)

        // TODO: build list of what to add / update then do it

        for (v_info, v_state) in store_raw.0.iter() {
            // Check if state is ok according to versioning info time range
            if !v_state.is_coherent_with(v_info) {
                // TODO: should we stop the process as soon as we find one incoherent ?
                println!("Not coherent...");
                continue;
            };

            if let Some(v_state_cur) = self.0.get_mut(v_info) {
                // Versioning info is already in store
                // Need to check state before merging
                let v_state_id: u32 = VersioningStateTypeId::from(&v_state.state).into();
                let v_state_cur_ref: &VersioningStateHistory = v_state_cur;
                let v_state_cur_id: u32 =
                    VersioningStateTypeId::from(&v_state_cur_ref.state).into();

                // TODO: should update if started and threshold is >
                match v_state_cur.state {
                    VersioningState::Defined(_)
                    | VersioningState::Started(_)
                    | VersioningState::LockedIn(_) => {
                        // Only accept 'higher' state (e.g. started if defined, lockedin if started...)
                        if v_state_id > v_state_cur_id {
                            *v_state_cur = v_state.clone();
                        }
                    }
                    _ => {
                        // Nothing to do for already Active / Failed / Error
                        // FIXME: reject Error?
                    }
                }
            } else {
                // Versioning info not in store - usually a new one

                // TODO / Optim: build a range of version & a set of names and use this
                // Check if name & version are uniques
                if !self
                    .0
                    .iter()
                    .all(|(i, _)| i.version != v_info.version && i.name != v_info.name)
                {
                    continue;
                }

                // Should we also check for version >
                // Check for time range
                if let Some((last_vi, _)) = self.0.last_key_value() {
                    // TODO: should we add a min duration from start + timeout
                    // TODO: add a const for the min timeout allowed
                    if v_info.start > last_vi.timeout && v_info.timeout > v_info.start {
                        // Time range is ok, let's add it
                        self.0.insert(v_info.clone(), v_state.clone());
                    }
                } else {
                    self.0.insert(v_info.clone(), v_state.clone());
                }
            }
        }
    }
}

// End Store

#[cfg(test)]
mod test {
    use super::*;
    use crate::version::Version;
    use chrono::{Days, NaiveDate, NaiveDateTime};
    use massa_serialization::DeserializeError;

    // Only for unit tests
    impl PartialEq<VersioningState> for VersioningStateHistory {
        fn eq(&self, other: &VersioningState) -> bool {
            self.state == *other
        }
    }

    fn get_a_version_info() -> (NaiveDateTime, NaiveDateTime, VersioningInfo) {
        // A helper function to provide a  default VersioningInfo
        // Models a Massa Improvements Proposal (MIP-0002), transitioning component address to v2

        let start: NaiveDateTime = NaiveDate::from_ymd_opt(2017, 11, 01)
            .unwrap()
            .and_hms_opt(7, 33, 44)
            .unwrap();

        let timeout: NaiveDateTime = NaiveDate::from_ymd_opt(2017, 11, 11)
            .unwrap()
            .and_hms_opt(7, 33, 44)
            .unwrap();

        return (
            start,
            timeout,
            VersioningInfo {
                name: "MIP-0002".to_string(),
                version: 2,
                component: VersioningComponent::Address,
                start: MassaTime::from(start.timestamp() as u64),
                timeout: MassaTime::from(timeout.timestamp() as u64),
            },
        );
    }

    fn advance_state_until(
        at_state: VersioningState,
        versioning_info: &VersioningInfo,
    ) -> VersioningStateHistory {
        // A helper function to advance a state
        // Assume enough time between versioning info start & timeout
        // TODO: allow to give a threshold as arg?

        let start = versioning_info.start;
        let timeout = versioning_info.timeout;

        if matches!(at_state, VersioningState::Error) {
            todo!()
        }

        let mut state = VersioningStateHistory::new(start.saturating_sub(MassaTime::from(1)));

        if matches!(at_state, VersioningState::Defined(_)) {
            return state;
        }

        let mut advance_msg = Advance {
            start_timestamp: start,
            timeout,
            threshold: Default::default(),
            now: start.saturating_add(MassaTime::from(1)),
        };
        state.on_advance(&advance_msg);

        if matches!(at_state, VersioningState::Started(_)) {
            return state;
        }

        if matches!(at_state, VersioningState::Failed(_)) {
            advance_msg.now = timeout.saturating_add(MassaTime::from(1));
            state.on_advance(&advance_msg);
            return state;
        }

        advance_msg.now = start.saturating_add(MassaTime::from(2));
        advance_msg.threshold = Amount::from_str(VERSIONING_THRESHOLD_TRANSITION_ACCEPTED).unwrap();
        state.on_advance(&advance_msg);

        if matches!(at_state, VersioningState::LockedIn(_)) {
            return state;
        }

        advance_msg.now = timeout.saturating_add(MassaTime::from(1));
        state.on_advance(&advance_msg);
        // Active
        return state;
    }

    #[test]
    fn test_state_advance_from_defined() {
        // Test Versioning state transition (from state: Defined)
        let (_, _, vi) = get_a_version_info();
        let mut state: VersioningState = Default::default();
        assert_eq!(state, VersioningState::defined());

        let now = vi.start;
        let mut advance_msg = Advance {
            start_timestamp: vi.start,
            timeout: vi.timeout,
            threshold: Amount::zero(),
            now,
        };

        state = state.on_advance(advance_msg.clone());
        assert_eq!(state, VersioningState::defined());

        let now = vi.start.saturating_add(MassaTime::from(5));
        advance_msg.now = now;
        state = state.on_advance(advance_msg);

        // println!("state: {:?}", state);
        assert_eq!(
            state,
            VersioningState::Started(Started {
                threshold: Amount::zero()
            })
        );
    }

    #[test]
    fn test_state_advance_from_started() {
        // Test Versioning state transition (from state: Started)
        let (_, _, vi) = get_a_version_info();
        let mut state: VersioningState = VersioningState::started(Default::default());

        let now = vi.start;
        let threshold_too_low = Amount::from_str("74.9").unwrap();
        let threshold_ok = Amount::from_str("82.42").unwrap();
        let mut advance_msg = Advance {
            start_timestamp: vi.start,
            timeout: vi.timeout,
            threshold: threshold_too_low,
            now,
        };

        state = state.on_advance(advance_msg.clone());
        assert_eq!(state, VersioningState::started(threshold_too_low));
        advance_msg.threshold = threshold_ok;
        state = state.on_advance(advance_msg);
        assert_eq!(state, VersioningState::locked_in());
    }

    #[test]
    fn test_state_advance_from_locked_in() {
        // Test Versioning state transition (from state: LockedIn)
        let (_, _, vi) = get_a_version_info();
        let mut state: VersioningState = VersioningState::locked_in();

        let now = vi.start;
        let mut advance_msg = Advance {
            start_timestamp: vi.start,
            timeout: vi.timeout,
            threshold: Amount::zero(),
            now,
        };

        state = state.on_advance(advance_msg.clone());
        assert_eq!(state, VersioningState::locked_in());

        advance_msg.now = advance_msg.timeout.saturating_add(MassaTime::from(1));
        state = state.on_advance(advance_msg);
        assert_eq!(state, VersioningState::active());
    }

    #[test]
    fn test_state_advance_from_active() {
        // Test Versioning state transition (from state: Active)
        let (_, _, vi) = get_a_version_info();
        let mut state = VersioningState::active();
        let now = vi.start;
        let advance = Advance {
            start_timestamp: vi.start,
            timeout: vi.timeout,
            threshold: Amount::zero(),
            now,
        };

        state = state.on_advance(advance);
        assert_eq!(state, VersioningState::active());
    }

    #[test]
    fn test_state_advance_from_failed() {
        // Test Versioning state transition (from state: Failed)
        let (_, _, vi) = get_a_version_info();
        let mut state = VersioningState::failed();
        let now = vi.start;
        let advance = Advance {
            start_timestamp: vi.start,
            timeout: vi.timeout,
            threshold: Amount::zero(),
            now,
        };

        state = state.on_advance(advance);
        assert_eq!(state, VersioningState::failed());
    }

    #[test]
    fn test_state_advance_to_failed() {
        // Test Versioning state transition (to state: Failed)
        let (_, _, vi) = get_a_version_info();
        let now = vi.start.saturating_add(MassaTime::from(1));
        let advance_msg = Advance {
            start_timestamp: vi.start,
            timeout: vi.start,
            threshold: Amount::zero(),
            now,
        };

        let mut state: VersioningState = Default::default();
        state = state.on_advance(advance_msg.clone());
        assert_eq!(state, VersioningState::Failed(Failed {}));

        let mut state: VersioningState = VersioningState::started(Default::default());
        state = state.on_advance(advance_msg.clone());
        assert_eq!(state, VersioningState::Failed(Failed {}));
    }

    #[test]
    fn test_state_with_history() {
        // Test VersioningStateHistory::state_at() function

        let (start, _, vi) = get_a_version_info();
        let now_0 = MassaTime::from(start.timestamp() as u64);
        let mut state = VersioningStateHistory::new(now_0);

        assert_eq!(state, VersioningState::defined());

        let now = vi.start.saturating_add(MassaTime::from(15));
        let mut advance_msg = Advance {
            start_timestamp: vi.start,
            timeout: vi.timeout,
            threshold: Amount::zero(),
            now,
        };

        // Move from Defined -> Started
        state.on_advance(&advance_msg);
        assert_eq!(state, VersioningState::started(Amount::zero()));

        // Check history
        assert_eq!(state.history.len(), 2);
        assert!(matches!(
            state.history.first_key_value(),
            Some((&Advance { .. }, &VersioningStateTypeId::Defined))
        ));
        assert!(matches!(
            state.history.last_key_value(),
            Some((&Advance { .. }, &VersioningStateTypeId::Started))
        ));

        // Query with timestamp

        // Before Defined
        let state_id_ = state.state_at(
            vi.start.saturating_sub(MassaTime::from(5)),
            vi.start,
            vi.timeout,
        );
        assert!(matches!(
            state_id_,
            Err(StateAtError::BeforeInitialState(_, _))
        ));
        // After Defined timestamp
        let state_id = state.state_at(vi.start, vi.start, vi.timeout).unwrap();
        assert_eq!(state_id, VersioningStateTypeId::Defined);
        // At Started timestamp
        let state_id = state.state_at(now, vi.start, vi.timeout).unwrap();
        assert_eq!(state_id, VersioningStateTypeId::Started);

        // After Started timestamp but before timeout timestamp
        let after_started_ts = now.saturating_add(MassaTime::from(15));
        let state_id_ = state.state_at(after_started_ts, vi.start, vi.timeout);
        assert_eq!(state_id_, Err(StateAtError::Unpredictable));

        // After Started timestamp and after timeout timestamp
        let after_timeout_ts = vi.timeout.saturating_add(MassaTime::from(15));
        let state_id = state
            .state_at(after_timeout_ts, vi.start, vi.timeout)
            .unwrap();
        assert_eq!(state_id, VersioningStateTypeId::Failed);

        // Move from Started to LockedIn
        let threshold = Amount::from_str(VERSIONING_THRESHOLD_TRANSITION_ACCEPTED).unwrap();
        advance_msg.threshold = threshold.saturating_add(Amount::from_str("1.0").unwrap());
        advance_msg.now = now.saturating_add(MassaTime::from(1));
        state.on_advance(&advance_msg);
        assert_eq!(state, VersioningState::locked_in());

        // Query with timestamp
        // After LockedIn timestamp and before timeout timestamp
        let after_locked_in_ts = now.saturating_add(MassaTime::from(10));
        let state_id = state
            .state_at(after_locked_in_ts, vi.start, vi.timeout)
            .unwrap();
        assert_eq!(state_id, VersioningStateTypeId::LockedIn);
        // After LockedIn timestamp and after timeout timestamp
        let state_id = state
            .state_at(after_timeout_ts, vi.start, vi.timeout)
            .unwrap();
        assert_eq!(state_id, VersioningStateTypeId::Active);
    }

    #[test]
    fn test_versioning_store_announce_current() {
        // Test VersioningInfo::get_version_to_announce() & ::get_version_current()

        let (start, timeout, vi) = get_a_version_info();

        let mut vi_2 = vi.clone();
        vi_2.version += 1;
        vi_2.start =
            MassaTime::from(timeout.checked_add_days(Days::new(2)).unwrap().timestamp() as u64);
        vi_2.timeout =
            MassaTime::from(timeout.checked_add_days(Days::new(5)).unwrap().timestamp() as u64);

        // Can only build such object in test - history is empty :-/
        let vs_1 = VersioningStateHistory {
            state: VersioningState::active(),
            history: Default::default(),
        };
        let vs_2 = VersioningStateHistory {
            state: VersioningState::started(Amount::zero()),
            history: Default::default(),
        };

        // TODO: Have VersioningStore::from ?
        let vs_raw = VersioningStoreRaw(BTreeMap::from([(vi.clone(), vs_1), (vi_2.clone(), vs_2)]));
        let vs = VersioningStore(Arc::new(RwLock::new(vs_raw)));

        assert_eq!(vs.get_version_current(), vi.version);
        assert_eq!(vs.get_version_to_announce(), vi_2.version);

        // Test also an empty versioning store
        let vs_raw = VersioningStoreRaw(Default::default());
        let vs = VersioningStore(Arc::new(RwLock::new(vs_raw)));
        assert_eq!(vs.get_version_current(), 0);
        assert_eq!(vs.get_version_to_announce(), 0);
    }

    #[test]
    fn test_is_coherent_with() {
        // Test VersioningStateHistory::is_coherent_with

        // Given the following versioning info, we expect state
        // Defined @ time <= 2
        // Started @ time > 2 && <= 5
        // LockedIn @ time > time(Started) && <= 5
        // Active @time > 5
        let vi_1 = VersioningInfo {
            name: "MIP-0002".to_string(),
            version: 2,
            component: VersioningComponent::Address,
            start: MassaTime::from(2),
            timeout: MassaTime::from(5),
        };
        // Another versioning info (from an attacker) for testing
        let vi_2 = VersioningInfo {
            name: "MIP-0002".to_string(),
            version: 2,
            component: VersioningComponent::Address,
            start: MassaTime::from(7),
            timeout: MassaTime::from(10),
        };

        let vsh = VersioningStateHistory {
            state: VersioningState::Error,
            history: Default::default(),
        };
        // At state Error -> (always) false
        assert_eq!(vsh.is_coherent_with(&vi_1), false);

        let vsh = VersioningStateHistory {
            state: VersioningState::defined(),
            history: Default::default(),
        };
        // At state Defined but no history -> false
        assert_eq!(vsh.is_coherent_with(&vi_1), false);

        let mut vsh = VersioningStateHistory::new(MassaTime::from(1));
        // At state Defined at time 1 -> true, given vi_1 @ time 1
        assert_eq!(vsh.is_coherent_with(&vi_1), true);
        // At state Defined at time 1 -> false given vi_1 @ time 3 (state should be Started)
        // assert_eq!(vsh.is_coherent_with(&vi_1, MassaTime::from(3)), false);

        // Advance to Started
        let now = MassaTime::from(3);
        vsh.on_advance(&Advance {
            start_timestamp: vi_1.start,
            timeout: vi_1.timeout,
            threshold: Amount::zero(),
            now,
        });

        // At state Started at time now -> true
        assert_eq!(vsh.state, VersioningState::started(Amount::zero()));
        assert_eq!(vsh.is_coherent_with(&vi_1), true);

        // Still coherent here (not after timeout)
        assert_eq!(vsh.is_coherent_with(&vi_1), true);
        assert_eq!(vsh.is_coherent_with(&vi_1), true);
        // Not coherent anymore (after timeout) -> should be in state Failed
        // assert_eq!(vsh.is_coherent_with(&vi_1, MassaTime::from(6)), false);
        // Now with another versioning info
        println!("vsh history: {:?}", vsh.history);
        println!("===");
        assert_eq!(vsh.is_coherent_with(&vi_2), false);

        // Advance to LockedIn
        let now = MassaTime::from(4);
        println!("Advancing with threshold...");
        vsh.on_advance(&Advance {
            start_timestamp: vi_1.start,
            timeout: vi_1.timeout,
            threshold: Amount::from_str(VERSIONING_THRESHOLD_TRANSITION_ACCEPTED).unwrap(),
            now,
        });
        println!("After advancing...");

        // At state LockedIn at time now -> true
        assert_eq!(vsh.state, VersioningState::locked_in());
        assert_eq!(vsh.is_coherent_with(&vi_1), true);
        assert_eq!(vsh.is_coherent_with(&vi_1), true);

        // edge cases
        // TODO: history all good but does not start with Defined, start with Started
    }

    #[test]
    fn test_merge_with() {
        let vi_1 = VersioningInfo {
            name: "MIP-0002".to_string(),
            version: 2,
            component: VersioningComponent::Address,
            start: MassaTime::from(2),
            timeout: MassaTime::from(5),
        };

        let vs_1 = advance_state_until(VersioningState::active(), &vi_1);
        assert_eq!(vs_1, VersioningState::active());

        let vi_2 = VersioningInfo {
            name: "MIP-0003".to_string(),
            version: 3,
            component: VersioningComponent::Address,
            start: MassaTime::from(17),
            timeout: MassaTime::from(27),
        };
        let vs_2 = advance_state_until(VersioningState::defined(), &vi_2);
        let mut vs_raw_1 = VersioningStoreRaw(BTreeMap::from([
            (vi_1.clone(), vs_1.clone()),
            (vi_2.clone(), vs_2.clone()),
        ]));

        let vs_2_2 = advance_state_until(VersioningState::locked_in(), &vi_2);
        assert_eq!(vs_2_2, VersioningState::locked_in());

        let vs_raw_2 = VersioningStoreRaw(BTreeMap::from([
            (vi_1.clone(), vs_1.clone()),
            (vi_2.clone(), vs_2_2.clone()),
        ]));

        // TODO: test with now=7 for merge_with(..)
        let now = MassaTime::from(20); // 20s after unix epoch - between vi_1.timeout && vi_2.start
        vs_raw_1.merge_with(&vs_raw_2, now);

        // Expect state 1 (for vi_1) no change, state 2 (for vi_2) updated to "LockedIn"
        assert_eq!(vs_raw_1.0.get(&vi_1).unwrap().state, vs_1.state);
        assert_eq!(vs_raw_1.0.get(&vi_2).unwrap().state, vs_2_2.state);
    }

    #[test]
    fn test_merge_with_invalid() {
        let vi_1 = VersioningInfo {
            name: "MIP-0002".to_string(),
            version: 2,
            component: VersioningComponent::Address,
            start: MassaTime::from(0),
            timeout: MassaTime::from(5),
        };
        let vs_1 = advance_state_until(VersioningState::active(), &vi_1);

        let vi_2 = VersioningInfo {
            name: "MIP-0003".to_string(),
            version: 3,
            component: VersioningComponent::Address,
            start: MassaTime::from(17),
            timeout: MassaTime::from(27),
        };
        let vs_2 = advance_state_until(VersioningState::defined(), &vi_2);

        let mut vs_raw_1 = VersioningStoreRaw(BTreeMap::from([
            (vi_1.clone(), vs_1.clone()),
            (vi_2.clone(), vs_2.clone()),
        ]));

        let mut vi_2_2 = vi_2.clone();
        // Make versioning info invalid (because start == vi_1.timeout)
        vi_2_2.start = MassaTime::from(5);
        let vs_2_2 = advance_state_until(VersioningState::defined(), &vi_2_2);
        let vs_raw_2 = VersioningStoreRaw(BTreeMap::from([
            (vi_1.clone(), vs_1.clone()),
            (vi_2_2.clone(), vs_2_2.clone()),
        ]));

        let now = MassaTime::from(7); // 7s after unix epoch - between vi_1.timeout && vi_2.start
        vs_raw_1.merge_with(&vs_raw_2, now);

        assert_eq!(vs_raw_1.0.get(&vi_1).unwrap().state, vs_1.state);
        assert_eq!(vs_raw_1.0.get(&vi_2).unwrap().state, vs_2.state);
    }
}
