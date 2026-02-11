# css/css-multicol/multicol-span-all-children-height-006-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-span-all-children-height-006-ref.html"
}
```

## style[0]

```css

  article {
    column-count: 2;
    width: 400px;
    background-color: lightgreen;
  }
  div.container {
    background-color: pink;
    border: 20px solid purple;
  }
  div.block {
    /* This block spreads evenly into two columns, each has 100px height. */
    width: 100px;
    height: 200px;
    background-color: yellow;
  }
  div.column-span {
    width: 400px;
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
