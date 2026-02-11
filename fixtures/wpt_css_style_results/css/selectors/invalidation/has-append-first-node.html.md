# css/selectors/invalidation/has-append-first-node.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/has-append-first-node.html"
}
```

## style[0]

```css

#dut {
  width: 100px;
  height: 100px;
  background: red;
}
#dut:has(span) {
  background: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
