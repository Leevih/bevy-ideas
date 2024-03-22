## Todo (in sense making execution order, indexed by "bevy plugin names")

### Pit

- Make system for resolving nearest stone from pit
- Make resource to store closest stones as a que-like structure

### Minion

- Add inventory array for holding Stone entities or sufferably resort to an integer
- Make system to update minion entities on demand (Event receiver system in the minion plugin)
- Read `UpdateTribeGoalEvent` and update `InstructionSet::SpatialTarget` to stone from nearest stone que

### Movement

- Handle `Minion::Transition::Translation` source from `Minion::InstructionSet::SpatialTarget`

### Stone

- Implement `PickupStoneEvent`
- Handle the event by either despawning the entity and incrementing minion inventory
  / Alternatively we can try if we can store the Stone entity-id into the minions inventory
  and then using that to either "move the stone with the minion" or by allowing the minion to trigger
  events with that id from pit to enable more complex interactions. However it's a streatch at best for now

### Lastly

- Create some kind of game of conclusion when all stones are gathered, make minions iddle or exhibit some nonsensical behaviour
