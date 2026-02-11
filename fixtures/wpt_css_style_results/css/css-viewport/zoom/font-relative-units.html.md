# css/css-viewport/zoom/font-relative-units.html

```json
{
  "format_version": 3,
  "file": "css/css-viewport/zoom/font-relative-units.html"
}
```

## style[0]

```css

    html {
      font-size: 20px;
      line-height: 1;
      zoom: 2;
    }

    .unit {
      height: 20px;
      outline: 1px solid black;
      outline-offset: -1px;
    }

    .unit::after {
      content: attr(id);
      font-size: 10px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
