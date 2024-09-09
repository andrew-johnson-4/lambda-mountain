# Hello Blob

LM is an assembler, which means it fundamentally just sticks data together.
When complex assembly is not necessary, we can say that data is a "blob".

```
in file: hello-blob.lm
----------------------
'A
'B
```

In this file we declare two literals A and B.
When we assemble this file in blob mode, the two pieces get stuck together to make AB.
Pretty simple right?

```
>>> lm --blob hello-blob.lm -o hello-blob.txt
>>> cat hello-blob.txt
AB
```

### [Next Chapter: Assemble an x86 Program](https://github.com/andrew-johnson-4/lambda-mountain/blob/main/TUTORIAL/hello-world.md)
