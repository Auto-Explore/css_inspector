# css/CSS2/generated-content/content-counter-016.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/generated-content/content-counter-016.xht"
}
```

## style[0]

```css


  body { white-space: nowrap; }


  #test { counter-reset: c 0 f 1000; }
  #test span { counter-increment: c; }
  #test span:before {
    content: counter(c);
    content: counter(f, ".");
    content: counter(f, ".", decimal);
    content: counter(f, decimal, ".");
    content: counter(f, decimal, decimal);
  }

  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
