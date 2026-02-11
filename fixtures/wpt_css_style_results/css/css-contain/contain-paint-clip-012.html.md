# css/css-contain/contain-paint-clip-012.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-paint-clip-012.html"
}
```

## style[0]

```css

  div
    {
      height: 100px;
      width: 100px;
    }

  div#red-overlapped-test
    {
      background-color: red;
      contain: paint;
    }

  img
    {
      vertical-align: bottom;
    }

  div#green-overlapping-reference
    {
      background-color: green;
      bottom: 100px;
      position: relative;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
