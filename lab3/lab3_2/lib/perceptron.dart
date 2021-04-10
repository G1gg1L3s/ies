class Point {
  final double x;
  final double y;

  const Point(this.x, this.y);

  String toString() => "($x, $y)";
}

class Perceptron {
  double w1;
  double w2;

  Perceptron(this.w1, this.w2);

  double y(Point p) => p.x * w1 + p.y * w2;
  void advance(Point a, double delta, double sigma) {
    w1 = w1 + a.x * delta * sigma;
    w2 = w2 + a.y * delta * sigma;
  }

  String toString() => "Perceptron{ w1: $w1, w2: $w2 }";
}

class Network {
  double p;
  double sigma;
  List<Point> points;
  int iters;

  Network(this.iters, this.p, this.sigma, this.points);

  Perceptron run() {
    int i = 0;
    var percep = Perceptron(0, 0);
    while (iters > 0) {
      var point = points[i];
      var delta = p - percep.y(point);
      percep.advance(point, delta, sigma);
      i = (i + 1) % points.length;
      iters--;
      print("[$iters] : $percep");
    }
    return percep;
  }
}

void main() {
  var points = [Point(0, 6), Point(1, 5), Point(3, 3), Point(2, 4)];
  var iters = 100;
  var sigma = 0.1;
  var p = 4.0;
  var network = Network(iters, p, sigma, points);
  var result = network.run();
  print("ok: $result");
}
