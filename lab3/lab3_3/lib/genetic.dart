import 'dart:math';

abstract class Phenotype {
  int fitness();
  List<Phenotype> crossover(Random rng, Phenotype other);
  Phenotype mutate(Random rng);
}

class CachedPhenotype {
  Phenotype ph;
  int fitness;

  CachedPhenotype(this.ph, this.fitness);
}

class WeightedPhenotype {
  Phenotype ph;
  double weight;

  WeightedPhenotype(this.ph, this.weight);

  String toString() => "Wp($weight)";
}

List<WeightedPhenotype> weight(List<CachedPhenotype> ph) {
  double sum =
      ph.map((w) => 1.0 / w.fitness).fold(0, (prev, next) => prev + next);

  var weighted =
      ph.map((c) => WeightedPhenotype(c.ph, 1.0 / c.fitness / sum)).toList();

  for (int i = 1; i < weighted.length; i++) {
    weighted[i].weight += weighted[i - 1].weight;
  }
  return weighted;
}

Phenotype chooseWeighted(Random rng, List<WeightedPhenotype> ph) {
  var random = rng.nextDouble();
  for (var w in ph) {
    if (random < w.weight) {
      return w.ph;
    }
  }
  throw Unreachable();
}

class Unreachable extends Error {}

class Simulator {
  List<Phenotype> _population;
  int maxIters;
  int iters = 0;
  Random rng = Random();
  Stopwatch stopwatch = Stopwatch();

  Simulator(this._population, {this.maxIters});

  Duration get elapsed => stopwatch.elapsed;

  Phenotype run() {
    stopwatch.start();
    while (true) {
      var sorted = _population
          .map((ph) => CachedPhenotype(ph, ph.fitness()))
          .toList()
            ..sort((a, b) => a.fitness.compareTo(b.fitness));
      assert(sorted.first.fitness >= 0);

      if (sorted.first.fitness == 0 || iters == maxIters) {
        stopwatch.stop();
        return sorted.first.ph;
      }

      var weights = weight(sorted);
      var nextPopulation = <Phenotype>[];
      for (int i = 0; i < _population.length / 2; i++) {
        var mom = chooseWeighted(rng, weights);
        var dad = chooseWeighted(rng, weights);
        var childs = mom.crossover(rng, dad);
        nextPopulation.addAll(childs);
      }
      for (var ph in nextPopulation) {
        ph.mutate(rng);
      }
      _population = nextPopulation;
      iters++;
    }
  }
}

class DiophantineEquation {
  List<int> coeffs;
  int goal;

  DiophantineEquation(this.coeffs, this.goal);
}

class Diophantine extends Phenotype {
  List<int> solutions;
  DiophantineEquation equation;

  Diophantine(this.equation, this.solutions);

  @override
  List<Phenotype> crossover(Random rng, Phenotype other) {
    var dad = other as Diophantine;
    var len = solutions.length;
    var mid = rng.nextInt(len);
    var alice = Diophantine(equation, List<int>.filled(len, 0));
    var bob = Diophantine(equation, List<int>.filled(len, 0));
    for (int i = 0; i < len; i++) {
      if (i < mid) {
        alice.solutions[i] = solutions[i];
        bob.solutions[i] = dad.solutions[i];
      } else {
        alice.solutions[i] = dad.solutions[i];
        bob.solutions[i] = solutions[i];
      }
    }
    return [alice, bob];
  }

  @override
  int fitness() {
    int sum = 0;
    for (int i = 0; i < solutions.length; i++) {
      sum += solutions[i] * equation.coeffs[i];
    }
    sum -= equation.goal;
    return sum < 0 ? -sum : sum;
  }

  @override
  Phenotype mutate(Random rng) {
    var index = rng.nextInt(solutions.length);
    var delta = rng.nextBool() ? 1 : -1;
    solutions[index] += delta;
    return this;
  }

  String toString() {
    var result = "";
    var plus = "";
    for (int i = 0; i < solutions.length; i++) {
      result += "$plus${solutions[i]} * ${equation.coeffs[i]}";
      plus = " + ";
    }
    result += " = ${equation.goal}";
    return result;
  }
}

// void main() {
//   var rng = Random();
//   var coeffs = List<int>.generate(10, (index) => index);
//   var eqaution = DiophantineEquation(coeffs, 10);
//   var start = List<Diophantine>.generate(10, (index) {
//     var solutions = List<int>.generate(10, (index) => rng.nextInt(5));
//     return Diophantine(eqaution, solutions);
//   });
//   var simulation = Simulator(start, maxIters: 100);
//   var result = simulation.run();
//   print(result);
// }
