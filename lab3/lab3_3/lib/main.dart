import 'dart:math';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:lab3_3/genetic.dart';

void main() {
  runApp(MyApp());
}

class MyApp extends StatelessWidget {
  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: MyHomePage(title: 'Flutter Demo Home Page'),
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
  List<int> result = [0, 0, 0, 0];
  List<int> coeffs = [1, 1, 1, 1, 1];
  String stats = "";
  Random rng = Random();
  final int populationSize = 10;
  final int eqautionSize = 4;
  final int maxIters = 100;

  void _runSimulation() {
    setState(() {
      var cfs = coeffs.take(eqautionSize).toList();
      var goal = coeffs.last;
      var eqaution = DiophantineEquation(cfs, eqautionSize);
      var start = List<Diophantine>.generate(populationSize, (index) {
        var solutions =
            List<int>.generate(eqautionSize, (index) => rng.nextInt(goal));
        return Diophantine(eqaution, solutions);
      });
      var simulation = Simulator(start, maxIters: maxIters);
      var res = simulation.run() as Diophantine;
      result = res.solutions;
      var time = simulation.elapsed.inMilliseconds;
      var iters = simulation.iters;
      stats = "Took: ${time}ms\nIterations: $iters";
    });
  }

  @override
  Widget build(BuildContext context) {
    // This method is rerun every time setState is called, for instance as done
    // by the _incrementCounter method above.
    //
    // The Flutter framework has been optimized to make rerunning build methods
    // fast, so that you can just rebuild anything that needs updating rather
    // than having to individually change instances of widgets.
    return Scaffold(
      appBar: AppBar(
        // Here we take the value from the MyHomePage object that was created by
        // the App.build method, and use it to set our appbar title.
        title: Text(widget.title),
      ),
      body: Container(
        padding: const EdgeInsets.all(20.0),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.spaceEvenly,
          children: <Widget>[
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceEvenly,
              children: List<Widget>.generate(
                5,
                (index) => _buildInputField(index),
              ),
            ),
            OutlinedButton(
                child: Text("Run simulation"),
                onPressed: () => _runSimulation()),
            Row(
                mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                children: List<Widget>.generate(
                    result.length, (i) => Text("x${i + 1} : ${result[i]}"))),
            Text(
              stats,
              textAlign: TextAlign.left,
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildInputField(int index) {
    var label = index < 4 ? "a${index + 1}" : "b";
    return Expanded(
      child: Container(
        child: TextField(
          decoration: InputDecoration(labelText: label),
          keyboardType:
              TextInputType.numberWithOptions(signed: true, decimal: false),
          onChanged: (String value) => coeffs[index] = int.tryParse(value) ?? 1,
          inputFormatters: [
            FilteringTextInputFormatter(RegExp("^-?[0-9]{0,4}"), allow: true)
          ],
        ),
        padding: const EdgeInsets.symmetric(horizontal: 10),
      ),
    );
  }
}
