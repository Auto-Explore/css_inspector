# css/css-break/rounded-clipped-border-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-break/rounded-clipped-border-ref.html"
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
    inline-size: 320px;
    block-size: 120px;
    border: solid;
    background: lightgray;
  }
  .column {
    float: left;
    inline-size: 100px;
    margin-inline-start: 10px;
  }
  .column:first-child {
    margin-inline-start:0;
  }
  .clipper {
    position: relative;
    border-radius: 50px;
    border: 20px solid blue;
    overflow: clip;
    background: red;
  }
  .clipper.part1 {
    block-size: 100px;
    border-block-end: none;
    border-end-start-radius: 0;
    border-end-end-radius: 0;
  }
  .clipper.part2 {
    block-size: 120px;
    border-block-start: none;
    border-block-end: none;
    border-radius: 0;
  }
  .clipper.part3 {
    block-size: 80px;
    border-block-start: none;
    border-start-start-radius: 0;
    border-start-end-radius: 0;
  }
  .child {
    block-size: 300px;
    background: yellow;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
