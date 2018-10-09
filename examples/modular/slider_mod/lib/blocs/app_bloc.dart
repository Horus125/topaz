// Copyright 2018 The Fuchsia Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'package:modular/module.dart';

import 'bloc_provider.dart';

/// The [AppBloc] provides app level actions like launching other modules
class AppBloc implements BlocBase {
  void launchSquare() {
    final intent = Intent.withAction('com.fuchsia.shapes_mod.show_square',
        scopedHandler: 'shapes_mod');

    Module().addModuleToStory(name: 'shape_module', intent: intent);
  }

  void launchCircle() {
    final intent = Intent.withAction('com.fuchsia.shapes_mod.show_circle',
        scopedHandler: 'shapes_mod');
    Module().addModuleToStory(name: 'shape_module', intent: intent);
  }

  @override
  void dispose() {}
}
