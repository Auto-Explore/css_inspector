# css/css-break/rounded-clipped-border.html

```json
{
  "format_version": 3,
  "file": "css/css-break/rounded-clipped-border.html"
}
```

## style[0]

```css

  .container {
    float: left;
    margin: 10px;
    text-align: center;
    inline-size: fit-content;
  }
  .multicol {
    columns: 3;
    column-fill: auto;
    gap: 10px;
    inline-size: 320px;
    block-size: 120px;
    border: solid;
    background: lightgray;
  }
  .clipper {
    block-size: 300px;
    border-radius: 50px;
    border: 20px solid blue;
    overflow: clip;
    background: red;
  }
  .child {
    block-size: 300px;
    background: yellow;
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
