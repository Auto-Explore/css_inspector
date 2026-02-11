# css/CSS2/generated-content/counters-scope-000.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/generated-content/counters-scope-000.xht"
}
```

## style[0]

```css


  body { white-space: nowrap; }


  .scope { counter-reset: c 1; }
  .scope:before, .scope:after { content: counter(c); }
  .c:before { content: counter(c); }

  .one:before { counter-reset: c 2; }
  .two { counter-reset: c 3; }

  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
