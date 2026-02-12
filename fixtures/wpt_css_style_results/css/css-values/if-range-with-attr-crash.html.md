# css/css-values/if-range-with-attr-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-values/if-range-with-attr-crash.html"
}
```

## style[0]

```css

  div {
    color: if(
      style(attr(data-foo <number>) = 3): green;
      else: red;
    );
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
