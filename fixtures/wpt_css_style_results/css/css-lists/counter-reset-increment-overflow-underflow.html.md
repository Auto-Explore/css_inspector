# css/css-lists/counter-reset-increment-overflow-underflow.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/counter-reset-increment-overflow-underflow.html"
}
```

## style[0]

```css

.overflow {
  counter-reset: over-counter 2000000000;
}
.overflow h3:before {
  counter-increment: over-counter 50000000;
}
.overflow h3::before {
  content: counter(over-counter);
}

.underflow {
  counter-reset: under-counter -2000000000;
}
.underflow h3:before {
  counter-increment: under-counter -50000000;
}
.underflow h3::before {
  content: counter(under-counter);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
