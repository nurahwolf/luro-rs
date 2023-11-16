CREATE VIEW current_connections AS
 SELECT datid,
    datname,
    pid,
    leader_pid,
    usesysid,
    usename,
    application_name,
    client_addr,
    client_hostname,
    client_port,
    backend_start,
    xact_start,
    query_start,
    state_change,
    wait_event_type,
    wait_event,
    state,
    backend_xid,
    backend_xmin,
    query_id,
    query,
    backend_type
   FROM pg_stat_activity;

COMMENT ON VIEW public.current_connections IS 'View current Postgres connections';