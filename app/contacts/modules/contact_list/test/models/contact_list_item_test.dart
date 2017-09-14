// Copyright 2017 The Fuchsia Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'package:contact_list/models.dart';
import 'package:test/test.dart';

void main() {
  group('ContactListItem', () {
    test('should throw if display name is empty', () {
      expect(() {
        new ContactListItem(id: 'id', displayName: '');
      }, throwsA(new isInstanceOf<AssertionError>()));
    });

    test('should throw if id is empty', () {
      expect(() {
        new ContactListItem(id: '', displayName: 'displayName');
      }, throwsA(new isInstanceOf<AssertionError>()));
    });
  });
}
