// Copyright (c) 2018 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use hab_net::app::prelude::*;
use hab_net::privilege::FeatureFlags;

use super::ServerState;
use error::SrvResult;
use protocol::net;
use protocol::originsrv as proto;
use server::session::encode_token;
use server::session::Session;

pub fn account_get_id(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::AccountGetId>()?;
    match state.datastore.get_account_by_id(&msg) {
        Ok(Some(account)) => conn.route_reply(req, &account)?,
        Ok(None) => {
            let err = NetError::new(ErrCode::ENTITY_NOT_FOUND, "ss:account-get-id:0");
            conn.route_reply(req, &*err)?;
        }
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "ss:account-get-id:1");
            error!("{}, {}", e, err);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn account_get(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::AccountGet>()?;
    match state.datastore.get_account(&msg) {
        Ok(Some(account)) => conn.route_reply(req, &account)?,
        Ok(None) => {
            let err = NetError::new(ErrCode::ENTITY_NOT_FOUND, "ss:account-get:0");
            conn.route_reply(req, &*err)?;
        }
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "ss:account-get:1");
            error!("{}, {}", e, err);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn account_update(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::AccountUpdate>()?;
    match state.datastore.update_account(&msg) {
        Ok(()) => conn.route_reply(req, &net::NetOk::new())?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "ss:account-update:0");
            error!("{}, {}", e, err);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn account_create(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::AccountCreate>()?;
    match state.datastore.create_account(&msg) {
        Ok(account) => conn.route_reply(req, &account)?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "ss:account-create:0");
            error!("{}, {}", e, err);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn account_find_or_create(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::AccountFindOrCreate>()?;
    match state.datastore.account_find_or_create(&msg) {
        Ok(account) => conn.route_reply(req, &account)?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "ss:account-foc:0");
            error!("{}, {}", e, err);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn account_token_create(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::AccountTokenCreate>()?;

    match state.datastore.create_account_token(&msg) {
        Ok(account_token) => conn.route_reply(req, &account_token)?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "ss:account-token-create:0");
            error!("{}, {}", e, err);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn account_token_revoke(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::AccountTokenRevoke>()?;

    let mut msg_get = proto::AccountTokenGet::new();
    msg_get.set_id(msg.get_id());

    match state.datastore.revoke_account_token(&msg) {
        Ok(_) => conn.route_reply(req, &net::NetOk::new())?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "ss:account-token-revoke:1");
            error!("{}, {}", e, err);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn account_tokens_get(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::AccountTokensGet>()?;
    match state.datastore.get_account_tokens(&msg) {
        Ok(account_tokens) => conn.route_reply(req, &account_tokens)?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "ss:account-tokens-get:0");
            error!("{}, {}", e, err);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn session_create(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let mut msg = req.parse::<proto::SessionCreate>()?;
    debug!("session-create, {:?}", msg);
    let flags = if msg.get_session_type() == proto::SessionType::Builder {
        FeatureFlags::all()
    } else {
        FeatureFlags::empty()
    };

    let account = if msg.get_session_type() == proto::SessionType::Builder {
        let mut account = proto::Account::new();
        account.set_id(0);
        account.set_email(msg.take_email());
        account.set_name(msg.take_name());
        account
    } else {
        let mut account_req = proto::AccountFindOrCreate::default();
        account_req.set_name(msg.take_name());
        account_req.set_email(msg.take_email());

        match conn.route::<proto::AccountFindOrCreate, proto::Account>(&account_req) {
            Ok(account) => account,
            Err(e) => {
                let err = NetError::new(ErrCode::DATA_STORE, "ss:session-create:5");
                error!("{}, {}", e, err);
                conn.route_reply(req, &*err)?;
                return Ok(());
            }
        }
    };

    let session = Session::build(msg, account, flags)?;
    {
        debug!("issuing session, {:?}", session);
        state.sessions.write().unwrap().insert(session.clone());
    }
    conn.route_reply(req, &*session)?;
    Ok(())
}

pub fn session_get(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::SessionGet>()?;
    let token = encode_token(msg.get_token())?;
    let expire_session = {
        match state.sessions.read().unwrap().get(token.as_str()) {
            Some(session) => {
                if session.expired() {
                    true
                } else {
                    conn.route_reply(req, &**session)?;
                    false
                }
            }
            None => {
                let err = NetError::new(ErrCode::SESSION_EXPIRED, "ss:session-get:0");
                conn.route_reply(req, &*err)?;
                false
            }
        }
    };
    // JW TODO: We should renew the session if it's within X time of expiring since the
    // user just confirmed they're still using this session.
    if expire_session {
        state.sessions.write().unwrap().remove(token.as_str());
    }
    Ok(())
}
