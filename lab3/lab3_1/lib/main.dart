import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:charts_flutter/flutter.dart' as charts;

import 'ferma.dart';

void main() {
  runApp(MyApp());
}

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: MyHomePage(title: 'Ferma Factorization'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  MyHomePage({Key key, this.title}) : super(key: key);

  final String title;

  @override
  _MyHomePageState createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  String number = "";

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(widget.title),
      ),
      body: Container(
        padding: const EdgeInsets.all(40.0),
        child: Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.start,
            children: <Widget>[
              new TextField(
                decoration: new InputDecoration(labelText: "Enter your number"),
                keyboardType: TextInputType.number,
                inputFormatters: <TextInputFormatter>[
                  FilteringTextInputFormatter.digitsOnly
                ],
                onChanged: (String value) => _setNumber(value),
              ),
              Expanded(
                  child: Padding(
                padding: EdgeInsets.only(top: 40.0, bottom: 40.0),
                child: _buildFactors(),
              )),
            ],
          ),
        ),
      ),
      floatingActionButton: FloatingActionButton(
        tooltip: 'Measures',
        child: Icon(Icons.show_chart_sharp),
        onPressed: () => {
          Navigator.push(context,
              MaterialPageRoute(builder: (context) => ChartPage.init()))
        },
      ), // This trailing comma makes auto-formatting nicer for build methods.
    );
  }

  void _setNumber(String num) {
    setState(() {
      number = num;
    });
  }

  List<int> _getFactors(String num) {
    var n = int.tryParse(num);
    if (n == null || n < 0) {
      return null;
    }
    return factor(n);
  }

  Widget _buildFactors() {
    var factors = _getFactors(number);
    if (factors == null) {
      return Text("Unsupported number");
    }
    return ListView.separated(
      padding: const EdgeInsets.all(8),
      itemCount: factors.length,
      itemBuilder: (BuildContext context, int index) {
        return Container(
          height: 50,
          child: Text('${factors[index]}'),
        );
      },
      separatorBuilder: (BuildContext context, int index) => const Divider(),
    );
  }
}

class ChartPage extends StatelessWidget {
  final List<Point> measures;

  ChartPage(this.measures);

  factory ChartPage.init() {
    var points = <Point>[];
    for (int i = 16; i < 2 << 17; i *= 2) {
      print("Processing $i");
      points.add(Point(i, testDuration(i, 100)));
    }
    return ChartPage(points);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
        appBar: AppBar(
          title: Text('Measures'),
        ),
        body: Chart(measures));
  }
}

class Chart extends StatelessWidget {
  final List<Point> points;

  Chart(this.points);

  @override
  Widget build(BuildContext context) {
    var serial = charts.Series<Point, int>(
      id: 'points',
      colorFn: (_, __) => charts.MaterialPalette.blue.shadeDefault,
      domainFn: (Point p, _) => p.x,
      measureFn: (Point p, _) => p.y,
      data: points,
    );
    var series = [serial];
    return new charts.LineChart(series, animate: false);
  }
}

class Point {
  int x;
  int y;

  Point(this.x, this.y);
}
