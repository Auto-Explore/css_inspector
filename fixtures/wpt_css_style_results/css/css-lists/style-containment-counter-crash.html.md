# css/css-lists/style-containment-counter-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/style-containment-counter-crash.html"
}
```

## style[0]

```css

  #test::before {
    content: counters(counter0, '');
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
