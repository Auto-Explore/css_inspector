# css/css-pseudo/marker-content-014.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-content-014.html"
}
```

## style[0]

```css

iframe {
  width: 300px;
  border: none;
}
iframe.big {
  width: 600px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css

  ::marker {
    font-size: 3vw;
  }
  ol {
    float: left;
  }
  .inside {
    list-style-position: inside;
  }
  .decimal {
    list-style-type: decimal;
  }
  .string {
    list-style-type: "1. ";
  }
  .marker::marker {
    content: "1. ";
  }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
