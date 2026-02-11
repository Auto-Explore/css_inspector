# css/selectors/invalidation/negated-last-of-type-invalidation.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/negated-last-of-type-invalidation.html"
}
```

## style[0]

```css

#dut {
  width: 100px;
  height: 100px;
  background: red;
}
.foo:not(:last-of-type ~ .bar) > #dut.baz {
  background-color: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
