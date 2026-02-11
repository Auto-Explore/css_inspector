# css/css-overflow/scrollable-overflow-with-nested-elements-003.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scrollable-overflow-with-nested-elements-003.html"
}
```

## style[0]

```css

    body {
        overflow-x: hidden;
    }

    .list .item {
        width: 10px;
        height: 10px;
        background: lime;
        position: absolute;
        margin: 0%;
    }

    .list {
        position: relative;
        background: grey;
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
