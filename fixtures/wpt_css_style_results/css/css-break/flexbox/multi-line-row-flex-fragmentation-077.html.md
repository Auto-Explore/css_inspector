# css/css-break/flexbox/multi-line-row-flex-fragmentation-077.html

```json
{
  "format_version": 3,
  "file": "css/css-break/flexbox/multi-line-row-flex-fragmentation-077.html"
}
```

## style[0]

```css

  .flex {
    display: flex;
    flex-flow: row wrap;
  }
  .flex > div {
    flex: 0 0 100%;
    background: green;
  }
  .flex > div > div {
    contain:size;
    height: 50px;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “flex-flow”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “flex”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
