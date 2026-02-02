import 'package:dio/dio.dart';

class ApiClient {
  // For Android Emulator, localhost is 10.0.2.2
  // For Windows EXE, localhost is 127.0.0.1
  static const String baseUrl = "http://10.0.2.2:8000"; 
  
  final Dio _dio = Dio(BaseOptions(baseUrl: baseUrl));

  Future<Map<String, dynamic>> checkRisk(String fyEnd, String formType) async {
    try {
      final response = await _dio.get(
        '/api/v1/compliance/calculate',
        queryParameters: {
          'fy_end': fyEnd,
          'filing_type': formType
        }
      );
      return response.data;
    } catch (e) {
      throw Exception("Failed to contact Pratyaksh Intelligence Engine");
    }
  }
}