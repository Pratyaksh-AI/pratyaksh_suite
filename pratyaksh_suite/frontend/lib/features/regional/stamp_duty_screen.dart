import 'package:flutter/material.dart';
import '../../core/api_client.dart';

class StampDutyScreen extends StatefulWidget {
  const StampDutyScreen({super.key});

  @override
  State<StampDutyScreen> createState() => _StampDutyScreenState();
}

class _StampDutyScreenState extends State<StampDutyScreen> {
  final ApiClient api = ApiClient();
  String _selectedState = "MAHARASHTRA";
  String _instrument = "SHARE_CERTIFICATE";
  final TextEditingController _valueController = TextEditingController();
  String _result = "";

  void _calculate() async {
    final res = await api.getStampDuty(_selectedState, _instrument, double.parse(_valueController.text));
    setState(() {
      _result = "Duty Payable: ₹${res['stamp_duty_payable']}\nAct: ${res['act_reference']}";
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text("Regional Stamp Duty")),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          children: [
            DropdownButton<String>(
              value: _selectedState,
              items: ["MAHARASHTRA", "KARNATAKA"].map((s) => DropdownMenuItem(value: s, child: Text(s))).toList(),
              onChanged: (v) => setState(() => _selectedState = v!),
            ),
            DropdownButton<String>(
              value: _instrument,
              items: ["SHARE_CERTIFICATE", "MOA"].map((s) => DropdownMenuItem(value: s, child: Text(s))).toList(),
              onChanged: (v) => setState(() => _instrument = v!),
            ),
            TextField(controller: _valueController, decoration: const InputDecoration(labelText: "Consideration Value (₹)"), keyboardType: TextInputType.number),
            const SizedBox(height: 20),
            ElevatedButton(onPressed: _calculate, child: const Text("Calculate Duty")),
            const SizedBox(height: 20),
            Text(_result, style: const TextStyle(fontSize: 18, fontWeight: FontWeight.bold)),
          ],
        ),
      ),
    );
  }
}