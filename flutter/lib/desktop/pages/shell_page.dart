import 'dart:convert';
import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:flutter_hbb/common.dart';
import 'package:flutter_hbb/models/model.dart';
import 'package:get/get.dart';
import 'package:xterm/core.dart';
import 'package:xterm/ui.dart';

class ShellPage extends StatefulWidget {
  final String id;
  final SessionID? sessionId;

  const ShellPage({super.key, required this.id, this.sessionId});

  @override
  State<StatefulWidget> createState() {
    return _ShellPageState();
  }
}

class _ShellPageState extends State<ShellPage> {
  late FFI _ffi;

  void putFFI() {
    _ffi = FFI(widget.sessionId);
    Get.put<FFI>(_ffi, tag: widget.id);
  }

  @override
  void initState() {
    super.initState();
    putFFI();
  }

  @override
  Widget build(BuildContext context) {
    // TODO: implement build
    throw UnimplementedError();
  }
}

class SSHSession extends TerminalSession {
  @override
  void resizeTerminal(int width, int height, int pixelWidth, int pixelHeight) {}
  @override
  void write(Uint8List data) {}
  @override
  void onTerminal(Terminal terminal) {}
}

abstract class TerminalSession {
  void resizeTerminal(int width, int height, int pixelWidth, int pixelHeight) {}

  void write(Uint8List data) {}

  void onTerminal(Terminal terminal) {}
}

class TerminalSessionWidget extends StatefulWidget {
  final TerminalSession session;

  const TerminalSessionWidget({super.key, required this.session});

  @override
  State<StatefulWidget> createState() {
    return _TerminalSessionState();
  }
}

class _TerminalSessionState extends State<TerminalSessionWidget> {
  TerminalSession get session => widget.session;
  late final terminal = Terminal();

  @override
  void initState() {
    super.initState();

    terminal.buffer.clear();
    terminal.buffer.setCursor(0, 0);

    terminal.onResize = (width, height, pixelWidth, pixelHeight) {
      session.resizeTerminal(width, height, pixelWidth, pixelHeight);
    };

    terminal.onOutput = (data) {
      session.write(utf8.encode(data));
    };
    session.onTerminal(terminal);
  }

  @override
  Widget build(BuildContext context) {
    return TerminalView(terminal);
  }
}
