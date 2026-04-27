use crate::{ComponentId, EntityId, IdentityDomain};
use engine_core::Generation;

#[derive(Debug)]
pub struct IdentityAllocator {
    domain: IdentityDomain,
    generations: Vec<Generation>,
    alive: Vec<bool>,
    free: Vec<(u32, u64)>,
    epoch: u64,
}

impl IdentityAllocator {
    pub fn new(domain: IdentityDomain) -> Self {
        Self {
            domain,
            generations: Vec::new(),
            alive: Vec::new(),
            free: Vec::new(),
            epoch: 0,
        }
    }

    pub fn advance_epoch(&mut self) {
        self.epoch = self.epoch.saturating_add(1);
    }

    pub fn issue_entity(&mut self) -> Option<EntityId> {
        (self.domain == IdentityDomain::Entity).then(|| {
            let (slot, generation) = self.allocate_slot();
            EntityId { slot, generation }
        })
    }

    pub fn issue_component(&mut self) -> Option<ComponentId> {
        (self.domain == IdentityDomain::Component).then(|| {
            let (slot, generation) = self.allocate_slot();
            ComponentId { slot, generation }
        })
    }

    pub fn retire_entity(&mut self, id: EntityId) -> bool {
        self.retire_slot(id.slot, id.generation)
    }
    pub fn retire_component(&mut self, id: ComponentId) -> bool {
        self.retire_slot(id.slot, id.generation)
    }
    pub fn is_live_entity(&self, id: EntityId) -> bool {
        self.is_live(id.slot, id.generation)
    }
    pub fn is_live_component(&self, id: ComponentId) -> bool {
        self.is_live(id.slot, id.generation)
    }

    fn allocate_slot(&mut self) -> (u32, Generation) {
        if let Some((index, (slot, _))) = self
            .free
            .iter()
            .enumerate()
            .find(|(_, (_, ready_epoch))| *ready_epoch <= self.epoch)
        {
            let slot = *slot;
            self.free.swap_remove(index);
            self.alive[slot as usize] = true;
            return (slot, self.generations[slot as usize]);
        }
        let slot = self.generations.len() as u32;
        self.generations.push(Generation::INITIAL);
        self.alive.push(true);
        (slot, Generation::INITIAL)
    }

    fn retire_slot(&mut self, slot: u32, generation: Generation) -> bool {
        let idx = slot as usize;
        if idx >= self.generations.len() || !self.alive[idx] || self.generations[idx] != generation
        {
            return false;
        }
        self.alive[idx] = false;
        self.generations[idx] = self.generations[idx].next();
        self.free.push((slot, self.epoch + 1));
        true
    }

    fn is_live(&self, slot: u32, generation: Generation) -> bool {
        let idx = slot as usize;
        idx < self.generations.len() && self.alive[idx] && self.generations[idx] == generation
    }
}
