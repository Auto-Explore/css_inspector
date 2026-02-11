# css/css-multicol/multicol-span-all-children-height-008.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-span-all-children-height-008.html"
}
```

## style[0]

```css

  article {
    column-count: 1;
    column-fill: auto;
    width: 200px;
    background-color: lightgreen;
  }
  div.container {
    height: auto;
    border: 20px solid purple;
    background-color: pink;
  }
  div.block {
    width: 100px;
    height: 100px;
    background-color: yellow;
  }
  div.column-span {
    column-span: all;
    height: 50px;
    background-color: lightblue;
  }
  
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
