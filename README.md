# [Lambda Mountain](https://github.com/andrew-johnson-4/-/wiki)

λ☶ (pronounced Lambda Mountain) is a compiler backend.
It is a typed fragment assembler,
which means it could generate machine code,
but right now the only backend targets GNU Assembler.

More information is available on the [λ☶ Wiki](https://github.com/andrew-johnson-4/-/wiki).

There is also a [Bootstrap Book](https://github.com/andrew-johnson-4/BootstrapBook) that explains the compiler internals in great detail.

For those looking for a TLDR, this project uses the following inference rules to turn Simply Typed Lambda Calculus into a workhorse.

$$abstraction \quad \frac{\Gamma \vdash a:A \quad \Gamma \vdash b:B \quad \Gamma \vdash x:X \quad \Gamma \vdash y:Y \quad λ⟨a.b⟩⟨x.y⟩}{\Gamma \vdash λ⟨a.b⟩⟨x.y⟩:(A \to B) + (X \to Y)}$$

$$application \quad \frac{\Gamma \vdash f:(A \to B) + (C \to D) + (X \to Y) \quad \Gamma \vdash x:A + X \quad f(x)}{\Gamma \vdash f(x):B + Y}$$

Code is modelled after Lambda Calculus, so instructions look like this.

```
(mov 1 rax)
(add rax rbx)
```

Instructions are not magic, they are defined as functions.

```
mov := λ(: src Imm64)(: tgt Reg64). (.text ..encode binary data..);
mov := λ(: src Reg64)(: tgt Reg64). (.text ..encode binary data..);
```

<a href="https://github.com/andrew-johnson-4/-/wiki#mascot"> <img src="https://raw.githubusercontent.com/andrew-johnson-4/-/main/DOBY.jpg" height=200 title="Doby being a prototypical ass."> </a>

