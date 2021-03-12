// Binary searched square root
import 'dart:math';

int intSqrt(int x) {
  var a = 0;
  var b = x;
  while (b - a > 1) {
    var mid = (a + b) ~/ 2;
    var sqr = mid * mid;
    if (sqr == x) {
      return mid;
    } else if (sqr < x) {
      a = mid;
    } else {
      b = mid;
    }
  }
  return a;
}

bool isSquare(int x) {
  var sqrt = intSqrt(x);
  return sqrt * sqrt == x;
}

// x should be odd
List<int> fermat(int n) {
  assert(n.isOdd);
  int a = sqrt(n).ceil();
  int b2 = a * a - n;
  int b = sqrt(b2).round();
  while (b * b != b2) {
    a = a + 1;
    b2 = a * a - n;
    b = sqrt(b2).round();
  }
  return [a - b, a + b];
}

List<int> factorInner(int x) {
  assert(x > 0);
  if (x <= 3) {
    return [x];
  }

  var result = <int>[];
  while (x.isEven) {
    x ~/= 2;
    result.add(2);
  }
  var factors = fermat(x);
  for (var f in factors) {
    if (f == x) {
      result.add(x);
    } else {
      result.addAll(fermat(f));
    }
  }

  return result;
}

List<int> factor(int x) {
  var primes = factorInner(x);
  var withoutone = primes.where((element) => element != 1).toList();
  withoutone.sort();
  return withoutone;
}

int randBetwen(Random rng, int a, int b) {
  var num = rng.nextInt(b - a);
  return num + a;
}

int testDuration(order, int tests) {
  var rng = Random();

  var delta = max((order * 0.1).round() as int, 1.0).round();
  var sum = 0;
  for (var i = 0; i < tests; i++) {
    var number = order + randBetwen(rng, -delta, delta);
    if (number < 0) {
      number = 2;
    }
    Stopwatch stopwatch = new Stopwatch()..start();
    factor(number);
    sum += stopwatch.elapsed.inMicroseconds;
  }

  sum ~/= tests;
  return sum;
}
