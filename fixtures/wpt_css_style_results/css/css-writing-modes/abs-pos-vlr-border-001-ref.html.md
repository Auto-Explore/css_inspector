# css/css-writing-modes/abs-pos-vlr-border-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/abs-pos-vlr-border-001-ref.html"
}
```

## style[0]

```css

  body { margin: 0; }
  .vert-cb {
    position: relative;
    width: 150px;
    height: 60px;
    writing-mode: vertical-lr;
    direction: rtl;
    background: lightblue;
    border: solid gray;
    border-width: 1px 2px 3px 4px;
    margin-bottom: 2px;
  }
  .horiz-parent {
    width: 120px;
    height: 100%;
    box-sizing: border-box;
    border: solid orange;
    border-width: 4px 3px 2px 1px;
    writing-mode: horizontal-tb;
  }
  .abspos-equivalent {
    height: 40px;
    background: pink;
    border: solid black;
    border-width: 2px 1px 4px 3px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
