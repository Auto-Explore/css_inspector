# css/css-scrollbars/scrollbar-width-paint-001-mis-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-scrollbars/scrollbar-width-paint-001-mis-ref.html"
}
```

## style[0]

```css

  body {
    display: flex;
    flex-wrap: wrap;
  }

  .container {
    scrollbar-gutter: stable;
    flex: 0 0;
    overflow: auto;
    height: 200px;
    min-width: 200px;
    margin: 1px;
    padding: 0px;
    border: none;
    background: deepskyblue;
  }

  .content {
    height: 300px;
    width: 300px;
  }

  .content.plain {
    background: red;
  }

  .content.gradient {
    background: linear-gradient(135deg, red, blue);
  }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “flex”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
