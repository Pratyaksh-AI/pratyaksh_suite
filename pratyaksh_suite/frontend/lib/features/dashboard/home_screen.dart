import 'package:flutter/material.dart';
import '../risk_assessment/client_risk_screen.dart';
import '../regional/stamp_duty_screen.dart';

class HomeScreen extends StatelessWidget {
  const HomeScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text("PratyakshAI CS Suite")),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          _navCard(context, "Client Risk Engine", const ClientRiskScreen(), Colors.redAccent),
          _navCard(context, "Stamp Duty (Regional)", const StampDutyScreen(), Colors.blueAccent),
          // Add other screens here
        ],
      ),
    );
  }

  Widget _navCard(BuildContext context, String title, Widget page, Color color) {
    return Card(
      color: color,
      child: ListTile(
        title: Text(title, style: const TextStyle(color: Colors.white, fontWeight: FontWeight.bold)),
        trailing: const Icon(Icons.arrow_forward, color: Colors.white),
        onTap: () => Navigator.push(context, MaterialPageRoute(builder: (_) => page)),
      ),
    );
  }
}