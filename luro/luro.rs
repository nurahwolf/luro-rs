impl LuroData {
    /// Return a default context, usually on bot startup.
    pub async fn init() -> Result<Self, Error> {
        let path_to_data = PathBuf::from("./data"); //env::current_dir().expect("Invaild executing directory").join("/data");
    
        // Initialise /data folder for toml. Otherwise it panics. 
        if !path_to_data.exists() {
            tracing::warn!("/data folder does not exist, creating it...");
            fs::create_dir(path_to_data).expect("Failed to make data subfolder");
            tracing::info!("/data folder successfully created!");
        }

        // Initialise our guild settings
        let guild_settings = match LuroGuilds::get().await {
            Ok(ok) => RwLock::new(ok),
            Err(why) => panic!("Failed to initialise guild settings - {why}"),
        };

        // Initialise our hecks settings
        let hecks = match Hecks::get().await {
            Ok(ok) => RwLock::new(ok),
            Err(why) => panic!("Failed to initialise guild settings - {why}"),
        };

        let commands = RwLock::new(LuroCommands::set_default_commands());
        let interaction_count = RwLock::new(0);

        Ok(
            Self{
                commands,
                guild_settings,
                hecks,
                interaction_count
        })
    }
}


// OLD STUFF BELOW
impl LuroContext {
    /// Initialise and return an instance of Luro
    pub async fn init() -> Result<(Arc<Self>, Vec<Shard>), Error> {

        // Loads dotenv. This allows std::env to view the variables in the file
        dotenv().ok();

        // Luro's Discord token, grabbed from the "DISCORD_TOKEN" environment variabled
        let token = env::var("DISCORD_TOKEN").expect("No DISCORD_TOKEN defined");

        // Lavalink host, defined by the "LAVALINK_HOST" environmental
        let lavalink_host = env::var("LAVALINK_HOST").expect("No LAVALINK_HOST defined");

        // Lavalink authorisation, defined by the "LAVALINK_AUTHORISATION" environmental
        let lavalink_auth = env::var("LAVALINK_AUTHORISATION").expect("No LAVALINK_AUTHORISATION defined: {err}");

        // Lavalink host, defined by the "LAVALINK_HOST" environmental
        let lavalink_host = match SocketAddr::from_str(&lavalink_host) {
            Ok(ok) => ok,
            Err(err) => panic!("Invaild LAVALINK_HOST defined: {err}"),
        };
        
        // How many shards we should create
        let shard_count = 1u64;

        // HTTP client, used for interacting with the Discord API
        let http = Client::new(token.clone());

        // Application ID
        let application_id = http
            .current_user_application()
            .await
            .unwrap()
            .model()
            .await
            .unwrap()
            .id;

        // Get our current discord user
        let user = match http.current_user().await {
            Ok(ok) => match ok.model().await {
                Ok(ok) => ok,
                Err(err) => panic!("Got Luro's current user, but failed to decode the JSON: {err}"),
            },
            Err(err) => panic!("Failed to get Luro's current user: {err}"),
        };

        // Initialise Lavalink
        let lavalink = Lavalink::new(user.id, shard_count);
        match lavalink.add(lavalink_host, lavalink_auth).await {
            Ok(ok) => ok,
            Err(err) => panic!("Failed to connect to lavalink: {err}"),
        };

        // Register our intents, then initialise a shard
        let intents =
            Intents::GUILD_MESSAGES | Intents::GUILD_VOICE_STATES | Intents::MESSAGE_CONTENT;

        // Create our shards
        let shards = match stream::create_recommended(
            &http,
            ConfigBuilder::new(token.clone(), intents).build(),
            |_, config_builder| config_builder.build(),
        )
        .await
        {
            Ok(ok) => ok.collect::<Vec<Shard>>(),
            Err(err) => panic!("Failed to start shards: {err}"),
        };

        let path_to_data = PathBuf::from("./data"); //env::current_dir().expect("Invaild executing directory").join("/data");

        // Initialise /data folder for toml. Otherwise it panics. 
        if !path_to_data.exists() {
            tracing::warn!("/data folder does not exist, creating it...");
            fs::create_dir(path_to_data).expect("Failed to make data subfolder");
            tracing::info!("/data folder successfully created!");
        }

        // Initialise our guild settings
        let guild_settings = match LuroGuilds::get().await {
            Ok(ok) => tokio::sync::RwLock::new(ok),
            Err(why) => panic!("Failed to initialise guild settings - {why}"),
        };

        // Initialise our hecks settings
        let hecks = match Hecks::get().await {
            Ok(ok) => tokio::sync::RwLock::new(ok),
            Err(why) => panic!("Failed to initialise guild settings - {why}"),
        };

        Ok((
            Arc::new(Self {
                application_id,
                http,
                lavalink,
                hyper: HyperClient::new(),
                user,
                standby: Standby::new(),
                commands: LuroCommands::set_default_commands().into(),
                guild_settings,
                hecks,
                interaction_count: tokio::sync::RwLock::new(0),
            }),
            shards,
        ))
    }
}