# css/css-position/position-absolute-crash-chrome-001.html

```json
{
  "format_version": 3,
  "file": "css/css-position/position-absolute-crash-chrome-001.html"
}
```

## style[0]

```css

  a {
    position: relative;
  }
  a:before {
    content: "foo";
    position: absolute;
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
