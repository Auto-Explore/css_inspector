# css/css-multicol/multicol-span-all-children-height-006.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-span-all-children-height-006.html"
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
    height: 250px;
    background-color: pink;
    border: 20px solid purple;
    margin-top: 1em;
    margin-bottom: 1em;
  }
  div.block {
    width: 100px;
    height: 200px;
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
