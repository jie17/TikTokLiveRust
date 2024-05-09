use crate::core::live_client_mapper::TikTokLiveMessageMapper;
use crate::generated::events::*;
use crate::generated::messages::webcast::*;
use protobuf::Message;
use tokio::sync::mpsc;
impl TikTokLiveMessageMapper {
    pub async fn handle_single_message(
        &self,
        message: crate::generated::messages::webcast::webcast_response::Message,
        event_sender: &mpsc::Sender<TikTokLiveEvent>,
    ) {
        let proto_message_name = &message.method;
        let proto_message_content = &message.payload;
        match proto_message_name.as_str() {
            "WebcastLikeMessage" => {
                let raw_data = WebcastLikeMessage::parse_from_bytes(proto_message_content).unwrap();
                let event = TikTokLikeEvent { raw_data };
                let _ = event_sender.send(TikTokLiveEvent::OnLike(event)).await;
            }
            "WebcastQuestionNewMessage" => {
                let raw_data =
                    WebcastQuestionNewMessage::parse_from_bytes(proto_message_content).unwrap();
                let event = TikTokQuestionNewEvent { raw_data };
                let _ = event_sender
                    .send(TikTokLiveEvent::OnQuestionNew(event))
                    .await;
            }
            "WebcastLinkMicBattlePunishFinish" => {
                let raw_data =
                    WebcastLinkMicBattlePunishFinish::parse_from_bytes(proto_message_content)
                        .unwrap();
                let event = TikTokLinkMicBattlePunishFinishEvent { raw_data };
                let _ = event_sender
                    .send(TikTokLiveEvent::OnLinkMicBattlePunishFinish(event))
                    .await;
            }
            "WebcastRankUpdateMessage" => {
                let raw_data =
                    WebcastRankUpdateMessage::parse_from_bytes(proto_message_content).unwrap();
                let event = TikTokRankUpdateEvent { raw_data };
                let _ = event_sender
                    .send(TikTokLiveEvent::OnRankUpdate(event))
                    .await;
            }
            "WebcastLinkMicFanTicketMethod" => {
                let raw_data =
                    WebcastLinkMicFanTicketMethod::parse_from_bytes(proto_message_content).unwrap();
                let event = TikTokLinkMicFanTicketMethodEvent { raw_data };
                let _ = event_sender
                    .send(TikTokLiveEvent::OnLinkMicFanTicketMethod(event))
                    .await;
            }
            "WebcastLiveIntroMessage" => {
                let raw_data =
                    WebcastLiveIntroMessage::parse_from_bytes(proto_message_content).unwrap();
                let event = TikTokLiveIntroEvent { raw_data };
                let _ = event_sender.send(TikTokLiveEvent::OnLiveIntro(event)).await;
            }
            "WebcastMemberMessage" => {
                let raw_data =
                    WebcastMemberMessage::parse_from_bytes(proto_message_content).unwrap();
                let event = TikTokMemberEvent { raw_data };
                let _ = event_sender.send(TikTokLiveEvent::OnMember(event)).await;
            }
            "WebcastChatMessage" => {
                let raw_data = WebcastChatMessage::parse_from_bytes(proto_message_content).unwrap();
                let event = TikTokChatEvent { raw_data };
                let _ = event_sender.send(TikTokLiveEvent::OnChat(event)).await;
            }
            "WebcastLinkMicArmies" => {
                let raw_data =
                    WebcastLinkMicArmies::parse_from_bytes(proto_message_content).unwrap();
                let event = TikTokLinkMicArmiesEvent { raw_data };
                let _ = event_sender
                    .send(TikTokLiveEvent::OnLinkMicArmies(event))
                    .await;
            }
            "WebcastLinkLayerMessage" => {
                let raw_data =
                    WebcastLinkLayerMessage::parse_from_bytes(proto_message_content).unwrap();
                let event = TikTokLinkLayerEvent { raw_data };
                let _ = event_sender.send(TikTokLiveEvent::OnLinkLayer(event)).await;
            }
            "WebcastResponse" => {
                let raw_data = WebcastResponse::parse_from_bytes(proto_message_content).unwrap();
                let event = TikTokResponseEvent { raw_data };
                let _ = event_sender.send(TikTokLiveEvent::OnResponse(event)).await;
            }
            "WebcastPushFrame" => {
                let raw_data = WebcastPushFrame::parse_from_bytes(proto_message_content).unwrap();
                let event = TikTokPushFrameEvent { raw_data };
                let _ = event_sender.send(TikTokLiveEvent::OnPushFrame(event)).await;
            }
            "WebcastRankTextMessage" => {
                let raw_data =
                    WebcastRankTextMessage::parse_from_bytes(proto_message_content).unwrap();
                let event = TikTokRankTextEvent { raw_data };
                let _ = event_sender.send(TikTokLiveEvent::OnRankText(event)).await;
            }
            "WebcastSystemMessage" => {
                let raw_data =
                    WebcastSystemMessage::parse_from_bytes(proto_message_content).unwrap();
                let event = TikTokSystemEvent { raw_data };
                let _ = event_sender.send(TikTokLiveEvent::OnSystem(event)).await;
            }
            "WebcastLinkmicBattleTaskMessage" => {
                let raw_data =
                    WebcastLinkmicBattleTaskMessage::parse_from_bytes(proto_message_content)
                        .unwrap();
                let event = TikTokLinkmicBattleTaskEvent { raw_data };
                let _ = event_sender
                    .send(TikTokLiveEvent::OnLinkmicBattleTask(event))
                    .await;
            }
            "WebcastGiftMessage" => {
                let raw_data = WebcastGiftMessage::parse_from_bytes(proto_message_content).unwrap();
                let event = TikTokGiftEvent { raw_data };
                let _ = event_sender.send(TikTokLiveEvent::OnGift(event)).await;
            }
            "WebcastInRoomBannerMessage" => {
                let raw_data =
                    WebcastInRoomBannerMessage::parse_from_bytes(proto_message_content).unwrap();
                let event = TikTokInRoomBannerEvent { raw_data };
                let _ = event_sender
                    .send(TikTokLiveEvent::OnInRoomBanner(event))
                    .await;
            }
            "WebcastMsgDetectMessage" => {
                let raw_data =
                    WebcastMsgDetectMessage::parse_from_bytes(proto_message_content).unwrap();
                let event = TikTokMsgDetectEvent { raw_data };
                let _ = event_sender.send(TikTokLiveEvent::OnMsgDetect(event)).await;
            }
            "WebcastControlMessage" => {
                let raw_data =
                    WebcastControlMessage::parse_from_bytes(proto_message_content).unwrap();
                let event = TikTokControlEvent { raw_data };
                let _ = event_sender.send(TikTokLiveEvent::OnControl(event)).await;
            }
            "WebcastLinkMicMethod" => {
                let raw_data =
                    WebcastLinkMicMethod::parse_from_bytes(proto_message_content).unwrap();
                let event = TikTokLinkMicMethodEvent { raw_data };
                let _ = event_sender
                    .send(TikTokLiveEvent::OnLinkMicMethod(event))
                    .await;
            }
            "WebcastGoalUpdateMessage" => {
                let raw_data =
                    WebcastGoalUpdateMessage::parse_from_bytes(proto_message_content).unwrap();
                let event = TikTokGoalUpdateEvent { raw_data };
                let _ = event_sender
                    .send(TikTokLiveEvent::OnGoalUpdate(event))
                    .await;
            }
            "WebcastCaptionMessage" => {
                let raw_data =
                    WebcastCaptionMessage::parse_from_bytes(proto_message_content).unwrap();
                let event = TikTokCaptionEvent { raw_data };
                let _ = event_sender.send(TikTokLiveEvent::OnCaption(event)).await;
            }
            "WebcastHourlyRankMessage" => {
                let raw_data =
                    WebcastHourlyRankMessage::parse_from_bytes(proto_message_content).unwrap();
                let event = TikTokHourlyRankEvent { raw_data };
                let _ = event_sender
                    .send(TikTokLiveEvent::OnHourlyRank(event))
                    .await;
            }
            "WebcastBarrageMessage" => {
                let raw_data =
                    WebcastBarrageMessage::parse_from_bytes(proto_message_content).unwrap();
                let event = TikTokBarrageEvent { raw_data };
                let _ = event_sender.send(TikTokLiveEvent::OnBarrage(event)).await;
            }
            "WebcastSubNotifyMessage" => {
                let raw_data =
                    WebcastSubNotifyMessage::parse_from_bytes(proto_message_content).unwrap();
                let event = TikTokSubNotifyEvent { raw_data };
                let _ = event_sender.send(TikTokLiveEvent::OnSubNotify(event)).await;
            }
            "RoomVerifyMessage" => {
                let raw_data = RoomVerifyMessage::parse_from_bytes(proto_message_content).unwrap();
                let event = TikTokRoomVerifyEvent { raw_data };
                let _ = event_sender
                    .send(TikTokLiveEvent::OnRoomVerify(event))
                    .await;
            }
            "WebcastSocialMessage" => {
                let raw_data =
                    WebcastSocialMessage::parse_from_bytes(proto_message_content).unwrap();
                let event = TikTokSocialEvent { raw_data };
                let _ = event_sender.send(TikTokLiveEvent::OnSocial(event)).await;
            }
            "WebcastEmoteChatMessage" => {
                let raw_data =
                    WebcastEmoteChatMessage::parse_from_bytes(proto_message_content).unwrap();
                let event = TikTokEmoteChatEvent { raw_data };
                let _ = event_sender.send(TikTokLiveEvent::OnEmoteChat(event)).await;
            }
            "WebcastPollMessage" => {
                let raw_data = WebcastPollMessage::parse_from_bytes(proto_message_content).unwrap();
                let event = TikTokPollEvent { raw_data };
                let _ = event_sender.send(TikTokLiveEvent::OnPoll(event)).await;
            }
            "WebcastRoomPinMessage" => {
                let raw_data =
                    WebcastRoomPinMessage::parse_from_bytes(proto_message_content).unwrap();
                let event = TikTokRoomPinEvent { raw_data };
                let _ = event_sender.send(TikTokLiveEvent::OnRoomPin(event)).await;
            }
            "WebcastRoomMessage" => {
                let raw_data = WebcastRoomMessage::parse_from_bytes(proto_message_content).unwrap();
                let event = TikTokRoomEvent { raw_data };
                let _ = event_sender.send(TikTokLiveEvent::OnRoom(event)).await;
            }
            "WebcastEnvelopeMessage" => {
                let raw_data =
                    WebcastEnvelopeMessage::parse_from_bytes(proto_message_content).unwrap();
                let event = TikTokEnvelopeEvent { raw_data };
                let _ = event_sender.send(TikTokLiveEvent::OnEnvelope(event)).await;
            }
            "WebcastImDeleteMessage" => {
                let raw_data =
                    WebcastImDeleteMessage::parse_from_bytes(proto_message_content).unwrap();
                let event = TikTokImDeleteEvent { raw_data };
                let _ = event_sender.send(TikTokLiveEvent::OnImDelete(event)).await;
            }
            "WebcastRoomUserSeqMessage" => {
                let raw_data =
                    WebcastRoomUserSeqMessage::parse_from_bytes(proto_message_content).unwrap();
                let event = TikTokRoomUserSeqEvent { raw_data };
                let _ = event_sender
                    .send(TikTokLiveEvent::OnRoomUserSeq(event))
                    .await;
            }
            "WebcastUnauthorizedMemberMessage" => {
                let raw_data =
                    WebcastUnauthorizedMemberMessage::parse_from_bytes(proto_message_content)
                        .unwrap();
                let event = TikTokUnauthorizedMemberEvent { raw_data };
                let _ = event_sender
                    .send(TikTokLiveEvent::OnUnauthorizedMember(event))
                    .await;
            }
            "WebcastOecLiveShoppingMessage" => {
                let raw_data =
                    WebcastOecLiveShoppingMessage::parse_from_bytes(proto_message_content).unwrap();
                let event = TikTokOecLiveShoppingEvent { raw_data };
                let _ = event_sender
                    .send(TikTokLiveEvent::OnOecLiveShopping(event))
                    .await;
            }
            "WebcastLinkMessage" => {
                let raw_data = WebcastLinkMessage::parse_from_bytes(proto_message_content).unwrap();
                let event = TikTokLinkEvent { raw_data };
                let _ = event_sender.send(TikTokLiveEvent::OnLink(event)).await;
            }
            "WebcastLinkMicBattle" => {
                let raw_data =
                    WebcastLinkMicBattle::parse_from_bytes(proto_message_content).unwrap();
                let event = TikTokLinkMicBattleEvent { raw_data };
                let _ = event_sender
                    .send(TikTokLiveEvent::OnLinkMicBattle(event))
                    .await;
            }
            _ => {
                let _ = event_sender
                    .send(TikTokLiveEvent::OnWebsocketUnknownMessage(
                        TikTokWebsocketUnknownMessageEvent {
                            message_name: message.method.clone(),
                            raw_data: message,
                        },
                    ))
                    .await;
            }
        }
    }
}
