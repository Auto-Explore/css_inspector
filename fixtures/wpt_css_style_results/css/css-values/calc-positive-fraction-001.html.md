# css/css-values/calc-positive-fraction-001.html

```json
{
  "format_version": 3,
  "file": "css/css-values/calc-positive-fraction-001.html"
}
```

## style[0]

```css

  div
    {
      height: 100px;
      position: absolute;
      width: 100px;
    }

  div#red-overlapped
    {
      background-color: red;
      z-index: 2;
    }

  div#green-overlapping
    {
      background-color: green;
      z-index: calc(3 / 2);
      /*
      should resolve to 'z-index: 2' since "values
      halfway between adjacent integers rounded
      towards positive infinity" and since
      div#green-overlapping is last in document
      tree order, then it should overlap
      div#red-overlapped
      */
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
