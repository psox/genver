# genver
Generic Version following these conventions

```regex
([a-zA-Z]+)?((\d+)(\.\d+)*)(\-([a-zA-Z]+)?((\d+)(\.\d+)*))?(\+([a-zA-Z]+)?((\d+)(\.\d+)*))?
```

Valid versions
```plaintext
v1
1.0.0
v1.0.2.1.3
v1.2.3-tag1.3.2.4+build7.6.3
1.2.3.4.5-6.7.8.9.0+7.6.3
1.2.3.4.5-a0+c2
```
