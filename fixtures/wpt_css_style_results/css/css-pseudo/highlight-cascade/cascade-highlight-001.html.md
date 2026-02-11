# css/css-pseudo/highlight-cascade/cascade-highlight-001.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/highlight-cascade/cascade-highlight-001.html"
}
```

## style[0]

```css

  div
    {
      font-size: 300%;
    }

  div::selection
    {
      background-color: green;
      color: yellow;
    }

  div > span::selection
  /*
  count the number of element names and pseudo-elements in the selector (= d)
  a=0 b=0 c=0 d=3 -> specificity = 0,0,0,3
  */
    {
      background-color: green;
    }

  span::selection
  /*
  count the number of element names and pseudo-elements in the selector (= d)
  a=0 b=0 c=0 d=2 -> specificity = 0,0,0,2
  */
    {
      background-color: red;
      color: yellow;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
