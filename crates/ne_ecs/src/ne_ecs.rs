struct Health(i32);
struct Name(&'static str);

//stores ECS data
struct World {
    health_components: Vec<Option<Health>>,
    name_components: Vec<Option<Name>>,
}
impl World {
    fn new() -> Self {
        Self {
            health_components: Vec::new(),
            name_components: Vec::new(),
        }
    }
    fn new_entity(&mut self, health: Option<Health>, name: Option<Name>) {
        self.health_components.push(health);
        self.name_components.push(name);
    }

}

fn iterator()
{
    let zip = world
    .health_components
    .iter()
    .zip(world.name_components.iter());

    //filter out entities that don't have both health, name
    let with_health_and_name =
    zip.filter_map(|(health, name): (&Option<Health>, &Option<Name>)| {
        Some((health.as_ref()?, name.as_ref()?))
    });

    for (health, name) in with_health_and_name {
        if health.0 < 0 {
            println!("{} has perished!", name.0);
        } else {
            println!("{} is still healthy", name.0);
        }
    }
    
}

fn run ()
{
    let mut world = World::new();
    // Icarus's health is *not* looking good.
    world.new_entity(Some(Health(-10)), Some(Name("Icarus"))); 
    // Prometheus is very healthy.
    world.new_entity(Some(Health(100)), Some(Name("Prometheus"))); 
    // Note that Zeus does not have a `Health` component.
    world.new_entity(None, Some(Name("Zeus"))); 
}

