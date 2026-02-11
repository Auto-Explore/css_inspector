# css/css-display/display-contents-alignment-002.html

```json
{
  "format_version": 3,
  "file": "css/css-display/display-contents-alignment-002.html"
}
```

## style[0]

```css

  .container {
    display: grid;
    width: 300px;
    height: 300px;
    justify-items: center;
    border: 2px solid purple;
  }
  .contents {
    display: contents;
    justify-items: start;
  }
  .contents > div {
    width: 100px;
    height: 100px;
    background: blue;
    justify-self: auto;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
