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
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
