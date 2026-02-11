# css/CSS2/generated-content/counters-multi-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/generated-content/counters-multi-001.xht"
}
```

## style[0]

```css


  body { white-space: nowrap; }


  body { counter-reset: one 7 two -2; }
  #one:before { counter-increment: one -6; content: counter(one) }
  #two:before { counter-increment: dummy two 7 two two -4 silly 7; content: counter(two) }

  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
