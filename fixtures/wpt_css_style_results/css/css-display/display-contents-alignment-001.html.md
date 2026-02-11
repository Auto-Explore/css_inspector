# css/css-display/display-contents-alignment-001.html

```json
{
  "format_version": 3,
  "file": "css/css-display/display-contents-alignment-001.html"
}
```

## style[0]

```css

  .container {
    display: flex;
    width: 300px;
    height: 300px;
    align-items: center;
    border: 2px solid purple;
  }
  .contents {
    display: contents;
    align-items: flex-start;
  }
  .contents > div {
    width: 100px;
    height: 100px;
    background: blue;
    align-self: auto;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
