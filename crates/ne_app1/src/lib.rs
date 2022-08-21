//Thanks bevy!

use std::collections::HashMap;

// use bevy_ecs::schedule::IntoSystemDescriptor;
use bevy_ecs::{world::{World, FromWorld}, schedule::{IntoSystemDescriptor, StageLabel, Schedule, Stage, SystemStage, ShouldRun}, system::{IntoExclusiveSystem, Resource}};
pub use ne::*;

    //================================================================
    //TODO Make this my own code
    //================================================================
    #[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
    pub enum CoreStage {
        /// The [`Stage`](bevy_ecs::schedule::Stage) that runs before all other app stages.
        First,
        /// The [`Stage`](bevy_ecs::schedule::Stage) that runs before [`CoreStage::Update`].
        PreUpdate,
        /// The [`Stage`](bevy_ecs::schedule::Stage) responsible for doing most app logic. Systems should be registered here by default.
        Update,
        /// The [`Stage`](bevy_ecs::schedule::Stage) that runs after [`CoreStage::Update`].
        PostUpdate,
        /// The [`Stage`](bevy_ecs::schedule::Stage) that runs after all other app stages.
        Last,
    }
    /// The names of the default [`App`] startup stages.
    #[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
    pub enum StartupStage {
        /// The [`Stage`](bevy_ecs::schedule::Stage) that runs once before [`StartupStage::Startup`].
        PreStartup,
        /// The [`Stage`](bevy_ecs::schedule::Stage) that runs once when an [`App`] starts up.
        Startup,
        /// The [`Stage`](bevy_ecs::schedule::Stage) that runs once after [`StartupStage::Startup`].
        PostStartup,
    }
    /// The label for the startup [`Schedule`](bevy_ecs::schedule::Schedule),
    /// which runs once at the beginning of the [`App`].
    ///
    /// When targeting a [`Stage`](bevy_ecs::schedule::Stage) inside this [`Schedule`](bevy_ecs::schedule::Schedule),
    /// you need to use [`StartupStage`] instead.
    #[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
    pub struct StartupSchedule;

/// Macro to define a new label trait
///
/// # Example
///
/// ```
/// # use bevy_utils::define_label;
/// define_label!(
///     /// A class of labels.
///     MyNewLabelTrait,
///     /// Identifies a value that implements `MyNewLabelTrait`.
///     MyNewLabelId,
/// );
/// ```
#[macro_export]
macro_rules! define_label {
    (
        $(#[$label_attr:meta])*
        $label_name:ident,

        $(#[$id_attr:meta])*
        $id_name:ident $(,)?
    ) => {
        $(#[$id_attr])*
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $id_name(::core::any::TypeId, &'static str);

        impl ::core::fmt::Debug for $id_name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                write!(f, "{}", self.1)
            }
        }

        $(#[$label_attr])*
        pub trait $label_name: 'static {
            /// Converts this type into an opaque, strongly-typed label.
            fn as_label(&self) -> $id_name {
                let id = self.type_id();
                let label = self.as_str();
                $id_name(id, label)
            }
            /// Returns the [`TypeId`] used to differentiate labels.
            fn type_id(&self) -> ::core::any::TypeId {
                ::core::any::TypeId::of::<Self>()
            }
            /// Returns the representation of this label as a string literal.
            ///
            /// In cases where you absolutely need a label to be determined at runtime,
            /// you can use [`Box::leak`] to get a `'static` reference.
            fn as_str(&self) -> &'static str;
        }

        impl $label_name for $id_name {
            fn as_label(&self) -> Self {
                *self
            }
            fn type_id(&self) -> ::core::any::TypeId {
                self.0
            }
            fn as_str(&self) -> &'static str {
                self.1
            }
        }

        impl $label_name for &'static str {
            fn as_str(&self) -> Self {
                self
            }
        }
    };
}
    #[cfg(feature = "trace")]
use bevy_utils::tracing::info_span;
    define_label!(
    /// A strongly-typed class of labels used to identify an [`App`].
    AppLabel,
    /// A strongly-typed identifier for an [`AppLabel`].
    AppLabelId,
);
//================================================================
//^^^TODO Make this my own code^^^
//================================================================
#[macro_export]
macro_rules! get_engine_assets_dir{
    () =>
    {
        // find_file!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets")
        // concat!(env!("CARGO_MANIFEST_DIR"), "/assets")
        todo!();
    }
}

pub struct App
{

    //================================================================
    //TODO Make this code my own
    //================================================================
    /// The main ECS [`World`] of the [`App`].
    /// This stores and provides access to all the main data of the application.
    /// The systems of the [`App`] will run using this [`World`].
    /// If additional separate [`World`]-[`Schedule`] pairs are needed, you can use [`sub_app`](App::add_sub_app)s.
    pub world: World,
    /// The [runner function](Self::set_runner) is primarily responsible for managing
    /// the application's event loop and advancing the [`Schedule`].
    /// Typically, it is not configured manually, but set by one of Bevy's built-in plugins.
    /// See `bevy::winit::WinitPlugin` and [`ScheduleRunnerPlugin`](crate::schedule_runner::ScheduleRunnerPlugin).
    pub runner: Box<dyn Fn(App)>,
    // world: World,
    pub schedule: Schedule,
    /// The [runner function](Self::set_runner) is primarily responsible for managing
    /// the application's event loop and advancing the [`Schedule`].
    /// Typically, it is not configured manually, but set by one of Bevy's built-in plugins.
    /// See `bevy::winit::WinitPlugin` and [`ScheduleRunnerPlugin`](crate::schedule_runner::ScheduleRunnerPlugin).
    sub_apps: HashMap<AppLabelId, SubApp>,
}
/// Each `SubApp` has its own [`Schedule`] and [`World`], enabling a separation of concerns.
struct SubApp {
    app: App,
    runner: Box<dyn Fn(&mut World, &mut App)>,
}

impl Default for App {
    fn default() -> Self {
        let mut app = App::empty();
        #[cfg(feature = "bevy_reflect")]
        app.init_resource::<bevy_reflect::TypeRegistryArc>();

        app.add_default_stages()

        //TODO
            // .add_event::<AppExit>()
            .add_system_to_stage(CoreStage::Last, World::clear_trackers.exclusive_system());

        #[cfg(feature = "bevy_ci_testing")]
        {
            crate::ci_testing::setup_app(&mut app);
        }
        app
    }
}

//TODO Is this needed?
// impl Default for app
// {
//     // fn default() -> Self {
//     // }
// }
impl App
{
    pub fn new() -> App
    {
        // App::default()
        App::default()
    }
    pub fn empty() -> App {
        Self {
            world: Default::default(),
            schedule: Default::default(),
            runner: Box::new(run_once),
            sub_apps: HashMap::default(),
        }
    }
    pub fn add_thread(&mut self, func: fn()) -> &mut Self
    {
        
        self
    }
    pub fn add_plugin<T>(&mut self, plugin: T) -> &mut Self
    where
        T: Plugin,
    {
        debug!("Initializing: {}", plugin.name());
        plugin.setup(self);
        self
    }

    //immediately calls function
    pub fn add_startup_func/* <Params> */
    (
        &mut self,
        func: fn(),
    ) -> &mut Self {
        
        // self.add_startup_system_to_stage(StartupStage::Startup, system)
        // system.into_descriptor();
        func();

        self
    }
    /// Inserts a non-send resource to the app.
    ///
    /// You usually want to use [`insert_resource`](Self::insert_resource),
    /// but there are some special cases when a resource cannot be sent across threads.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bevy_app::prelude::*;
    /// # use bevy_ecs::prelude::*;
    /// #
    /// struct MyCounter {
    ///     counter: usize,
    /// }
    ///
    /// App::new()
    ///     .insert_non_send_resource(MyCounter { counter: 0 });
    /// ```
    pub fn insert_non_send_resource<R: 'static>(&mut self, resource: R) -> &mut Self {
        self.world.insert_non_send_resource(resource);
        self
    }

    //================================================================
    //TODO Make this code my own
    //================================================================
    /// The names of the default [`App`] stages.
    ///
    /// The relative [`Stages`](bevy_ecs::schedule::Stage) are added by [`App::add_default_stages`].


    /// Adds a system to the [update stage](Self::add_default_stages) of the app's [`Schedule`].
    ///
    /// Refer to the [system module documentation](bevy_ecs::system) to see how a system
    /// can be defined.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bevy_app::prelude::*;
    /// # use bevy_ecs::prelude::*;
    /// #
    /// # fn my_system() {}
    /// # let mut app = App::new();
    /// #
    /// app.add_system(my_system);
    /// ```
    pub fn add_running<Params>(&mut self, system: impl IntoSystemDescriptor<Params>) -> &mut Self {
        self.add_system_to_stage(CoreStage::Update, system)
    }
    /// Adds a system to the [`Stage`] identified by `stage_label`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bevy_app::prelude::*;
    /// # use bevy_ecs::prelude::*;
    /// #
    /// # let mut app = App::new();
    /// # fn my_system() {}
    /// #
    /// app.add_system_to_stage(CoreStage::PostUpdate, my_system);
    /// ```
    pub fn add_system_to_stage<Params>(
        &mut self,
        stage_label: impl StageLabel,
        system: impl IntoSystemDescriptor<Params>,
    ) -> &mut Self {
        use std::any::TypeId;
        assert!(
            stage_label.type_id() != TypeId::of::<StartupStage>(),
            "add systems to a startup stage using App::add_startup_system_to_stage"
        );
        self.schedule.add_system_to_stage(stage_label, system);
        self
    }

    //TODO this should initialize the job scheduler. And maybe return self
    pub fn run(&mut self)
    {
        println!("run");
        while true
        {
            self.schedule.run(&mut self.world);

            // #[cfg(feature = "trace")]
            // let _bevy_app_run_span = info_span!("bevy_app").entered();
    
            // let mut app = std::mem::replace(self, App::empty());
            // let runner = std::mem::replace(&mut app.runner, Box::new(run_once));
            // (runner)(app);
        }
    }

    /// Adds a [`Stage`] with the given `label` to the last position of the app's
    /// [`Schedule`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use bevy_app::prelude::*;
    /// # use bevy_ecs::prelude::*;
    /// # let mut app = App::new();
    /// #
    /// app.add_stage("my_stage", SystemStage::parallel());
    /// ```
    pub fn add_stage<S: Stage>(&mut self, label: impl StageLabel, stage: S) -> &mut Self {
        self.schedule.add_stage(label, stage);
        self
    }

    /// Adds utility stages to the [`Schedule`], giving it a standardized structure.
    ///
    /// Adding those stages is necessary to make some core engine features work, like
    /// adding systems without specifying a stage, or registering events. This is however
    /// done by default by calling `App::default`, which is in turn called by
    /// [`App::new`].
    ///
    /// # The stages
    ///
    /// All the added stages, with the exception of the startup stage, run every time the
    /// schedule is invoked. The stages are the following, in order of execution:
    ///
    /// - **First:** Runs at the very start of the schedule execution cycle, even before the
    ///   startup stage.
    /// - **Startup:** This is actually a schedule containing sub-stages. Runs only once
    ///   when the app starts.
    ///     - **Pre-startup:** Intended for systems that need to run before other startup systems.
    ///     - **Startup:** The main startup stage. Startup systems are added here by default.
    ///     - **Post-startup:** Intended for systems that need to run after other startup systems.
    /// - **Pre-update:** Often used by plugins to prepare their internal state before the
    ///   update stage begins.
    /// - **Update:** Intended for user defined logic. Systems are added here by default.
    /// - **Post-update:** Often used by plugins to finalize their internal state after the
    ///   world changes that happened during the update stage.
    /// - **Last:** Runs right before the end of the schedule execution cycle.
    ///
    /// The labels for those stages are defined in the [`CoreStage`] and [`StartupStage`] `enum`s.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bevy_app::prelude::*;
    /// #
    /// let app = App::empty().add_default_stages();
    /// ```
    pub fn add_default_stages(&mut self) -> &mut Self {
        self.add_stage(CoreStage::First, SystemStage::parallel())
            .add_stage(
                StartupSchedule,
                Schedule::default()
                    .with_run_criteria(ShouldRun::once)
                    .with_stage(StartupStage::PreStartup, SystemStage::parallel())
                    .with_stage(StartupStage::Startup, SystemStage::parallel())
                    .with_stage(StartupStage::PostStartup, SystemStage::parallel()),
            )
            .add_stage(CoreStage::PreUpdate, SystemStage::parallel())
            .add_stage(CoreStage::Update, SystemStage::parallel())
            .add_stage(CoreStage::PostUpdate, SystemStage::parallel())
            .add_stage(CoreStage::Last, SystemStage::parallel())
    }
    // pub fn add_startup_system<Params>(
    //     &mut self,
    //     system: impl IntoSystemDescriptor<Params>,
    // ) -> &mut Self {
    //     self.add_startup_system_to_stage(StartupStage::Startup, system)
    // }


    /// Advances the execution of the [`Schedule`] by one cycle.
    ///
    /// This method also updates sub apps.
    ///
    /// See [`add_sub_app`](Self::add_sub_app) and [`run_once`](Schedule::run_once) for more details.
    pub fn update(&mut self) {
        #[cfg(feature = "trace")]
        let _bevy_frame_update_span = info_span!("frame").entered();
        self.schedule.run(&mut self.world);
        for sub_app in self.sub_apps.values_mut() {
            (sub_app.runner)(&mut self.world, &mut sub_app.app);
        }
    }
    /// Initialize a non-send [`Resource`] with standard starting values by adding it to the [`World`].
    ///
    /// The [`Resource`] must implement the [`FromWorld`] trait.
    /// If the [`Default`] trait is implemented, the [`FromWorld`] trait will use
    /// the [`Default::default`] method to initialize the [`Resource`].
    pub fn init_non_send_resource<R: 'static + FromWorld>(&mut self) -> &mut Self {
        self.world.init_non_send_resource::<R>();
        self
    }
    /// Inserts a [`Resource`] to the current [`App`] and overwrites any [`Resource`] previously added of the same type.
    ///
    /// A [`Resource`] in Bevy represents globally unique data. [`Resource`]s must be added to Bevy apps
    /// before using them. This happens with [`insert_resource`](Self::insert_resource).
    ///
    /// See [`init_resource`](Self::init_resource) for [`Resource`]s that implement [`Default`] or [`FromWorld`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use bevy_app::prelude::*;
    /// #
    /// struct MyCounter {
    ///     counter: usize,
    /// }
    ///
    /// App::new()
    ///    .insert_resource(MyCounter { counter: 0 });
    /// ```
    pub fn insert_resource<R: Resource>(&mut self, resource: R) -> &mut Self {
        self.world.insert_resource(resource);
        self
    }

}
pub trait Plugin: /* Any + Send + Sync */ {
    /// Configures the [`App`] to which this plugin is added.
    fn setup(&self, app: &mut App);
    /// Configures a name for the [`Plugin`] which is primarily used for debugging.
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}


fn run_once(mut app: App) {
    app.update();
}
