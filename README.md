# [Lambda Mountain](https://github.com/andrew-johnson-4/-/wiki)

λ☶ (pronounced Lambda Mountain) is a compiler backend.
It is a typed fragment assembler which means that it generates machine object files or GNU Assembly.

Right now development is focused towards
* interoperability
* performance
* ergonomics

More information is available on the [λ☶ Wiki](https://github.com/andrew-johnson-4/-/wiki).

There is also a [Bootstrap Book](https://github.com/andrew-johnson-4/BootstrapBook) that explains the compiler internals in great detail.

### Performance

![Calculating the Fibonacci Sequence](https://raw.githubusercontent.com/andrew-johnson-4/-/master/benchmarks/fibonacci.svg)

### What is Ad-Hoc Specialization?

If we have several overloaded functions then specialization lets us choose the best fit for any particular application.

```
f := λ(: x X). x;
f := λ(: y Y). y;

f (: x X)
```

In this example the function application does not “fit” the application that expects a Y type argument, so there is only one possible candidate function.

---

```
type X implies Y;

f := λ(: x X). x;
f := λ(: y Y). y;

f (: x X)
```

Now both candidate functions “fit”, however X is a narrower type than Y.
All X are Y, but not all Y are X.
In this case we say that X is a “better fit” than Y.

---

```
f := λ(: x X). form 1;
f := λ(: x X). form 2;

f (: x X)
```

In this example both candidate functions “fit” AND are equivalent.
In this case we apply *metrics* to determine the best fit.
A metric is an order that can be applied to term/type pairs to determine which is a “better fit” in non-semantic cases.
Metrics are very useful when there exist multiple equivalent forms of code representation that have different performance characteristics.

### How does Ad-Hoc Specialization Work?

The language here is based on [System F-sub](https://en.wikipedia.org/wiki/System_F) with the following inference rules added.

$$abstraction \quad \frac{\Gamma \vdash a:A \quad \Gamma \vdash b:B \quad \Gamma \vdash x:X \quad \Gamma \vdash y:Y \quad λ⟨a.b⟩⟨x.y⟩}{\Gamma \vdash λ⟨a.b⟩⟨x.y⟩:(A \to B) + (X \to Y)}$$

$$application \quad \frac{\Gamma \vdash f:(A \to B) + (C \to D) + (X \to Y) \quad \Gamma \vdash x:A + X \quad f(x)}{\Gamma \vdash f(x):B + Y}$$

### Why is Ad-Hoc Specialization so Important For an Assembler?

Specialization allows us to express high-level ideas at the level of a generic functional language
AND compile the code down to machine code transparently.
There are no hidden layers in the compiler.
The programmer gets to inspect and verify *every single transformation down to individual instructions*.

### Is the Type System Novel?

LM soundly integrates several features that are useful but historically hard to combine.
The type system is not decidable in the general case.
However, by providing type annotations on all overloaded functions it becomes decidable.

* Plural Types (Types behave more like logical predicates)
* Higher Order Functions (Functional Programming)
* Parametric Polymorphism (Generic Programming)
* Subtyping (Object Hierarchies)
* Ad-Hoc Polymorphism (Function Hierarchies)

<a href="https://github.com/andrew-johnson-4/-/wiki#mascot"> <img src="https://raw.githubusercontent.com/andrew-johnson-4/-/main/DOBY.jpg" height=200 title="Doby being a prototypical ass."> </a>

