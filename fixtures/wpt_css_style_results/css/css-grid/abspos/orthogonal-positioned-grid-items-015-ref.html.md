# css/css-grid/abspos/orthogonal-positioned-grid-items-015-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/abspos/orthogonal-positioned-grid-items-015-ref.html"
}
```

## style[0]

```css

#grid {
  display: grid;
  grid: 150px 100px / 200px 300px;
  justify-items: start;
  align-items: end;
  margin: 1px 2px 3px 4px;
  padding: 20px 15px 10px 5px;
  border-width: 9px 3px 12px 6px;
  border-style: solid;
  width: 550px;
  height: 400px;
}

#grid > div {
  writing-mode: vertical-lr;
  margin-bottom: 20px;
}

#firstItem {
  background: magenta;
}

#secondItem {
  background: cyan;
}

#thirdItem {
  background: yellow;
}

#fourthItem {
  background: lime;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “grid”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
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
