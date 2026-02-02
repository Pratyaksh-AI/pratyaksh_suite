import 'package:flutter/material.dart';
import '../../core/api_client.dart';

class ClientRiskScreen extends StatefulWidget {
  const ClientRiskScreen({super.key});

  @override
  State<ClientRiskScreen> createState() => _ClientRiskScreenState();
}

class _ClientRiskScreenState extends State<ClientRiskScreen> {
  final ApiClient api = ApiClient();
  final TextEditingController _lateController = TextEditingController(text: "0");
  final TextEditingController _devController = TextEditingController(text: "0");
  final TextEditingController _caseController = TextEditingController(text: "0");
  
  String _status = "";
  int _score = 100;

  void _calculate() async {
    final res = await api.getClientScore(
      int.parse(_lateController.text),
      int.parse(_devController.text),
      int.parse(_caseController.text),
    );
    setState(() {
      _score = res['score'];
      _status = res['status'] + "\n" + res['advisory'];
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text("Client Risk Engine")),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          children: [
            Text("Trust Score: $_score/100", style: const TextStyle(fontSize: 30, fontWeight: FontWeight.bold)),
            Text(_status, textAlign: TextAlign.center, style: TextStyle(color: _score < 50 ? Colors.red : Colors.green)),
            const SizedBox(height: 20),
            TextField(controller: _lateController, decoration: const InputDecoration(labelText: "Late Payments Count"), keyboardType: TextInputType.number),
            TextField(controller: _devController, decoration: const InputDecoration(labelText: "Compliance Deviations"), keyboardType: TextInputType.number),
            TextField(controller: _caseController, decoration: const InputDecoration(labelText: "Active Litigation Cases"), keyboardType: TextInputType.number),
            const SizedBox(height: 20),
            ElevatedButton(onPressed: _calculate, child: const Text("Analyze Client Risk")),
          ],
        ),
      ),
    );
  }
}