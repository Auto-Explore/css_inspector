# css/css-scrollbars/scrollbar-color-dynamic-5-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-scrollbars/scrollbar-color-dynamic-5-ref.html"
}
```

## style[0]

```css

  :root {
    scrollbar-color: green green;
  }
  body {
    display: flex;
    flex-wrap: wrap;
    width: 200vw;
    height: 200vh;
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
    background: red;
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
