# css/css-multicol/multicol-span-all-children-height-001.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-span-all-children-height-001.html"
}
```

## style[0]

```css

  article {
    column-count: 2;
    width: 400px;
    height: 200px;
    outline: 1px solid black;
  }
  div {
    height: 50%; /* Spread evenly into two colums, each 25%. */
  }
  div.spanner {
    column-span: all;
    outline: 1px solid blue;
    height: 50%;
  }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
