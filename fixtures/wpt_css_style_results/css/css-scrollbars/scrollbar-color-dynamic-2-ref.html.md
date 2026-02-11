# css/css-scrollbars/scrollbar-color-dynamic-2-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-scrollbars/scrollbar-color-dynamic-2-ref.html"
}
```

## style[0]

```css

  :root {
    scrollbar-color: blue green;
  }
  body {
    display: flex;
    flex-wrap: wrap;
    overflow: scroll;
  }
  .container {
    scrollbar-gutter: stable;
    overflow: scroll;
    flex: 0 0;
    height: 200px;
    min-width: 200px;
    margin: 1px;
    padding: 0px;
    border: none;
    background: deepskyblue;
  }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “scrollbar-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “flex”.",
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
