# css/css-multicol/multicol-span-all-children-height-002.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-span-all-children-height-002.html"
}
```

## style[0]

```css

  article {
    column-count: 2;
    width: 400px;
    height: 200px;
    background-color: lightgreen;
    border: 5px solid purple;
  }
  div.block1 {
    background-color: yellow;
    height: 100%; /* Spread evenly into two columns, each 100px. */
  }
  div.spanner {
    column-span: all;
    height: 25%;
    background-color: lightblue;
  }
  div.block2 {
    background-color: yellow;
    /* Column container has only 25% height left, so two extra overflow columns
       are created. Total 4 columns, each 50px. */
    height: 100%;
  }
  
```

```json
{
  "errors": 2,
  "messages": [
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
