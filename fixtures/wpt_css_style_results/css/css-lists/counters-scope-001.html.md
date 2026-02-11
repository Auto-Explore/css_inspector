# css/css-lists/counters-scope-001.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/counters-scope-001.html"
}
```

## style[0]

```css

.scope { counter-reset: c 1; }
.scope::before, .scope::after { content: counter(c); }
.c::before { content: counter(c); }
.one::before { counter-reset: c 2; }
.two { counter-reset: c 3; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
