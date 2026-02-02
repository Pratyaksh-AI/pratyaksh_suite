import 'package:dio/dio.dart';

class ApiClient {
  // For Android Emulator use 10.0.2.2, For Windows use 127.0.0.1
  static const String baseUrl = "http://10.0.2.2:8000"; 
  final Dio _dio = Dio(BaseOptions(baseUrl: baseUrl));

  // --- PHASE 1: COMPLIANCE ---
  Future<Map<String, dynamic>> checkCompliance(String fyEnd, String formType) async {
    final response = await _dio.get('/api/v1/compliance/calculate', 
      queryParameters: {'fy_end': fyEnd, 'filing_type': formType});
    return response.data;
  }

  // --- PHASE 2: GOVERNANCE ---
  Future<Map<String, dynamic>> analyzeResolution(String text) async {
    final response = await _dio.post('/api/v1/governance/analyze-resolution', 
      data: {'agenda_text': text});
    return response.data;
  }

  // --- PHASE 3: CLIENT RISK ---
  Future<Map<String, dynamic>> getClientScore(int late, int deviations, int cases) async {
    final response = await _dio.get('/api/v1/risk/score-client', 
      queryParameters: {
        'late_payments': late, 
        'deviations': deviations, 
        'litigations': cases
      });
    return response.data;
  }

  // --- PHASE 3: REGIONAL ---
  Future<Map<String, dynamic>> getStampDuty(String state, String instrument, double value) async {
    final response = await _dio.get('/api/v1/regional/calc/stamp-duty', 
      queryParameters: {
        'state': state, 
        'instrument': instrument, 
        'value': value
      });
    return response.data;
  }
}