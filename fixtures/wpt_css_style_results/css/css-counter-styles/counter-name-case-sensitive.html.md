# css/css-counter-styles/counter-name-case-sensitive.html

```json
{
  "format_version": 3,
  "file": "css/css-counter-styles/counter-name-case-sensitive.html"
}
```

## style[0]

```css

    body { counter-reset: foo 0 Foo 5; }
    div:before { content: counters(foo, ".") "-" counters(Foo, "."); }
    div.a { counter-increment: foo; }
    div.b { counter-increment: Foo 2; }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
