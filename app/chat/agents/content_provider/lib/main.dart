// Copyright 2017 The Fuchsia Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'dart:async';

import 'package:application.lib.app.dart/app.dart';
import 'package:apps.maxwell.services.context/context_provider.fidl.dart';
import 'package:apps.maxwell.services.suggestion/proposal_publisher.fidl.dart';
import 'package:apps.maxwell.services.user/intelligence_services.fidl.dart';
import 'package:apps.modular.services.user/device_map.fidl.dart';
import 'package:apps.modules.chat.services/chat_content_provider.fidl.dart';
import 'package:lib.fidl.dart/bindings.dart';
import 'package:lib.logging/logging.dart';
import 'package:lib.modular/modular.dart';
import 'package:meta/meta.dart';

import 'src/chat_content_provider_impl.dart';
import 'src/firebase_chat_message_transporter.dart';
import 'src/proposer.dart';

const Duration _kTimeout = const Duration(seconds: 3);

ChatContentProviderAgent _agent;

/// An implementation of the [Agent] interface.
class ChatContentProviderAgent extends AgentImpl {
  final ProposalPublisherProxy _proposalPublisher =
      new ProposalPublisherProxy();
  final ContextProviderProxy _contextProvider = new ContextProviderProxy();
  final ContextListenerBinding _proposerBinding = new ContextListenerBinding();
  ChatContentProviderImpl _contentProviderImpl;

  /// Creates a new instance of [ChatContentProviderAgent].
  ChatContentProviderAgent({@required ApplicationContext applicationContext})
      : super(applicationContext: applicationContext);

  @override
  Future<Null> onReady(
    ApplicationContext applicationContext,
    AgentContext agentContext,
    ComponentContext componentContext,
    TokenProvider tokenProvider,
    ServiceProviderImpl outgoingServices,
  ) async {
    log.fine('onReady start.');

    // Get the device id.
    DeviceMapProxy deviceMap = new DeviceMapProxy();
    connectToService(applicationContext.environmentServices, deviceMap.ctrl);
    Completer<DeviceMapEntry> entryCompleter = new Completer<DeviceMapEntry>();
    deviceMap.getCurrentDevice(entryCompleter.complete);
    DeviceMapEntry entry = await entryCompleter.future.timeout(_kTimeout);
    String deviceId = entry.deviceId;
    deviceMap.ctrl.close();

    IntelligenceServicesProxy intelligenceServices =
        new IntelligenceServicesProxy();
    agentContext.getIntelligenceServices(intelligenceServices.ctrl.request());
    intelligenceServices.getProposalPublisher(
      _proposalPublisher.ctrl.request(),
    );
    intelligenceServices.getContextProvider(
      _contextProvider.ctrl.request(),
    );
    intelligenceServices.ctrl.close();

    Proposer proposer = new Proposer(proposalPublisher: _proposalPublisher);

    _contextProvider.subscribe(
      new ContextQuery()
        ..topics = <String>[
          '/location/home_work',
          '/story/visible_ids',
        ],
      _proposerBinding.wrap(proposer),
    );

    // Initialize the content provider.
    _contentProviderImpl = new ChatContentProviderImpl(
      componentContext: componentContext,
      chatMessageTransporter: new FirebaseChatMessageTransporter(
        tokenProvider: tokenProvider,
      ),
      deviceId: deviceId,
      onMessageReceived: proposer.onMessageReceived,
    );
    await _contentProviderImpl.initialize();

    // Register the ChatContentProvider service to the outgoingServices
    // service provider.
    outgoingServices.addServiceForName(
      (InterfaceRequest<ChatContentProvider> request) {
        log.fine('Received a ChatContentProvider request');
        _contentProviderImpl.addBinding(request);
      },
      ChatContentProvider.serviceName,
    );

    proposer.load();
    log.fine('onReady end.');
  }

  @override
  Future<Null> onStop() async {
    _proposalPublisher.ctrl.close();
    _contextProvider.ctrl.close();
    _proposerBinding.close();
  }
}

/// Main entry point.
Future<Null> main(List<String> args) async {
  setupLogger(name: 'chat/agent');

  _agent = new ChatContentProviderAgent(
    applicationContext: new ApplicationContext.fromStartupInfo(),
  );
  _agent.advertise();
}
