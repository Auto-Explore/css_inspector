# css/css-break/break-float-dynamic-001.html

```json
{
  "format_version": 3,
  "file": "css/css-break/break-float-dynamic-001.html"
}
```

## style[0]

```css

  #multicol {
    columns: 2;
    column-fill: auto;
    column-gap: 0;
    height: 400px; /* Must be large enough so that .target doesn't break initially. */
    width: 100px;
    background: red;
  }
  .block {
    width: 50px;
    background: green;
  }
  .target {
    float: left;
    height: 160px; /* Expected to be broken into 2 columns after runTest() */
  }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
