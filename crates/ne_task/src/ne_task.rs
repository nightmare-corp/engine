// mod thread_pool;



//This is a rough sketch of some early ideas. For now use bevy.
//It is important that I understand how threads/tasks/ecs work. Otherwise I won't understand how to use it
//efficiently.
use std::thread;

// thread_pools:vec<std::thread>;

struct thread_pools;

pub fn init_threads()
{
    // count logical cores this process could try to use
    let num = num_cpus::get();
}
pub fn get_pool_count()
{

}
pub fn add_task_priority()
{

}

pub fn add_task_id()
{

}

pub fn add_task_package()
{

}

//These will be assigned together in the same thread, with shared resources.
struct task_package
{
    // id:vec<task>,
}

//I don't actually like this, the priority and id aren't use continiously.
//It's better to just use it as a parameter, maybe.
//OR ecs might have an answer for this.
struct task
{
    priority:u32, //high priority will be asigned a private thread.
    id:u32,
    func:fn(), //should be replaced by some kind of descriptor..?
}


struct schedule
{
    
}

impl schedule
{
    //runs main loop tasks!
    pub fn run()
    {

    }
} 

//idea?
//is it posisble to make multiple pools and make some of them exclusive for certain pipelines?
//so add_task(task, enum::graphics) Maybe it shares the same variables also? 
//o prevent double variabels changes 