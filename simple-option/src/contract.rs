#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, IbcMsg, IbcTimeout, MessageInfo, Order, Reply, Response,
    StdError, StdResult, SubMsg, IbcBasicResponse, IbcChannelCloseMsg, IbcChannelConnectMsg, IbcChannelOpenMsg, 
    IbcPacketAckMsg, IbcPacketReceiveMsg, IbcPacketTimeoutMsg, IbcReceiveResponse, from_slice
};


use crate::error::ContractError;
use crate::ibc_msg::PacketMsg;
use crate::msg::{
    ExecuteMsg, InstantiateMsg, QueryMsg
};
use crate::state::{
    STATE, State
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:simple-storage";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const REQUEST_REPLY_ID: u64 = 100;
pub const SUGGEST_REPLY_ID: u64 = 101;
pub const PROOF_REPLY_ID: u64 = 102;
pub const PROPOSE_REPLY_ID: u64 = 103;
pub const VIEW_TIMEOUT_SECONDS: u64 = 10;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State::new(msg.chain_id, msg.input, env.block.time);
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

// execute entry_point is used for beginning new instance of IT-HS consensus
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Input { value, channel_id } => handle_execute_input(deps, env, info, value, channel_id),
    }
}

pub fn handle_execute_input(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    value: String,
    channel_id: String,
) -> Result<Response, ContractError> {
    let mut messages: Vec<IbcMsg> = Vec::new();

    let packet = PacketMsg::TestMsg { val: value.clone() };

    let msg = IbcMsg::SendPacket {
        channel_id: channel_id.clone(),
        data: to_binary(&packet)?,
        timeout: env.block.time.plus_seconds(600).into(),
    };

    messages.push(msg.clone());

    if value.eq("test") {
        let packet_2 = PacketMsg::TestMsg { val: "test_second".into() };

        let msg_2 = IbcMsg::SendPacket {
            channel_id: channel_id.clone(),
            data: to_binary(&packet_2)?,
            timeout: env.block.time.plus_seconds(600).into(),
        };    
        messages.push(msg_2.clone())
    }

    Ok(Response::new().add_messages(messages))
}



#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    Ok(b"{}".into())
}

// entry_point for sub-messages
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> StdResult<Response> {
    Ok(Response::new())
}



#[entry_point]
/// enforces ordering and versioing constraints
pub fn ibc_channel_open(_deps: DepsMut, _env: Env, _msg: IbcChannelOpenMsg) -> StdResult<()> {
    // verify_channel(msg)?;
    Ok(())
}

#[entry_point]
/// once it's established, we send a WhoAmI message
pub fn ibc_channel_connect(
    deps: DepsMut,
    env: Env,
    msg: IbcChannelConnectMsg,
) -> StdResult<IbcBasicResponse> {
    let channel = msg.channel();
    let channel_id = &channel.endpoint.channel_id;

    let mut state = STATE.load(deps.storage)?;
    state.channel_ids.push(channel_id.to_string());
    state.n += 1;
    STATE.save(deps.storage, &state)?;


    let packet = PacketMsg::WhoAmI {
        chain_id: state.chain_id,
    };
    let msg = IbcMsg::SendPacket {
        channel_id: channel_id.clone(),
        data: to_binary(&packet)?,
        timeout: get_timeout(&env)
    };

    Ok(IbcBasicResponse::new()
        .add_message(msg)
        .add_attribute("action", "ibc_connect")
        .add_attribute("channel_id", channel_id))
}

#[entry_point]
pub fn ibc_packet_receive(
    deps: DepsMut,
    env: Env,
    msg: IbcPacketReceiveMsg,
) -> StdResult<IbcReceiveResponse> {
    Ok(IbcReceiveResponse::new().set_ack(b"{}"))
}


#[entry_point]
pub fn ibc_packet_ack(
    _deps: DepsMut,
    _env: Env,
    msg: IbcPacketAckMsg,
) -> StdResult<IbcBasicResponse> {
    let packet: PacketMsg = from_slice(&msg.original_packet.data)?;
    match packet {
        PacketMsg::TestMsg{val: _} => Ok(IbcBasicResponse::new()),
        PacketMsg::WhoAmI { chain_id: _ } => Ok(IbcBasicResponse::new()),
    }
}


#[entry_point]
/// On closed channel, simply delete the channel_id local state
pub fn ibc_channel_close(
    _deps: DepsMut,
    _env: Env,
    msg: IbcChannelCloseMsg,
) -> StdResult<IbcBasicResponse> {
    // fetch the connected channel_id
    let channel = msg.channel();
    let channel_id = &channel.endpoint.channel_id;

    Ok(IbcBasicResponse::new()
        .add_attribute("action", "ibc_close")
        .add_attribute("channel_id", channel_id))
}

#[entry_point]
/// we just ignore these now. shall we store some info?
pub fn ibc_packet_timeout(
    _deps: DepsMut,
    _env: Env,
    _msg: IbcPacketTimeoutMsg,
) -> StdResult<IbcBasicResponse> {
    Ok(IbcBasicResponse::new().add_attribute("action", "ibc_packet_timeout"))
}




pub fn get_timeout(env: &Env) -> IbcTimeout {
    env.block.time.plus_seconds(600).into()
}
