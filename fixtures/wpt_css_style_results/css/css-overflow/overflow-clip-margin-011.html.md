# css/css-overflow/overflow-clip-margin-011.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/overflow-clip-margin-011.html"
}
```

## style[0]

```css

  .overflow-hidden {
    width: 200px;
    height: 200px;
    overflow: hidden;
  }
  .parent {
    width: 100px;
    height: 100px;
    position: relative;
    top: 500px;
    overflow: clip;
    overflow-clip-margin: 500px;
    z-index: 1;
    background: red;
  }
  .child {
    width: 100px;
    height: 100px;
    position: relative;
    top: -500px;
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
